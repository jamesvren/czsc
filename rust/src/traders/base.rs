//! 缠中说禅技术分析理论之多级别信号计算和交易决策
//!
//! 该模块实现了缠中说禅技术分析理论的多级别信号计算和交易决策功能，
//! 包括信号计算、策略执行和回测等功能。

use std::collections::HashMap;
use std::collections::VecDeque;
use chrono::{DateTime, Utc, TimeZone};
use serde::{Deserialize, Serialize};

use crate::objects::{RawBar, Position};
use crate::enums::Freq;
use crate::enums::*;
use crate::bar_generator::BarGenerator;

/// 缠中说禅技术分析理论之多级别信号计算
#[derive(Debug, Clone)]
pub struct CzscSignals {
    pub name: String,
    /// 信号计算过程的缓存容器
    pub cache: HashMap<String, String>,
    /// 关键字参数
    pub kwargs: HashMap<String, String>,
    /// 信号配置
    pub signals_config: Vec<HashMap<String, String>>,
    
    /// K线合成器
    pub bg: Option<BarGenerator>,
    /// 标的代码
    pub symbol: Option<String>,
    /// 基础频率
    pub base_freq: Option<Freq>,
    /// 频率列表
    pub freqs: Vec<Freq>,
    /// 不同频率的CZSC分析对象
    // pub kas: HashMap<Freq, CZSC>,  // 暂时注释，因为CZSC还没定义
    
    /// 结束时间
    pub end_dt: Option<DateTime<Utc>>,
    /// K线ID
    pub bid: Option<i64>,
    /// 最新价格
    pub latest_price: Option<f64>,
    /// 信号字典
    pub s: HashMap<String, String>,
}

impl CzscSignals {
    /// 创建新的CzscSignals实例
    pub fn new(bg: Option<BarGenerator>, kwargs: HashMap<String, String>) -> Self {
        let signals_config = kwargs.get("signals_config")
            .and_then(|s| serde_json::from_str(s).ok())
            .unwrap_or_default();
        
        let mut instance = CzscSignals {
            name: "CzscSignals".to_string(),
            cache: Default::default(),
            kwargs,
            signals_config,
            bg,
            symbol: None,
            base_freq: None,
            freqs: vec![],
            // kas: Default::default(),  // 暂时注释
            end_dt: None,
            bid: None,
            latest_price: None,
            s: Default::default(),
        };
        
        if let Some(ref bg) = instance.bg {
            assert!(bg.symbol.is_some(), "bg.symbol is None");
            instance.symbol = bg.symbol.clone();
            instance.base_freq = Some(bg.base_freq);
            instance.freqs = bg.bars.keys().cloned().collect();
            
            // TODO: 初始化kas map，需要CZSC结构体
            if let Some(last_bar) = bg.get_last_bar(instance.base_freq.unwrap()) {
                instance.end_dt = Some(last_bar.dt);
                instance.bid = Some(last_bar.id as i64);
                instance.latest_price = Some(last_bar.close);
                
                // 更新信号
                let signals = instance.get_signals_by_conf();
                instance.s.extend(signals);
                
                // 添加K线数据到信号
                instance.s.insert("symbol".to_string(), last_bar.symbol.clone());
                instance.s.insert("dt".to_string(), last_bar.dt.to_rfc3339());
                instance.s.insert("id".to_string(), last_bar.id.to_string());
                instance.s.insert("close".to_string(), last_bar.close.to_string());
            }
        }
        
        instance
    }
    
    /// 通过信号参数配置获取信号
    pub fn get_signals_by_conf(&self) -> HashMap<String, String> {
        let mut s = HashMap::new();
        
        if self.signals_config.is_empty() {
            return s;
        }
        
        // 这里需要根据信号配置来调用具体的信号函数
        // 由于Rust的动态特性限制，我们暂时返回空map
        // 在实际应用中，这需要更复杂的实现
        
        for param in &self.signals_config {
            // 提取信号名称和频率
            if let Some(sig_name) = param.get("name") {
                let freq_str = param.get("freq").unwrap_or(&"".to_string());
                
                // 如果指定了频率，需要相应处理
                // 这里只是占位实现
                s.insert(format!("{}_signal", sig_name), "placeholder".to_string());
            }
        }
        
        s
    }
    
    /// 输入基础周期已完成K线，更新信号，更新仓位
    pub fn update_signals(&mut self, bar: &RawBar) {
        if let Some(ref mut bg) = self.bg {
            bg.update(bar);
        }
        
        self.symbol = Some(bar.symbol.clone());
        
        // TODO: 更新不同频率的K线分析对象
        if let Some(ref bg) = self.bg {
            if let Some(last_bar) = bg.get_last_bar(self.base_freq.unwrap()) {
                self.end_dt = Some(last_bar.dt);
                self.bid = Some(last_bar.id as i64);
                self.latest_price = Some(last_bar.close);
                
                self.s = Default::default();
                self.s.extend(self.get_signals_by_conf());
                
                // 添加K线数据到信号
                self.s.insert("symbol".to_string(), last_bar.symbol.clone());
                self.s.insert("dt".to_string(), last_bar.dt.to_rfc3339());
                self.s.insert("id".to_string(), last_bar.id.to_string());
                self.s.insert("close".to_string(), last_bar.close.to_string());
            }
        }
    }
}

/// 缠中说禅技术分析理论之多级别联立交易决策类（支持多策略独立执行）
#[derive(Debug, Clone)]
pub struct CzscTrader {
    /// 继承CzscSignals的所有字段
    pub signals: CzscSignals,
    /// 仓位列表
    pub positions: Vec<Position>,
    /// 多个仓位集成一个仓位的方法
    ensemble_method: String,
}

impl CzscTrader {
    /// 创建新的CzscTrader实例
    pub fn new(
        bg: Option<BarGenerator>,
        positions: Option<Vec<Position>>,
        ensemble_method: Option<String>,
        kwargs: HashMap<String, String>,
    ) -> Self {
        let positions = positions.unwrap_or_default();
        
        // 检查仓位策略名称是否唯一
        let mut pos_names = Vec::new();
        for pos in &positions {
            pos_names.push(pos.name.clone());
        }
        assert_eq!(
            pos_names.len(),
            pos_names.iter().collect::<std::collections::HashSet<_>>().len(),
            "仓位策略名称不能重复"
        );
        
        let ensemble_method = ensemble_method.unwrap_or_else(|| "mean".to_string());
        
        CzscTrader {
            signals: CzscSignals::new(bg, kwargs),
            positions,
            ensemble_method,
        }
    }
    
    /// 输入基础周期已完成K线，更新信号，更新仓位
    pub fn update(&mut self, bar: &RawBar) {
        self.signals.update_signals(bar);
        
        for position in &mut self.positions {
            // 这里需要将信号字典转换为适当的格式传递给position.update
            // 由于信号类型可能复杂，这里简化处理
            position.update(&self.signals.s);
        }
    }
    
    /// 通过信号字典直接交易，用于快速回测场景
    pub fn on_sig(&mut self, sig: &HashMap<String, String>) {
        self.signals.s = sig.clone();
        
        if let Some(symbol) = sig.get("symbol") {
            self.signals.symbol = Some(symbol.clone());
        }
        
        if let Some(dt_str) = sig.get("dt") {
            if let Ok(dt) = DateTime::parse_from_rfc3339(dt_str) {
                self.signals.end_dt = Some(dt.with_timezone(&Utc));
            }
        }
        
        if let Some(id_str) = sig.get("id") {
            if let Ok(id) = id_str.parse::<i64>() {
                self.signals.bid = Some(id);
            }
        }
        
        if let Some(close_str) = sig.get("close") {
            if let Ok(close) = close_str.parse::<f64>() {
                self.signals.latest_price = Some(close);
            }
        }
        
        for position in &mut self.positions {
            position.update(sig);
        }
    }
    
    /// 输入基础周期已完成K线，更新信号，更新仓位
    pub fn on_bar(&mut self, bar: &RawBar) {
        self.update(bar);
    }
    
    /// 判断仓位是否发生变化
    pub fn pos_changed(&self) -> bool {
        if self.positions.is_empty() {
            return false;
        }
        self.positions.iter().any(|pos| pos.pos_changed())
    }
    
    /// 获取多个仓位的集成仓位
    pub fn get_ensemble_pos(&self, method: Option<&str>) -> f64 {
        if self.positions.is_empty() {
            return 0.0;
        }
        
        let method = method.unwrap_or(&self.ensemble_method);
        let pos_seq: Vec<f64> = self.positions.iter()
            .map(|pos| pos.get_pos())  // 假设Position有get_pos方法
            .collect();
        
        match method.to_lowercase().as_str() {
            "mean" => pos_seq.iter().sum::<f64>() / pos_seq.len() as f64,
            "vote" => {
                let sum: f64 = pos_seq.iter().sum();
                sum.signum()
            },
            "max" => pos_seq.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)),
            _ => panic!("Invalid ensemble method"),
        }
    }
    
    /// 获取指定名称的仓位策略对象
    pub fn get_position(&self, name: &str) -> Option<&Position> {
        if self.positions.is_empty() {
            return None;
        }
        
        self.positions.iter().find(|pos| pos.name == name)
    }
}

/// 使用CzscSignals生成信号
pub fn generate_czsc_signals(
    bars: &[RawBar],
    signals_config: &[HashMap<String, String>],
    sdt: Option<DateTime<Utc>>,
    init_n: Option<usize>,
    df: Option<bool>,
    kwargs: &HashMap<String, String>,
) -> Vec<HashMap<String, String>> {
    let init_n = init_n.unwrap_or(500);
    let sdt = sdt.unwrap_or_else(|| Utc.timestamp_opt(2017, 1).unwrap());
    
    let bars_left: Vec<&RawBar> = bars.iter()
        .filter(|bar| bar.dt < sdt)
        .take(init_n)
        .collect();
    
    let bars_right: Vec<&RawBar> = if bars_left.len() <= init_n {
        bars.iter().skip(init_n).collect()
    } else {
        bars.iter().filter(|bar| bar.dt >= sdt).collect()
    };
    
    if bars_right.is_empty() {
        println!("警告：右侧K线为空，无法进行信号生成");
        return vec![];
    }
    
    let base_freq = bars[0].freq;
    let freqs: Vec<Freq> = signals_config.iter()
        .filter_map(|config| config.get("freq").map(|f| Freq::try_from(f.as_str()).unwrap()))
        .filter(|&freq| freq != base_freq)
        .collect();
    
    let mut bg = BarGenerator::new(&base_freq.to_string(), &freqs, kwargs.get("bg_max_count").unwrap_or(&"5000".to_string()).parse().unwrap_or(5000));
    
    for bar in &bars_left {
        bg.update(bar);
    }
    
    let mut _sigs = Vec::new();
    let mut cs = CzscSignals::new(Some(bg), kwargs.clone());
    cs.cache.insert("gsc_kwargs".to_string(), format!("{:?}", kwargs));
    
    for bar in bars_right {
        cs.update_signals(bar);
        _sigs.push(cs.s.clone());
    }
    
    _sigs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_czsc_signals_creation() {
        let kwargs = HashMap::new();
        let signals = CzscSignals::new(None, kwargs);
        assert_eq!(signals.name, "CzscSignals");
    }

    #[test]
    fn test_czsc_trader_creation() {
        let kwargs = HashMap::new();
        let trader = CzscTrader::new(None, None, None, kwargs);
        assert_eq!(trader.signals.name, "CzscSignals");
        assert_eq!(trader.ensemble_method, "mean");
    }
}