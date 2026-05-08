use crate::enums::Freq;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// 获取交易品种列表
pub fn get_symbols(exchange: &str) -> HashMap<String, String> {
    // 这里只是一个示例实现，实际应该通过 CCXT 库获取真实数据
    let mut result = HashMap::new();
    
    match exchange {
        "币安期货" => {
            result.insert("BTCUSDT".to_string(), "BTC/USDT".to_string());
            result.insert("ETHUSDT".to_string(), "ETH/USDT".to_string());
            result.insert("ADAUSDT".to_string(), "ADA/USDT".to_string());
        }
        "币安现货" => {
            result.insert("BTCUSDT".to_string(), "BTC/USDT".to_string());
            result.insert("ETHUSDT".to_string(), "ETH/USDT".to_string());
            result.insert("BNBUSDT".to_string(), "BNB/USDT".to_string());
        }
        _ => {
            println!("不支持的交易类型: {}", exchange);
        }
    }
    
    result
}

/// 获取原始K线数据
pub fn get_raw_bars(
    symbol: &str,
    period: &str,
    sdt: &str,
    edt: &str,
    exchange: &str,
) -> Vec<HashMap<String, String>> {
    // 这里是一个模拟实现，实际应通过 CCXT 库获取真实数据
    let mut result = Vec::new();
    
    // 验证时间周期
    let supported_periods = ["1m", "5m", "15m", "30m", "1h", "2h", "4h", "6h", "8h", "12h", "1d"];
    if !supported_periods.contains(&period) {
        panic!("不支持的时间周期: {}", period);
    }
    
    // 模拟一些K线数据
    for i in 0..10 {
        let mut kline = HashMap::new();
        kline.insert("dt".to_string(), format!("2024-01-01T10:0{}:00+08:00", i));
        kline.insert("open".to_string(), (10000.0 + (i as f64) * 10.0).to_string());
        kline.insert("high".to_string(), (10050.0 + (i as f64) * 10.0).to_string());
        kline.insert("low".to_string(), (9950.0 + (i as f64) * 10.0).to_string());
        kline.insert("close".to_string(), (10030.0 + (i as f64) * 10.0).to_string());
        kline.insert("vol".to_string(), (1000.0 + (i as f64) * 100.0).to_string());
        kline.insert("amount".to_string(), (10000000.0 + (i as f64) * 100000.0).to_string());
        kline.insert("symbol".to_string(), symbol.to_string());
        
        result.push(kline);
    }
    
    result
}

/// 获取最新的K线数据
pub fn get_latest_klines(
    symbol: &str,
    period: &str,
    sdt: Option<&str>,
    exchange: &str,
) -> Vec<HashMap<String, String>> {
    let start_date = sdt.unwrap_or("2017-01-01");
    let end_date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    get_raw_bars(symbol, period, start_date, &end_date, exchange)
}

/// 将时间周期字符串转换为 Freq 枚举
pub fn period_to_freq(period: &str) -> Freq {
    match period {
        "1m" => Freq::F1,
        "5m" => Freq::F5,
        "15m" => Freq::F15,
        "30m" => Freq::F30,
        "1h" => Freq::F60,
        "4h" => Freq::F60,
        "1d" => Freq::D,
        "1w" => Freq::W,
        "1M" => Freq::M,
        _ => Freq::F1, // 默认返回分钟级别
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_symbols() {
        let symbols = get_symbols("币安期货");
        assert!(symbols.contains_key("BTCUSDT"));
        assert!(symbols.contains_key("ETHUSDT"));
    }

    #[test]
    fn test_get_raw_bars() {
        let bars = get_raw_bars("BTCUSDT", "1h", "2024-01-01", "2024-01-02", "币安期货");
        assert!(!bars.is_empty());
        assert!(bars[0].contains_key("symbol"));
        assert_eq!(bars[0].get("symbol").unwrap(), "BTCUSDT");
    }

    #[test]
    fn test_period_to_freq() {
        assert_eq!(period_to_freq("1m"), Freq::F1);
        assert_eq!(period_to_freq("1d"), Freq::D);
        assert_eq!(period_to_freq("1w"), Freq::W);
    }
}