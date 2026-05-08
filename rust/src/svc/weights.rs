//! 持仓权重分析相关的可视化组件
//!
//! 包含权重时间序列、权重分布、权重累积分布等功能

use std::collections::HashMap;

/// 展示权重时间序列
pub fn show_weight_ts(
    df: Vec<HashMap<String, String>>,
    weight_col: Option<&str>,
    sub_title: Option<&str>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("展示权重时间序列");
    
    let weight_col = weight_col.unwrap_or("weight");
    let sub_title = sub_title.unwrap_or("权重时间序列");
    
    println!("权重列: {}", weight_col);
    println!("子标题: {}", sub_title);
    
    Ok(())
}

/// 展示权重分布
pub fn show_weight_dist(
    df: Vec<HashMap<String, String>>,
    weight_col: Option<&str>,
    bins: Option<usize>,
    sub_title: Option<&str>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("展示权重分布");
    
    let weight_col = weight_col.unwrap_or("weight");
    let bins = bins.unwrap_or(50);
    let sub_title = sub_title.unwrap_or("权重分布");
    
    println!("权重列: {}, 直方图箱数: {}", weight_col, bins);
    println!("子标题: {}", sub_title);
    
    Ok(())
}

/// 展示权重累积分布
pub fn show_weight_cdf(
    df: Vec<HashMap<String, String>>,
    weight_col: Option<&str>,
    sub_title: Option<&str>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("展示权重累积分布");
    
    let weight_col = weight_col.unwrap_or("weight");
    let sub_title = sub_title.unwrap_or("权重累积分布");
    
    println!("权重列: {}", weight_col);
    println!("子标题: {}", sub_title);
    
    Ok(())
}

/// 展示权重绝对值
pub fn show_weight_abs(
    df: Vec<HashMap<String, String>>,
    weight_col: Option<&str>,
    sub_title: Option<&str>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("展示权重绝对值");
    
    let weight_col = weight_col.unwrap_or("weight");
    let sub_title = sub_title.unwrap_or("权重绝对值");
    
    println!("权重列: {}", weight_col);
    println!("子标题: {}", sub_title);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_show_weight_ts() {
        let df = vec![];
        let result = show_weight_ts(df, Some("weight"), Some("测试权重时间序列"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_show_weight_dist() {
        let df = vec![];
        let result = show_weight_dist(df, Some("weight"), Some(50), Some("测试权重分布"));
        assert!(result.is_ok());
    }
}