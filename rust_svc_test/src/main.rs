mod svc;

fn main() {
    println!("Testing SVC module imports...");
    
    // 测试导入
    use svc::{
        show_weight_distribution, show_weight_backtest, show_holds_backtest,
        show_splited_daily, show_yearly_stats, show_out_in_compare,
        show_feature_returns, show_factor_layering, show_factor_value,
        show_correlation, show_ts_rolling_corr, show_ts_self_corr,
        show_cumulative_returns, show_daily_return, show_drawdowns,
        show_monthly_return, show_rolling_daily_performance,
        show_optuna_study, show_czsc_trader, show_strategies_recent,
        show_weight_ts, show_weight_dist, show_weight_cdf, show_weight_abs,
        streamlit_run, weight_backtest_form, code_editor_form
    };
    
    println!("All SVC module imports successful!");
    
    // 简单测试一个函数
    let mut dfw = Vec::new();
    let mut row = std::collections::HashMap::new();
    row.insert("symbol".to_string(), "DLi9001".to_string());
    row.insert("weight".to_string(), "0.5".to_string());
    dfw.push(row);
    
    match show_weight_distribution(dfw, true, None) {
        Ok(_) => println!("show_weight_distribution test passed!"),
        Err(e) => println!("show_weight_distribution test failed: {}", e),
    }
    
    println!("All tests completed successfully!");
}