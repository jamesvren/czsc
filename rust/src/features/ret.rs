//! 用于计算未来收益相关的因子，含有未来信息，不可用于实际交易
//! 通常用作模型训练、因子评价的标准

use std::collections::HashMap;
use crate::features::utils::DataFrame;

/// 用 close 价格计算未来 N 根K线的收益率
/// 
/// # Arguments
/// * `df` - DataFrame结构的标准K线数据
/// * `kwargs` - 其他参数
///     - tag: str, 因子字段标记
///     - n: int, 计算未来N根K线的收益率
/// 
/// # Returns
/// * `DataFrame` - 包含新因子列的DataFrame
pub fn ret001(mut df: DataFrame, kwargs: HashMap<String, String>) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let tag = kwargs.get("tag").unwrap_or(&"A".to_string()).clone();
    let n: usize = kwargs.get("n").unwrap_or(&"5".to_string()).parse().unwrap_or(5);

    let col_name = format!("F#RET001#{}", tag);
    
    // 获取close列
    let close_values = df.column("close")?.clone();
    let mut result = vec![0.0; close_values.len()];
    
    // 计算未来N根K线的收益率
    for i in 0..(close_values.len() - n) {
        if close_values[i] != 0.0 {
            result[i] = close_values[i + n] / close_values[i] - 1.0;
        }
    }

    df.with_column(col_name, result)?;
    
    Ok(df)
}

/// 用 open 价格计算未来 N 根K线的收益率
/// 
/// # Arguments
/// * `df` - DataFrame结构的标准K线数据
/// * `kwargs` - 其他参数
///     - tag: str, 因子字段标记
///     - n: int, 计算未来N根K线的收益率
/// 
/// # Returns
/// * `DataFrame` - 包含新因子列的DataFrame
pub fn ret002(mut df: DataFrame, kwargs: HashMap<String, String>) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let tag = kwargs.get("tag").unwrap_or(&"A".to_string()).clone();
    let n: usize = kwargs.get("n").unwrap_or(&"5".to_string()).parse().unwrap_or(5);

    let col_name = format!("F#RET002#{}", tag);
    
    // 获取open列
    let open_values = df.column("open")?.clone();
    let mut result = vec![0.0; open_values.len()];
    
    // 计算未来N+1根K线的收益率
    for i in 0..(open_values.len() - n - 1) {
        if open_values[i + 1] != 0.0 {
            result[i] = open_values[i + n + 1] / open_values[i + 1] - 1.0;
        }
    }

    df.with_column(col_name, result)?;
    
    Ok(df)
}

/// 未来 N 根K线的收益波动率
/// 
/// # Arguments
/// * `df` - DataFrame结构的标准K线数据
/// * `kwargs` - 其他参数
///     - tag: str, 因子字段标记
///     - n: int, 计算未来N根K线的收益波动率
/// 
/// # Returns
/// * `DataFrame` - 包含新因子列的DataFrame
pub fn ret003(mut df: DataFrame, kwargs: HashMap<String, String>) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let tag = kwargs.get("tag").unwrap_or(&"A".to_string()).clone();
    let n: usize = kwargs.get("n").unwrap_or(&"5".to_string()).parse().unwrap_or(5);

    let col_name = format!("F#RET003#{}", tag);
    
    // 获取close列并计算收益率
    let close_values = df.column("close")?.clone();
    let mut returns = vec![0.0; close_values.len()];
    
    for i in 1..close_values.len() {
        if close_values[i-1] != 0.0 {
            returns[i] = (close_values[i] - close_values[i-1]) / close_values[i-1];
        }
    }
    
    // 计算未来N期的波动率
    let mut result = vec![0.0; close_values.len()];
    for i in 0..(close_values.len() - n) {
        let mut sum = 0.0;
        let mut count = 0;
        for j in 0..n {
            if i + j + 1 < returns.len() {
                sum += returns[i + j + 1] * returns[i + j + 1];
                count += 1;
            }
        }
        if count > 0 {
            result[i] = (sum / count as f64).sqrt();
        }
    }

    df.with_column(col_name, result)?;
    
    Ok(df)
}

/// 未来 N 根K线的最大收益盈亏比
/// 
/// 注意：
/// 1. 约束盈亏比的范围是 [0, 10]
/// 2. 当未来 N 根K线内收益最小值为0时，会导致计算结果为无穷大，此时将结果设置为10
/// 
/// # Arguments
/// * `df` - DataFrame结构的标准K线数据
/// * `kwargs` - 其他参数
///     - tag: str, 因子字段标记
///     - n: int, 计算未来N根K线的收益盈亏比
/// 
/// # Returns
/// * `DataFrame` - 包含新因子列的DataFrame
pub fn ret004(mut df: DataFrame, kwargs: HashMap<String, String>) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let tag = kwargs.get("tag").unwrap_or(&"A".to_string()).clone();
    let n: usize = kwargs.get("n").unwrap_or(&"5".to_string()).parse().unwrap_or(5);

    let col_name = format!("F#RET004#{}", tag);
    
    // 获取close列
    let close_values = df.column("close")?.clone();
    let mut result = vec![0.0; close_values.len()];
    
    for i in 0..(close_values.len() - n) {
        let start_price = close_values[i];
        if start_price == 0.0 {
            continue;
        }
        
        let mut max_val = close_values[i];
        let mut min_val = close_values[i];
        
        for j in 1..=n {
            if i + j < close_values.len() {
                if close_values[i + j] > max_val {
                    max_val = close_values[i + j];
                }
                if close_values[i + j] < min_val {
                    min_val = close_values[i + j];
                }
            }
        }
        
        let max_ret = max_val / start_price - 1.0;
        let min_ret = min_val / start_price - 1.0;
        
        let abs_min_ret = min_ret.abs();
        if abs_min_ret > 1e-10 {  // 避免除零
            let ratio = max_ret / abs_min_ret;
            result[i] = ratio.clamp(0.0, 10.0);
        } else {
            result[i] = 10.0;  // 当分母接近0时设为上限值
        }
    }

    df.with_column(col_name, result)?;
    
    Ok(df)
}

/// 未来 N 根K线的逐K胜率
/// 
/// # Arguments
/// * `df` - DataFrame结构的标准K线数据
/// * `kwargs` - 其他参数
///     - tag: str, 因子字段标记
///     - n: int, 滚动窗口大小
/// 
/// # Returns
/// * `DataFrame` - 包含新因子列的DataFrame
pub fn ret005(mut df: DataFrame, kwargs: HashMap<String, String>) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let tag = kwargs.get("tag").unwrap_or(&"A".to_string()).clone();
    let n: usize = kwargs.get("n").unwrap_or(&"5".to_string()).parse().unwrap_or(5);

    let col_name = format!("F#RET005#{}", tag);
    
    // 获取close列并计算收益率
    let close_values = df.column("close")?.clone();
    let mut returns = vec![0.0; close_values.len()];
    
    for i in 1..close_values.len() {
        if close_values[i-1] != 0.0 {
            returns[i] = (close_values[i] - close_values[i-1]) / close_values[i-1];
        }
    }
    
    // 计算未来N期的胜率
    let mut result = vec![0.0; close_values.len()];
    for i in 0..(close_values.len() - n) {
        let mut positive_count = 0;
        let mut total_count = 0;
        
        for j in 1..=n {
            if i + j < returns.len() {
                if returns[i + j] > 0.0 {
                    positive_count += 1;
                }
                total_count += 1;
            }
        }
        
        if total_count > 0 {
            result[i] = positive_count as f64 / total_count as f64;
        }
    }

    df.with_column(col_name, result)?;
    
    Ok(df)
}

/// 未来 N 根K线的逐K盈亏比
/// 
/// 注意：
/// 1. 约束盈亏比的范围是 [0, 10]
/// 
/// # Arguments
/// * `df` - DataFrame结构的标准K线数据
/// * `kwargs` - 其他参数
///     - tag: str, 因子字段标记
///     - n: int, 滚动窗口大小
/// 
/// # Returns
/// * `DataFrame` - 包含新因子列的DataFrame
pub fn ret006(mut df: DataFrame, kwargs: HashMap<String, String>) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let tag = kwargs.get("tag").unwrap_or(&"A".to_string()).clone();
    let n: usize = kwargs.get("n").unwrap_or(&"5".to_string()).parse().unwrap_or(5);

    let col_name = format!("F#RET006#{}", tag);
    
    // 获取close列并计算收益率
    let close_values = df.column("close")?.clone();
    let mut returns = vec![0.0; close_values.len()];
    
    for i in 1..close_values.len() {
        if close_values[i-1] != 0.0 {
            returns[i] = (close_values[i] - close_values[i-1]) / close_values[i-1];
        }
    }
    
    // 计算未来N期的盈亏比
    let mut result = vec![0.0; close_values.len()];
    for i in 0..(close_values.len() - n) {
        let mut win_total = 0.0;
        let mut loss_total = 0.0;
        let mut win_count = 0;
        let mut loss_count = 0;
        
        for j in 1..=n {
            if i + j < returns.len() {
                if returns[i + j] > 0.0 {
                    win_total += returns[i + j];
                    win_count += 1;
                } else if returns[i + j] < 0.0 {
                    loss_total += returns[i + j];
                    loss_count += 1;
                }
            }
        }
        
        let avg_win = if win_count > 0 { win_total / win_count as f64 } else { 0.0 };
        let avg_loss = if loss_count > 0 { loss_total.abs() / loss_count as f64 } else { 1e-10 }; // 避免除零
        
        let ratio = avg_win / avg_loss;
        result[i] = ratio.clamp(0.0, 10.0);
    }

    df.with_column(col_name, result)?;
    
    Ok(df)
}

/// 未来 N 根K线的最大跌幅
/// 
/// # Arguments
/// * `df` - DataFrame结构的标准K线数据
/// * `kwargs` - 其他参数
///     - tag: str, 因子字段标记
///     - n: int, 滚动窗口大小
/// 
/// # Returns
/// * `DataFrame` - 包含新因子列的DataFrame
pub fn ret007(mut df: DataFrame, kwargs: HashMap<String, String>) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let tag = kwargs.get("tag").unwrap_or(&"A".to_string()).clone();
    let n: usize = kwargs.get("n").unwrap_or(&"5".to_string()).parse().unwrap_or(5);

    let col_name = format!("F#RET007#{}", tag);
    
    // 获取close列
    let close_values = df.column("close")?.clone();
    let mut result = vec![0.0; close_values.len()];
    
    for i in 0..(close_values.len() - n) {
        let start_price = close_values[i];
        if start_price == 0.0 {
            continue;
        }
        
        let mut min_val = close_values[i];
        
        for j in 1..=n {
            if i + j < close_values.len() && close_values[i + j] < min_val {
                min_val = close_values[i + j];
            }
        }
        
        result[i] = min_val / start_price - 1.0;
    }

    df.with_column(col_name, result)?;
    
    Ok(df)
}

/// 未来 N 根K线的最大涨幅
/// 
/// # Arguments
/// * `df` - DataFrame结构的标准K线数据
/// * `kwargs` - 其他参数
///     - tag: str, 因子字段标记
///     - n: int, 滚动窗口大小
/// 
/// # Returns
/// * `DataFrame` - 包含新因子列的DataFrame
pub fn ret008(mut df: DataFrame, kwargs: HashMap<String, String>) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let tag = kwargs.get("tag").unwrap_or(&"A".to_string()).clone();
    let n: usize = kwargs.get("n").unwrap_or(&"5".to_string()).parse().unwrap_or(5);

    let col_name = format!("F#RET008#{}", tag);
    
    // 获取close列
    let close_values = df.column("close")?.clone();
    let mut result = vec![0.0; close_values.len()];
    
    for i in 0..(close_values.len() - n) {
        let start_price = close_values[i];
        if start_price == 0.0 {
            continue;
        }
        
        let mut max_val = close_values[i];
        
        for j in 1..=n {
            if i + j < close_values.len() && close_values[i + j] > max_val {
                max_val = close_values[i + j];
            }
        }
        
        result[i] = max_val / start_price - 1.0;
    }

    df.with_column(col_name, result)?;
    
    Ok(df)
}