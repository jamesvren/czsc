//! 事件检测和匹配模块
//!
//! 提供事件检测、匹配和分析功能

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// 事件匹配器
#[derive(Debug, Clone)]
pub struct EventMatcher {
    /// 事件模式定义
    pub pattern: HashMap<String, String>,
}

/// 检测到的事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedEvent {
    /// 事件时间戳
    pub timestamp: String,
    /// 事件类型
    pub event_type: String,
    /// 事件描述
    pub description: String,
    /// 相关数据
    pub data: HashMap<String, String>,
}

impl EventMatcher {
    /// 创建新的事件匹配器
    pub fn new(pattern: HashMap<String, String>) -> Self {
        Self { pattern }
    }

    /// 检测事件
    pub fn detect_events(&self, data: &[HashMap<String, f64>]) -> Result<Vec<DetectedEvent>, Box<dyn std::error::Error>> {
        let mut events = Vec::new();

        // 这里可以根据模式检测事件
        // 示例：检测价格突破事件
        if let Some(condition) = self.pattern.get("condition") {
            match condition.as_str() {
                "price_breakout" => {
                    // 检测价格突破
                    for (i, item) in data.iter().enumerate() {
                        if i > 0 && data.len() > i {
                            if let (Some(current_high), Some(prev_high)) = (item.get("high"), data[i-1].get("high")) {
                                if current_high > prev_high {
                                    events.push(DetectedEvent {
                                        timestamp: format!("index_{}", i),
                                        event_type: "breakout".to_string(),
                                        description: format!("Price breakout detected at index {}", i),
                                        data: HashMap::new(),
                                    });
                                }
                            }
                        }
                    }
                }
                "volume_spike" => {
                    // 检测成交量突增
                    for (i, item) in data.iter().enumerate() {
                        if i > 0 && data.len() > i {
                            if let (Some(current_vol), Some(prev_vol)) = (item.get("volume"), data[i-1].get("volume")) {
                                if *current_vol > *prev_vol * 1.5 { // 成交量增加50%以上
                                    events.push(DetectedEvent {
                                        timestamp: format!("index_{}", i),
                                        event_type: "volume_spike".to_string(),
                                        description: format!("Volume spike detected at index {}", i),
                                        data: HashMap::new(),
                                    });
                                }
                            }
                        }
                    }
                }
                _ => {
                    // 其他条件处理
                }
            }
        }

        Ok(events)
    }
}

/// 检测简单事件（基于单行条件）
pub fn detect_events_simple<F>(data: &[HashMap<String, f64>], condition: F) -> Result<Vec<bool>, Box<dyn std::error::Error>>
where
    F: Fn(&HashMap<String, f64>) -> bool,
{
    let mut results = Vec::with_capacity(data.len());
    
    for item in data {
        results.push(condition(item));
    }
    
    Ok(results)
}

/// 事件统计分析
pub fn analyze_events(events: &[DetectedEvent]) -> HashMap<String, usize> {
    let mut stats = HashMap::new();
    
    for event in events {
        *stats.entry(event.event_type.clone()).or_insert(0) += 1;
    }
    
    stats
}

/// 检测多重事件
pub fn detect_multiple_events(data: &[HashMap<String, f64>], patterns: &[HashMap<String, String>]) -> Result<HashMap<String, Vec<DetectedEvent>>, Box<dyn std::error::Error>> {
    let mut all_events = HashMap::new();
    
    for (i, pattern) in patterns.iter().enumerate() {
        let matcher = EventMatcher::new(pattern.clone());
        let events = matcher.detect_events(data)?;
        all_events.insert(format!("pattern_{}", i), events);
    }
    
    Ok(all_events)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_matcher_creation() {
        let mut pattern = HashMap::new();
        pattern.insert("type".to_string(), "price_pattern".to_string());
        pattern.insert("condition".to_string(), "price_breakout".to_string());
        
        let matcher = EventMatcher::new(pattern);
        assert!(!matcher.pattern.is_empty());
    }

    #[test]
    fn test_event_detection() {
        let mut item1 = HashMap::new();
        item1.insert("high".to_string(), 100.0);
        item1.insert("low".to_string(), 95.0);
        
        let mut item2 = HashMap::new();
        item2.insert("high".to_string(), 105.0);
        item2.insert("low".to_string(), 100.0);
        
        let mut item3 = HashMap::new();
        item3.insert("high".to_string(), 103.0);
        item3.insert("low".to_string(), 98.0);
        
        let data = vec![item1, item2, item3];
        
        let mut pattern = HashMap::new();
        pattern.insert("condition".to_string(), "price_breakout".to_string());
        
        let matcher = EventMatcher::new(pattern);
        let events = matcher.detect_events(&data).unwrap();
        
        // 应该检测到一个突破事件（第二个数据点的high高于第一个）
        assert!(!events.is_empty());
    }

    #[test]
    fn test_event_analysis() {
        let events = vec![
            DetectedEvent {
                timestamp: "2023-01-01".to_string(),
                event_type: "buy".to_string(),
                description: "Buy signal".to_string(),
                data: HashMap::new(),
            },
            DetectedEvent {
                timestamp: "2023-01-02".to_string(),
                event_type: "sell".to_string(),
                description: "Sell signal".to_string(),
                data: HashMap::new(),
            },
            DetectedEvent {
                timestamp: "2023-01-03".to_string(),
                event_type: "buy".to_string(),
                description: "Buy signal".to_string(),
                data: HashMap::new(),
            },
        ];
        
        let stats = analyze_events(&events);
        assert_eq!(stats.get("buy"), Some(&2));
        assert_eq!(stats.get("sell"), Some(&1));
    }
}