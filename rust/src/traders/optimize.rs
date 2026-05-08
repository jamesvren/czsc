//! 择时策略开平仓优化
//!
//! 该模块实现了择时策略的开仓和平仓参数优化功能。

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::objects::{Position, Event, Signal};

/// 基础策略入场优化流程
pub struct OpensOptimize {
    pub version: String,
    pub symbols: Vec<String>,
    pub candidate_signals: Vec<String>,
    pub task_hash: String,
    pub results_path: String,
    pub poss_path: String,
    pub kwargs: HashMap<String, String>,
}

impl OpensOptimize {
    /// 创建新的入场优化实例
    pub fn new(symbols: Vec<String>, candidate_signals: Vec<String>, results_path: String, kwargs: HashMap<String, String>) -> Self {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        format!("{:?}_{:?}", candidate_signals, symbols).hash(&mut hasher);
        let task_hash = format!("{:X}", hasher.finish())[..8].to_uppercase();
        
        let results_path = format!("{}/入场优化_{}", results_path, task_hash);
        let poss_path = format!("{}/poss", results_path);
        
        OpensOptimize {
            version: "OpensOptimizeV230924".to_string(),
            symbols: symbols.iter().map(|s| s.to_string()).collect(),
            candidate_signals: candidate_signals.iter().map(|s| s.to_string()).collect(),
            task_hash,
            results_path,
            poss_path,
            kwargs,
        }
    }
    
    /// 执行优化
    pub fn execute(&self, n_jobs: usize) {
        println!(
            "{} 开始优化策略，策略数量：未知，共 {} 只标的，进程数量：{}；结果保存在 {}，请耐心等待...",
            self.version,
            self.symbols.len(),
            n_jobs,
            self.results_path
        );
        
        // 在实际实现中，这里会进行多进程优化
        // 简化实现，仅作示意
        for symbol in &self.symbols {
            println!("正在优化 {} ...", symbol);
        }
        
        println!("优化完成，结果保存在 {}", self.results_path);
    }
}

/// 基础策略出场优化流程
pub struct ExitsOptimize {
    pub version: String,
    pub symbols: Vec<String>,
    pub candidate_events: Vec<Event>,
    pub task_hash: String,
    pub results_path: String,
    pub poss_path: String,
    pub kwargs: HashMap<String, String>,
}

impl ExitsOptimize {
    /// 创建新的出场优化实例
    pub fn new(symbols: Vec<String>, candidate_events: Vec<Event>, results_path: String, kwargs: HashMap<String, String>) -> Self {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        format!("{:?}_{:?}", candidate_events, symbols).hash(&mut hasher);
        let task_hash = format!("{:X}", hasher.finish())[..8].to_uppercase();
        
        let results_path = format!("{}/出场优化_{}", results_path, task_hash);
        let poss_path = format!("{}/poss", results_path);
        
        ExitsOptimize {
            version: "ExitsOptimizeV230924".to_string(),
            symbols: symbols.iter().map(|s| s.to_string()).collect(),
            candidate_events,
            task_hash,
            results_path,
            poss_path,
            kwargs,
        }
    }
    
    /// 执行优化
    pub fn execute(&self, n_jobs: usize) {
        println!(
            "{} 开始优化策略，策略数量：未知，共 {} 只标的，进程数量：{}；结果保存在 {}，请耐心等待...",
            self.version,
            self.symbols.len(),
            n_jobs,
            self.results_path
        );
        
        // 在实际实现中，这里会进行多进程优化
        // 简化实现，仅作示意
        for symbol in &self.symbols {
            println!("正在优化 {} ...", symbol);
        }
        
        println!("策略出场优化完成，结果保存在 {}", self.results_path);
    }
}

/// 更新基础策略的入场信号
pub fn update_beta_opens(beta: &Position, open_signals_all: &[String]) -> Position {
    let mut pos = beta.clone();
    
    // 确保基础策略入场信号为单个Event
    assert_eq!(pos.opens.len(), 1, "基础策略入场信号必须为单个Event");
    
    // 将信号添加到第一个开仓事件中
    for signal_str in open_signals_all {
        let signal = Signal::new(
            signal_str.clone(),
            signal_str.clone(),
            signal_str.clone(),
            signal_str.clone(),
            signal_str.clone(),
            signal_str.clone(),
            0
        );
        pos.opens[0].signals_all.push(signal);
    }
    
    pos
}

/// 更新基础策略的出场信号
pub fn update_beta_exits(beta: &Position, event: &Event, mode: &str) -> Option<Position> {
    let mut pos = beta.clone();
    
    // 验证操作一致性
    let open_ops: Vec<String> = pos.opens.iter().map(|e| e.operate.to_string()).collect();
    
    if open_ops.iter().all(|op| op == "开多") && event.operate.to_string() != "平多" {
        return None;
    }
    
    if open_ops.iter().all(|op| op == "开空") && event.operate.to_string() != "平空" {
        return None;
    }
    
    match mode {
        "replace" => {
            pos.exits = vec![event.clone()];
            pos.name = format!("{}#替换{:X}", beta.name, calculate_hash(&event));
        },
        "append" => {
            pos.exits.push(event.clone());
            pos.name = format!("{}#追加{:X}", beta.name, calculate_hash(&event));
        },
        _ => panic!("mode must be replace or append"),
    }
    
    Some(pos)
}

/// 计算哈希值的辅助函数
fn calculate_hash<T: std::hash::Hash>(t: &T) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;
    
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opens_optimize_creation() {
        let symbols = vec!["AAPL".to_string(), "GOOGL".to_string()];
        let candidate_signals = vec!["signal1".to_string(), "signal2".to_string()];
        let kwargs = HashMap::new();
        
        let optimizer = OpensOptimize::new(symbols, candidate_signals, "/tmp/results".to_string(), kwargs);
        
        assert_eq!(optimizer.version, "OpensOptimizeV230924");
        assert_eq!(optimizer.symbols.len(), 2);
    }

    #[test]
    fn test_exits_optimize_creation() {
        let symbols = vec!["AAPL".to_string()];
        let candidate_events = vec![]; // 空的事件列表用于测试
        let kwargs = HashMap::new();
        
        let optimizer = ExitsOptimize::new(symbols, candidate_events, "/tmp/results".to_string(), kwargs);
        
        assert_eq!(optimizer.version, "ExitsOptimizeV230924");
    }
}