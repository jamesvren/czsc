//! 飞书API基础封装
//! 
//! 提供了飞书API的基础请求功能和基类

use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use reqwest::header::{HeaderMap, HeaderValue};

/// 飞书API响应结构
#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponse<T> {
    pub code: Option<i32>,
    pub msg: Option<String>,
    pub StatusCode: Option<i32>,
    pub data: Option<T>,
}

/// 飞书API基础类
pub struct FeishuApiBase {
    pub app_id: String,
    pub app_secret: String,
    pub host: String,
    pub headers: HeaderMap,
    pub cache: HashMap<String, serde_json::Value>,
    // pub logger: Box<dyn Logger>, // 暂时省略logger
}

impl FeishuApiBase {
    /// 创建新的飞书API基础实例
    pub fn new(app_id: String, app_secret: String) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        
        FeishuApiBase {
            app_id,
            app_secret,
            host: "https://open.feishu.cn".to_string(),
            headers,
            cache: HashMap::new(),
        }
    }

    /// 获取访问令牌
    pub async fn get_access_token(&mut self, key: &str) -> Result<String, Box<dyn std::error::Error>> {
        if !["app_access_token", "tenant_access_token"].contains(&key) {
            return Err("Invalid key, must be 'app_access_token' or 'tenant_access_token'".into());
        }

        let cache_key = "access_token_data";
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs() as f64;

        let cached_data = self.cache.get(cache_key);
        let need_refresh = match cached_data {
            Some(data) => {
                if let (Some(update_time), Some(expire)) = (
                    data.get("update_time").and_then(|v| v.as_f64()),
                    data.get("expire").and_then(|v| v.as_f64()),
                ) {
                    current_time - update_time > expire * 0.8
                } else {
                    true
                }
            }
            None => true,
        };

        if need_refresh {
            let url = "https://open.feishu.cn/open-apis/auth/v3/app_access_token/internal";
            let payload = serde_json::json!({
                "app_id": &self.app_id,
                "app_secret": &self.app_secret
            });
            
            let client = reqwest::Client::new();
            let response = client
                .post(url)
                .headers(self.headers.clone())
                .json(&payload)
                .send()
                .await?;
                
            let resp: serde_json::Value = response.json().await?;
            
            let mut updated_resp = resp.clone();
            updated_resp["update_time"] = serde_json::Value::Number(((current_time * 1000.0) as u64).into());
            
            self.cache.insert(cache_key.to_string(), updated_resp);
        }

        let cached_data = self.cache.get(cache_key).unwrap();
        if let Some(token) = cached_data.get(key).and_then(|v| v.as_str()) {
            Ok(token.to_string())
        } else {
            Err("Access token not found in response".into())
        }
    }

    /// 获取带认证头的请求头
    pub async fn get_headers(&mut self) -> Result<HeaderMap, Box<dyn std::error::Error>> {
        let mut headers = self.headers.clone();
        let token = self.get_access_token("app_access_token").await?;
        headers.insert("Authorization", HeaderValue::from_str(&format!("Bearer {}", token))?);
        Ok(headers)
    }

    /// 获取飞书云空间根目录 token
    pub async fn get_root_folder_token(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{}/open-apis/drive/explorer/v2/root_folder/meta", self.host);
        let headers = self.get_headers().await?;
        
        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .headers(headers)
            .send()
            .await?;
            
        let resp: serde_json::Value = response.json().await?;
        
        if let Some(data) = resp.get("data").and_then(|d| d.as_object()) {
            if let Some(token) = data.get("token").and_then(|t| t.as_str()) {
                Ok(token.to_string())
            } else {
                Err("Root folder token not found in response".into())
            }
        } else {
            Err("Invalid response format".into())
        }
    }

    /// 删除用户在云空间内的文件或者文件夹
    pub async fn remove(&mut self, token: &str, kind: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("{}/open-apis/drive/v1/files/{}?type={}", self.host, token, kind.to_lowercase());
        let headers = self.get_headers().await?;
        
        let client = reqwest::Client::new();
        let response = client
            .delete(&url)
            .headers(headers)
            .send()
            .await?;
            
        let resp: serde_json::Value = response.json().await?;
        Ok(resp)
    }

    /// 将文件或者文件夹移动到用户云空间的其他位置
    pub async fn move_item(&mut self, token: &str, payload: &serde_json::Value) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("{}/open-apis/drive/v1/files/{}/move", self.host, token);
        let headers = self.get_headers().await?;
        
        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .headers(headers)
            .json(payload)
            .send()
            .await?;
            
        let resp: serde_json::Value = response.json().await?;
        Ok(resp)
    }

    /// 将文件复制到用户云空间的其他文件夹中
    pub async fn copy(&mut self, token: &str, payload: &serde_json::Value) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("{}/open-apis/drive/v1/files/{}/copy", self.host, token);
        let headers = self.get_headers().await?;
        
        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .headers(headers)
            .json(payload)
            .send()
            .await?;
            
        let resp: serde_json::Value = response.json().await?;
        Ok(resp)
    }
}

/// 飞书API标准请求函数
pub async fn request(
    method: &str,
    url: &str,
    headers: &HeaderMap,
    payload: Option<&serde_json::Value>,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut req_builder = match method.to_uppercase().as_str() {
        "GET" => client.get(url),
        "POST" => client.post(url),
        "PUT" => client.put(url),
        "DELETE" => client.delete(url),
        "PATCH" => client.patch(url),
        _ => return Err(format!("Unsupported HTTP method: {}", method).into()),
    };

    // 添加请求头
    req_builder = req_builder.headers(headers.clone());

    // 添加请求体（如果有）
    if let Some(payload) = payload {
        req_builder = req_builder.json(payload);
    }

    let response = req_builder.send().await?;
    
    println!("{}", "+".repeat(88));
    println!("URL: {}", url);
    // 打印响应头信息（简化处理）
    
    let text = response.text().await?;
    println!("Response: {}", text);

    let resp: serde_json::Value = if text.starts_with('{') {
        serde_json::from_str(&text)?
    } else {
        serde_json::json!(text)
    };

    // 检查响应状态
    let code = resp.get("code")
        .and_then(|v| v.as_i64())
        .or_else(|| resp.get("StatusCode").and_then(|v| v.as_i64()))
        .unwrap_or(-1);

    if code != 0 {
        let msg = resp.get("msg").and_then(|v| v.as_str()).unwrap_or("");
        return Err(format!("Request failed: code={}, msg={}", code, msg).into());
    }

    Ok(resp)
}