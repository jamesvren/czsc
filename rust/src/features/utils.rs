//! 特征相关的工具函数

use std::collections::HashMap;
use std::vec::Vec;

// 简单的DataFrame结构，用于替代polars
#[derive(Debug, Clone)]
pub struct DataFrame {
    pub columns: HashMap<String, Vec<f64>>,
    pub length: usize,
}

impl DataFrame {
    pub fn new(columns: HashMap<String, Vec<f64>>) -> Self {
        let length = columns.values().next().map_or(0, |col| col.len());
        DataFrame { columns, length }
    }

    pub fn column(&self, name: &str) -> Result<&Vec<f64>, Box<dyn std::error::Error>> {
        self.columns.get(name).ok_or_else(|| {
            Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, format!("Column {} not found", name))) as Box<dyn std::error::Error>
        })
    }

    pub fn with_column(&mut self, name: String, values: Vec<f64>) -> Result<(), Box<dyn std::error::Error>> {
        if values.len() != self.length {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Column length does not match DataFrame length"
            )));
        }
        self.columns.insert(name, values);
        Ok(())
    }

    pub fn unique(&self, col: &str) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
        let values = self.column(col)?;
        let mut unique_values = Vec::new();
        for val in values {
            if !unique_values.contains(val) {
                unique_values.push(*val);
            }
        }
        Ok(unique_values)
    }
}

// 辅助函数：计算移动平均
fn rolling_mean(values: &[f64], window: usize) -> Vec<Option<f64>> {
    let mut result = vec![None; values.len()];
    if values.len() < window {
        return result;
    }
    
    for i in 0..=(values.len() - window) {
        let mut sum = 0.0;
        for j in 0..window {
            sum += values[i + j];
        }
        result[i + window - 1] = Some(sum / window as f64);
    }
    result
}

// 辅助函数：计算标准差
fn rolling_std(values: &[f64], window: usize) -> Vec<Option<f64>> {
    let mut result = vec![None; values.len()];
    if values.len() < window {
        return result;
    }
    
    for i in 0..=(values.len() - window) {
        let mut sum = 0.0;
        for j in 0..window {
            sum += values[i + j];
        }
        let mean = sum / window as f64;
        
        let mut variance = 0.0;
        for j in 0..window {
            variance += (values[i + j] - mean) * (values[i + j] - mean);
        }
        variance /= window as f64;
        result[i + window - 1] = Some(variance.sqrt());
    }
    result
}

// 辅助函数：计算两个序列的相关系数
fn correlation(x: &[f64], y: &[f64]) -> Option<f64> {
    if x.len() != y.len() || x.is_empty() {
        return None;
    }
    
    let n = x.len() as f64;
    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    let mut sum_xy = 0.0;
    let mut sum_x_sq = 0.0;
    let mut sum_y_sq = 0.0;
    
    for i in 0..x.len() {
        sum_x += x[i];
        sum_y += y[i];
        sum_xy += x[i] * y[i];
        sum_x_sq += x[i] * x[i];
        sum_y_sq += y[i] * y[i];
    }
    
    let numerator = n * sum_xy - sum_x * sum_y;
    let denom_x = n * sum_x_sq - sum_x * sum_x;
    let denom_y = n * sum_y_sq - sum_y * sum_y;
    let denominator = (denom_x * denom_y).sqrt();
    
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

// 辅助函数：计算百分比变化
fn pct_change(values: &[f64]) -> Vec<Option<f64>> {
    let mut result = vec![None; values.len()];
    for i in 1..values.len() {
        if values[i-1] != 0.0 {
            result[i] = Some((values[i] - values[i-1]) / values[i-1]);
        } else {
            result[i] = Some(0.0);
        }
    }
    result
}

// 辅助函数：获取前n个元素的切片
fn take_n<T: Clone>(vec: &[T], n: usize) -> Vec<T> {
    vec.iter().take(n).cloned().collect()
}

// 辅助函数：获取唯一的值
fn unique<T: PartialEq + Clone>(vec: &[T]) -> Vec<T> {
    let mut unique_values = Vec::new();
    for item in vec {
        if !unique_values.contains(item) {
            unique_values.push(item.clone());
        }
    }
    unique_values
}

// 辅助函数：滚动相关系数
fn rolling_correlation(x: &[f64], y: &[f64], window: usize) -> Vec<Option<f64>> {
    let mut result = vec![None; x.len()];
    if x.len() < window || y.len() < window {
        return result;
    }
    
    for i in 0..=(x.len() - window) {
        let x_slice = &x[i..i+window];
        let y_slice = &y[i..i+window];
        result[i + window - 1] = correlation(x_slice, y_slice);
    }
    result
}

/// 事件类因子的判断函数
/// 
/// 事件因子的特征：多头事件发生时，因子值为1；空头事件发生时，因子值为-1；其他情况，因子值为0。
/// 
/// # Arguments
/// * `df` - DataFrame
/// * `col` - str, 因子字段名称
/// 
/// # Returns
/// * `bool` - 是否为事件类因子
pub fn is_event_feature(df: &DataFrame, col: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let unique_values = df.unique(col)?;
    let all_valid = unique_values.iter().all(|&x| x == 0.0 || x == 1.0 || x == -1.0);
    Ok(all_valid)
}

/// 滚动计算两个序列的相关系数
/// 
/// # Arguments
/// * `df` - DataFrame
/// * `col1` - str
/// * `col2` - str
/// * `kwargs` - dict
///     - window: int, default 300, 滚动窗口大小
///     - min_periods: int, default 100, 最小观测数量
///     - new_col: str, 新列名
/// 
/// # Returns
/// * `DataFrame` - 包含新列的DataFrame
pub fn rolling_corr(
    mut df: DataFrame,
    col1: &str,
    col2: &str,
    kwargs: HashMap<String, String>
) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let window: usize = kwargs.get("window").unwrap_or(&"300".to_string()).parse().unwrap_or(300);
    let min_periods: usize = kwargs.get("min_periods").unwrap_or(&"100".to_string()).parse().unwrap_or(100);
    let new_col = kwargs.get("new_col").unwrap_or(&format!("{}_corr_{}", col1, col2)).clone();

    let series1 = df.column(col1)?;
    let series2 = df.column(col2)?;

    let correlations = rolling_correlation(series1, series2, window);
    let filled_correlations: Vec<f64> = correlations
        .iter()
        .map(|&opt| opt.unwrap_or(0.0))
        .collect();

    df.with_column(new_col, filled_correlations)?;
    Ok(df)
}

/// 计算序列的滚动排名
/// 
/// # Arguments
/// * `df` - DataFrame, 待计算的数据
/// * `col` - str, 待计算的列
/// * `kwargs` - dict
///     - window: int, 滚动窗口大小, 默认为300
///     - min_periods: int, 最小计算周期, 默认为100
///     - new_col: str, 新列名
/// 
/// # Returns
/// * `DataFrame` - 包含新列的DataFrame
pub fn rolling_rank(
    mut df: DataFrame,
    col: &str,
    kwargs: HashMap<String, String>
) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let window: usize = kwargs.get("window").unwrap_or(&"300".to_string()).parse().unwrap_or(300);
    let min_periods: usize = kwargs.get("min_periods").unwrap_or(&"100".to_string()).parse().unwrap_or(100);
    let new_col = kwargs.get("new_col").unwrap_or(&format!("{}_rank", col)).clone();

    let series = df.column(col)?;
    let mut ranks = vec![0.0; series.len()];
    
    for i in 0..series.len() {
        let start = if i >= window { i - window + 1 } else { 0 };
        let end = i + 1;
        let slice_len = end - start;
        
        if slice_len >= min_periods {
            let mut slice_with_idx: Vec<(usize, f64)> = (start..end)
                .enumerate()
                .map(|(idx, actual_idx)| (idx, series[actual_idx]))
                .collect();
            
            slice_with_idx.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            
            for (sorted_idx, (orig_idx, _)) in slice_with_idx.iter().enumerate() {
                let actual_idx = start + orig_idx;
                if actual_idx == i {
                    ranks[i] = (sorted_idx + 1) as f64 / slice_len as f64;
                    break;
                }
            }
        }
    }

    df.with_column(new_col, ranks)?;
    Ok(df)
}

/// 计算序列的滚动归一化值
/// 
/// # Arguments
/// * `df` - DataFrame, 待计算的数据
/// * `col` - str, 待计算的列
/// * `kwargs` - dict
///     - window: int, 滚动窗口大小, 默认为300
///     - min_periods: int, 最小计算周期, 默认为100
///     - new_col: str, 新列名
/// 
/// # Returns
/// * `DataFrame` - 包含新列的DataFrame
pub fn rolling_norm(
    mut df: DataFrame,
    col: &str,
    kwargs: HashMap<String, String>
) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let window: usize = kwargs.get("window").unwrap_or(&"300".to_string()).parse().unwrap_or(300);
    let min_periods: usize = kwargs.get("min_periods").unwrap_or(&"100".to_string()).parse().unwrap_or(100);
    let new_col = kwargs.get("new_col").unwrap_or(&format!("{}_norm", col)).clone();

    let series = df.column(col)?;
    let means = rolling_mean(series, window);
    let stds = rolling_std(series, window);
    
    let mut normalized = vec![0.0; series.len()];
    for i in 0..series.len() {
        if let (Some(mean), Some(std_val)) = (means[i], stds[i]) {
            if std_val != 0.0 {
                normalized[i] = (series[i] - mean) / std_val;
            } else {
                normalized[i] = 0.0;
            }
        } else {
            normalized[i] = 0.0;
        }
    }

    df.with_column(new_col, normalized)?;
    Ok(df)
}

/// 对序列进行滚动归一化
/// 
/// # Arguments
/// * `df` - DataFrame, 待计算的数据
/// * `col` - str, 待计算的列
/// * `kwargs` - dict
///     - window: int, 滚动窗口大小, 默认为300
///     - min_periods: int, 最小计算周期, 默认为100
///     - new_col: str, 新列名
///     - method: str, 归一化方法
/// 
/// # Returns
/// * `DataFrame` - 包含新列的DataFrame
pub fn rolling_scale(
    mut df: DataFrame,
    col: &str,
    kwargs: HashMap<String, String>
) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let window: usize = kwargs.get("window").unwrap_or(&"300".to_string()).parse().unwrap_or(300);
    let min_periods: usize = kwargs.get("min_periods").unwrap_or(&"100".to_string()).parse().unwrap_or(100);
    let method = kwargs.get("method").unwrap_or(&"scale".to_string()).clone();
    let new_col = kwargs.get("new_col").unwrap_or(&format!("{}_scale", col)).clone();

    let series = df.column(col)?;
    let means = rolling_mean(series, window);
    let stds = rolling_std(series, window);
    
    let mut scaled = vec![0.0; series.len()];
    
    match method.as_str() {
        "minmax_scale" => {
            // Min-Max scaling to [-1, 1]
            for i in 0..series.len() {
                if let Some(_) = means[i] {
                    // Find min and max in the window
                    let start = if i >= window { i - window + 1 } else { 0 };
                    let end = i + 1;
                    let window_slice = &series[start..end];
                    
                    let min_val = window_slice.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                    let max_val = window_slice.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
                    
                    if max_val != min_val {
                        scaled[i] = -1.0 + 2.0 * (series[i] - min_val) / (max_val - min_val);
                    } else {
                        scaled[i] = 0.0;
                    }
                }
            }
        }
        _ => {
            // Standard scaling: (x - mean) / std
            for i in 0..series.len() {
                if let (Some(mean), Some(std_val)) = (means[i], stds[i]) {
                    if std_val != 0.0 {
                        scaled[i] = (series[i] - mean) / std_val;
                    } else {
                        scaled[i] = 0.0;
                    }
                } else {
                    scaled[i] = 0.0;
                }
            }
        }
    }

    df.with_column(new_col, scaled)?;
    Ok(df)
}