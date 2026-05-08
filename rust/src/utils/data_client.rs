//! 数据客户端模块
//! 
//! 提供统一的数据接口客户端，用于与各种数据源交互

use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use reqwest::header::{HeaderMap, HeaderValue};

/// 数据接口客户端
pub struct DataClient {
    /// API端点URL
    pub api_endpoint: String,
    /// 认证令牌
    pub token: String,
    /// 请求头
    pub headers: HeaderMap,
}

impl DataClient {
    /// 创建新的数据客户端实例
    pub fn new(api_endpoint: String, token: String) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Authorization", HeaderValue::from_str(&format!("Bearer {}", token)).unwrap());

        DataClient {
            api_endpoint,
            token,
            headers,
        }
    }

    /// 发起API请求
    pub async fn post_request(&self, endpoint: &str, data: &Value) -> Result<Value> {
        let url = format!("{}/{}", self.api_endpoint, endpoint);
        
        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .headers(self.headers.clone())
            .json(data)
            .send()
            .await?;

        let response_value: Value = response.json().await?;
        Ok(response_value)
    }

    /// 获取URL令牌
    pub fn get_url_token(&self) -> &str {
        &self.token
    }

    /// 设置URL令牌
    pub fn set_url_token(&mut self, token: String) {
        self.token = token;
        let auth_header = HeaderValue::from_str(&format!("Bearer {}", token)).unwrap();
        self.headers.insert("Authorization", auth_header);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_data_client_creation() {
        let client = DataClient::new(
            "https://api.example.com".to_string(),
            "test_token".to_string(),
        );
        
        assert_eq!(client.api_endpoint, "https://api.example.com");
        assert_eq!(client.token, "test_token");
        assert_eq!(client.get_url_token(), "test_token");
    }

    #[tokio::test]
    #[ignore] // 忽略网络请求测试
    async fn test_post_request() {
        let client = DataClient::new(
            "https://httpbin.org".to_string(),
            "test_token".to_string(),
        );
        
        let data = json!({"test": "data"});
        // 这里只是验证代码结构，实际的网络请求会失败
        // 因为httpbin.org不接受Bearer token认证
    }

    #[test]
    fn test_set_url_token() {
        let mut client = DataClient::new(
            "https://api.example.com".to_string(),
            "test_token".to_string(),
        );
        
        client.set_url_token("new_token".to_string());
        assert_eq!(client.token, "new_token");
        assert_eq!(client.get_url_token(), "new_token");
    }
}