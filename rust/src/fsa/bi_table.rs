//! 飞书多维表格功能封装
//! 
//! 提供了对飞书多维表格的增删改查等操作

use std::collections::HashMap;
use serde_json::Value;
use crate::fsa::base::{FeishuApiBase, request};

/// 多维表格类
pub struct BiTable {
    base: FeishuApiBase,
    pub app_token: String,
}

impl BiTable {
    /// 创建新的BiTable实例
    pub fn new(app_id: String, app_secret: String, app_token: String) -> Self {
        let base = FeishuApiBase::new(app_id, app_secret);
        BiTable { base, app_token }
    }

    /// 根据 record_id 的值检索现有记录
    pub async fn one_record(&mut self, table_id: &str, record_id: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/records/{}",
            self.base.host, self.app_token, table_id, record_id
        );
        let headers = self.base.get_headers().await?;
        request("GET", &url, &headers, None).await
    }

    /// 列出数据表中的记录
    pub async fn list_records(&mut self, table_id: &str, mut kwargs: HashMap<String, String>) -> Result<Value, Box<dyn std::error::Error>> {
        if !kwargs.contains_key("page_size") {
            kwargs.insert("page_size".to_string(), "500".to_string());
        }
        if !kwargs.contains_key("page_token") {
            kwargs.insert("page_token".to_string(), "".to_string());
        }

        let query_params: Vec<String> = kwargs
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();
        let query_string = query_params.join("&");

        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/records?{}",
            self.base.host, self.app_token, table_id, query_string
        );
        let headers = self.base.get_headers().await?;
        request("GET", &url, &headers, None).await
    }

    // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
    // 数据表相关api
    // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

    /// 新增一个仅包含索引列的空数据表
    pub async fn create_table(
        &mut self,
        name: Option<&str>,
        default_view_name: Option<&str>,
        fields: Option<&Value>,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let mut params = serde_json::Map::new();
        if let Some(n) = name {
            params.insert("name".to_string(), Value::String(n.to_string()));
        }
        if let Some(dvn) = default_view_name {
            params.insert("default_view_name".to_string(), Value::String(dvn.to_string()));
        }
        if let Some(f) = fields {
            params.insert("fields".to_string(), f.clone());
        }

        let url = format!("{}/open-apis/bitable/v1/apps/{}/tables", self.base.host, self.app_token);
        let payload = serde_json::json!({"table": params});
        let headers = self.base.get_headers().await?;
        request("POST", &url, &headers, Some(&payload)).await
    }

    /// 新增多维表格数据表
    pub async fn batch_create_table(&mut self, names: Option<Vec<&str>>) -> Result<Value, Box<dyn std::error::Error>> {
        let mut params = Vec::new();
        if let Some(name_list) = names {
            for name in name_list {
                params.push(serde_json::json!({"name": name}));
            }
        }

        let url = format!("{}/open-apis/bitable/v1/apps/{}/tables/batch_create", self.base.host, self.app_token);
        let payload = serde_json::json!({"tables": params});
        let headers = self.base.get_headers().await?;
        request("POST", &url, &headers, Some(&payload)).await
    }

    /// 删除一个数据表
    pub async fn delete_table(&mut self, table_id: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/open-apis/bitable/v1/apps/{}/tables/{}", self.base.host, self.app_token, table_id);
        let headers = self.base.get_headers().await?;
        request("DELETE", &url, &headers, None).await
    }

    /// 删除多个数据表
    pub async fn batch_delete_table(&mut self, table_ids: Option<Vec<&str>>) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/open-apis/bitable/v1/apps/{}/tables/batch_delete", self.base.host, self.app_token);
        let payload = if let Some(ids) = table_ids {
            serde_json::json!({"table_ids": ids})
        } else {
            serde_json::json!({"table_ids": []})
        };
        let headers = self.base.get_headers().await?;
        request("POST", &url, &headers, Some(&payload)).await
    }

    /// 更新数据表的基本信息
    pub async fn patch_table(&mut self, table_id: &str, name: Option<&str>) -> Result<Value, Box<dyn std::error::Error>> {
        let mut params = serde_json::Map::new();
        if let Some(n) = name {
            params.insert("name".to_string(), Value::String(n.to_string()));
        }

        let url = format!("{}/open-apis/bitable/v1/apps/{}/tables/{}", self.base.host, self.app_token, table_id);
        let headers = self.base.get_headers().await?;
        request("PATCH", &url, &headers, Some(&Value::Object(params))).await
    }

    /// 获取多维表格下的所有数据表
    pub async fn list_tables(&mut self, page_token: Option<&str>, page_size: u32) -> Result<Value, Box<dyn std::error::Error>> {
        let mut url = format!("{}/open-apis/bitable/v1/apps/{}/tables?page_size={}", self.base.host, self.app_token, page_size);
        if let Some(pt) = page_token {
            url.push_str(&format!("&page_token={}", pt));
        }
        let headers = self.base.get_headers().await?;
        request("GET", &url, &headers, None).await
    }

    // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
    // 记录相关api
    // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

    /// 获取记录
    pub async fn table_record_get(
        &mut self,
        table_id: &str,
        record_id: &str,
        text_field_as_array: Option<bool>,
        user_id_type: Option<&str>,
        display_formula_ref: Option<bool>,
        with_shared_url: Option<bool>,
        automatic_fields: Option<bool>,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let mut url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/records/{}?1=1",
            self.base.host, self.app_token, table_id, record_id
        );

        if let Some(tf) = text_field_as_array {
            url.push_str(&format!("&text_field_as_array={}", tf));
        }
        if let Some(uid) = user_id_type {
            url.push_str(&format!("&user_id_type={}", uid));
        }
        if let Some(dfr) = display_formula_ref {
            url.push_str(&format!("&display_formula_ref={}", dfr));
        }
        if let Some(wsurl) = with_shared_url {
            url.push_str(&format!("&with_shared_url={}", wsurl));
        }
        if let Some(af) = automatic_fields {
            url.push_str(&format!("&automatic_fields={}", af));
        }

        let headers = self.base.get_headers().await?;
        request("GET", &url, &headers, None).await
    }

    /// 搜索记录
    pub async fn table_record_search(
        &mut self,
        table_id: &str,
        user_id_type: Option<&str>,
        page_token: Option<&str>,
        page_size: u32,
        view_id: Option<&str>,
        field_names: Option<Vec<&str>>,
        sort: Option<&Value>,
        filter: Option<&Value>,
        automatic_fields: Option<bool>,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let mut url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/records/search?page_size={}",
            self.base.host, self.app_token, table_id, page_size
        );

        if let Some(uid) = user_id_type {
            url.push_str(&format!("&user_id_type={}", uid));
        }
        if let Some(pt) = page_token {
            url.push_str(&format!("&page_token={}", pt));
        }

        let mut params = serde_json::Map::new();
        if let Some(v_id) = view_id {
            params.insert("view_id".to_string(), Value::String(v_id.to_string()));
        }
        if let Some(f_names) = field_names {
            params.insert("field_names".to_string(), Value::Array(f_names.iter().map(|s| Value::String(s.to_string())).collect()));
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s.clone());
        }
        if let Some(f) = filter {
            params.insert("filter".to_string(), f.clone());
        }
        if let Some(af) = automatic_fields {
            params.insert("automatic_fields".to_string(), Value::Bool(af));
        }

        let headers = self.base.get_headers().await?;
        request("POST", &url, &headers, Some(&Value::Object(params))).await
    }

    /// 创建记录
    pub async fn table_record_create(
        &mut self,
        table_id: &str,
        fields: &Value,
        user_id_type: Option<&str>,
        client_token: Option<&str>,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let mut url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/records?1=1",
            self.base.host, self.app_token, table_id
        );

        if let Some(uid) = user_id_type {
            url.push_str(&format!("&user_id_type={}", uid));
        }
        if let Some(ct) = client_token {
            url.push_str(&format!("&client_token={}", ct));
        }

        let payload = serde_json::json!({"fields": fields});
        let headers = self.base.get_headers().await?;
        request("POST", &url, &headers, Some(&payload)).await
    }

    /// 更新记录
    pub async fn table_record_update(
        &mut self,
        table_id: &str,
        record_id: &str,
        fields: &Value,
        user_id_type: Option<&str>,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let mut url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/records/{}/?1=1",
            self.base.host, self.app_token, table_id, record_id
        );

        if let Some(uid) = user_id_type {
            url.push_str(&format!("&user_id_type={}", uid));
        }

        let payload = serde_json::json!({"fields": fields});
        let headers = self.base.get_headers().await?;
        request("PUT", &url, &headers, Some(&payload)).await
    }

    /// 删除记录
    pub async fn table_record_delete(&mut self, table_id: &str, record_id: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/records/{}",
            self.base.host, self.app_token, table_id, record_id
        );
        let headers = self.base.get_headers().await?;
        request("DELETE", &url, &headers, None).await
    }

    /// 批量创建记录
    pub async fn table_record_batch_create(
        &mut self,
        table_id: &str,
        records: &Value,
        user_id_type: Option<&str>,
        client_token: Option<&str>,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let mut url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_create?1=1",
            self.base.host, self.app_token, table_id
        );

        if let Some(uid) = user_id_type {
            url.push_str(&format!("&user_id_type={}", uid));
        }
        if let Some(ct) = client_token {
            url.push_str(&format!("&client_token={}", ct));
        }

        let payload = serde_json::json!({"records": records});
        let headers = self.base.get_headers().await?;
        request("POST", &url, &headers, Some(&payload)).await
    }

    /// 批量更新记录
    pub async fn table_record_batch_update(
        &mut self,
        table_id: &str,
        records: &Value,
        user_id_type: Option<&str>,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let mut url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_update?1=1",
            self.base.host, self.app_token, table_id
        );

        if let Some(uid) = user_id_type {
            url.push_str(&format!("&user_id_type={}", uid));
        }

        let payload = serde_json::json!({"records": records});
        let headers = self.base.get_headers().await?;
        request("POST", &url, &headers, Some(&payload)).await
    }

    /// 批量删除记录
    pub async fn table_record_batch_delete(
        &mut self,
        table_id: &str,
        record_ids: Vec<&str>,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_delete",
            self.base.host, self.app_token, table_id
        );
        let payload = serde_json::json!({"records": Value::Array(record_ids.iter().map(|s| Value::String(s.to_string())).collect())});
        let headers = self.base.get_headers().await?;
        request("POST", &url, &headers, Some(&payload)).await
    }

    // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
    // 视图相关api
    // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

    /// 增量修改视图信息
    pub async fn table_view_patch(
        &mut self,
        table_id: &str,
        view_id: &str,
        infos: &Value,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/views/{}",
            self.base.host, self.app_token, table_id, view_id
        );
        let headers = self.base.get_headers().await?;
        request("PATCH", &url, &headers, Some(infos)).await
    }

    /// 根据 view_id 检索现有视图
    pub async fn table_view_get(
        &mut self,
        table_id: &str,
        view_id: &str,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/views/{}",
            self.base.host, self.app_token, table_id, view_id
        );
        let headers = self.base.get_headers().await?;
        request("GET", &url, &headers, None).await
    }

    /// 获取数据表的所有视图
    pub async fn table_view_list(
        &mut self,
        table_id: &str,
        page_size: u32,
        user_id_type: Option<&str>,
        page_token: Option<&str>,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let mut url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/views?page_size={}",
            self.base.host, self.app_token, table_id, page_size
        );

        if let Some(uid) = user_id_type {
            url.push_str(&format!("&user_id_type={}", uid));
        }
        if let Some(pt) = page_token {
            url.push_str(&format!("&page_token={}", pt));
        }

        let headers = self.base.get_headers().await?;
        request("GET", &url, &headers, None).await
    }

    /// 在数据表中新增一个视图
    pub async fn table_view_create(
        &mut self,
        table_id: &str,
        view_name: &str,
        view_type: &str,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/views",
            self.base.host, self.app_token, table_id
        );
        let payload = serde_json::json!({
            "view_name": view_name,
            "view_type": view_type
        });
        let headers = self.base.get_headers().await?;
        request("POST", &url, &headers, Some(&payload)).await
    }

    /// 删除视图
    pub async fn table_view_delete(
        &mut self,
        table_id: &str,
        view_id: &str,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/views/{}",
            self.base.host, self.app_token, table_id, view_id
        );
        let headers = self.base.get_headers().await?;
        request("DELETE", &url, &headers, None).await
    }

    // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
    // 字段相关api
    // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

    /// 获取数据表的所有字段
    pub async fn table_field_list(
        &mut self,
        table_id: &str,
        page_size: u32,
        view_id: Option<&str>,
        text_field_as_array: Option<bool>,
        page_token: Option<&str>,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let mut url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/fields?page_size={}",
            self.base.host, self.app_token, table_id, page_size
        );

        if let Some(v_id) = view_id {
            url.push_str(&format!("&view_id={}", v_id));
        }
        if let Some(tf) = text_field_as_array {
            url.push_str(&format!("&text_field_as_array={}", tf));
        }
        if let Some(pt) = page_token {
            url.push_str(&format!("&page_token={}", pt));
        }

        let headers = self.base.get_headers().await?;
        request("GET", &url, &headers, None).await
    }

    /// 在数据表中新增一个字段
    pub async fn table_field_create(
        &mut self,
        table_id: &str,
        field_name: &str,
        field_type: i32,
        property: Option<&Value>,
        description: Option<&str>,
        ui_type: Option<&str>,
        client_token: Option<&str>,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let mut url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/fields?1=1",
            self.base.host, self.app_token, table_id
        );

        if let Some(ct) = client_token {
            url.push_str(&format!("&client_token={}", ct));
        }

        let mut params = serde_json::Map::new();
        params.insert("field_name".to_string(), Value::String(field_name.to_string()));
        params.insert("type".to_string(), Value::Number(field_type.into()));

        if let Some(prop) = property {
            params.insert("property".to_string(), prop.clone());
        }
        if let Some(desc) = description {
            params.insert("description".to_string(), Value::String(desc.to_string()));
        }
        if let Some(ui_t) = ui_type {
            params.insert("ui_type".to_string(), Value::String(ui_t.to_string()));
        }

        let headers = self.base.get_headers().await?;
        request("POST", &url, &headers, Some(&Value::Object(params))).await
    }

    /// 更新数据表中的一个字段
    pub async fn table_field_update(
        &mut self,
        table_id: &str,
        field_id: &str,
        field_name: &str,
        field_type: i32,
        property: Option<&Value>,
        description: Option<&str>,
        ui_type: Option<&str>,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/fields/{}",
            self.base.host, self.app_token, table_id, field_id
        );

        let mut params = serde_json::Map::new();
        params.insert("field_name".to_string(), Value::String(field_name.to_string()));
        params.insert("type".to_string(), Value::Number(field_type.into()));

        if let Some(prop) = property {
            params.insert("property".to_string(), prop.clone());
        }
        if let Some(desc) = description {
            params.insert("description".to_string(), Value::String(desc.to_string()));
        }
        if let Some(ui_t) = ui_type {
            params.insert("ui_type".to_string(), Value::String(ui_t.to_string()));
        }

        let headers = self.base.get_headers().await?;
        request("PUT", &url, &headers, Some(&Value::Object(params))).await
    }

    /// 删除数据表中的一个字段
    pub async fn table_field_delete(
        &mut self,
        table_id: &str,
        field_id: &str,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/fields/{}",
            self.base.host, self.app_token, table_id, field_id
        );
        let headers = self.base.get_headers().await?;
        request("DELETE", &url, &headers, None).await
    }

    // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
    // 多维表格相关api
    // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

    /// 复制一个多维表格
    pub async fn table_copy(
        &mut self,
        app_token: Option<&str>,
        name: Option<&str>,
        folder_token: Option<&str>,
        without_content: Option<bool>,
        time_zone: Option<&str>,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let token = app_token.unwrap_or(&self.app_token);
        let url = format!("{}/open-apis/bitable/v1/apps/{}/copy", self.base.host, token);

        let mut params = serde_json::Map::new();
        if let Some(n) = name {
            params.insert("name".to_string(), Value::String(n.to_string()));
        }
        if let Some(ft) = folder_token {
            params.insert("folder_token".to_string(), Value::String(ft.to_string()));
        }
        if let Some(wc) = without_content {
            params.insert("without_content".to_string(), Value::Bool(wc));
        }
        if let Some(tz) = time_zone {
            params.insert("time_zone".to_string(), Value::String(tz.to_string()));
        }

        let headers = self.base.get_headers().await?;
        request("POST", &url, &headers, Some(&Value::Object(params))).await
    }

    /// 创建多维表格
    pub async fn table_create(
        &mut self,
        name: Option<&str>,
        folder_token: Option<&str>,
        time_zone: Option<&str>,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/open-apis/bitable/v1/apps", self.base.host);

        let mut params = serde_json::Map::new();
        if let Some(n) = name {
            params.insert("name".to_string(), Value::String(n.to_string()));
        }
        if let Some(ft) = folder_token {
            params.insert("folder_token".to_string(), Value::String(ft.to_string()));
        }
        if let Some(tz) = time_zone {
            params.insert("time_zone".to_string(), Value::String(tz.to_string()));
        }

        let headers = self.base.get_headers().await?;
        request("POST", &url, &headers, Some(&Value::Object(params))).await
    }

    /// 获取多维表格信息
    pub async fn table_get(&mut self, app_token: Option<&str>) -> Result<Value, Box<dyn std::error::Error>> {
        let token = app_token.unwrap_or(&self.app_token);
        let url = format!("{}/open-apis/bitable/v1/apps/{}", self.base.host, token);
        let headers = self.base.get_headers().await?;
        request("GET", &url, &headers, None).await
    }

    /// 更新多维表格元数据
    pub async fn table_update(
        &mut self,
        app_token: Option<&str>,
        name: Option<&str>,
        is_advanced: Option<bool>,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let token = app_token.unwrap_or(&self.app_token);
        let url = format!("{}/open-apis/bitable/v1/apps/{}", self.base.host, token);

        let mut params = serde_json::Map::new();
        if let Some(n) = name {
            params.insert("name".to_string(), Value::String(n.to_string()));
        }
        if let Some(ia) = is_advanced {
            params.insert("is_advanced".to_string(), Value::Bool(ia));
        }

        let headers = self.base.get_headers().await?;
        request("PUT", &url, &headers, Some(&Value::Object(params))).await
    }
}