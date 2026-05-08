# CZSC Rust 测试套件

此目录包含了从 Python 版本的 `test/` 目录转换而来的 Rust 测试用例。

## 已转换的测试文件

### 1. test_analyze.rs
- 测试 CZSC 分析模块的核心功能
- 包括 CZSC 基础功能测试 (`test_czsc_basic`)
- 验证 CZSC 信号计算功能 (`test_czsc_signals`)
- 测试 UBI 属性功能 (`test_czsc_ubi_properties`)

### 2. test_features.rs
- 测试特征模块的功能
- 包括事件特征检测 (`test_is_event_feature`)
- 测试滚动双曲正切变换 (`test_rolling_tanh`)
- 验证相关性归一化功能 (`test_normalize_corr`)

### 3. test_sensors.rs
- 测试传感器模块的功能
- CTA 研究框架测试 (`test_cta_research`)
- 特征选择器测试 (`test_feature_selector`)
- 事件检测器测试 (`test_event_detection`)

## 测试运行方式

运行所有测试：
```bash
cd rust
cargo test
```

运行特定测试文件：
```bash
cargo test --test test_analyze
cargo test --test test_features
cargo test --test test_sensors
```

## 设计说明

- 所有测试都遵循 Rust 的测试最佳实践
- 使用 `#[cfg(test)]` 来隔离测试代码
- 模拟数据生成以避免对外部依赖的需要
- 测试覆盖了核心功能，确保 Rust 实现与 Python 版本行为一致