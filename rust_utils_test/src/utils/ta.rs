//! 技术分析相关的工具函数
//!
//! 包括各种技术指标的计算功能

use std::vec::Vec;

/// 简单移动平均
pub fn sma(data: &[f64], timeperiod: usize) -> Vec<f64> {
    let mut result = Vec::with_capacity(data.len());
    
    if timeperiod == 0 || data.is_empty() {
        // 如果timeperiod为0或数据为空，返回原数据
        return data.iter().map(|x| x.round_digits(4)).collect();
    }
    
    for i in 0..data.len() {
        if i + 1 < timeperiod {
            // 当数据不足时，计算已有数据的平均值
            let slice = &data[0..i+1];
            let avg = slice.iter().sum::<f64>() / slice.len() as f64;
            result.push(avg.round_digits(4));
        } else {
            // 计算指定周期内的平均值
            let start_idx = i + 1 - timeperiod;  // 使用 i + 1 - timeperiod 避免下溢
            let slice = &data[start_idx..=i];
            let avg = slice.iter().sum::<f64>() / slice.len() as f64;
            result.push(avg.round_digits(4));
        }
    }
    
    result
}

/// 指数移动平均
pub fn ema(data: &[f64], timeperiod: usize) -> Vec<f64> {
    let mut result = Vec::with_capacity(data.len());
    
    for i in 0..data.len() {
        if i == 0 {
            result.push(data[i]);
        } else {
            let prev_ema = result[i - 1];
            let ema = (2.0 * data[i] + prev_ema * (timeperiod as f64 - 1.0)) / (timeperiod as f64 + 1.0);
            result.push(ema.round_digits(4));
        }
    }
    
    result
}

/// MACD 指标计算
pub fn macd(
    data: &[f64], 
    fastperiod: usize, 
    slowperiod: usize, 
    signalperiod: usize
) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let ema_fast = ema(data, fastperiod);
    let ema_slow = ema(data, slowperiod);
    
    let mut diff = Vec::with_capacity(data.len());
    for i in 0..data.len() {
        diff.push((ema_fast[i] - ema_slow[i]).round_digits(4));
    }
    
    let dea = ema(&diff, signalperiod);
    
    let mut macd = Vec::with_capacity(data.len());
    for i in 0..data.len() {
        macd.push(((diff[i] - dea[i]) * 2.0).round_digits(4));
    }
    
    (diff, dea, macd)
}

/// 拟合优度 R Square
pub fn rsq(data: &[f64]) -> f64 {
    let n = data.len();
    if n <= 1 {
        return 0.0;
    }
    
    let x: Vec<f64> = (0..n).map(|i| i as f64).collect();
    let y = data;
    
    let x_sum: f64 = x.iter().sum();
    let y_sum: f64 = y.iter().sum();
    let x_squared_sum: f64 = x.iter().map(|xi| xi * xi).sum();
    let xy_product_sum: f64 = (0..n).map(|i| x[i] * y[i]).sum();
    
    let num = n as f64;
    let delta = num * x_squared_sum - x_sum * x_sum;
    
    if delta == 0.0 {
        return 0.0;
    }
    
    let y_intercept = (x_squared_sum * y_sum - x_sum * xy_product_sum) / delta;
    let slope = (num * xy_product_sum - x_sum * y_sum) / delta;
    
    let y_mean = y_sum / num;
    let ss_tot = y.iter().map(|yi| (yi - y_mean) * (yi - y_mean)).sum::<f64>() + 0.00001;
    
    let ss_err = (0..n)
        .map(|i| (y[i] - slope * x[i] - y_intercept) * (y[i] - slope * x[i] - y_intercept))
        .sum::<f64>();
    
    let rsq = 1.0 - ss_err / ss_tot;
    rsq.round_digits(4)
}

/// 扩展 f64 类型以支持四舍五入到指定位数
trait RoundDigits {
    fn round_digits(self, digits: u32) -> Self;
}

impl RoundDigits for f64 {
    fn round_digits(self, digits: u32) -> Self {
        let multiplier = 10f64.powi(digits as i32);
        (self * multiplier).round() / multiplier
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sma() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = sma(&data, 3);
        assert_eq!(result, vec![1.0, 1.5, 2.0, 3.0, 4.0]); // 第三个元素之后才开始真正的3期平均
    }

    #[test]
    fn test_ema() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = ema(&data, 3);
        assert!(!result.is_empty());
        assert_eq!(result[0], 1.0); // 第一个值应该是原始值
    }

    #[test]
    fn test_rsq() {
        // 完全线性的数据，R² 应该接近 1
        let linear_data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let rsq_val = rsq(&linear_data);
        assert!(rsq_val >= 0.99); // 应该非常接近 1
        
        // 随机数据，R² 应该较低
        let random_data = vec![1.0, 5.0, 2.0, 4.0, 3.0];
        let rsq_val2 = rsq(&random_data);
        assert!(rsq_val2 >= 0.0); // R² 应该是非负的
    }
}