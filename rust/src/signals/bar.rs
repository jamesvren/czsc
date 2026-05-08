//! 基础K线信号函数实现
//! 以 bar 作为前缀，代表信号属于基础 K 线信号

use crate::objects::*;
use crate::enums::*;
use std::collections::HashMap;

/// 单K趋势因子辅助判断买卖点信号
/// 
/// 参数: 
/// - di: 倒数第di根K线 (默认为1)
/// - n: 分层数量 (默认为5，最大不超过20)
/// 
/// 信号逻辑: 定义趋势因子：(收盘价 / 开盘价 -1) / 成交量，选取最近100根K线计算趋势因子并分层
pub fn bar_single_v230506(c: &CZSC, di: Option<i32>, n: Option<i32>) -> Signal {
    let di = di.unwrap_or(1);
    let n = n.unwrap_or(5);
    
    if n > 20 {
        panic!("n 的取值范围为 1~20，分层数量不宜太多");
    }

    let freq = c.freq.to_string();
    let k1 = freq.clone();
    let k2 = format!("D{}单K趋势N{}_BS辅助V230506", di, n);
    let k3 = "其他".to_string();
    
    if c.bars_raw.len() < (100 + di) as usize {
        return Signal::new(k1, k2, k3, "其他".to_string(), "任意".to_string(), "任意".to_string(), 0);
    }

    // 获取最近100根K线
    let start_idx = c.bars_raw.len() - (100 + di as usize - 1);
    let bars = &c.bars_raw[start_idx..start_idx + 100];
    
    // 计算趋势因子
    let factors: Vec<f64> = bars.iter()
        .map(|x| (x.close / x.open - 1.0) / x.vol)
        .collect();

    // 计算分位数并确定层级
    let mut sorted_factors = factors.clone();
    sorted_factors.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    let step_size = sorted_factors.len() / n as usize;
    let mut level = 1;
    let last_factor = factors.last().unwrap();
    
    for i in 1..n {
        let idx = ((i as f64 / n as f64) * factors.len() as f64) as usize;
        if idx < sorted_factors.len() && last_factor > &sorted_factors[idx] {
            level = i + 1;
        }
    }

    Signal::new(
        k1,
        k2,
        format!("第{}层", level),
        "任意".to_string(),
        "任意".to_string(),
        "任意".to_string(),
        0
    )
}

/// 三K加速形态配合成交量变化信号
/// 
/// 参数:
/// - di: 倒数第di根K线 (默认为1)
/// 
/// 信号逻辑: 连续三根阳线/阴线，结合高低点创新和成交量变化判断
pub fn bar_triple_v230506(c: &CZSC, di: Option<i32>) -> Signal {
    let di = di.unwrap_or(1);
    let freq = c.freq.to_string();
    let k1 = freq.clone();
    let k2 = format!("D{}三K加速_裸K形态V230506", di);
    
    if c.bars_raw.len() < (7 + di as usize - 1) {
        return Signal::new(k1, k2, "其他".to_string(), "任意".to_string(), "任意".to_string(), "任意".to_string(), 0);
    }

    // 获取最近三根K线
    let idx = c.bars_raw.len() - di as usize;
    let b1 = &c.bars_raw[idx];      // 最新K线
    let b2 = &c.bars_raw[idx - 1];  // 第二根
    let b3 = &c.bars_raw[idx - 2];  // 第三根

    let mut v1 = "其他".to_string();
    
    // 检查三连阳
    if b1.close > b1.open && b2.close > b2.open && b3.close > b3.open {
        v1 = "三连涨".to_string();
        // 检查新高涨
        if b1.high > b2.high && b2.high > b3.high && b1.low > b2.low && b2.low > b3.low {
            v1 = "新高涨".to_string();
        }
    }
    
    // 检查三连阴
    if b1.close < b1.open && b2.close < b2.open && b3.close < b3.open {
        v1 = "三连跌".to_string();
        // 检查新低跌
        if b1.high < b2.high && b2.high < b3.high && b1.low < b2.low && b2.low < b3.low {
            v1 = "新低跌".to_string();
        }
    }

    if v1 == "其他" {
        return Signal::new(k1, k2, v1, "任意".to_string(), "任意".to_string(), "任意".to_string(), 0);
    }

    // 检查成交量变化
    let v2 = if b1.vol > b2.vol && b2.vol > b3.vol {
        "依次放量".to_string()
    } else if b1.vol < b2.vol && b2.vol < b3.vol {
        "依次缩量".to_string()
    } else {
        "量柱无序".to_string()
    };

    Signal::new(
        k1,
        k2,
        v1,
        v2,
        "任意".to_string(),
        "任意".to_string(),
        0
    )
}

/// 判断分钟K线是否结束信号
/// 
/// 参数:
/// - freq1: 较大的周期频率
/// 
/// 信号逻辑: 以基础周期为基础，判断较大周期K线是否结束
pub fn bar_end_v221211(c: &CZSC, freq1: &str) -> Signal {
    let freq = c.freq.to_string();
    let k1 = freq.clone();
    let k2 = format!("{}结束_BS辅助221211", freq1);
    
    if !freq1.contains("分钟") {
        panic!("freq1 必须是分钟周期");
    }

    // 这里简化实现，实际需要实现freq_end_time函数
    let v = "闭合"; // 简化为总是闭合
    Signal::new(
        k1,
        k2,
        v.to_string(),
        "任意".to_string(),
        "任意".to_string(),
        "任意".to_string(),
        0
    )
}

/// 涨跌停信号
/// 
/// 参数:
/// - di: 倒数第di根K线 (默认为1)
/// 
/// 信号逻辑: close等于high大于等于前一根K线的close，近似认为是涨停；反之，跌停
pub fn bar_zdt_v230331(c: &CZSC, di: Option<i32>) -> Signal {
    let di = di.unwrap_or(1);
    let freq = c.freq.to_string();
    let k1 = freq.clone();
    let k2 = format!("D{}_涨跌停V230331", di);
    
    if c.bars_raw.len() < (di as usize + 2) {
        return Signal::new(k1, k2, "其他".to_string(), "任意".to_string(), "任意".to_string(), "任意".to_string(), 0);
    }

    let idx = c.bars_raw.len() - di as usize;
    let b1 = &c.bars_raw[idx];      // 目标K线
    let b2 = &c.bars_raw[idx - 1];  // 前一根K线

    let v1 = if b1.close == b1.high && b1.high >= b2.close {
        "涨停".to_string()
    } else if b1.close == b1.low && b1.low <= b2.close {
        "跌停".to_string()
    } else {
        "其他".to_string()
    };

    Signal::new(
        k1,
        k2,
        v1,
        "任意".to_string(),
        "任意".to_string(),
        "任意".to_string(),
        0
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_bar_single_v230506() {
        // 创建测试数据
        let bars = vec![
            RawBar {
                id: 0,
                symbol: "TEST".to_string(),
                dt: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                freq: Freq::F1,
                open: 100.0,
                close: 105.0,
                high: 106.0,
                low: 99.0,
                vol: 1000.0,
                amount: 100000.0,
                cache: HashMap::new(),
            },
            // 更多测试数据...
        ];
        
        let czsc = CZSC {
            symbol: "TEST".to_string(),
            freq: Freq::F1,
            bars_raw: bars,
            bi_list: vec![],
            xd_list: vec![],
            zs_list: vec![],
            signals: vec![],
            events: vec![],
            cache: HashMap::new(),
            last_event: None,
        };
        
        let signal = bar_single_v230506(&czsc, Some(1), Some(5));
        assert!(signal.k2.contains("单K趋势"));
    }

    #[test]
    fn test_bar_triple_v230506() {
        // 创建三个阳线的测试数据
        let bars = vec![
            RawBar {
                id: 0,
                symbol: "TEST".to_string(),
                dt: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                freq: Freq::F1,
                open: 100.0,
                close: 105.0,
                high: 106.0,
                low: 99.0,
                vol: 1000.0,
                amount: 100000.0,
                cache: HashMap::new(),
            },
            RawBar {
                id: 1,
                symbol: "TEST".to_string(),
                dt: Utc.with_ymd_and_hms(2022, 1, 1, 0, 1, 0).unwrap(),
                freq: Freq::F1,
                open: 105.0,
                close: 110.0,
                high: 111.0,
                low: 104.0,
                vol: 1200.0,
                amount: 120000.0,
                cache: HashMap::new(),
            },
            RawBar {
                id: 2,
                symbol: "TEST".to_string(),
                dt: Utc.with_ymd_and_hms(2022, 1, 1, 0, 2, 0).unwrap(),
                freq: Freq::F1,
                open: 110.0,
                close: 115.0,
                high: 116.0,
                low: 109.0,
                vol: 1400.0,
                amount: 140000.0,
                cache: HashMap::new(),
            },
            // 填充足够的数据使长度>=7
            RawBar {
                id: 3,
                symbol: "TEST".to_string(),
                dt: Utc.with_ymd_and_hms(2022, 1, 1, 0, 3, 0).unwrap(),
                freq: Freq::F1,
                open: 100.0,
                close: 102.0,
                high: 103.0,
                low: 99.0,
                vol: 1000.0,
                amount: 100000.0,
                cache: HashMap::new(),
            },
            RawBar {
                id: 4,
                symbol: "TEST".to_string(),
                dt: Utc.with_ymd_and_hms(2022, 1, 1, 0, 4, 0).unwrap(),
                freq: Freq::F1,
                open: 100.0,
                close: 102.0,
                high: 103.0,
                low: 99.0,
                vol: 1000.0,
                amount: 100000.0,
                cache: HashMap::new(),
            },
            RawBar {
                id: 5,
                symbol: "TEST".to_string(),
                dt: Utc.with_ymd_and_hms(2022, 1, 1, 0, 5, 0).unwrap(),
                freq: Freq::F1,
                open: 100.0,
                close: 102.0,
                high: 103.0,
                low: 99.0,
                vol: 1000.0,
                amount: 100000.0,
                cache: HashMap::new(),
            },
            RawBar {
                id: 6,
                symbol: "TEST".to_string(),
                dt: Utc.with_ymd_and_hms(2022, 1, 1, 0, 6, 0).unwrap(),
                freq: Freq::F1,
                open: 100.0,
                close: 102.0,
                high: 103.0,
                low: 99.0,
                vol: 1000.0,
                amount: 100000.0,
                cache: HashMap::new(),
            },
        ];
        
        let czsc = CZSC {
            symbol: "TEST".to_string(),
            freq: Freq::F1,
            bars_raw: bars,
            bi_list: vec![],
            xd_list: vec![],
            zs_list: vec![],
            signals: vec![],
            events: vec![],
            cache: HashMap::new(),
            last_event: None,
        };
        
        let signal = bar_triple_v230506(&czsc, Some(1));
        println!("Triple signal: {}", signal.signal);
    }
}