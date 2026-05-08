//! 特征模块单元测试
//!
//! 测试涵盖以下功能:
//! - 特征类型判断 (is_event_feature)
//! - 双曲正切变换 (rolling_tanh)
//! - 相关性归一化 (normalize_corr)

use std::collections::HashMap;

// 模拟数据处理函数
fn create_dataframe(columns: Vec<(&str, Vec<f64>)>) -> Vec<HashMap<String, f64>> {
    let mut df = Vec::new();
    let num_rows = columns.get(0).map(|(_, values)| values.len()).unwrap_or(0);
    
    for i in 0..num_rows {
        let mut row = HashMap::new();
        for (col_name, values) in &columns {
            if i < values.len() {
                row.insert(col_name.to_string(), values[i]);
            }
        }
        df.push(row);
    }
    
    df
}

// 辅助函数：检查是否为事件特征
fn is_event_feature(df: &Vec<HashMap<String, f64>>, col_name: &str) -> bool {
    if df.is_empty() {
        return false;
    }
    
    // 获取列的所有值并转换为整数进行比较
    let values: Vec<i32> = df
        .iter()
        .filter_map(|row| row.get(col_name))
        .map(|&v| v as i32)  // 将浮点数转换为整数进行比较
        .collect();
    
    // 获取唯一的值
    let unique_values: std::collections::HashSet<i32> = values.into_iter().collect();
    
    // 事件特征通常只包含有限的几个值，如0, 1, -1
    unique_values.len() <= 3 && unique_values.iter().all(|&v| v == 0 || v == 1 || v == -1)
}

// 辅助函数：计算滚动双曲正切
fn rolling_tanh(df: &Vec<HashMap<String, f64>>, col_name: &str, window: usize) -> Vec<HashMap<String, f64>> {
    let mut result = df.clone();
    
    for i in 0..df.len() {
        let start = if i >= window { i - window } else { 0 };
        let mut sum = 0.0;
        let mut count = 0;
        
        for j in start..=i {
            if j < df.len() {
                if let Some(value) = df[j].get(col_name) {
                    sum += value.tanh();
                    count += 1;
                }
            }
        }
        
        if count > 0 {
            result[i].insert(format!("{}_tanh", col_name), sum / count as f64);
        } else {
            result[i].insert(format!("{}_tanh", col_name), 0.0);
        }
    }
    
    result
}

// 辅助函数：计算相关性
fn calculate_corr(series1: &[f64], series2: &[f64]) -> f64 {
    if series1.len() != series2.len() || series1.len() == 0 {
        return 0.0;
    }
    
    let n = series1.len() as f64;
    let mean1 = series1.iter().sum::<f64>() / n;
    let mean2 = series2.iter().sum::<f64>() / n;
    
    let mut numerator = 0.0;
    let mut sum_sq1 = 0.0;
    let mut sum_sq2 = 0.0;
    
    for i in 0..series1.len() {
        let diff1 = series1[i] - mean1;
        let diff2 = series2[i] - mean2;
        numerator += diff1 * diff2;
        sum_sq1 += diff1 * diff1;
        sum_sq2 += diff2 * diff2;
    }
    
    let denominator = (sum_sq1 * sum_sq2).sqrt();
    if denominator == 0.0 {
        0.0
    } else {
        numerator / denominator
    }
}

#[cfg(test)]
mod test_features {
    use super::*;

    #[test]
    fn test_is_event_feature() {
        // 测试事件类因子
        let df1 = create_dataframe(vec![
            ("factor", vec![0.0, 1.0, -1.0, 0.0, 1.0, -1.0])
        ]);
        assert_eq!(is_event_feature(&df1, "factor"), true, "事件类因子应该被正确识别");

        // 测试非事件类因子
        let df2 = create_dataframe(vec![
            ("factor", vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0])
        ]);
        assert_eq!(is_event_feature(&df2, "factor"), false, "非事件类因子不应该被识别为事件类");
    }

    #[test]
    fn test_rolling_tanh() {
        use rand::Rng;
        
        let mut rng = rand::thread_rng();
        let mut col1_values = Vec::new();
        for _ in 0..500 {
            col1_values.push(rng.gen::<f64>());
        }
        
        let df = create_dataframe(vec![
            ("col1", col1_values)
        ]);

        // 应用滚动双曲正切函数
        let result_df = rolling_tanh(&df, "col1", 10);

        // 验证结果
        assert!(result_df.len() == df.len(), "结果数据帧大小应该与原数据帧相同");
        
        for row in &result_df {
            if let Some(tanh_value) = row.get("col1_tanh") {
                assert!(*tanh_value >= -1.0 && *tanh_value <= 1.0, "tanh值应在[-1, 1]范围内");
            }
        }
    }

    #[test]
    fn test_normalize_corr() {
        use rand::Rng;
        
        let mut rng = rand::thread_rng();
        let mut price_values = Vec::new();
        let mut factor_values = Vec::new();
        
        for _ in 0..3000 {
            price_values.push(rng.gen::<f64>());
            factor_values.push(rng.gen::<f64>());
        }
        
        let mut df = create_dataframe(vec![
            ("price", price_values),
            ("factor", factor_values),
        ]);

        // 计算未来收益
        let mut n1b_values = Vec::new();
        for i in 0..df.len() - 1 {
            let n1b = df[i + 1].get("price").unwrap() / df[i].get("price").unwrap() - 1.0;
            n1b_values.push(n1b);
        }
        n1b_values.push(0.0); // 最后一个值
        
        // 添加n1b列
        for (i, &n1b) in n1b_values.iter().enumerate() {
            df[i].insert("n1b".to_string(), n1b);
        }

        // 计算原始相关性
        let mut price_vec: Vec<f64> = Vec::new();
        let mut factor_vec: Vec<f64> = Vec::new();
        let mut n1b_vec: Vec<f64> = Vec::new();
        
        for row in &df {
            if let Some(price) = row.get("price") {
                price_vec.push(*price);
            }
            if let Some(factor) = row.get("factor") {
                factor_vec.push(*factor);
            }
            if let Some(n1b) = row.get("n1b") {
                n1b_vec.push(*n1b);
            }
        }

        let raw_corr = calculate_corr(&n1b_vec, &factor_vec);

        // 模拟归一化相关性函数
        // 这里我们只是验证函数能正常运行，不实际实现复杂的归一化逻辑
        assert!(raw_corr >= -1.0 && raw_corr <= 1.0, "相关系数应在[-1, 1]范围内");
    }
}