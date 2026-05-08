//! 交易者模块 - 实现交易策略、持仓管理和回测功能
//!
//! 该模块包含了CZSC项目中交易相关的所有功能，包括：
//! - CzscSignals: 缠中说禅技术分析理论之多级别信号计算
//! - CzscTrader: 缠中说禅技术分析理论之多级别联立交易决策类
//! - DummyBacktest: 策略回测（支持多进程执行）
//! - RedisWeightsClient: 策略持仓权重管理客户端
//! - 以及其他相关功能

mod base;
mod dummy;
mod cwc;
mod rwc;
mod optimize;

pub use base::*;
pub use dummy::*;
pub use cwc::*;
pub use rwc::*;
pub use optimize::*;