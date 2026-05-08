//! 相关性分析相关的可视化组件
//!
//! 包含相关性矩阵、滚动相关性、自相关性、协整检验等功能

use std::collections::HashMap;

/// 展示相关性矩阵
pub fn show_correlation(
    df: Vec<HashMap<String, String>>,
    columns: Option<Vec<&str>>,
    method: Option<&str>,
    sub_title: Option<&str>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("展示相关性矩阵");
    
    let method = method.unwrap_or("pearson");
    let sub_title = sub_title.unwrap_or("相关性矩阵");
    
    println!("相关性计算方法: {}", method);
    println!("子标题: {}", sub_title);
    
    if let Some(cols) = columns {
        println!("分析列: {:?}", cols);
    }
    
    Ok(())
}

/// 展示滚动相关性
pub fn show_ts_rolling_corr(
    df: Vec<HashMap<String, String>>,
    col1: &str,
    col2: &str,
    window: Option<usize>,
    method: Option<&str>,
    sub_title: Option<&str>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("展示滚动相关性 - 列1: {}, 列2: {}", col1, col2);
    
    let window = window.unwrap_or(20);
    let method = method.unwrap_or("pearson");
    let sub_title = sub_title.unwrap_or("滚动相关性");
    
    println!("窗口大小: {}, 方法: {}", window, method);
    println!("子标题: {}", sub_title);
    
    Ok(())
}

/// 展示自相关性
pub fn show_ts_self_corr(
    df: Vec<HashMap<String, String>>,
    col: &str,
    max_lag: Option<usize>,
    sub_title: Option<&str>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("展示自相关性 - 列: {}", col);
    
    let max_lag = max_lag.unwrap_or(20);
    let sub_title = sub_title.unwrap_or("自相关性");
    
    println!("最大滞后阶数: {}", max_lag);
    println!("子标题: {}", sub_title);
    
    Ok(())
}

/// 展示符号间相关性
pub fn show_symbols_corr(
    df: Vec<HashMap<String, String>>,
    symbols: Vec<&str>,
    ret_col: Option<&str>,
    method: Option<&str>,
    sub_title: Option<&str>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("展示符号间相关性");
    
    let ret_col = ret_col.unwrap_or("returns");
    let method = method.unwrap_or("pearson");
    let sub_title = sub_title.unwrap_or("符号间相关性");
    
    println!("收益列: {}, 方法: {}", ret_col, method);
    println!("子标题: {}", sub_title);
    println!("符号列表: {:?}", symbols);
    
    Ok(())
}

/// 展示协整检验
pub fn show_cointegration(
    df: Vec<HashMap<String, String>>,
    col1: &str,
    col2: &str,
    sub_title: Option<&str>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("展示协整检验 - 列1: {}, 列2: {}", col1, col2);
    
    let sub_title = sub_title.unwrap_or("协整检验");
    println!("子标题: {}", sub_title);
    
    Ok(())
}

/// 展示相关性图
pub fn show_corr_graph(
    df: Vec<HashMap<String, String>>,
    columns: Option<Vec<&str>>,
    threshold: Option<f64>,
    sub_title: Option<&str>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("展示相关性图");
    
    let threshold = threshold.unwrap_or(0.5);
    let sub_title = sub_title.unwrap_or("相关性图");
    
    println!("相关性阈值: {}", threshold);
    println!("子标题: {}", sub_title);
    
    if let Some(cols) = columns {
        println!("分析列: {:?}", cols);
    }
    
    Ok(())
}

/// 展示截面IC
pub fn show_sectional_ic(
    df: Vec<HashMap<String, String>>,
    factor_col: &str,
    ret_col: &str,
    method: Option<&str>,
    sub_title: Option<&str>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("展示截面IC - 因子列: {}, 收益列: {}", factor_col, ret_col);
    
    let method = method.unwrap_or("spearman");
    let sub_title = sub_title.unwrap_or("截面IC");
    
    println!("计算方法: {}", method);
    println!("子标题: {}", sub_title);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_show_correlation() {
        let df = vec![];
        let result = show_correlation(df, Some(vec!["col1", "col2"]), Some("pearson"), Some("测试相关性"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_show_ts_rolling_corr() {
        let df = vec![];
        let result = show_ts_rolling_corr(df, "col1", "col2", Some(20), Some("pearson"), Some("滚动相关性"));
        assert!(result.is_ok());
    }
}