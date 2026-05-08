//! 收益相关的可视化组件
//!
//! 包含累计收益、日收益分析、回撤分析、月度收益等功能

use std::collections::HashMap;

/// 展示累计收益
pub fn show_cumulative_returns(
    df: Vec<HashMap<String, String>>,
    ret_col: Option<&str>,
    legend_only_cols: Option<Vec<&str>>,
    yearly_days: Option<i32>,
    sub_title: Option<&str>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("展示累计收益");
    
    let ret_col = ret_col.unwrap_or("returns");
    let yearly_days = yearly_days.unwrap_or(252);
    let sub_title = sub_title.unwrap_or("累计收益");
    
    println!("收益列: {}, 年化交易日数: {}", ret_col, yearly_days);
    println!("子标题: {}", sub_title);
    
    if let Some(cols) = legend_only_cols {
        println!("图例列: {:?}", cols);
    }
    
    Ok(())
}

/// 展示日收益
pub fn show_daily_return(
    df: Vec<HashMap<String, String>>,
    ret_col: Option<&str>,
    legend_only_cols: Option<Vec<&str>>,
    yearly_days: Option<i32>,
    stat_hold_days: Option<bool>,
    sub_title: Option<&str>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("展示日收益");
    
    let ret_col = ret_col.unwrap_or("returns");
    let yearly_days = yearly_days.unwrap_or(252);
    let stat_hold_days = stat_hold_days.unwrap_or(true);
    let sub_title = sub_title.unwrap_or("日收益");
    
    println!("收益列: {}, 年化交易日数: {}, 统计持仓天数: {}", ret_col, yearly_days, stat_hold_days);
    println!("子标题: {}", sub_title);
    
    Ok(())
}

/// 展示回撤分析
pub fn show_drawdowns(
    df: Vec<HashMap<String, String>>,
    ret_col: Option<&str>,
    sub_title: Option<&str>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("展示回撤分析");
    
    let ret_col = ret_col.unwrap_or("returns");
    let sub_title = sub_title.unwrap_or("最大回撤分析");
    
    println!("收益列: {}", ret_col);
    println!("子标题: {}", sub_title);
    
    Ok(())
}

/// 展示月度收益
pub fn show_monthly_return(
    df: Vec<HashMap<String, String>>,
    ret_col: Option<&str>,
    sub_title: Option<&str>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("展示月度收益");
    
    let ret_col = ret_col.unwrap_or("returns");
    let sub_title = sub_title.unwrap_or("月度累计收益");
    
    println!("收益列: {}", ret_col);
    println!("子标题: {}", sub_title);
    
    Ok(())
}

/// 展示滚动日收益表现
pub fn show_rolling_daily_performance(
    df: Vec<HashMap<String, String>>,
    ret_col: Option<&str>,
    window: Option<usize>,
    yearly_days: Option<i32>,
    sub_title: Option<&str>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("展示滚动日收益表现");
    
    let ret_col = ret_col.unwrap_or("returns");
    let window = window.unwrap_or(252);
    let yearly_days = yearly_days.unwrap_or(252);
    let sub_title = sub_title.unwrap_or("滚动日收益表现");
    
    println!("收益列: {}, 窗口大小: {}, 年化交易日数: {}", ret_col, window, yearly_days);
    println!("子标题: {}", sub_title);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_show_cumulative_returns() {
        let df = vec![];
        let result = show_cumulative_returns(
            df, 
            Some("returns"), 
            Some(vec!["strategy1", "strategy2"]), 
            Some(252), 
            Some("测试累计收益")
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_show_daily_return() {
        let df = vec![];
        let result = show_daily_return(
            df, 
            Some("returns"), 
            None, 
            Some(252), 
            Some(true), 
            Some("测试日收益")
        );
        assert!(result.is_ok());
    }
}