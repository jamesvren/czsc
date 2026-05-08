//! 技术分析指标模块
//! 
//! 提供常用的技术分析指标计算功能

use anyhow::Result;
use std::cmp;

/// 简单移动平均
pub fn sma(close: &[f64], period: usize) -> Result<Vec<f64>> {
    if close.len() < period {
        return Ok(vec![]);
    }

    let mut result = Vec::new();
    for i in 0..close.len() - period + 1 {
        let sum: f64 = close[i..i + period].iter().sum();
        result.push(sum / period as f64);
    }
    Ok(result)
}

/// 指数移动平均
pub fn ema(close: &[f64], period: usize) -> Result<Vec<f64>> {
    if close.is_empty() {
        return Ok(vec![]);
    }

    let mut result = Vec::with_capacity(close.len());
    let alpha = 2.0 / (period as f64 + 1.0);
    
    // 第一个EMA值等于第一个收盘价
    result.push(close[0]);
    
    for i in 1..close.len() {
        let ema_val = alpha * close[i] + (1.0 - alpha) * result[i - 1];
        result.push(ema_val);
    }
    
    Ok(result)
}

/// MACD指标计算
pub fn macd(
    close: &[f64],
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
) -> Result<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    if close.is_empty() {
        return Ok((vec![], vec![], vec![]));
    }

    // 计算快速EMA
    let fast_ema = ema(close, fast_period)?;
    // 计算慢速EMA
    let slow_ema = ema(close, slow_period)?;

    // 确保两个EMA序列长度相同
    let min_len = cmp::min(fast_ema.len(), slow_ema.len());
    let mut dif = Vec::with_capacity(min_len);
    for i in 0..min_len {
        dif.push(fast_ema[i] - slow_ema[i]);
    }

    // 计算DEA (signal line)
    let dea = ema(&dif, signal_period)?;

    // 计算柱状图 (histogram)
    let min_len2 = cmp::min(dif.len(), dea.len());
    let mut hist = Vec::with_capacity(min_len2);
    for i in 0..min_len2 {
        hist.push((dif[i] - dea[i]) * 2.0); // 通常乘以2
    }

    Ok((dif, dea, hist))
}

/// 拟合优度 R Square
pub fn rsq<T>(x: &[T], y: &[T]) -> Result<f64>
where
    T: Into<f64> + Copy,
{
    if x.len() != y.len() || x.len() < 2 {
        return Ok(0.0);
    }

    let n = x.len() as f64;
    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    let mut sum_xy = 0.0;
    let mut sum_xx = 0.0;
    let mut sum_yy = 0.0;

    for i in 0..x.len() {
        let xi: f64 = x[i].into();
        let yi: f64 = y[i].into();
        sum_x += xi;
        sum_y += yi;
        sum_xy += xi * yi;
        sum_xx += xi * xi;
        sum_yy += yi * yi;
    }

    let denominator = (n * sum_xx - sum_x * sum_x) * (n * sum_yy - sum_y * sum_y);
    if denominator == 0.0 {
        return Ok(0.0);
    }

    let r = (n * sum_xy - sum_x * sum_y) / denominator.sqrt();
    Ok(r * r)
}

/// 扩展f64类型以支持四舍五入到指定位数
pub trait RoundDigits {
    fn round_digits(self, digits: u32) -> f64;
}

impl RoundDigits for f64 {
    fn round_digits(self, digits: u32) -> f64 {
        let multiplier = 10_f64.powi(digits as i32);
        (self * multiplier).round() / multiplier
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sma() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = sma(&data, 3).unwrap();
        assert_eq!(result, vec![2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_ema() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = ema(&data, 2).unwrap();
        assert_eq!(result.len(), 5);
        assert_eq!(result[0], 1.0);
    }

    #[test]
    fn test_rsq() {
        // 完全正相关的数据
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        let result = rsq(&x, &y).unwrap();
        assert!((result - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_round_digits() {
        let value = 3.14159;
        assert_eq!(value.round_digits(2), 3.14);
        assert_eq!(value.round_digits(0), 3.0);
    }
}