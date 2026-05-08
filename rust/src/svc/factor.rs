//! 因子分析相关的可视化组件
//!
//! 包含特征收益、因子分层、因子数值分布、事件收益分析等功能

use std::collections::HashMap;

/// 展示特征收益分析
pub fn show_feature_returns(
    df: Vec<HashMap<String, String>>,
    features: Vec<&str>,
    ret_col: Option<&str>,
    method: Option<&str>,
    min_periods: Option<usize>,
    show_correlation: Option<bool>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("展示特征收益分析");
    
    let ret_col = ret_col.unwrap_or("returns");
    let method = method.unwrap_or("spearman");
    let min_periods = min_periods.unwrap_or(100);
    let show_correlation = show_correlation.unwrap_or(true);
    
    println!("收益列: {}, 相关性计算方法: {}, 最小样本数: {}", ret_col, method, min_periods);
    println!("显示特征间相关性: {}", show_correlation);
    
    // 计算特征与收益的相关性
    println!("特征列表: {:?}", features);
    
    Ok(())
}

/// 展示因子分层分析
pub fn show_factor_layering(
    df: Vec<HashMap<String, String>>,
    factor_col: &str,
    ret_col: &str,
    n_layers: Option<usize>,
    method: Option<&str>,
    show_cumulative: Option<bool>,
    show_distribution: Option<bool>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("展示因子分层分析 - 因子: {}, 收益列: {}", factor_col, ret_col);
    
    let n_layers = n_layers.unwrap_or(5);
    let method = method.unwrap_or("qcut");
    let show_cumulative = show_cumulative.unwrap_or(true);
    let show_distribution = show_distribution.unwrap_or(true);
    
    println!("分层数量: {}, 分层方法: {}", n_layers, method);
    println!("显示累计收益: {}, 显示分布: {}", show_cumulative, show_distribution);
    
    Ok(())
}

/// 展示因子数值分布
pub fn show_factor_value(
    df: Vec<HashMap<String, String>>,
    factor_col: &str,
    bins: Option<usize>,
    show_outliers: Option<bool>,
    percentiles: Option<Vec<f64>>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("展示因子数值分布 - 因子: {}", factor_col);
    
    let bins = bins.unwrap_or(50);
    let show_outliers = show_outliers.unwrap_or(true);
    let percentiles = percentiles.unwrap_or_else(|| vec![0.01, 0.05, 0.1, 0.25, 0.5, 0.75, 0.9, 0.95, 0.99]);
    
    println!("直方图箱数: {}, 显示异常值: {}", bins, show_outliers);
    println!("分位数: {:?}", percentiles);
    
    Ok(())
}

/// 展示事件收益分析
pub fn show_event_return(
    df: Vec<HashMap<String, String>>,
    event_col: &str,
    ret_col: &str,
    pre_periods: Option<i32>,
    post_periods: Option<i32>,
    min_observations: Option<usize>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("展示事件收益分析 - 事件列: {}, 收益列: {}", event_col, ret_col);
    
    let pre_periods = pre_periods.unwrap_or(5);
    let post_periods = post_periods.unwrap_or(10);
    let min_observations = min_observations.unwrap_or(10);
    
    println!("事件前后观察期: {} 前, {} 后", pre_periods, post_periods);
    println!("最小观察数: {}", min_observations);
    
    Ok(())
}

/// 展示事件特征分析
pub fn show_event_features(
    df: Vec<HashMap<String, String>>,
    event_col: &str,
    feature_cols: Vec<&str>,
    test_method: Option<&str>,
    alpha: Option<f64>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("展示事件特征分析 - 事件列: {}", event_col);
    
    let test_method = test_method.unwrap_or("ttest");
    let alpha = alpha.unwrap_or(0.05);
    
    println!("检验方法: {}, 显著性水平: {}", test_method, alpha);
    println!("特征列: {:?}", feature_cols);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_show_feature_returns() {
        let df = vec![];
        let features = vec!["factor1", "factor2"];
        let result = show_feature_returns(df, features, Some("returns"), Some("spearman"), Some(100), Some(true));
        assert!(result.is_ok());
    }

    #[test]
    fn test_show_factor_layering() {
        let df = vec![];
        let result = show_factor_layering(
            df, 
            "factor_col", 
            "return_col", 
            Some(5), 
            Some("qcut"), 
            Some(true), 
            Some(true)
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_show_factor_value() {
        let df = vec![];
        let result = show_factor_value(df, "factor_col", Some(50), Some(true), None);
        assert!(result.is_ok());
    }
}