//! 传感器模块 - 用于金融数据分析和事件检测
//!
//! 该模块提供了多种传感器功能，包括：
//! - CTA研究框架
//! - 特征选择和分析
//! - 事件检测和匹配
//! - 金融指标计算

pub mod cta;
pub mod feature;
pub mod event;
pub mod utils;

pub use cta::*;
pub use feature::*;
pub use event::*;
pub use utils::*;