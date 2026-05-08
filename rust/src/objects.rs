use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::enums::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub name: String,
    pub opens: Vec<Event>,
    pub exits: Vec<Event>,
    pub holds: Vec<HashMap<String, String>>,
    pub pairs: Vec<HashMap<String, String>>,
    pub symbol: String,
    pub pos: f64,
    pub operatetimes: i32,
}

impl Position {
    pub fn new(name: &str, symbol: &str) -> Self {
        Position {
            name: name.to_string(),
            opens: Vec::new(),
            exits: Vec::new(),
            holds: Vec::new(),
            pairs: Vec::new(),
            symbol: symbol.to_string(),
            pos: 0.0,
            operatetimes: 0,
        }
    }
    
    pub fn update(&mut self, s: &HashMap<String, String>) {
        // 根据信号更新仓位
        // 简化实现
        if let Some(pos_str) = s.get("pos") {
            if let Ok(pos) = pos_str.parse::<f64>() {
                self.pos = pos;
            }
        }
        
        // 检查是否有开仓信号
        for event in &self.opens {
            if event.is_match(s).unwrap_or((false, String::new())).0 {
                self.operatetimes += 1;
            }
        }
        
        // 检查是否有平仓信号
        for event in &self.exits {
            if event.is_match(s).unwrap_or((false, String::new())).0 {
                self.operatetimes += 1;
            }
        }
    }
    
    pub fn pos_changed(&self) -> bool {
        // 简化实现：如果有操作次数变化则认为仓位变化
        self.operatetimes > 0
    }
    
    pub fn get_pos(&self) -> f64 {
        self.pos
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawBar {
    pub symbol: String,
    pub id: i32,  // id 必须是升序
    pub dt: DateTime<Utc>,
    pub freq: Freq,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub vol: f64,
    pub amount: f64,
    #[serde(default)]
    pub cache: HashMap<String, serde_json::Value>,  // cache 用户缓存，一个最常见的场景是缓存技术指标计算结果
}

impl RawBar {
    /// 上影
    pub fn upper(&self) -> f64 {
        self.high - self.open.max(self.close)
    }

    /// 下影
    pub fn lower(&self) -> f64 {
        self.open.min(self.close) - self.low
    }

    /// 实体
    pub fn solid(&self) -> f64 {
        (self.open - self.close).abs()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewBar {
    pub symbol: String,
    pub id: i32,  // id 必须是升序
    pub dt: DateTime<Utc>,
    pub freq: Freq,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub vol: f64,
    pub amount: f64,
    #[serde(default)]
    pub elements: Vec<RawBar>,  // 存入具有包含关系的原始K线
    #[serde(default)]
    pub cache: HashMap<String, serde_json::Value>,  // cache 用户缓存
}

impl NewBar {
    pub fn raw_bars(&self) -> &Vec<RawBar> {
        &self.elements
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FX {
    pub symbol: String,
    pub dt: DateTime<Utc>,
    pub mark: Mark,
    pub high: f64,
    pub low: f64,
    pub fx: f64,
    #[serde(default)]
    pub elements: Vec<NewBar>,
    #[serde(default)]
    pub cache: HashMap<String, serde_json::Value>,  // cache 用户缓存
}

impl FX {
    /// 构成分型的无包含关系K线
    pub fn new_bars(&self) -> &Vec<NewBar> {
        &self.elements
    }

    /// 构成分型的原始K线
    pub fn raw_bars(&self) -> Vec<RawBar> {
        let mut result = Vec::new();
        for e in &self.elements {
            result.extend(e.raw_bars().clone());
        }
        result
    }

    /// 力度强度描述
    pub fn power_str(&self) -> String {
        assert_eq!(self.elements.len(), 3);
        let k1 = &self.elements[0];
        let k2 = &self.elements[1];
        let k3 = &self.elements[2];

        match self.mark {
            Mark::D => {
                if k3.close > k1.high {
                    "强".to_string()
                } else if k3.close > k2.high {
                    "中".to_string()
                } else {
                    "弱".to_string()
                }
            }
            Mark::G => {
                if k3.close < k1.low {
                    "强".to_string()
                } else if k3.close < k2.low {
                    "中".to_string()
                } else {
                    "弱".to_string()
                }
            }
        }
    }

    /// 成交量力度
    pub fn power_volume(&self) -> f64 {
        assert_eq!(self.elements.len(), 3);
        self.elements.iter().map(|x| x.vol).sum()
    }

    /// 构成分型的三根无包含K线是否有重叠中枢
    pub fn has_zs(&self) -> bool {
        assert_eq!(self.elements.len(), 3);
        let zd = self.elements.iter().map(|x| x.low).fold(f64::NEG_INFINITY, |a, b| a.max(b));
        let zg = self.elements.iter().map(|x| x.high).fold(f64::INFINITY, |a, b| a.min(b));
        zg >= zd
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FakeBI {
    /// 虚拟笔：主要为笔的内部分析提供便利
    pub symbol: String,
    pub sdt: DateTime<Utc>,  // 开始时间
    pub edt: DateTime<Utc>,  // 结束时间
    pub direction: Direction,
    pub high: f64,
    pub low: f64,
    pub power: f64,
    #[serde(default)]
    pub cache: HashMap<String, serde_json::Value>,  // cache 用户缓存
}

/// 创建 fake_bis 列表
pub fn create_fake_bis(fxs: &[FX]) -> Vec<FakeBI> {
    let fxs_len = if fxs.len() % 2 != 0 {
        fxs.len() - 1
    } else {
        fxs.len()
    };

    let mut fake_bis = Vec::new();
    for i in 1..fxs_len {
        let fx1 = &fxs[i - 1];
        let fx2 = &fxs[i];
        assert_ne!(fx1.mark, fx2.mark);
        
        let fake_bi = if fx1.mark == Mark::D {
            FakeBI {
                symbol: fx1.symbol.clone(),
                sdt: fx1.dt,
                edt: fx2.dt,
                direction: Direction::Up,
                high: fx2.high,
                low: fx1.low,
                power: (fx2.high - fx1.low).round(),
                cache: HashMap::new(),
            }
        } else if fx1.mark == Mark::G {
            FakeBI {
                symbol: fx1.symbol.clone(),
                sdt: fx1.dt,
                edt: fx2.dt,
                direction: Direction::Down,
                high: fx1.high,
                low: fx2.low,
                power: (fx1.high - fx2.low).round(),
                cache: HashMap::new(),
            }
        } else {
            panic!("Invalid mark type");
        };
        
        fake_bis.push(fake_bi);
    }
    fake_bis
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BI {
    pub symbol: String,
    pub fx_a: FX,  // 笔开始的分型
    pub fx_b: FX,  // 笔结束的分型
    pub fxs: Vec<FX>,  // 笔内部的分型列表
    pub direction: Direction,
    #[serde(default)]
    pub bars: Vec<NewBar>,
    #[serde(default)]
    pub cache: HashMap<String, serde_json::Value>,  // cache 用户缓存
}

impl BI {
    pub fn new(fx_a: FX, fx_b: FX, fxs: Vec<FX>, bars: Vec<NewBar>) -> Self {
        let direction = if fx_a.mark == Mark::D { Direction::Up } else { Direction::Down };
        
        BI {
            symbol: fx_a.symbol.clone(),
            fx_a,
            fx_b,
            fxs,
            direction,
            bars,
            cache: HashMap::new(),
        }
    }

    pub fn sdt(&self) -> DateTime<Utc> {
        self.fx_a.dt
    }

    pub fn edt(&self) -> DateTime<Utc> {
        self.fx_b.dt
    }

    /// 笔的内部分型连接得到近似次级别笔列表
    pub fn fake_bis(&self) -> Vec<FakeBI> {
        create_fake_bis(&self.fxs)
    }

    pub fn high(&self) -> f64 {
        self.fx_a.high.max(self.fx_b.high)
    }

    pub fn low(&self) -> f64 {
        self.fx_a.low.min(self.fx_b.low)
    }

    pub fn power(&self) -> f64 {
        self.power_price()
    }

    /// 价差力度
    pub fn power_price(&self) -> f64 {
        (self.fx_b.fx - self.fx_a.fx).abs().round()
    }

    /// 成交量力度
    pub fn power_volume(&self) -> f64 {
        self.bars.iter().skip(1).take(self.bars.len().saturating_sub(2)).map(|x| x.vol).sum()
    }

    /// 笔的涨跌幅
    pub fn change(&self) -> f64 {
        (self.fx_b.fx - self.fx_a.fx) / self.fx_a.fx
    }

    /// 笔的无包含关系K线数量
    pub fn length(&self) -> usize {
        self.bars.len()
    }

    /// 构成笔的原始K线序列，不包含首尾分型的首根K线
    pub fn raw_bars(&self) -> Vec<RawBar> {
        let mut value = Vec::new();
        // 去掉首尾分型的第一根K线
        for bar in self.bars.iter().skip(1).take(self.bars.len().saturating_sub(2)) {
            value.extend(bar.raw_bars().clone());
        }
        value
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZS {
    /// 中枢对象，主要用于辅助信号函数计算
    pub bis: Vec<BI>,
    #[serde(default)]
    pub cache: HashMap<String, serde_json::Value>,  // cache 用户缓存
}

impl ZS {
    pub fn new(bis: Vec<BI>) -> Self {
        ZS {
            bis,
            cache: HashMap::new(),
        }
    }

    pub fn symbol(&self) -> &str {
        &self.bis[0].symbol
    }

    /// 中枢开始时间
    pub fn sdt(&self) -> DateTime<Utc> {
        self.bis[0].sdt()
    }

    /// 中枢结束时间
    pub fn edt(&self) -> DateTime<Utc> {
        self.bis[self.bis.len() - 1].edt()
    }

    /// 中枢第一笔方向，sdir 是 start direction 的缩写
    pub fn sdir(&self) -> Direction {
        self.bis[0].direction
    }

    /// 中枢倒一笔方向，edir 是 end direction 的缩写
    pub fn edir(&self) -> Direction {
        self.bis[self.bis.len() - 1].direction
    }

    /// 中枢中轴
    pub fn zz(&self) -> f64 {
        self.zd() + (self.zg() - self.zd()) / 2.0
    }

    /// 中枢最高点
    pub fn gg(&self) -> f64 {
        self.bis.iter().map(|x| x.high()).fold(f64::NEG_INFINITY, |a, b| a.max(b))
    }

    /// 中枢上沿
    pub fn zg(&self) -> f64 {
        self.bis.iter().take(3).map(|x| x.high()).fold(f64::INFINITY, |a, b| a.min(b))
    }

    /// 中枢最低点
    pub fn dd(&self) -> f64 {
        self.bis.iter().map(|x| x.low()).fold(f64::INFINITY, |a, b| a.min(b))
    }

    /// 中枢下沿
    pub fn zd(&self) -> f64 {
        self.bis.iter().take(3).map(|x| x.low()).fold(f64::NEG_INFINITY, |a, b| a.max(b))
    }

    /// 中枢是否有效
    pub fn is_valid(&self) -> bool {
        if self.zg() < self.zd() {
            return false;
        }

        for bi in &self.bis {
            // 中枢内的笔必须与中枢的上下沿有交集
            if self.zg() >= bi.high() && bi.high() >= self.zd() ||
               self.zg() >= bi.low() && bi.low() >= self.zd() ||
               bi.high() >= self.zg() && self.zg() >= self.zd() && self.zd() >= bi.low() {
                continue;
            } else {
                return false;
            }
        }

        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CZSC {
    pub symbol: String,
    pub freq: Freq,
    pub bars_raw: Vec<RawBar>,
    pub bi_list: Vec<BI>,
    pub xd_list: Vec<FX>,  // 线段列表
    pub zs_list: Vec<ZS>,  // 中枢列表
    pub signals: Vec<Signal>,
    pub events: Vec<Event>,
    pub cache: HashMap<String, serde_json::Value>,
    pub last_event: Option<Event>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Hash)]
pub struct Signal {
    #[serde(default)]
    pub signal: String,

    // score 取值在 0~100 之间，得分越高，信号越强
    #[serde(default)]
    pub score: i32,

    // k1, k2, k3 是信号名称
    #[serde(default)]
    pub k1: String,  // k1 一般是指明信号计算的K线周期，如 60分钟，日线，周线等
    #[serde(default)]
    pub k2: String,  // k2 一般是记录信号计算的参数
    #[serde(default)]
    pub k3: String,  // k3 用于区分信号，必须具有唯一性，推荐使用信号分类和开发日期进行标记

    // v1, v2, v3 是信号取值
    #[serde(default)]
    pub v1: String,
    #[serde(default)]
    pub v2: String,
    #[serde(default)]
    pub v3: String,
}

impl Signal {
    pub fn new(k1: String, k2: String, k3: String, v1: String, v2: String, v3: String, score: i32) -> Self {
        let signal = format!("{}_{}_{}_{}_{}_{}/{}", k1, k2, k3, v1, v2, v3, score);
        Signal {
            signal,
            score,
            k1,
            k2,
            k3,
            v1,
            v2,
            v3,
        }
    }

    /// 获取信号名称
    pub fn key(&self) -> String {
        let mut key = String::new();
        for k in [&self.k1, &self.k2, &self.k3] {
            if k != "任意" {
                key.push_str(k);
                key.push('_');
            }
        }
        key.trim_end_matches('_').to_string()
    }

    /// 获取信号值
    pub fn value(&self) -> String {
        format!("{}_{}_{}/{}", self.v1, self.v2, self.v3, self.score)
    }

    /// 判断信号是否与信号列表中的值匹配
    pub fn is_match(&self, s: &HashMap<String, String>) -> Result<bool, String> {
        let key = self.key();
        let v = s.get(&key).ok_or_else(|| format!("{} 不在信号列表中", key))?;

        let parts: Vec<&str> = v.split('/').collect();
        if parts.len() != 2 {
            return Err("信号格式错误".to_string());
        }
        
        let values: Vec<&str> = parts[0].split('_').collect();
        if values.len() != 3 {
            return Err("信号值格式错误".to_string());
        }
        
        let score: i32 = parts[1].parse().map_err(|_| "分数解析错误".to_string())?;
        let v1 = values[0];
        let v2 = values[1];
        let v3 = values[2];

        Ok(
            score >= self.score &&
            (v1 == self.v1 || self.v1 == "任意") &&
            (v2 == self.v2 || self.v2 == "任意") &&
            (v3 == self.v3 || self.v3 == "任意")
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Hash)]
pub struct Event {
    pub operate: Operate,

    // signals_all 必须全部满足的信号，允许为空
    #[serde(default)]
    pub signals_all: Vec<Signal>,

    // signals_any 满足其中任一信号，允许为空
    #[serde(default)]
    pub signals_any: Vec<Signal>,

    // signals_not 不能满足其中任一信号，允许为空
    #[serde(default)]
    pub signals_not: Vec<Signal>,

    #[serde(default)]
    pub name: String,
}

impl Event {
    pub fn new(operate: Operate, signals_all: Vec<Signal>, signals_any: Vec<Signal>, signals_not: Vec<Signal>, name: String) -> Self {
        Event {
            operate,
            signals_all,
            signals_any,
            signals_not,
            name,
        }
    }

    /// 获取 Event 的唯一信号列表
    pub fn unique_signals(&self) -> Vec<String> {
        let mut signals = Vec::new();
        if !self.signals_all.is_empty() {
            for signal in &self.signals_all {
                signals.push(signal.signal.clone());
            }
        }
        if !self.signals_any.is_empty() {
            for signal in &self.signals_any {
                signals.push(signal.signal.clone());
            }
        }
        if !self.signals_not.is_empty() {
            for signal in &self.signals_not {
                signals.push(signal.signal.clone());
            }
        }
        signals.sort();
        signals.dedup();
        signals
    }

    /// 判断 event 是否满足
    pub fn is_match(&self, s: &HashMap<String, String>) -> Result<(bool, String), String> {
        // 首先判断 signals_not 中的信号是否得到满足
        if !self.signals_not.is_empty() {
            for signal in &self.signals_not {
                if signal.is_match(s)? {
                    return Ok((false, String::new()));
                }
            }
        }

        // 接着判断 signals_all 中的信号是否全部得到满足
        if !self.signals_all.is_empty() {
            for signal in &self.signals_all {
                if !signal.is_match(s)? {
                    return Ok((false, String::new()));
                }
            }
        }

        // 然后判断 signals_any 中的信号是否有一个得到满足
        if !self.signals_any.is_empty() {
            for signal in &self.signals_any {
                if signal.is_match(s)? {
                    return Ok((true, signal.key()));
                }
            }
            return Ok((false, String::new()));
        }

        // 如果 signals_any 为空但其他条件都满足
        if self.signals_all.is_empty() && self.signals_not.is_empty() {
            return Ok((true, String::new()));
        } else if !self.signals_all.is_empty() {
            return Ok((true, self.signals_all.last().unwrap().key()));
        }

        Ok((true, String::new()))
    }

    /// 将 Event 对象转存为 HashMap
    pub fn dump(&self) -> HashMap<String, serde_json::Value> {
        let signals_all = self.signals_all.iter().map(|x| serde_json::Value::String(x.signal.clone())).collect();
        let signals_any = self.signals_any.iter().map(|x| serde_json::Value::String(x.signal.clone())).collect();
        let signals_not = self.signals_not.iter().map(|x| serde_json::Value::String(x.signal.clone())).collect();

        let mut raw = HashMap::new();
        raw.insert("name".to_string(), serde_json::Value::String(self.name.clone()));
        raw.insert("operate".to_string(), serde_json::Value::String(self.operate.to_string()));
        raw.insert("signals_all".to_string(), serde_json::Value::Array(signals_all));
        raw.insert("signals_any".to_string(), serde_json::Value::Array(signals_any));
        raw.insert("signals_not".to_string(), serde_json::Value::Array(signals_not));
        raw
    }
}

/// 计算单笔收益序列的盈亏平衡点
pub fn cal_break_even_point(seq: &[f64]) -> f64 {
    if seq.is_empty() || seq.iter().sum::<f64>() < 0.0 {
        return 1.0;
    }

    let mut sorted_seq = seq.to_vec();
    sorted_seq.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut sub = 0.0;
    let mut sub_i = 0;
    for (i, s) in sorted_seq.iter().enumerate() {
        sub += s;
        sub_i = i + 1;
        if sub >= 0.0 {
            break;
        }
    }

    sub_i as f64 / seq.len() as f64
}