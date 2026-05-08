use crate::objects::RawBar;
use crate::enums::Freq;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Tushare K线数据转换
/// 
/// 将 Tushare 数据接口返回的K线数据转换为 RawBar 对象列表
pub fn format_kline(kline: Vec<HashMap<String, String>>, freq: Freq) -> Vec<RawBar> {
    let mut bars = Vec::new();
    
    // 根据频率确定时间键
    let dt_key = if format!("{:?}", freq).contains("分钟") { "trade_time" } else { "trade_date" };
    
    // 排序数据
    // Note: 在实际实现中，我们需要对数据进行排序，这里假设输入数据已经排序
    
    for (i, record) in kline.iter().enumerate() {
        let vol = if format!("{:?}", freq).contains("D") {  // 日线数据
            // 日线数据：成交量单位为万股，转换为股
            (record.get("vol").unwrap_or(&"0".to_string()).parse::<f64>().unwrap_or(0.0) * 100.0)
        } else {
            // 分钟线数据：成交量单位为股
            record.get("vol").unwrap_or(&"0".to_string()).parse::<f64>().unwrap_or(0.0)
        };
        
        let amount = if format!("{:?}", freq).contains("D") {  // 日线数据
            // 日线数据：成交额单位为万元，转换为元
            (record.get("amount").unwrap_or(&"0".to_string()).parse::<f64>().unwrap_or(0.0) * 1000.0)
        } else {
            record.get("amount").unwrap_or(&"0".to_string()).parse::<f64>().unwrap_or(0.0)
        };

        // 将每一根K线转换成 RawBar 对象
        let bar = RawBar {
            symbol: record.get("ts_code").unwrap_or(&"".to_string()).clone(),
            dt: parse_datetime(record.get(dt_key).unwrap_or(&"".to_string())),
            id: i as i32,
            freq: freq.clone(),
            open: record.get("open").unwrap_or(&"0".to_string()).parse::<f64>().unwrap_or(0.0),
            close: record.get("close").unwrap_or(&"0".to_string()).parse::<f64>().unwrap_or(0.0),
            high: record.get("high").unwrap_or(&"0".to_string()).parse::<f64>().unwrap_or(0.0),
            low: record.get("low").unwrap_or(&"0".to_string()).parse::<f64>().unwrap_or(0.0),
            vol: record.get("vol").unwrap_or(&"0".to_string()).parse::<f64>().unwrap_or(0.0),
            amount: if format!("{:?}", freq).contains("D") {  // 日线数据
                // 日线数据：成交额单位为万元，转换为元
                (record.get("amount").unwrap_or(&"0".to_string()).parse::<f64>().unwrap_or(0.0) * 1000.0)
            } else {
                record.get("amount").unwrap_or(&"0".to_string()).parse::<f64>().unwrap_or(0.0)
            },
            cache: Default::default(),
        };
        bars.push(bar);
    }
    bars
}

/// 解析日期时间字符串
fn parse_datetime(dt_str: &str) -> DateTime<Utc> {
    // 简单的日期时间解析，实际应用中可能需要更复杂的逻辑
    match dt_str.len() {
        8 => { // YYYYMMDD format
            let year = dt_str[0..4].parse::<i32>().unwrap_or(1970);
            let month = dt_str[4..6].parse::<u32>().unwrap_or(1);
            let day = dt_str[6..8].parse::<u32>().unwrap_or(1);
            chrono::NaiveDate::from_ymd_opt(year, month, day)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_utc()
        }
        15 => { // YYYYMMDD HH:MM:SS format (with extra characters)
            // For trade_time format
            let date_part = &dt_str[0..8];
            let time_part = &dt_str[9..15]; // HH:MM:SS
            
            let year = date_part[0..4].parse::<i32>().unwrap_or(1970);
            let month = date_part[4..6].parse::<u32>().unwrap_or(1);
            let day = date_part[6..8].parse::<u32>().unwrap_or(1);
            
            let hour = time_part[0..2].parse::<u32>().unwrap_or(0);
            let minute = time_part[3..5].parse::<u32>().unwrap_or(0);
            let second = time_part[6..8].parse::<u32>().unwrap_or(0);
            
            chrono::NaiveDate::from_ymd_opt(year, month, day)
                .unwrap()
                .and_hms_opt(hour, minute, second)
                .unwrap()
                .and_utc()
        }
        _ => Utc::now(),
    }
}

/// 获取标的代码
pub fn get_symbols(step: &str) -> Vec<String> {
    let mut stocks_map: HashMap<String, Vec<String>> = HashMap::new();
    
    // 示例数据 - 实际实现中应该从数据源获取
    stocks_map.insert("index".to_string(), vec![
        "000905.SH".to_string(),
        "000016.SH".to_string(),
        "000300.SH".to_string(),
        "000001.SH".to_string(),
        "000852.SH".to_string(),
        "399001.SZ".to_string(),
        "399006.SZ".to_string(),
        "399376.SZ".to_string(),
        "399377.SZ".to_string(),
        "399317.SZ".to_string(),
        "399303.SZ".to_string(),
    ]);
    
    stocks_map.insert("stock".to_string(), vec![
        "000001.SZ".to_string(),
        "000002.SZ".to_string(),
    ]);
    
    stocks_map.insert("etfs".to_string(), vec![
        "512880.SH".to_string(),
        "518880.SH".to_string(),
        "515880.SH".to_string(),
    ]);
    
    let asset_map: HashMap<String, String> = [
        ("index".to_string(), "I".to_string()),
        ("stock".to_string(), "E".to_string()),
        ("check".to_string(), "E".to_string()),
        ("train".to_string(), "E".to_string()),
        ("valid".to_string(), "E".to_string()),
        ("etfs".to_string(), "FD".to_string()),
    ].iter().cloned().collect();

    let mut symbols = Vec::new();
    
    if step.to_lowercase() == "all" {
        for (k, v) in stocks_map.iter() {
            for ts_code in v {
                if let Some(asset) = asset_map.get(k) {
                    symbols.push(format!("{}#{}", ts_code, asset));
                }
            }
        }
    } else if let Some(stock_codes) = stocks_map.get(step) {
        if let Some(asset) = asset_map.get(step) {
            for ts_code in stock_codes {
                symbols.push(format!("{}#{}", ts_code, asset));
            }
        }
    }

    symbols
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_kline() {
        let mut kline_record = HashMap::new();
        kline_record.insert("ts_code".to_string(), "000001.SZ".to_string());
        kline_record.insert("trade_date".to_string(), "20240101".to_string());
        kline_record.insert("open".to_string(), "10.0".to_string());
        kline_record.insert("close".to_string(), "11.0".to_string());
        kline_record.insert("high".to_string(), "12.0".to_string());
        kline_record.insert("low".to_string(), "9.0".to_string());
        kline_record.insert("vol".to_string(), "1000".to_string());
        kline_record.insert("amount".to_string(), "10000".to_string());

        let kline_data = vec![kline_record];
        let freq = Freq::D;
        let bars = format_kline(kline_data, freq);

        assert_eq!(bars.len(), 1);
        assert_eq!(bars[0].symbol, "000001.SZ");
        assert_eq!(bars[0].open, 10.0);
    }
}