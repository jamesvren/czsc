use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// 买卖操作枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum Operate {
    /// 持有状态
    HL, // Hold Long
    HS, // Hold Short
    HO, // Hold Other

    /// 多头操作
    LO, // Long Open
    LE, // Long Exit

    /// 空头操作
    SO, // Short Open
    SE, // Short Exit
}

impl Default for Operate {
    fn default() -> Self {
        Operate::HO // 默认为持有其他状态
    }
}

impl std::fmt::Display for Operate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operate::HL => write!(f, "持多"),
            Operate::HS => write!(f, "持空"),
            Operate::HO => write!(f, "持币"),
            Operate::LO => write!(f, "开多"),
            Operate::LE => write!(f, "平多"),
            Operate::SO => write!(f, "开空"),
            Operate::SE => write!(f, "平空"),
        }
    }
}

/// 分型标记枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mark {
    D, // 底分型
    G, // 顶分型
}

impl std::fmt::Display for Mark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mark::D => write!(f, "底分型"),
            Mark::G => write!(f, "顶分型"),
        }
    }
}

/// 方向枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    Up,   // 向上
    Down, // 向下
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "向上"),
            Direction::Down => write!(f, "向下"),
        }
    }
}

/// 频率/周期枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum Freq {
    Tick,
    F1,    // 1分钟
    F2,    // 2分钟
    F3,    // 3分钟
    F4,    // 4分钟
    F5,    // 5分钟
    F6,    // 6分钟
    F10,   // 10分钟
    F12,   // 12分钟
    F15,   // 15分钟
    F20,   // 20分钟
    F30,   // 30分钟
    F60,   // 60分钟
    F120,  // 120分钟
    D,     // 日线
    W,     // 周线
    M,     // 月线
    S,     // 季线
    Y,     // 年线
}

impl std::fmt::Display for Freq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Freq::Tick => write!(f, "Tick"),
            Freq::F1 => write!(f, "1分钟"),
            Freq::F2 => write!(f, "2分钟"),
            Freq::F3 => write!(f, "3分钟"),
            Freq::F4 => write!(f, "4分钟"),
            Freq::F5 => write!(f, "5分钟"),
            Freq::F6 => write!(f, "6分钟"),
            Freq::F10 => write!(f, "10分钟"),
            Freq::F12 => write!(f, "12分钟"),
            Freq::F15 => write!(f, "15分钟"),
            Freq::F20 => write!(f, "20分钟"),
            Freq::F30 => write!(f, "30分钟"),
            Freq::F60 => write!(f, "60分钟"),
            Freq::F120 => write!(f, "120分钟"),
            Freq::D => write!(f, "日线"),
            Freq::W => write!(f, "周线"),
            Freq::M => write!(f, "月线"),
            Freq::S => write!(f, "季线"),
            Freq::Y => write!(f, "年线"),
        }
    }
}

impl FromStr for Freq {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Tick" | "tick" => Ok(Freq::Tick),
            "1分钟" | "1m" | "F1" | "f1" => Ok(Freq::F1),
            "2分钟" | "2m" | "F2" | "f2" => Ok(Freq::F2),
            "3分钟" | "3m" | "F3" | "f3" => Ok(Freq::F3),
            "4分钟" | "4m" | "F4" | "f4" => Ok(Freq::F4),
            "5分钟" | "5m" | "F5" | "f5" => Ok(Freq::F5),
            "6分钟" | "6m" | "F6" | "f6" => Ok(Freq::F6),
            "10分钟" | "10m" | "F10" | "f10" => Ok(Freq::F10),
            "12分钟" | "12m" | "F12" | "f12" => Ok(Freq::F12),
            "15分钟" | "15m" | "F15" | "f15" => Ok(Freq::F15),
            "20分钟" | "20m" | "F20" | "f20" => Ok(Freq::F20),
            "30分钟" | "30m" | "F30" | "f30" => Ok(Freq::F30),
            "60分钟" | "60m" | "F60" | "f60" => Ok(Freq::F60),
            "120分钟" | "120m" | "F120" | "f120" => Ok(Freq::F120),
            "日线" | "D" | "d" => Ok(Freq::D),
            "周线" | "W" | "w" => Ok(Freq::W),
            "月线" | "M" | "m" => Ok(Freq::M),
            "季线" | "S" | "s" => Ok(Freq::S),
            "年线" | "Y" | "y" => Ok(Freq::Y),
            _ => Err(()),
        }
    }
}

impl TryFrom<&str> for Freq {
    type Error = ();
    
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Freq::from_str(s)
    }
}