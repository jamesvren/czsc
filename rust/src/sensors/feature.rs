//! 特征选择和分析模块
//!
//! 提供特征工程相关功能，包括特征选择、滚动特征计算等

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// 特征选择器
#[derive(Debug, Clone)]
pub struct FeatureSelector {
    /// 输入数据 (列名 -> 数值向量)
    pub data: HashMap<String, Vec<f64>>,
}

/// 特征重要性结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureImportance {
    /// 特征名称
    pub feature: String,
    /// 重要性得分
    pub score: f64,
}

impl FeatureSelector {
    /// 创建新的特征选择器
    pub fn new(data: HashMap<String, Vec<f64>>) -> Self {
        Self { data }
    }

    /// 执行特征选择
    pub fn select(&self, n_features: Option<usize>) -> Result<HashMap<String, Vec<f64>>, Box<dyn std::error::Error>> {
        let n = n_features.unwrap_or_else(|| {
            std::cmp::min(self.data.len(), 10) // 默认选择最多10个特征
        });
        
        let mut selected = HashMap::new();
        let mut count = 0;
        for (key, value) in &self.data {
            if count >= n {
                break;
            }
            selected.insert(key.clone(), value.clone());
            count += 1;
        }
        
        Ok(selected)
    }

    /// 计算滚动特征
    pub fn rolling_features(&self, windows: &[usize]) -> Result<HashMap<String, Vec<f64>>, Box<dyn std::error::Error>> {
        let mut result = self.data.clone();
        
        for (col_name, values) in &self.data {
            for &window in windows {
                if values.len() >= window {
                    // 计算滚动均值
                    let rolling_means = calculate_rolling_mean(values, window);
                    result.insert(format!("{}_ma{}", col_name, window), rolling_means);

                    // 计算滚动标准差
                    let rolling_std = calculate_rolling_std(values, window);
                    result.insert(format!("{}_std{}", col_name, window), rolling_std);
                }
            }
        }
        
        Ok(result)
    }
}

/// 计算滚动均值
fn calculate_rolling_mean(values: &[f64], window: usize) -> Vec<f64> {
    let mut result = Vec::new();
    
    for i in 0..values.len() {
        if i < window - 1 || window == 0 {
            // 不足窗口大小时，用NaN表示
            result.push(f64::NAN);
        } else {
            let sum: f64 = values[(i + 1 - window)..=i].iter().sum();
            result.push(sum / window as f64);
        }
    }
    
    result
}

/// 计算滚动标准差
fn calculate_rolling_std(values: &[f64], window: usize) -> Vec<f64> {
    let mut result = Vec::new();
    
    for i in 0..values.len() {
        if i < window - 1 || window == 0 {
            // 不足窗口大小时，用NaN表示
            result.push(f64::NAN);
        } else {
            let slice = &values[(i + 1 - window)..=i];
            let mean = slice.iter().sum::<f64>() / slice.len() as f64;
            let variance = slice.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / slice.len() as f64;
            result.push(variance.sqrt());
        }
    }
    
    result
}

/// 计算特征重要性（基于相关系数）
pub fn calculate_feature_importance(
    features: &HashMap<String, Vec<f64>>,
    target: &[f64],
) -> Result<Vec<FeatureImportance>, Box<dyn std::error::Error>> {
    let mut importance_scores = Vec::new();
    
    for (feature_name, feature_values) in features {
        if feature_values.len() != target.len() {
            return Err("Feature and target lengths must match".into());
        }
        
        let correlation = pearson_correlation(feature_values, target);
        let abs_corr = correlation.abs();
        
        importance_scores.push(FeatureImportance {
            feature: feature_name.clone(),
            score: abs_corr,
        });
    }
    
    // 按重要性得分排序
    importance_scores.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    
    Ok(importance_scores)
}

/// 计算皮尔逊相关系数
fn pearson_correlation(x: &[f64], y: &[f64]) -> f64 {
    if x.len() != y.len() || x.len() == 0 {
        return 0.0;
    }
    
    let n = x.len() as f64;
    let sum_x = x.iter().sum::<f64>();
    let sum_y = y.iter().sum::<f64>();
    let sum_xy: f64 = x.iter().zip(y.iter()).map(|(xi, yi)| xi * yi).sum();
    let sum_x_sq: f64 = x.iter().map(|xi| xi * xi).sum();
    let sum_y_sq: f64 = y.iter().map(|yi| yi * yi).sum();
    
    let numerator = n * sum_xy - sum_x * sum_y;
    let denominator = ((n * sum_x_sq - sum_x * sum_x) * (n * sum_y_sq - sum_y * sum_y)).sqrt();
    
    if denominator == 0.0 {
        0.0
    } else {
        numerator / denominator
    }
}

/// 创建价格特征
pub fn create_price_features(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> Result<HashMap<String, Vec<f64>>, Box<dyn std::error::Error>> {
    if open.len() != high.len() || high.len() != low.len() || low.len() != close.len() {
        return Err("All price arrays must have the same length".into());
    }
    
    let mut features = HashMap::new();
    
    // 涨跌幅
    let mut returns = Vec::with_capacity(close.len());
    returns.push(0.0); // 第一天无涨跌
    for i in 1..close.len() {
        if close[i-1] != 0.0 {
            returns.push((close[i] - close[i-1]) / close[i-1]);
        } else {
            returns.push(0.0);
        }
    }
    features.insert("returns".to_string(), returns);
    
    // 振幅
    let mut amplitude = Vec::with_capacity(high.len());
    for i in 0..high.len() {
        if close[i] != 0.0 {
            amplitude.push((high[i] - low[i]) / close[i]);
        } else {
            amplitude.push(0.0);
        }
    }
    features.insert("amplitude".to_string(), amplitude);
    
    // 上影线
    let mut upper_shadow = Vec::with_capacity(high.len());
    for i in 0..high.len() {
        let max_open_close = open[i].max(close[i]);
        upper_shadow.push(if max_open_close != 0.0 {
            (high[i] - max_open_close) / max_open_close
        } else {
            0.0
        });
    }
    features.insert("upper_shadow".to_string(), upper_shadow);
    
    // 下影线
    let mut lower_shadow = Vec::with_capacity(low.len());
    for i in 0..low.len() {
        let min_open_close = open[i].min(close[i]);
        lower_shadow.push(if min_open_close != 0.0 {
            (min_open_close - low[i]) / min_open_close
        } else {
            0.0
        });
    }
    features.insert("lower_shadow".to_string(), lower_shadow);
    
    Ok(features)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_selector_creation() {
        let mut data = HashMap::new();
        data.insert("feature1".to_string(), vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        data.insert("feature2".to_string(), vec![2.0, 3.0, 4.0, 5.0, 6.0]);
        
        let selector = FeatureSelector::new(data);
        assert_eq!(selector.data.len(), 2);
    }

    #[test]
    fn test_feature_selection() {
        let mut data = HashMap::new();
        data.insert("feature1".to_string(), vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        data.insert("feature2".to_string(), vec![2.0, 3.0, 4.0, 5.0, 6.0]);
        data.insert("feature3".to_string(), vec![3.0, 4.0, 5.0, 6.0, 7.0]);
        
        let selector = FeatureSelector::new(data);
        let selected = selector.select(Some(2)).unwrap();
        
        assert_eq!(selected.len(), 2);
    }

    #[test]
    fn test_rolling_features() {
        let mut data = HashMap::new();
        data.insert("price".to_string(), vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]);
        
        let selector = FeatureSelector::new(data);
        let features = selector.rolling_features(&[3]).unwrap();
        
        assert!(features.contains_key("price_ma3"));
    }

    #[test]
    fn test_pearson_correlation() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![2.0, 4.0, 6.0, 8.0, 10.0]; // Perfect positive correlation
        
        let corr = pearson_correlation(&x, &y);
        assert!((corr - 1.0).abs() < 0.001);
    }
}