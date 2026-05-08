//! CZSC 工具模块的 Rust 实现
//! 
//! 包含缓存、数据处理、技术分析等常用功能

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use hex;

pub mod cache;
pub mod io;
pub mod ta;
pub mod data_client;

// 从 __init__.py 中提取的基本工具函数

/// 用去尾法截断小数
pub fn x_round(x: f64, digit: u32) -> f64 {
    let digit_factor = 10f64.powi(digit as i32);
    (x * digit_factor).floor() / digit_factor
}

/// MAC地址获取（模拟实现）
pub fn mac_address() -> String {
    // 生成一个模拟的MAC地址格式 (XX-XX-XX-XX-XX-XX)
    use rand::Rng;
    let mut rng = rand::thread_rng();
    format!(
        "{:02X}-{:02X}-{:02X}-{:02X}-{:02X}-{:02X}",
        rng.gen::<u8>(),
        rng.gen::<u8>(),
        rng.gen::<u8>(),
        rng.gen::<u8>(),
        rng.gen::<u8>(),
        rng.gen::<u8>()
    )
}

/// 频率排序（模拟实现）
pub fn freqs_sorted(freqs: Vec<&str>) -> Vec<&str> {
    let sorted_freqs = vec![
        "Tick", "1分钟", "2分钟", "3分钟", "4分钟", "5分钟", "6分钟", "10分钟", 
        "12分钟", "15分钟", "20分钟", "30分钟", "60分钟", "120分钟", 
        "日线", "周线", "月线", "季线", "年线"
    ];
    
    sorted_freqs.into_iter()
        .filter(|&f| freqs.contains(&f))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x_round() {
        assert_eq!(x_round(3.1415926, 4), 3.1415);
        assert_eq!(x_round(2.7182818, 2), 2.71);
    }

    #[test]
    fn test_mac_address() {
        let mac = mac_address();
        assert!(!mac.is_empty());
        assert_eq!(mac.len(), 17); // Should be in format XX-XX-XX-XX-XX-XX (17 characters)
    }

    #[test]
    fn test_freqs_sorted() {
        let input = vec!["日线", "5分钟", "1分钟"];
        let result = freqs_sorted(input);
        assert_eq!(result, vec!["1分钟", "5分钟", "日线"]);
    }
}