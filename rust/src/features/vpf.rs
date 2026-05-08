//! 标准量价因子

use std::collections::HashMap;
use crate::features::utils::DataFrame;

/// 比较开盘价、收盘价与当日最高价和最低价的中点的关系，来判断市场的强弱
/// 
/// # Arguments
/// * `df` - DataFrame结构的标准K线数据
/// * `kwargs` - 其他参数
///     - tag: str, defaults to 'N2'  因子字段标记
///     - num: int, defaults to 2  参数值
/// 
/// # Returns
/// * `DataFrame` - 包含新因子列的DataFrame
pub fn vpf001(mut df: DataFrame, kwargs: HashMap<String, String>) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let num: f64 = kwargs.get("num").unwrap_or(&"2".to_string()).parse().unwrap_or(2.0);
    let tag = kwargs.get("tag").unwrap_or(&format!("N{}", num as i32)).clone();

    let factor_name = "VPF001";
    let factor_col = format!("F#{}#{}", factor_name, tag);

    let open_values = df.column("open")?.clone();
    let close_values = df.column("close")?.clone();
    let high_values = df.column("high")?.clone();
    let low_values = df.column("low")?.clone();

    // 检查长度是否一致
    if open_values.len() != close_values.len() || 
       open_values.len() != high_values.len() || 
       open_values.len() != low_values.len() {
        return Err("All input columns must have the same length".into());
    }

    let mut result = vec![0i32; open_values.len()];
    
    for i in 0..open_values.len() {
        let midpoint = (high_values[i] + low_values[i]) / num;
        
        let condition1 = open_values[i] >= midpoint && close_values[i] >= midpoint;
        let condition2 = open_values[i] < midpoint && close_values[i] < midpoint;
        
        if condition1 {
            result[i] = -1;  // 看跌
        } else if condition2 {
            result[i] = 1;   // 看涨
        }
        // 默认为0，不需要特别设置
    }

    // 将i32向量转换为f64向量以匹配DataFrame的要求
    let result_f64: Vec<f64> = result.iter().map(|&x| x as f64).collect();
    df.with_column(factor_col, result_f64)?;

    Ok(df)
}

/// 比较过去收益率的正负，以及当日最高价、最低价与开盘价或收盘价的关系
/// 
/// # Arguments
/// * `df` - DataFrame结构的标准K线数据
/// * `kwargs` - 其他参数
///     - tag: str, defaults to 'N4'  因子字段标记
///     - num: int, defaults to 4  参数值
/// 
/// # Returns
/// * `DataFrame` - 包含新因子列的DataFrame
pub fn vpf002(mut df: DataFrame, kwargs: HashMap<String, String>) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let num: usize = kwargs.get("num").unwrap_or(&"4".to_string()).parse().unwrap_or(4);
    let tag = kwargs.get("tag").unwrap_or(&format!("N{}", num)).clone();

    let factor_name = "VPF002";
    let factor_col = format!("F#{}#{}", factor_name, tag);

    let close_values = df.column("close")?.clone();
    let high_values = df.column("high")?.clone();
    let low_values = df.column("low")?.clone();

    // 计算收益率
    let mut returns = vec![0.0; close_values.len()];
    for i in 1..close_values.len() {
        if close_values[i-1] != 0.0 {
            returns[i] = (close_values[i] - close_values[i-1]) / close_values[i-1];
        }
    }

    // 计算过去num天的累计收益率
    let mut cum_return = vec![0.0; close_values.len()];
    for i in 0..close_values.len() {
        let start = if i >= num { i - num + 1 } else { 0 };
        let mut sum = 0.0;
        for j in start..=i {
            sum += returns[j];
        }
        cum_return[i] = sum;
    }

    let mut result = vec![0i32; close_values.len()];
    
    for i in 0..close_values.len() {
        let cum_return_positive = cum_return[i] >= 0.0;
        
        // 计算 (high - close) / (close - low)，避免除零
        let denominator = close_values[i] - low_values[i];
        let ratio = if denominator.abs() > 1e-10 {
            (high_values[i] - close_values[i]) / denominator
        } else {
            0.0  // 避免除零
        };
        
        let ratio_ge_one = ratio >= 1.0;
        
        result[i] = if cum_return_positive || ratio_ge_one { 1 } else { -1 };
    }

    // 将i32向量转换为f64向量以匹配DataFrame的要求
    let result_f64: Vec<f64> = result.iter().map(|&x| x as f64).collect();
    df.with_column(factor_col, result_f64)?;

    Ok(df)
}

/// 比较过去N天最高价、最低价、开盘价和收盘价的比例，判断市场强弱
/// 
/// # Arguments
/// * `df` - DataFrame结构的标准K线数据
/// * `kwargs` - 其他参数
///     - tag: str
///     - num: int, defaults to 2  参数值
/// 
/// # Returns
/// * `DataFrame` - 包含新因子列的DataFrame
pub fn vpf003(mut df: DataFrame, kwargs: HashMap<String, String>) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let num: usize = kwargs.get("num").unwrap_or(&"2".to_string()).parse().unwrap_or(2);
    let tag = kwargs.get("tag").unwrap_or(&format!("N{}", num)).clone();

    let factor_name = "VPF003";
    let factor_col = format!("F#{}#{}", factor_name, tag);

    let high_values = df.column("high")?.clone();
    let low_values = df.column("low")?.clone();
    let open_values = df.column("open")?.clone();
    let close_values = df.column("close")?.clone();

    // 计算 hol = (high - open) / (high - low)
    let mut hol = vec![0.0; high_values.len()];
    for i in 0..high_values.len() {
        let denominator = high_values[i] - low_values[i];
        hol[i] = if denominator.abs() > 1e-10 {
            (high_values[i] - open_values[i]) / denominator
        } else {
            0.0  // 避免除零
        };
    }

    // 计算 clh = (close - low) / (high - low)
    let mut clh = vec![0.0; high_values.len()];
    for i in 0..high_values.len() {
        let denominator = high_values[i] - low_values[i];
        clh[i] = if denominator.abs() > 1e-10 {
            (close_values[i] - low_values[i]) / denominator
        } else {
            0.0  // 避免除零
        };
    }

    // 计算 hol 的滚动平均值
    let mut hol_ma = vec![0.0; hol.len()];
    for i in 0..hol.len() {
        let start = if i >= num { i - num + 1 } else { 0 };
        let mut sum = 0.0;
        let mut count = 0;
        for j in start..=i {
            sum += hol[j];
            count += 1;
        }
        hol_ma[i] = if count > 0 { sum / count as f64 } else { 0.0 };
    }

    // 计算 clh 的滚动平均值
    let mut clh_ma = vec![0.0; clh.len()];
    for i in 0..clh.len() {
        let start = if i >= num { i - num + 1 } else { 0 };
        let mut sum = 0.0;
        let mut count = 0;
        for j in start..=i {
            sum += clh[j];
            count += 1;
        }
        clh_ma[i] = if count > 0 { sum / count as f64 } else { 0.0 };
    }

    let mut result = vec![0i32; high_values.len()];
    
    for i in 0..high_values.len() {
        let hol_condition = hol_ma[i] >= 0.5;
        let price_condition = (high_values[i] + low_values[i] - open_values[i] - close_values[i]) >= 0.0;
        
        // 如果满足 hol 或 price 条件，则为1，否则为-1
        let initial_value = if hol_condition || price_condition { 1 } else { -1 };
        
        // 如果 clh 条件满足，则覆盖为-1
        if clh_ma[i] >= 0.5 {
            result[i] = -1;
        } else {
            result[i] = initial_value;
        }
    }

    // 将i32向量转换为f64向量以匹配DataFrame的要求
    let result_f64: Vec<f64> = result.iter().map(|&x| x as f64).collect();
    df.with_column(factor_col, result_f64)?;

    Ok(df)
}

/// EMA指标
/// 
/// # Arguments
/// * `df` - DataFrame结构的标准K线数据
/// * `kwargs` - 其他参数
///     - tag: str, 因子字段标记
///     - n: int, EMA的周期参数
/// 
/// # Returns
/// * `DataFrame` - 包含新因子列的DataFrame
pub fn vpf004(mut df: DataFrame, kwargs: HashMap<String, String>) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let n: usize = kwargs.get("n").unwrap_or(&"7".to_string()).parse().unwrap_or(7);
    let tag = kwargs.get("tag").unwrap_or(&format!("N{}", n)).clone();

    let factor_name = "VPF004";
    let factor_col = format!("F#{}#{}", factor_name, tag);

    let close_values = df.column("close")?.clone();

    // 计算EMA: 使用指数移动平均
    let alpha = 2.0 / (n as f64 + 1.0);
    
    // 计算第一个EMA值（简单移动平均的近似）
    let mut ema1 = vec![0.0; close_values.len()];
    let start_sum: f64 = close_values.iter().take(n).sum();
    let start_avg = start_sum / n as f64;
    ema1[n-1] = start_avg;
    
    for i in n..close_values.len() {
        ema1[i] = alpha * close_values[i] + (1.0 - alpha) * ema1[i-1];
    }
    
    // 计算EMA2
    let mut ema2 = vec![0.0; ema1.len()];
    ema2[n-1] = ema1[n-1];  // 从相同的位置开始
    
    for i in n..ema1.len() {
        ema2[i] = alpha * ema1[i] + (1.0 - alpha) * ema2[i-1];
    }
    
    // 计算EMA3
    let mut ema3 = vec![0.0; ema2.len()];
    ema3[n-1] = ema2[n-1];  // 从相同的位置开始
    
    for i in n..ema2.len() {
        ema3[i] = alpha * ema2[i] + (1.0 - alpha) * ema3[i-1];
    }

    // 计算因子值: 3 * (ema1 - ema2) + ema3
    let mut result = vec![0.0; close_values.len()];
    for i in 0..close_values.len() {
        if i >= n-1 {
            result[i] = 3.0 * (ema1[i] - ema2[i]) + ema3[i];
        }
    }

    df.with_column(factor_col, result)?;

    Ok(df)
}