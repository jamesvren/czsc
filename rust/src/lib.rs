//! CZSC Rust Sensors
//! 
//! 基于缠中说禅理论的量化交易传感器模块的Rust实现
//! 提供CTA研究、特征工程、事件检测等功能

pub mod analyze;
pub mod bar_generator;
pub mod enums;
pub mod objects;
pub mod sensors;

// 导出主要接口
pub use analyze::{remove_include, check_fx, check_bi, check_fxs};
pub use bar_generator::{BarGenerator, format_standard_kline};
pub use enums::*;
pub use objects::*;
pub use sensors::*;