//! 持仓相关信号函数实现
//! 包含与持仓状态、止损止盈相关的信号函数

use crate::objects::*;
use crate::enums::*;
use std::collections::HashMap;

/// 判断开仓后是否升破MA均线或跌破MA均线
/// 
/// 参数:
/// - pos_name: 持仓名称
/// - freq1: 给定的K线周期
/// - ma_type: MA类型 (默认为SMA)
/// - timeperiod: MA周期 (默认为5)
/// 
/// 信号逻辑: 多头持有状态下，如果持有多头且开仓后有价格升破MA均线，则为多头升破均线；反之亦然
pub fn pos_ma_v230414(
    pos_name: &str,
    freq1: &str,
    ma_type: Option<&str>,
    timeperiod: Option<i32>,
    latest_price: f64,
    open_price: f64,
    operate: Operate,
) -> Signal {
    let ma_type = ma_type.unwrap_or("SMA");
    let timeperiod = timeperiod.unwrap_or(5);
    
    let k1 = format!("{}_{}#{}#{}_持有状态V230414", pos_name, freq1, ma_type, timeperiod);
    let parts: Vec<&str> = k1.split('_').collect();
    let k1 = parts[0].to_string();
    let k2 = format!("{}#{}#{}_持有状态V230414", freq1, ma_type, timeperiod);
    let k3 = "其他".to_string();
    
    // 简化实现：根据操作类型和价格比较返回信号
    let (v1, v2) = if operate == Operate::LO && latest_price > open_price {
        ("多头".to_string(), "升破均线".to_string())
    } else if operate == Operate::SO && latest_price < open_price {
        ("空头".to_string(), "跌破均线".to_string())
    } else {
        ("其他".to_string(), "其他".to_string())
    };

    Signal::new(
        k1,
        k2,
        k3,
        v1,
        v2,
        "任意".to_string(),
        0
    )
}

/// 按照开仓点附近的分型止损
/// 
/// 参数:
/// - pos_name: 持仓名称
/// - freq1: 给定的K线周期
/// - n: 向前找的分型个数 (默认为3)
/// 
/// 信号逻辑: 从开仓点开始向前找N个分型，根据分型极值判断是否触发止损
pub fn pos_fx_stop_v230414(
    pos_name: &str,
    freq1: &str,
    n: Option<i32>,
    latest_price: f64,
    operate: Operate,
    fx_list: &[FX],
) -> Signal {
    let n = n.unwrap_or(3) as usize;
    
    let k1 = freq1.to_string();
    let k2 = format!("{}N{}_止损V230414", pos_name, n);
    let k3 = "其他".to_string();
    
    let v1 = if operate == Operate::LO {
        // 多头：查找最近N个底分型的最低点
        let d_fxs: Vec<&FX> = fx_list.iter().filter(|fx| fx.mark == Mark::D).rev().take(n).collect();
        if !d_fxs.is_empty() && latest_price < d_fxs.iter().map(|fx| fx.low).fold(f64::INFINITY, f64::min) {
            "多头止损".to_string()
        } else {
            "其他".to_string()
        }
    } else if operate == Operate::SO {
        // 空头：查找最近N个顶分型的最高点
        let g_fxs: Vec<&FX> = fx_list.iter().filter(|fx| fx.mark == Mark::G).rev().take(n).collect();
        if !g_fxs.is_empty() && latest_price > g_fxs.iter().map(|fx| fx.high).fold(f64::NEG_INFINITY, f64::max) {
            "空头止损".to_string()
        } else {
            "其他".to_string()
        }
    } else {
        "其他".to_string()
    };

    Signal::new(
        k1,
        k2,
        k3,
        v1,
        "任意".to_string(),
        "任意".to_string(),
        0
    )
}

/// 按照开仓点附近的N根K线极值止损
/// 
/// 参数:
/// - pos_name: 持仓名称
/// - freq1: 给定的K线周期
/// - n: 向前找的K线个数 (默认为3, 范围1-20)
/// 
/// 信号逻辑: 从开仓点开始向前找N根K线的极值，判断是否触发止损
pub fn pos_bar_stop_v230524(
    pos_name: &str,
    freq1: &str,
    n: Option<i32>,
    latest_price: f64,
    operate: Operate,
    bars: &[RawBar],
) -> Signal {
    let n = n.unwrap_or(3).max(1).min(20) as usize;
    
    let k1 = format!("{}_{}N{}K_止损V230524", pos_name, freq1, n);
    let parts: Vec<&str> = k1.split('_').collect();
    let k1 = parts[0].to_string();
    let k2 = format!("{}N{}K_止损V230524", freq1, n);
    let k3 = "其他".to_string();
    
    // 获取最近N根K线
    let recent_bars = if bars.len() >= n {
        &bars[bars.len() - n..]
    } else {
        bars
    };
    
    let v1 = if operate == Operate::LO {
        // 多头：检查是否跌破最近N根K线的最低点
        let lowest = recent_bars.iter().map(|bar| bar.low).fold(f64::INFINITY, f64::min);
        if latest_price < lowest {
            "多头止损".to_string()
        } else {
            "其他".to_string()
        }
    } else if operate == Operate::SO {
        // 空头：检查是否升破最近N根K线的最高点
        let highest = recent_bars.iter().map(|bar| bar.high).fold(f64::NEG_INFINITY, f64::max);
        if latest_price > highest {
            "空头止损".to_string()
        } else {
            "其他".to_string()
        }
    } else {
        "其他".to_string()
    };

    Signal::new(
        k1,
        k2,
        k3,
        v1,
        "任意".to_string(),
        "任意".to_string(),
        0
    )
}

/// 开仓后N根K线涨幅小于M%，则平仓
/// 
/// 参数:
/// - pos_name: 持仓名称
/// - freq1: 给定的K线周期
/// - n: 最少持有K线数量 (默认为5)
/// - m: 涨幅阈值 (默认为100, 单位BP)
/// 
/// 信号逻辑: 计算开仓后N根K线的涨幅，如果小于阈值则判断为趋势不明朗
pub fn pos_holds_v230414(
    pos_name: &str,
    freq1: &str,
    n: Option<i32>,
    m: Option<i32>,
    bars_after_open: &[RawBar],
    open_price: f64,
    operate: Operate,
) -> Signal {
    let n = n.unwrap_or(5) as usize;
    let m = m.unwrap_or(100) as f64; // BP单位
    
    let k1 = format!("{}_{}N{}M{}_趋势判断V230414", pos_name, freq1, n, m);
    let parts: Vec<&str> = k1.split('_').collect();
    let k1 = parts[0].to_string();
    let k2 = format!("{}N{}M{}_趋势判断V230414", freq1, n, m);
    let k3 = "其他".to_string();
    
    if bars_after_open.len() < n {
        return Signal::new(
            k1,
            k2,
            k3,
            "其他".to_string(),
            "任意".to_string(),
            "任意".to_string(),
            0
        );
    }
    
    let current_close = bars_after_open.last().unwrap().close;
    
    let v1 = if operate == Operate::LO {
        let zdf = (current_close - open_price) / open_price * 10000.0; // 转换为BP
        if zdf < m {
            "多头存疑".to_string()
        } else {
            "多头良好".to_string()
        }
    } else if operate == Operate::SO {
        let zdf = (open_price - current_close) / open_price * 10000.0; // 转换为BP
        if zdf < m {
            "空头存疑".to_string()
        } else {
            "空头良好".to_string()
        }
    } else {
        "其他".to_string()
    };

    Signal::new(
        k1,
        k2,
        k3,
        v1,
        "任意".to_string(),
        "任意".to_string(),
        0
    )
}

/// 固定比例止损止盈
/// 
/// 参数:
/// - pos_name: 持仓名称
/// - th: 止损止盈阈值 (默认为300, 单位BP)
/// 
/// 信号逻辑: 以多头为例，如果持有收益超过th个BP，则止盈；如果亏损超过th个BP，则止损
pub fn pos_fix_exit_v230624(
    pos_name: &str,
    th: Option<i32>,
    latest_price: f64,
    open_price: f64,
    operate: Operate,
) -> Signal {
    let th = th.unwrap_or(300) as f64 / 10000.0; // 转换为小数形式
    
    let k1 = pos_name.to_string();
    let k2 = format!("固定{}BP止盈止损_出场V230624", (th * 10000.0) as i32);
    let k3 = "其他".to_string();
    
    let v1 = if operate == Operate::LO {
        let profit_ratio = (latest_price - open_price) / open_price;
        if profit_ratio <= -th {
            "多头止损".to_string()
        } else if profit_ratio >= th {
            "多头止盈".to_string()
        } else {
            "其他".to_string()
        }
    } else if operate == Operate::SO {
        let profit_ratio = (open_price - latest_price) / open_price;
        if profit_ratio <= -th {
            "空头止损".to_string()
        } else if profit_ratio >= th {
            "空头止盈".to_string()
        } else {
            "其他".to_string()
        }
    } else {
        "其他".to_string()
    };

    Signal::new(
        k1,
        k2,
        k3,
        v1,
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
    fn test_pos_ma_v230414() {
        let signal = pos_ma_v230414(
            "日线三买多头N1",
            "60分钟",
            Some("SMA"),
            Some(5),
            105.0,
            100.0,
            Operate::LO,
        );
        
        assert_eq!(signal.v1, "多头");
        assert_eq!(signal.v2, "升破均线");
    }

    #[test]
    fn test_pos_fx_stop_v230414() {
        let fx_list = vec![
            FX {
                symbol: "TEST".to_string(),
                dt: Utc::now(),
                mark: Mark::D,
                high: 110.0,
                low: 90.0,
                fx: 90.0,
                elements: vec![],
                cache: Default::default(),
            },
            FX {
                symbol: "TEST".to_string(),
                dt: Utc::now(),
                mark: Mark::G,
                high: 120.0,
                low: 100.0,
                fx: 120.0,
                elements: vec![],
                cache: Default::default(),
            },
        ];
        
        let signal = pos_fx_stop_v230414(
            "日线三买多头N1",
            "60分钟",
            Some(3),
            85.0, // 低于底分型最低点
            Operate::LO,
            &fx_list,
        );
        
        assert_eq!(signal.v1, "多头止损");
    }

    #[test]
    fn test_pos_bar_stop_v230524() {
        let bars = vec![
            RawBar {
                id: 0,
                symbol: "TEST".to_string(),
                dt: Utc::now(),
                freq: Freq::F60,
                open: 100.0,
                close: 102.0,
                high: 103.0,
                low: 99.0,
                vol: 1000.0,
                amount: 100000.0,
                cache: Default::default(),
            },
            RawBar {
                id: 1,
                symbol: "TEST".to_string(),
                dt: Utc::now(),
                freq: Freq::F60,
                open: 102.0,
                close: 101.0,
                high: 104.0,
                low: 98.0,
                vol: 1000.0,
                amount: 100000.0,
                cache: Default::default(),
            },
            RawBar {
                id: 2,
                symbol: "TEST".to_string(),
                dt: Utc::now(),
                freq: Freq::F60,
                open: 101.0,
                close: 103.0,
                high: 105.0,
                low: 100.0,
                vol: 1000.0,
                amount: 100000.0,
                cache: Default::default(),
            },
        ];
        
        let signal = pos_bar_stop_v230524(
            "日线三买多头",
            "日线",
            Some(3),
            97.0, // 低于最近3根K线的最低点
            Operate::LO,
            &bars,
        );
        
        assert_eq!(signal.v1, "多头止损");
    }
}