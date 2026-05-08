//! 飞书即时消息功能封装
//! 
//! 提供了发送文本、卡片、图片、文件等消息的功能

use std::collections::HashMap;
use tokio;
use serde_json::Value;
use crate::fsa::base::{FeishuApiBase, request};

/// 即时消息发送类
pub struct IM {
    base: FeishuApiBase,
}

impl IM {
    /// 创建新的IM实例
    pub fn new(app_id: String, app_secret: String) -> Self {
        let base = FeishuApiBase::new(app_id, app_secret);
        IM { base }
    }

    /// 获取用户ID
    pub async fn get_user_id(&mut self, payload: &Value, user_id_type: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!(
            "https://open.feishu.cn/open-apis/contact/v3/users/batch_get_id?user_id_type={}",
            user_id_type
        );
        let headers = self.base.get_headers().await?;
        request("POST", &url, &headers, Some(payload)).await
    }

    /// 上传文件
    pub async fn upload_im_file(&mut self, file_path: &str, file_type: &str) -> Result<String, Box<dyn std::error::Error>> {
        // 注意：实际实现需要使用 multipart/form-data，这里只是占位实现
        // 在真实场景中，需要使用 reqwest 的 multipart 功能
        let url = "https://open.feishu.cn/open-apis/im/v1/files";
        let token = self.base.get_access_token("app_access_token").await?;
        
        let client = reqwest::Client::new();
        let form = reqwest::multipart::Form::new()
            .text("file_name", std::path::Path::new(file_path).file_name()
                .unwrap_or(std::ffi::OsStr::new("unknown"))
                .to_string_lossy().to_string())
            .text("file_type", file_type.to_string());

        let response = client
            .post(url)
            .header("Authorization", format!("Bearer {}", token))
            .multipart(form)
            .send()
            .await?;

        let result: Value = response.json().await?;
        if let Some(data) = result.get("data").and_then(|d| d.as_object()) {
            if let Some(file_key) = data.get("file_key").and_then(|k| k.as_str()) {
                Ok(file_key.to_string())
            } else {
                Err("file_key not found in response".into())
            }
        } else {
            Err("invalid response format".into())
        }
    }

    /// 上传图片
    pub async fn upload_im_image(&mut self, image_path: &str, image_type: &str) -> Result<String, Box<dyn std::error::Error>> {
        // 注意：实际实现需要使用 multipart/form-data，这里只是占位实现
        let url = "https://open.feishu.cn/open-apis/im/v1/images";
        let token = self.base.get_access_token("app_access_token").await?;
        
        let client = reqwest::Client::new();
        let form = reqwest::multipart::Form::new()
            .text("image_type", image_type.to_string());

        let response = client
            .post(url)
            .header("Authorization", format!("Bearer {}", token))
            .multipart(form)
            .send()
            .await?;

        let result: Value = response.json().await?;
        if let Some(data) = result.get("data").and_then(|d| d.as_object()) {
            if let Some(image_key) = data.get("image_key").and_then(|k| k.as_str()) {
                Ok(image_key.to_string())
            } else {
                Err("image_key not found in response".into())
            }
        } else {
            Err("invalid response format".into())
        }
    }

    /// 发送消息
    pub async fn send(&mut self, mut payload: Value, receive_id_type: &str) -> Result<Value, Box<dyn std::error::Error>> {
        if let Some(content) = payload.get("content") {
            if content.is_object() {
                let content_str = serde_json::to_string(content)?;
                payload["content"] = Value::String(content_str);
            }
        }

        let url = format!(
            "https://open.feishu.cn/open-apis/im/v1/messages?receive_id_type={}",
            receive_id_type
        );
        let headers = self.base.get_headers().await?;
        request("POST", &url, &headers, Some(&payload)).await
    }

    /// 发送文本消息
    pub async fn send_text(
        &mut self,
        text: &str,
        receive_id: &str,
        receive_id_type: &str,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let payload = serde_json::json!({
            "receive_id": receive_id,
            "content": {
                "text": text
            },
            "msg_type": "text"
        });
        self.send(payload, receive_id_type).await
    }

    /// 发送卡片消息
    pub async fn send_card(
        &mut self,
        card: &Value,
        receive_id: &str,
        receive_id_type: &str,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let payload = serde_json::json!({
            "receive_id": receive_id,
            "content": card,
            "msg_type": "interactive"
        });
        self.send(payload, receive_id_type).await
    }

    /// 发送图片
    pub async fn send_image(
        &mut self,
        image_path: &str,
        receive_id: &str,
        receive_id_type: &str,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let image_key = self.upload_im_image(image_path, "message").await?;
        let payload = serde_json::json!({
            "receive_id": receive_id,
            "content": {
                "image_key": image_key
            },
            "msg_type": "image"
        });
        self.send(payload, receive_id_type).await
    }

    /// 发送文件
    pub async fn send_file(
        &mut self,
        file_path: &str,
        receive_id: &str,
        receive_id_type: &str,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let file_key = self.upload_im_file(file_path, "stream").await?;
        let payload = serde_json::json!({
            "receive_id": receive_id,
            "content": {
                "file_key": file_key
            },
            "msg_type": "file"
        });
        self.send(payload, receive_id_type).await
    }
}