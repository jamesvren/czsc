//! CZSC Utils 模块
//! 
//! 提供各种实用工具函数，包括：
//! - 缓存管理
//! - 文件IO操作
//! - 技术分析指标
//! - 数据客户端
//! - 基础工具函数

pub mod cache;
pub mod io;
pub mod ta;
pub mod data_client;

pub use cache::*;
pub use io::*;
pub use ta::*;
pub use data_client::*;

/// 用去尾法截断小数
pub fn x_round(x: f64, n: usize) -> f64 {
    let tmp = 10_f64.powi(n as i32);
    (x * tmp).trunc() / tmp
}

/// 获取MAC地址（模拟实现）
pub fn mac_address() -> String {
    // 在实际应用中，应该使用更可靠的方法获取MAC地址
    // 这里返回一个模拟的MAC地址
    "AA-BB-CC-DD-EE-FF".to_string()
}

/// 对K线周期列表进行排序
pub fn freqs_sorted(freqs: Vec<&str>) -> Vec<&str> {
    let mut sorted_freqs = freqs;
    sorted_freqs.sort_by(|a, b| {
        // 定义周期排序规则，从小到大
        let order = ["1分钟", "5分钟", "15分钟", "30分钟", "60分钟", "日线", "周线", "月线"];
        let a_pos = order.iter().position(|&x| x == *a);
        let b_pos = order.iter().position(|&x| x == *b);
        
        match (a_pos, b_pos) {
            (Some(pos_a), Some(pos_b)) => pos_a.cmp(&pos_b),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        }
    });
    sorted_freqs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x_round() {
        assert_eq!(x_round(3.14159, 2), 3.14);
        assert_eq!(x_round(3.14159, 0), 3.0);
        assert_eq!(x_round(-3.14159, 2), -3.14);
    }

    #[test]
    fn test_mac_address() {
        let mac = mac_address();
        assert_eq!(mac.len(), 17); // MAC address format: AA-BB-CC-DD-EE-FF
    }

    #[test]
    fn test_freqs_sorted() {
        let freqs = vec!["日线", "5分钟", "30分钟", "1分钟"];
        let sorted = freqs_sorted(freqs);
        assert_eq!(sorted, vec!["1分钟", "5分钟", "30分钟", "日线"]);
    }
}