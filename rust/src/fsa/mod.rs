//! 飞书应用API接口封装 - Rust实现
//! 
//! 该模块提供了对飞书API的封装，包括：
//! - 基础API封装和请求函数
//! - 即时消息功能
//! - 电子表格功能
//! - 多维表格功能

pub mod base;
pub mod im;
pub mod sheets;
pub mod bi_table;

pub use base::*;
pub use im::*;
pub use sheets::*;
pub use bi_table::*;