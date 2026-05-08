//! 策略分析相关的可视化组件
//!
//! 包含策略回测、多策略对比、策略优化等功能

use std::collections::HashMap;

/// 展示策略优化研究
pub fn show_optuna_study(
    study: Option<String>,
    sub_title: Option<&str>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("展示策略优化研究");
    
    let sub_title = sub_title.unwrap_or("Optuna 优化研究");
    println!("子标题: {}", sub_title);
    
    if let Some(study_info) = study {
        println!("研究信息: {}", study_info);
    }
    
    Ok(())
}

/// 展示 CzscTrader
pub fn show_czsc_trader(
    trader: Option<String>,
    sub_title: Option<&str>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("展示 CzscTrader");
    
    let sub_title = sub_title.unwrap_or("CzscTrader 分析");
    println!("子标题: {}", sub_title);
    
    if let Some(trader_info) = trader {
        println!("交易者信息: {}", trader_info);
    }
    
    Ok(())
}

/// 展示最近策略
pub fn show_strategies_recent(
    strategies: Vec<String>,
    sub_title: Option<&str>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("展示最近策略");
    
    let sub_title = sub_title.unwrap_or("最近策略");
    println!("子标题: {}", sub_title);
    println!("策略数量: {}", strategies.len());
    
    Ok(())
}

/// 展示收益贡献
pub fn show_returns_contribution(
    df: Vec<HashMap<String, String>>,
    ret_col: Option<&str>,
    factor_col: Option<&str>,
    sub_title: Option<&str>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("展示收益贡献");
    
    let ret_col = ret_col.unwrap_or("returns");
    let factor_col = factor_col.unwrap_or("factor");
    let sub_title = sub_title.unwrap_or("收益贡献分析");
    
    println!("收益列: {}, 因子列: {}", ret_col, factor_col);
    println!("子标题: {}", sub_title);
    
    Ok(())
}

/// 展示符号基准
pub fn show_symbols_bench(
    df: Vec<HashMap<String, String>>,
    price_col: Option<&str>,
    sub_title: Option<&str>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("展示符号基准");
    
    let price_col = price_col.unwrap_or("price");
    let sub_title = sub_title.unwrap_or("符号基准");
    
    println!("价格列: {}", price_col);
    println!("子标题: {}", sub_title);
    
    Ok(())
}

/// 展示季度效应
pub fn show_quarterly_effect(
    df: Vec<HashMap<String, String>>,
    ret_col: Option<&str>,
    sub_title: Option<&str>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("展示季度效应");
    
    let ret_col = ret_col.unwrap_or("returns");
    let sub_title = sub_title.unwrap_or("季度效应");
    
    println!("收益列: {}", ret_col);
    println!("子标题: {}", sub_title);
    
    Ok(())
}

/// 展示CTA周期分类
pub fn show_cta_periods_classify(
    df: Vec<HashMap<String, String>>,
    period_col: Option<&str>,
    ret_col: Option<&str>,
    sub_title: Option<&str>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("展示CTA周期分类");
    
    let period_col = period_col.unwrap_or("period");
    let ret_col = ret_col.unwrap_or("returns");
    let sub_title = sub_title.unwrap_or("CTA周期分类");
    
    println!("周期列: {}, 收益列: {}", period_col, ret_col);
    println!("子标题: {}", sub_title);
    
    Ok(())
}

/// 展示波动率分类
pub fn show_volatility_classify(
    df: Vec<HashMap<String, String>>,
    vol_col: Option<&str>,
    ret_col: Option<&str>,
    sub_title: Option<&str>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("展示波动率分类");
    
    let vol_col = vol_col.unwrap_or("volatility");
    let ret_col = ret_col.unwrap_or("returns");
    let sub_title = sub_title.unwrap_or("波动率分类");
    
    println!("波动率列: {}, 收益列: {}", vol_col, ret_col);
    println!("子标题: {}", sub_title);
    
    Ok(())
}

/// 展示投资组合
pub fn show_portfolio(
    weights: Vec<(String, f64)>,
    assets: Vec<String>,
    sub_title: Option<&str>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("展示投资组合");
    
    let sub_title = sub_title.unwrap_or("投资组合分析");
    println!("子标题: {}", sub_title);
    println!("资产数量: {}", assets.len());
    println!("权重数量: {}", weights.len());
    
    Ok(())
}

/// 展示换手率
pub fn show_turnover_rate(
    df: Vec<HashMap<String, String>>,
    weight_col: Option<&str>,
    sub_title: Option<&str>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("展示换手率");
    
    let weight_col = weight_col.unwrap_or("weight");
    let sub_title = sub_title.unwrap_or("换手率分析");
    
    println!("权重列: {}", weight_col);
    println!("子标题: {}", sub_title);
    
    Ok(())
}

/// 展示统计对比
pub fn show_stats_compare(
    stats1: HashMap<String, f64>,
    stats2: HashMap<String, f64>,
    sub_title: Option<&str>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("展示统计对比");
    
    let sub_title = sub_title.unwrap_or("统计对比");
    println!("子标题: {}", sub_title);
    println!("第一个统计集合的指标数量: {}", stats1.len());
    println!("第二个统计集合的指标数量: {}", stats2.len());
    
    Ok(())
}

/// 展示符号惩罚
pub fn show_symbol_penalty(
    penalties: HashMap<String, f64>,
    sub_title: Option<&str>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("展示符号惩罚");
    
    let sub_title = sub_title.unwrap_or("符号惩罚");
    println!("子标题: {}", sub_title);
    println!("惩罚项数量: {}", penalties.len());
    
    Ok(())
}

/// 展示多回测
pub fn show_multi_backtest(
    backtests: HashMap<String, String>,
    show_describe: Option<bool>,
    sub_title: Option<&str>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("展示多回测");
    
    let show_describe = show_describe.unwrap_or(true);
    let sub_title = sub_title.unwrap_or("多回测对比");
    
    println!("显示描述: {}", show_describe);
    println!("子标题: {}", sub_title);
    println!("回测数量: {}", backtests.len());
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_show_optuna_study() {
        let result = show_optuna_study(Some("Study Info".to_string()), Some("测试优化"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_show_multi_backtest() {
        let mut backtests = HashMap::new();
        backtests.insert("Strategy1".to_string(), "Details1".to_string());
        let result = show_multi_backtest(backtests, Some(true), Some("测试多回测"));
        assert!(result.is_ok());
    }
}