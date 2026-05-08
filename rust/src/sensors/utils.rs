//! 传感器工具函数
//!
//! 提供通用的传感器相关工具函数

use std::collections::HashMap;

/// 最大回撤计算
/// 
/// 计算收益率序列的最大回撤
/// 
/// # Arguments
/// * `returns` - 收益率序列 (Vec<f64>)
/// 
/// # Returns
/// * `(usize, usize, f64)` - (开始位置, 结束位置, 最大回撤值)
pub fn max_drawdown(returns: &[f64]) -> (usize, usize, f64) {
    if returns.is_empty() {
        return (0, 0, 0.0);
    }

    // 计算累计收益曲线
    let mut cumulative_returns = Vec::with_capacity(returns.len());
    let mut current_return = 1.0; // 基准值为1
    
    for &r in returns {
        current_return *= 1.0 + r / 100.0; // 假设输入的是百分比
        cumulative_returns.push(current_return);
    }

    // 计算最大回撤
    let mut max_val = cumulative_returns[0];
    let mut max_val_pos = 0;
    let mut drawdown_start = 0;
    let mut drawdown_end = 0;
    let mut max_drawdown_val = 0.0;

    for (i, &current) in cumulative_returns.iter().enumerate() {
        if current > max_val {
            max_val = current;
            max_val_pos = i;
        }

        let drawdown = (max_val - current) / max_val;
        if drawdown > max_drawdown_val {
            max_drawdown_val = drawdown;
            drawdown_start = max_val_pos;
            drawdown_end = i;
        }
    }

    (drawdown_start, drawdown_end, max_drawdown_val)
}

/// 换手率计算
/// 
/// 计算持仓的换手率
/// 
/// # Arguments
/// * `holds` - 持仓数据，包含日期、证券代码和持仓权重
/// 
/// # Returns
/// * `(Vec<HashMap<String, f64>>, f64)` - (换手率数据, 总换手率)
pub fn turnover_rate(holds: &[HashMap<String, f64>]) -> (Vec<HashMap<String, f64>>, f64) {
    let mut turnover_data = Vec::new();
    let mut total_change = 0.0;

    // 这里简化实现，实际应用中需要更复杂的计算逻辑
    for hold in holds {
        let mut row = HashMap::new();
        
        // 假设持有数据包含日期和权重信息
        if let Some(date) = hold.get("date") {
            row.insert("date".to_string(), *date);
        }
        
        if let Some(weight) = hold.get("weight") {
            row.insert("weight".to_string(), *weight);
        }
        
        // 计算当日变化
        let change = hold.get("change").copied().unwrap_or(0.0);
        row.insert("change".to_string(), change);
        total_change += change.abs();
        
        turnover_data.push(row);
    }

    let turnover = total_change / 2.0; // 简化的换手率计算
    (turnover_data, turnover)
}

/// 持仓概念板块效应分析
/// 
/// 分析股票持仓的概念板块效应
/// 
/// # Arguments
/// * `holds` - 持仓数据
/// * `concepts` - 概念板块映射
/// * `top_n` - 选取前N个密集概念
/// * `min_n` - 单股票至少要有N个概念在top_n中
/// 
/// # Returns
/// * `(Vec<HashMap<String, f64>>, Vec<String>)` - (过滤后的持仓, 每期强势概念)
pub fn holds_concepts_effect(
    holds: &[HashMap<String, f64>],
    _concepts: &HashMap<String, Vec<String>>,
    _top_n: usize,
    _min_n: usize,
) -> (Vec<HashMap<String, f64>>, Vec<String>) {
    // 这里简化实现，只返回原始数据和空概念列表
    // 实际应用中需要复杂的概念分析逻辑
    (holds.to_vec(), Vec::new())
}

/// 计算重叠统计（连续出现次数）
/// 
/// 计算给定序列中相同值的连续出现次数
/// 
/// # Arguments
/// * `sequence` - 输入序列
/// * `max_overlap` - 最大允许连续出现次数
/// 
/// # Returns
/// * `Vec<u32>` - 重叠计数序列
pub fn overlap(sequence: &[f64], max_overlap: Option<u32>) -> Vec<u32> {
    let max_ov = max_overlap.unwrap_or(10);
    
    if sequence.is_empty() {
        return vec![];
    }
    
    let mut result = Vec::with_capacity(sequence.len());
    let mut current_count = 1u32;
    
    for i in 1..sequence.len() {
        if sequence[i] == sequence[i-1] {
            current_count = std::cmp::min(current_count + 1, max_ov);
        } else {
            current_count = 1;
        }
        result.push(current_count);
    }
    
    // 第一个元素总是1（单独出现）
    let mut final_result = vec![1u32];
    final_result.extend(result);
    
    // 截断到原序列长度
    final_result.truncate(sequence.len());
    
    final_result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_drawdown() {
        let returns = vec![1.0, -0.5, 2.0, -1.0, 0.5];
        let (start, end, dd) = max_drawdown(&returns);
        
        assert!(start <= end);
        assert!(dd >= 0.0);
    }

    #[test]
    fn test_overlap() {
        let sequence = vec![1.0, 1.0, 2.0, 2.0, 2.0, 3.0, 3.0];
        let result = overlap(&sequence, Some(5));
        
        assert_eq!(result, vec![1, 2, 1, 2, 3, 1, 2]); // [1,1,2,2,2,3,3] -> [1,2,1,2,3,1,2]
    }
}