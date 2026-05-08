use rs_czsc::{
    RawBar, Freq, BarGenerator, 
    remove_include, check_fx, check_bi, check_fxs,
    Direction, Mark, FX, BI
};
use chrono::{Utc, TimeZone};

fn main() {
    println!("CZSC Rust Implementation Demo");

    // 创建一些示例K线数据
    let bars = vec![
        RawBar {
            symbol: "TEST".to_string(),
            id: 1,
            dt: Utc.with_ymd_and_hms(2023, 1, 1, 9, 30, 0).unwrap(),
            freq: Freq::F5,
            open: 100.0,
            close: 95.0,
            high: 105.0,
            low: 90.0,
            vol: 1000.0,
            amount: 100000.0,
            cache: std::collections::HashMap::new(),
        },
        RawBar {
            symbol: "TEST".to_string(),
            id: 2,
            dt: Utc.with_ymd_and_hms(2023, 1, 1, 9, 35, 0).unwrap(),
            freq: Freq::F5,
            open: 95.0,
            close: 92.0,
            high: 98.0,
            low: 88.0,
            vol: 1200.0,
            amount: 120000.0,
            cache: std::collections::HashMap::new(),
        },
        RawBar {
            symbol: "TEST".to_string(),
            id: 3,
            dt: Utc.with_ymd_and_hms(2023, 1, 1, 9, 40, 0).unwrap(),
            freq: Freq::F5,
            open: 92.0,
            close: 96.0,
            high: 97.0,
            low: 90.0,
            vol: 800.0,
            amount: 80000.0,
            cache: std::collections::HashMap::new(),
        },
    ];

    // 使用K线生成器
    let mut bg = BarGenerator::new("F5", &[Freq::F15], 1000);
    for bar in &bars {
        bg.update(bar);
        println!("更新K线: {}条基础K线", bg.get_bars(Freq::F5).len());
    }

    // 演示分型序列识别
    let raw_bars = bg.get_bars(Freq::F5);
    if raw_bars.len() >= 3 {
        // 将RawBar转换为NewBar以便进行分型识别
        let new_bars: Vec<_> = raw_bars.iter().enumerate().map(|(i, bar)| {
            rs_czsc::objects::NewBar {
                symbol: bar.symbol.clone(),
                id: bar.id,
                dt: bar.dt,
                freq: bar.freq,
                open: bar.open,
                close: bar.close,
                high: bar.high,
                low: bar.low,
                vol: bar.vol,
                amount: bar.amount,
                elements: vec![bar.clone()],
                cache: std::collections::HashMap::new(),
            }
        }).collect();

        let fxs = check_fxs(&new_bars);
        println!("发现 {} 个分型", fxs.len());
    }

    // 演示笔的识别
    let raw_bars = bg.get_bars(Freq::F5);
    if raw_bars.len() >= 5 {
        // 将RawBar转换为NewBar以便进行笔识别
        let new_bars: Vec<_> = raw_bars.iter().enumerate().map(|(i, bar)| {
            rs_czsc::objects::NewBar {
                symbol: bar.symbol.clone(),
                id: bar.id,
                dt: bar.dt,
                freq: bar.freq,
                open: bar.open,
                close: bar.close,
                high: bar.high,
                low: bar.low,
                vol: bar.vol,
                amount: bar.amount,
                elements: vec![bar.clone()],
                cache: std::collections::HashMap::new(),
            }
        }).collect();

        let (bi_opt, _remaining_bars) = check_bi(&new_bars);
        if let Some(bi) = bi_opt {
            println!("发现笔: 方向={:?}, 高点={}, 低点={}", bi.direction, bi.high(), bi.low());
        } else {
            println!("未发现笔");
        }
    }

    println!("演示完成！");
}