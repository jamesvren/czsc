//! 用户输入表单组件
//!
//! 包含各种用户输入表单组件

use std::collections::HashMap;

/// 权重回测表单
pub fn weight_backtest_form(params: HashMap<String, String>) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    println!("显示权重回测表单");
    
    // 在实际实现中，这会显示一个表单供用户输入参数
    // 返回用户输入的参数
    Ok(params)
}

/// 代码编辑器表单
pub fn code_editor_form(
    code: Option<&str>,
    language: Option<&str>,
    label: Option<&str>
) -> Result<String, Box<dyn std::error::Error>> {
    println!("显示代码编辑器表单");
    
    let language = language.unwrap_or("python");
    let label = label.unwrap_or("代码编辑器");
    
    println!("语言: {}, 标签: {}", language, label);
    
    // 在实际实现中，这会显示一个代码编辑器供用户编辑
    let code = code.unwrap_or("");
    Ok(code.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_weight_backtest_form() {
        let mut params = HashMap::new();
        params.insert("param1".to_string(), "value1".to_string());
        let result = weight_backtest_form(params);
        assert!(result.is_ok());
    }

    #[test]
    fn test_code_editor_form() {
        let result = code_editor_form(Some("print('hello')"), Some("python"), Some("测试代码编辑器"));
        assert!(result.is_ok());
    }
}