//! 飞书电子表格功能封装
//! 
//! 提供了对飞书电子表格的增删改查等操作

use std::collections::HashMap;
use serde_json::Value;
use crate::fsa::base::{FeishuApiBase, request};

/// 电子表格类
pub struct SpreadSheets {
    base: FeishuApiBase,
}

impl SpreadSheets {
    /// 创建新的SpreadSheets实例
    pub fn new(app_id: String, app_secret: String) -> Self {
        let base = FeishuApiBase::new(app_id, app_secret);
        SpreadSheets { base }
    }

    /// 创建电子表格
    pub async fn create(&mut self, folder_token: &str, title: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/open-apis/sheets/v3/spreadsheets", self.base.host);
        let payload = serde_json::json!({
            "title": title,
            "folder_token": folder_token
        });
        let headers = self.base.get_headers().await?;
        request("POST", &url, &headers, Some(&payload)).await
    }

    /// 获取电子表格信息
    pub async fn check(&mut self, token: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/open-apis/sheets/v3/spreadsheets/{}", self.base.host, token);
        let headers = self.base.get_headers().await?;
        request("GET", &url, &headers, None).await
    }

    /// 获取工作表
    pub async fn get_sheets(&mut self, token: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/open-apis/sheets/v3/spreadsheets/{}/sheets/query", self.base.host, token);
        let headers = self.base.get_headers().await?;
        request("GET", &url, &headers, None).await
    }

    /// 查询工作表
    pub async fn get_sheet_meta(&mut self, token: &str, sheet_id: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/open-apis/sheets/v3/spreadsheets/{}/sheets/{}", self.base.host, token, sheet_id);
        let headers = self.base.get_headers().await?;
        request("GET", &url, &headers, None).await
    }

    /// 向多个范围写入数据
    pub async fn update_values(&mut self, token: &str, data: &Value) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/open-apis/sheets/v2/spreadsheets/{}/values_batch_update", self.base.host, token);
        let headers = self.base.get_headers().await?;
        request("POST", &url, &headers, Some(data)).await
    }

    /// 更新样式
    pub async fn update_styles(&mut self, token: &str, data: &Value) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/open-apis/sheets/v2/spreadsheets/{}/styles_batch_update", self.base.host, token);
        let headers = self.base.get_headers().await?;
        request("POST", &url, &headers, Some(data)).await
    }

    /// 获取工作表中的单个数据范围
    pub async fn read_sheet(&mut self, token: &str, sheet_id: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/open-apis/sheets/v2/spreadsheets/{}/values/{}", self.base.host, token, sheet_id);
        let headers = self.base.get_headers().await?;
        request("GET", &url, &headers, None).await
    }

    /// 删除行列，清空数据
    pub async fn delete_values(&mut self, token: &str, sheet_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("https://open.feishu.cn/open-apis/sheets/v2/spreadsheets/{}/dimension_range", token);

        // 删除行
        loop {
            let meta = self.get_sheet_meta(token, sheet_id).await?;
            let row_count = meta["data"]["sheet"]["grid_properties"]["row_count"]
                .as_u64()
                .unwrap_or(0) as u32 - 1;

            if row_count <= 1 {
                break;
            }

            let end_index = std::cmp::min(4001, row_count);
            let data = serde_json::json!({
                "dimension": {
                    "sheetId": sheet_id,
                    "majorDimension": "ROWS",
                    "startIndex": 1,
                    "endIndex": end_index
                }
            });

            let headers = self.base.get_headers().await?;
            request("DELETE", &url, &headers, Some(&data)).await?;
        }

        // 删除列
        let meta = self.get_sheet_meta(token, sheet_id).await?;
        let col_count = meta["data"]["sheet"]["grid_properties"]["column_count"]
            .as_u64()
            .unwrap_or(0) as u32 - 1;

        if col_count > 1 {
            let data = serde_json::json!({
                "dimension": {
                    "sheetId": sheet_id,
                    "majorDimension": "COLUMNS",
                    "startIndex": 1,
                    "endIndex": std::cmp::min(4001, col_count)
                }
            });

            let headers = self.base.get_headers().await?;
            request("DELETE", &url, &headers, Some(&data)).await?;
        }

        Ok(())
    }

    /// 增加行列
    pub async fn dimension_range(&mut self, token: &str, data: &Value) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("https://open.feishu.cn/open-apis/sheets/v2/spreadsheets/{}/dimension_range", token);
        let headers = self.base.get_headers().await?;
        request("POST", &url, &headers, Some(data)).await
    }

    /// 增加工作表，复制工作表、删除工作表
    pub async fn update_sheets(&mut self, token: &str, operates: &Value) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/open-apis/sheets/v2/spreadsheets/{}/sheets_batch_update", self.base.host, token);
        let headers = self.base.get_headers().await?;
        request("POST", &url, &headers, Some(operates)).await
    }

    /// 添加权限成员
    pub async fn add_permissions_member(
        &mut self,
        token: &str,
        doctype: &str,
        member_type: &str,
        member_id: &str,
        perm: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!(
            "{}/open-apis/drive/v1/permissions/{}?type={}&need_notification=false",
            self.base.host, token, doctype
        );
        let payload = serde_json::json!({
            "member_type": member_type,
            "member_id": member_id,
            "perm": perm
        });
        let headers = self.base.get_headers().await?;
        request("POST", &url, &headers, Some(&payload)).await?;
        Ok(())
    }

    /// 读取表格
    pub async fn read_table(&mut self, token: &str, sheet_id: &str) -> Result<Vec<Vec<Value>>, Box<dyn std::error::Error>> {
        let res = self.read_sheet(token, sheet_id).await?;
        if let Some(data) = res.get("data").and_then(|d| d.as_object()) {
            if let Some(value_range) = data.get("valueRange").and_then(|vr| vr.as_object()) {
                if let Some(values) = value_range.get("values").and_then(|v| v.as_array()) {
                    // 将 Vec<Value> 转换为 Vec<Vec<Value>>
                    let mut result = Vec::new();
                    for row in values {
                        if let Some(row_array) = row.as_array() {
                            result.push(row_array.clone());
                        } else {
                            // 如果不是数组，则将其作为一个元素添加到新数组中
                            result.push(vec![row.clone()]);
                        }
                    }
                    return Ok(result);
                }
            }
        }
        Ok(Vec::new())
    }

    /// 获取表格条件格式
    pub async fn get_condition_formats(&mut self, token: &str, sheet_id: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/open-apis/sheets/v2/spreadsheets/{}/condition_formats?sheet_ids={}",
            self.base.host, token, sheet_id
        );
        let headers = self.base.get_headers().await?;
        request("GET", &url, &headers, None).await
    }

    /// 设置表格条件格式
    pub async fn set_condition_formats(&mut self, token: &str, data: &Value) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/open-apis/sheets/v2/spreadsheets/{}/condition_formats/batch_create",
            self.base.host, token
        );
        let headers = self.base.get_headers().await?;
        request("POST", &url, &headers, Some(data)).await
    }

    /// 批量设置表格普通样式
    pub async fn set_styles_batch(&mut self, token: &str, data: &Value) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/open-apis/sheets/v2/spreadsheets/{}/styles_batch_update", self.base.host, token);
        let headers = self.base.get_headers().await?;
        request("PUT", &url, &headers, Some(data)).await
    }
}

/// 单个工作表操作类
pub struct SingleSheet {
    sheets: SpreadSheets,
    token: String,
    sheet_id: String,
}

impl SingleSheet {
    /// 创建新的SingleSheet实例
    pub fn new(app_id: String, app_secret: String, token: String, sheet_id: String) -> Self {
        let sheets = SpreadSheets::new(app_id, app_secret);
        SingleSheet {
            sheets,
            token,
            sheet_id,
        }
    }

    /// 获取电子表格的元数据
    pub async fn get_meta(&mut self) -> Result<Value, Box<dyn std::error::Error>> {
        self.sheets.get_sheet_meta(&self.token, &self.sheet_id).await
    }

    /// 获取电子表格的列名
    pub async fn get_cols(&mut self, n: u32) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let meta = self.get_meta().await?;
        let col_count = meta["data"]["sheet"]["grid_properties"]["column_count"]
            .as_u64()
            .unwrap_or(0) as u32;
        
        let range = format!("{}!A{}:{}{}", self.sheet_id, n, ('A' as u8 + col_count as u8 - 1) as char, n);
        let res = self.sheets.read_sheet(&self.token, &range).await?;
        
        if let Some(data) = res.get("data").and_then(|d| d.as_object()) {
            if let Some(value_range) = data.get("valueRange").and_then(|vr| vr.as_object()) {
                if let Some(values) = value_range.get("values").and_then(|v| v.as_array()) {
                    if let Some(first_row) = values.first() {
                        let mut cols = Vec::new();
                        if let Some(row_array) = first_row.as_array() {
                            for val in row_array {
                                if let Some(str_val) = val.as_str() {
                                    cols.push(str_val.to_string());
                                } else {
                                    cols.push(val.to_string());
                                }
                            }
                        }
                        return Ok(cols);
                    }
                }
            }
        }
        Ok(Vec::new())
    }

    /// 读取整个电子表格的数据
    pub async fn read_table(&mut self) -> Result<Vec<Vec<Value>>, Box<dyn std::error::Error>> {
        self.sheets.read_table(&self.token, &self.sheet_id).await
    }
}