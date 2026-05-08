//! 上下文相关信号函数实现
//! 包含基于市场状态、趋势和其他上下文因素的信号函数

use crate::objects::*;
use crate::enums::Freq;
use std::collections::HashMap;

/// 判断当前是否处于多头趋势中
/// 
/// 参数:
/// - freq: K线周期
/// - ma_type: 移动平均类型 (默认为SMA)
/// - timeperiod: MA周期 (默认为20)
/// - close: 当前收盘价
/// - ma_value: 对应MA的值
/// 
/// 信号逻辑: 当收盘价持续在均线上方时判断为多头趋势
pub fn cxt_ma_trend_v230414(
    freq: &str,
    ma_type: Option<&str>,
    timeperiod: Option<i32>,
    close: f64,
    ma_value: f64,
) -> Signal {
    let ma_type = ma_type.unwrap_or("SMA");
    let timeperiod = timeperiod.unwrap_or(20);
    
    let k1 = freq.to_string();
    let k2 = format!("{}#{}#{}_趋势V230414", ma_type, timeperiod, "MA");
    let k3 = "状态".to_string();
    
    let v1 = if close > ma_value {
        "多头趋势".to_string()
    } else {
        "空头趋势".to_string()
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

/// 判断当前是否处于多头趋势中（基于MA排列）
/// 
/// 参数:
/// - freq: K线周期
/// - ma_values: 不同周期的MA值，按短周期到长周期排序
/// 
/// 信号逻辑: 当短期均线上穿长期均线时判断为多头趋势，反之为空头趋势
pub fn cxt_ma_arrangement_v230414(
    freq: &str,
    ma_values: &[f64],  // 按短周期到长周期排序
) -> Signal {
    let k1 = freq.to_string();
    let k2 = "MA排列_趋势V230414".to_string();
    let k3 = "状态".to_string();
    
    let mut is_bullish = true;
    let mut is_bearish = true;
    
    // 检查是否呈现多头排列（短期均线上穿长期均线）
    for i in 1..ma_values.len() {
        if ma_values[i-1] <= ma_values[i] {
            is_bullish = false;
        }
        if ma_values[i-1] >= ma_values[i] {
            is_bearish = false;
        }
    }
    
    let v1 = if is_bullish {
        "多头排列".to_string()
    } else if is_bearish {
        "空头排列".to_string()
    } else {
        "震荡".to_string()
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

/// 判断波动率状态
/// 
/// 参数:
/// - freq: K线周期
/// - volatility: 当前波动率值
/// - threshold_low: 低波动率阈值 (默认为0.01)
/// - threshold_high: 高波动率阈值 (默认为0.03)
/// 
/// 信号逻辑: 根据波动率水平判断市场状态
pub fn cxt_volatility_state_v230414(
    freq: &str,
    volatility: f64,
    threshold_low: Option<f64>,
    threshold_high: Option<f64>,
) -> Signal {
    let threshold_low = threshold_low.unwrap_or(0.01);
    let threshold_high = threshold_high.unwrap_or(0.03);
    
    let k1 = freq.to_string();
    let k2 = "波动率_状态V230414".to_string();
    let k3 = "状态".to_string();
    
    let v1 = if volatility < threshold_low {
        "低波动".to_string()
    } else if volatility > threshold_high {
        "高波动".to_string()
    } else {
        "中波动".to_string()
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

/// 判断趋势强度
/// 
/// 参数:
/// - freq: K线周期
/// - price_change: 价格变化幅度
/// - avg_change: 平均变化幅度
/// 
/// 信号逻辑: 根据当前价格变化与平均变化的比较判断趋势强度
pub fn cxt_trend_strength_v230414(
    freq: &str,
    price_change: f64,
    avg_change: f64,
) -> Signal {
    let k1 = freq.to_string();
    let k2 = "趋势_强度V230414".to_string();
    let k3 = "强度".to_string();
    
    let strength_ratio = if avg_change != 0.0 { 
        (price_change / avg_change).abs()
    } else { 
        0.0 
    };
    
    let v1 = if strength_ratio > 2.0 {
        "强势".to_string()
    } else if strength_ratio > 1.0 {
        "中势".to_string()
    } else {
        "弱势".to_string()
    };
    
    let v2 = if price_change > 0.0 {
        "多头".to_string()
    } else {
        "空头".to_string()
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

/// 判断市场情绪状态
/// 
/// 参数:
/// - freq: K线周期
/// - bullish_count: 看多信号数量
/// - bearish_count: 看空信号数量
/// - total_count: 总信号数量
/// 
/// 信号逻辑: 根据多空信号的比例判断市场情绪
pub fn cxt_market_sentiment_v230414(
    freq: &str,
    bullish_count: i32,
    bearish_count: i32,
    total_count: i32,
) -> Signal {
    let k1 = freq.to_string();
    let k2 = "情绪_状态V230414".to_string();
    let k3 = "情绪".to_string();
    
    let total = total_count.max(1) as f64;
    let bullish_ratio = bullish_count as f64 / total;
    let bearish_ratio = bearish_count as f64 / total;
    
    let v1 = if bullish_ratio > 0.6 {
        "极度乐观".to_string()
    } else if bullish_ratio > 0.4 {
        "乐观".to_string()
    } else if bearish_ratio > 0.6 {
        "极度悲观".to_string()
    } else if bearish_ratio > 0.4 {
        "悲观".to_string()
    } else {
        "中性".to_string()
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

/// 判断市场阶段（牛熊市）
/// 
/// 参数:
/// - freq: K线周期
/// - current_price: 当前价格
/// - baseline_price: 基准价格（如年线）
/// - period: 参考周期类型 ("month", "quarter", "year")
/// 
/// 信号逻辑: 根据价格相对于基准的高低判断市场阶段
pub fn cxt_market_phase_v230414(
    freq: &str,
    current_price: f64,
    baseline_price: f64,
    period: &str,
) -> Signal {
    let k1 = freq.to_string();
    let k2 = format!("{}_阶段V230414", period);
    let k3 = "阶段".to_string();
    
    let ratio = current_price / baseline_price;
    let v1 = if ratio > 1.3 {
        "牛市主升".to_string()
    } else if ratio > 1.1 {
        "牛市回调".to_string()
    } else if ratio > 0.9 {
        "震荡".to_string()
    } else if ratio > 0.7 {
        "熊市反弹".to_string()
    } else {
        "熊市主跌".to_string()
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
    fn test_cxt_ma_trend_v230414() {
        let signal = cxt_ma_trend_v230414(
            "日线",
            Some("SMA"),
            Some(20),
            105.0,  // 收盘价
            100.0,  // MA值
        );
        
        assert_eq!(signal.v1, "多头趋势");
    }

    #[test]
    fn test_cxt_ma_arrangement_v230414() {
        let signal = cxt_ma_arrangement_v230414(
            "60分钟",
            &[100.0, 98.0, 95.0],  // 短期到长期，呈多头排列
        );
        
        assert_eq!(signal.v1, "多头排列");
    }

    #[test]
    fn test_cxt_volatility_state_v230414() {
        let signal = cxt_volatility_state_v230414(
            "30分钟",
            0.05,  // 高波动率
            Some(0.01),
            Some(0.03),
        );
        
        assert_eq!(signal.v1, "高波动");
    }

    #[test]
    fn test_cxt_trend_strength_v230414() {
        let signal = cxt_trend_strength_v230414(
            "15分钟",
            3.0,   // 当前价格变化
            1.0,   // 平均变化
        );
        
        assert_eq!(signal.v1, "强势");
        assert_eq!(signal.v2, "多头");
    }

    #[test]
    fn test_cxt_market_sentiment_v230414() {
        let signal = cxt_market_sentiment_v230414(
            "日线",
            7,   // 看多信号
            2,   // 看空信号
            10,  // 总信号
        );
        
        assert_eq!(signal.v1, "极度乐观");
    }

    #[test]
    fn test_cxt_market_phase_v230414() {
        let signal = cxt_market_phase_v230414(
            "周线",
            150.0,  // 当前价格
            100.0,  // 基准价格
            "year",
        );
        
        assert_eq!(signal.v1, "牛市主升");
    }
}