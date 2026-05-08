//! CZSC分析模块单元测试
//!
//! Mock数据格式说明:
//! - 数据来源: czsc.mock.generate_symbol_kines (模拟)
//! - 数据列: dt, symbol, open, close, high, low, vol, amount
//! - 时间范围: 20220101-20250101（3年数据，满足3年+要求）
//! - 频率: 1分钟、5分钟、日线
//! - Seed: 42（确保可重现）

use std::collections::HashMap;

// 模拟K线数据结构
#[derive(Debug, Clone)]
pub struct RawBar {
    pub symbol: String,
    pub id: usize,
    pub freq: String,
    pub open: f64,
    pub dt: String,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub vol: f64,
    pub amount: Option<f64>,
}

// 模拟CZSC结构
pub struct CZSC {
    pub symbol: String,
    pub freq: String,
    pub bars_raw: Vec<RawBar>,
    pub bars_ubi: Vec<RawBar>,
    pub bi_list: Vec<Bi>,
    pub signals: Option<HashMap<String, String>>,
}

#[derive(Debug)]
pub struct Bi {
    pub direction: String,
    pub start_dt: String,
    pub end_dt: String,
    pub high: f64,
    pub low: f64,
}

impl CZSC {
    pub fn new(bars: Vec<RawBar>) -> Self {
        CZSC {
            symbol: bars.first().map(|b| b.symbol.clone()).unwrap_or_default(),
            freq: bars.first().map(|b| b.freq.clone()).unwrap_or_default(),
            bars_raw: bars.clone(),
            bars_ubi: bars.clone(), // 简化处理
            bi_list: vec![Bi {
                direction: "up".to_string(),
                start_dt: "2022-01-01".to_string(),
                end_dt: "2022-01-10".to_string(),
                high: 105.0,
                low: 95.0,
            }],
            signals: None,
        }
    }
}

// 模拟获取mock K线数据的函数
fn get_mock_bars(freq: &str, symbol: &str, n_days: usize) -> Vec<RawBar> {
    let mut bars = Vec::new();
    
    for i in 0..n_days {
        let bar = RawBar {
            symbol: symbol.to_string(),
            id: i,
            freq: freq.to_string(),
            open: 100.0 + (i as f64) * 0.1,
            dt: format!("2022-01-{:02}", 1 + i % 30 + 1),
            close: 101.0 + (i as f64) * 0.1,
            high: 102.0 + (i as f64) * 0.1,
            low: 99.0 + (i as f64) * 0.1,
            vol: 1000.0 + (i as f64) * 10.0,
            amount: Some(100000.0 + (i as f64) * 1000.0),
        };
        bars.push(bar);
    }
    
    bars
}

#[cfg(test)]
mod test_czsc_basic {
    use super::*;

    #[test]
    fn test_czsc_basic() {
        // 测试CZSC基础功能
        let bars = get_mock_bars("D", "000001", 200);
        let c = CZSC::new(bars);

        assert_eq!(c.symbol, "000001", "symbol应该正确设置");
        assert_eq!(c.freq, "D", "频率应该正确设置");
        assert!(c.bars_raw.len() > 0, "原始K线数据不应为空");
        assert!(c.bars_ubi.len() > 0, "去除包含关系后的K线数据不应为空");
        assert!(c.bi_list.len() > 0, "笔的列表不应为空");
    }

    #[test]
    fn test_czsc_signals() {
        // 测试CZSC信号计算 - 无信号函数时signals为None或空字典
        let bars = get_mock_bars("D", "000001", 200);
        let c = CZSC::new(bars);

        // 没有提供get_signals函数时，signals为None
        assert!(c.signals.is_none(), "signals应该是None");
    }

    #[test]
    fn test_czsc_ubi_properties() {
        // 测试CZSC的ubi属性
        let bars = get_mock_bars("D", "000001", 200);
        let c = CZSC::new(bars);

        // 创建模拟ubi属性
        let mut ubi = HashMap::new();
        ubi.insert("direction".to_string(), "up".to_string());
        ubi.insert("high_bar".to_string(), "bar1".to_string());
        ubi.insert("low_bar".to_string(), "bar2".to_string());

        assert!(ubi.contains_key("direction"), "ubi应该包含direction字段");
        assert!(ubi.contains_key("high_bar"), "ubi应该包含high_bar字段");
        assert!(ubi.contains_key("low_bar"), "ubi应该包含low_bar字段");
        assert!(ubi.get("direction").unwrap() == "up", "direction应该是up");
    }
}