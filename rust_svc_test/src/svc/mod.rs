//! Streamlit Visualize Components (SVC) - 缠中说禅可视化组件库
//!
//! 该模块提供了一套完整的 Streamlit 可视化组件，用于金融数据分析、策略回测、因子分析等场景。
//!
//! 主要功能模块：
//! - backtest: 回测相关组件
//! - statistics: 统计分析组件
//! - factor: 因子分析组件
//! - correlation: 相关性分析组件
//! - returns: 收益相关的可视化组件
//! - strategy: 策略分析组件
//! - weights: 持仓权重分析组件
//! - utils: 工具类组件
//! - forms: 用户输入表单组件

pub mod backtest;
pub mod statistics;
pub mod factor;
pub mod correlation;
pub mod returns;
pub mod strategy;
pub mod weights;
pub mod utils;
pub mod forms;
pub mod base;

// 从子模块导出关键函数
pub use backtest::*;
pub use statistics::*;
pub use factor::*;
pub use correlation::*;
pub use returns::*;
pub use strategy::*;
pub use weights::*;
pub use utils::*;
pub use forms::*;