use crate::objects::*;
use crate::enums::*;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// K线生成器
#[derive(Debug, Clone)]
pub struct BarGenerator {
    pub base_freq: Freq,
    pub freqs: Vec<Freq>,
    pub bi_zs: bool,
    pub max_count: usize,
    pub symbol: Option<String>,
    pub bars: HashMap<Freq, Vec<RawBar>>,
    pub completed_k_data: HashMap<Freq, Vec<RawBar>>,
}

impl BarGenerator {
    /// 创建新的K线生成器
    pub fn new(base_freq: &str, sec_freqs: &[Freq], max_count: usize) -> Self {
        let mut bars = HashMap::new();
        let base_freq_enum = Freq::try_from(base_freq).unwrap();
        bars.insert(base_freq_enum, Vec::new());
        
        for freq in sec_freqs {
            bars.insert(*freq, Vec::new());
        }

        BarGenerator {
            base_freq: base_freq_enum,
            freqs: sec_freqs.to_vec(),
            bi_zs: false,  // 默认值
            max_count,
            symbol: None,
            bars,
            completed_k_data: HashMap::new(),
        }
    }

    /// 更新K线生成器
    pub fn update(&mut self, bar: &RawBar) {
        // 更新symbol
        self.symbol = Some(bar.symbol.clone());
        
        // 将新K线添加到基础频率的缓存中
        self.bars.get_mut(&self.base_freq).unwrap().push(bar.clone());
        
        // 保持缓存大小不超过max_count
        if self.bars.get(&self.base_freq).unwrap().len() > self.max_count {
            self.bars.get_mut(&self.base_freq).unwrap().remove(0);
        }
        
        // 生成更高级别的K线
        self.update_high_freq_bars(bar);
    }
    
    /// 更新高级别频率的K线
    fn update_high_freq_bars(&mut self, bar: &RawBar) {
        for freq in &self.freqs {
            // 这里需要根据基础频率和目标频率的关系来合并K线
            // 简化实现，只做基础的合并逻辑
            let base_bars = self.bars.get(&self.base_freq).unwrap();
            let high_freq_bars = self.resample_bars(base_bars, self.base_freq, *freq);
            self.bars.insert(*freq, high_freq_bars);
        }
    }

    /// 重新采样K线到指定频率
    fn resample_bars(&self, bars: &[RawBar], from_freq: Freq, to_freq: Freq) -> Vec<RawBar> {
        if bars.is_empty() {
            return Vec::new();
        }

        // 这是一个简化的重新采样实现
        // 在实际应用中，可能需要更复杂的采样逻辑
        let factor = self.get_resample_factor(from_freq, to_freq);
        
        if factor <= 1 {
            return bars.to_vec();
        }

        let mut resampled_bars = Vec::new();
        let mut current_bar: Option<RawBar> = None;
        let mut count = 0;

        for bar in bars {
            if count == 0 {
                // 开始新的K线
                current_bar = Some(RawBar {
                    symbol: bar.symbol.clone(),
                    id: bar.id,
                    dt: bar.dt,
                    freq: to_freq,
                    open: bar.open,
                    close: bar.close,
                    high: bar.high,
                    low: bar.low,
                    vol: bar.vol,
                    amount: bar.amount,
                    cache: HashMap::new(),
                });
            } else if let Some(ref mut curr_bar) = current_bar {
                // 更新当前K线的值
                curr_bar.high = curr_bar.high.max(bar.high);
                curr_bar.low = curr_bar.low.min(bar.low);
                curr_bar.close = bar.close;
                curr_bar.vol += bar.vol;
                curr_bar.amount += bar.amount;
            }

            count += 1;
            
            if count >= factor {
                if let Some(final_bar) = current_bar.take() {
                    resampled_bars.push(final_bar);
                }
                count = 0;
            }
        }

        // 处理剩余的K线
        if count > 0 {
            if let Some(final_bar) = current_bar.take() {
                resampled_bars.push(final_bar);
            }
        }

        resampled_bars
    }

    /// 获取重新采样的因子
    fn get_resample_factor(&self, from_freq: Freq, to_freq: Freq) -> usize {
        // 简化的频率转换逻辑，实际实现可能需要更精确的转换
        let from_minutes = self.freq_to_minutes(from_freq);
        let to_minutes = self.freq_to_minutes(to_freq);
        
        if to_minutes >= from_minutes {
            (to_minutes / from_minutes) as usize
        } else {
            1
        }
    }

    /// 将频率转换为分钟数
    fn freq_to_minutes(&self, freq: Freq) -> u32 {
        match freq {
            Freq::Tick => 0,
            Freq::F1 => 1,
            Freq::F2 => 2,
            Freq::F3 => 3,
            Freq::F4 => 4,
            Freq::F5 => 5,
            Freq::F6 => 6,
            Freq::F10 => 10,
            Freq::F12 => 12,
            Freq::F15 => 15,
            Freq::F20 => 20,
            Freq::F30 => 30,
            Freq::F60 => 60,
            Freq::F120 => 120,
            Freq::D => 24 * 60,  // 日线，假设每天1440分钟
            Freq::W => 7 * 24 * 60,  // 周线
            Freq::M => 30 * 24 * 60,  // 月线
            Freq::S => 90 * 24 * 60,  // 季线
            Freq::Y => 365 * 24 * 60, // 年线
        }
    }

    /// 获取指定频率的K线
    pub fn get_bars(&self, freq: Freq) -> Vec<RawBar> {
        self.bars.get(&freq).unwrap_or(&Vec::new()).clone()
    }

    /// 获取基础频率的K线
    pub fn get_base_bars(&self) -> Vec<RawBar> {
        self.get_bars(self.base_freq)
    }
    
    /// 获取指定频率的最后一个K线
    pub fn get_last_bar(&self, freq: Freq) -> Option<RawBar> {
        let bars = self.get_bars(freq);
        bars.last().cloned()
    }
}

/// 格式化标准K线数据
pub fn format_standard_kline(data: Vec<HashMap<String, String>>, freq: Freq) -> Vec<RawBar> {
    let mut bars = Vec::new();
    
    for (id, row) in data.iter().enumerate() {
        let bar = RawBar {
            symbol: row.get("symbol").unwrap_or(&"".to_string()).clone(),
            id: id as i32,
            dt: parse_datetime(row.get("dt").unwrap_or(&"".to_string())),
            freq,
            open: row.get("open").unwrap_or(&"0.0".to_string()).parse().unwrap_or(0.0),
            close: row.get("close").unwrap_or(&"0.0".to_string()).parse().unwrap_or(0.0),
            high: row.get("high").unwrap_or(&"0.0".to_string()).parse().unwrap_or(0.0),
            low: row.get("low").unwrap_or(&"0.0".to_string()).parse().unwrap_or(0.0),
            vol: row.get("vol").unwrap_or(&"0.0".to_string()).parse().unwrap_or(0.0),
            amount: row.get("amount").unwrap_or(&"0.0".to_string()).parse().unwrap_or(0.0),
            cache: HashMap::new(),
        };
        bars.push(bar);
    }
    
    bars
}

/// 解析日期时间字符串
fn parse_datetime(dt_str: &str) -> DateTime<Utc> {
    // 简化版本，实际实现可能需要更复杂的日期解析
    // 这里只是占位符实现
    Utc::now()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_bar_generator_creation() {
        let bg = BarGenerator::new("F5", &[Freq::F15, Freq::F30], 1000);
        assert_eq!(bg.base_freq, Freq::F5);
        assert_eq!(bg.freqs, vec![Freq::F15, Freq::F30]);
    }

    #[test]
    fn test_update_bar() {
        let mut bg = BarGenerator::new("F1", &[], 1000);
        
        let bar = RawBar {
            symbol: "TEST".to_string(),
            id: 1,
            dt: Utc.with_ymd_and_hms(2023, 1, 1, 10, 0, 0).unwrap(),
            freq: Freq::F1,
            open: 100.0,
            close: 105.0,
            high: 108.0,
            low: 99.0,
            vol: 1000.0,
            amount: 100000.0,
            cache: HashMap::new(),
        };
        
        bg.update(&bar);
        let base_bars = bg.get_base_bars();
        assert_eq!(base_bars.len(), 1);
        assert_eq!(base_bars[0].symbol, "TEST");
    }
}