use crate::objects::RawBar;
use crate::enums::Freq;
use chrono::{DateTime, Utc, NaiveDateTime};
use std::collections::HashMap;

/// 对分钟K线进行格式化
pub fn format_kline(df: Vec<HashMap<String, String>>, freq: Freq) -> Vec<RawBar> {
    let mut raw_bars = Vec::new();
    
    for (i, row) in df.iter().enumerate() {
        // 将 datetime 时间戳从纳秒转换为秒，然后转换为 DateTime
        let datetime_ns_str = &row.get("datetime").unwrap_or(&"0".to_string()).clone();
        let datetime_ns = datetime_ns_str.parse::<i64>().unwrap_or(0);
        let datetime_sec = datetime_ns / 1_000_000_000; // Convert nanoseconds to seconds
        
        // Add 1 minute to get the end time of the candle
        let dt = DateTime::from_timestamp(datetime_sec + 60, 0).unwrap_or(Utc::now());
        
        let bar = RawBar {
            symbol: row.get("symbol").unwrap_or(&"".to_string()).clone(),
            id: i as i32,
            freq: freq.clone(),
            dt,
            open: row.get("open").unwrap_or(&"0".to_string()).parse::<f64>().unwrap_or(0.0),
            close: row.get("close").unwrap_or(&"0".to_string()).parse::<f64>().unwrap_or(0.0),
            high: row.get("high").unwrap_or(&"0".to_string()).parse::<f64>().unwrap_or(0.0),
            low: row.get("low").unwrap_or(&"0".to_string()).parse::<f64>().unwrap_or(0.0),
            vol: row.get("volume").unwrap_or(&"0".to_string()).parse::<f64>().unwrap_or(0.0),
            amount: (row.get("volume").unwrap_or(&"0".to_string()).parse::<f64>().unwrap_or(0.0) *
                     row.get("close").unwrap_or(&"0".to_string()).parse::<f64>().unwrap_or(0.0)),
            cache: Default::default(),
        };
        raw_bars.push(bar);
    }
    raw_bars
}

/// 判断交易时间是否结束
pub fn is_trading_end() -> bool {
    let now = chrono::Local::now();
    let time_str = now.format("%H:%M").to_string();

    // Check if between 8:30 and 16:35 and also after 15:16 (day trading ended)
    if ("08:30".to_string() <= time_str.clone() && time_str.clone() <= "16:35".to_string()) && time_str.as_str() >= "15:16" {
        return true; // Day trading ended
    }

    // Night session ends
    // Check if between 00:30 and 04:00 and also after 02:31 (night trading ended)
    if ("00:30".to_string() <= time_str.clone() && time_str.clone() <= "04:00".to_string()) && time_str.as_str() >= "02:31" {
        return true; // Night trading ended
    }

    false
}

/// 获取期货品种列表
pub fn get_symbols() -> Vec<String> {
    vec![
        // SHFE 上海期货交易所
        "KQ.m@SHFE.rb".to_string(),
        "KQ.m@SHFE.fu".to_string(),
        "KQ.m@SHFE.ag".to_string(),
        "KQ.m@SHFE.hc".to_string(),
        "KQ.m@SHFE.sp".to_string(),
        "KQ.m@SHFE.ru".to_string(),
        "KQ.m@SHFE.bu".to_string(),
        "KQ.m@SHFE.ni".to_string(),
        "KQ.m@SHFE.ss".to_string(),
        "KQ.m@SHFE.au".to_string(),
        "KQ.m@SHFE.sn".to_string(),
        "KQ.m@SHFE.al".to_string(),
        "KQ.m@SHFE.ao".to_string(),
        "KQ.m@SHFE.zn".to_string(),
        "KQ.m@SHFE.cu".to_string(),
        "KQ.m@SHFE.pb".to_string(),
        "KQ.m@SHFE.br".to_string(),
        // CZCE 郑州商品交易所
        "KQ.m@CZCE.SA".to_string(),
        "KQ.m@CZCE.FG".to_string(),
        "KQ.m@CZCE.TA".to_string(),
        "KQ.m@CZCE.MA".to_string(),
        "KQ.m@CZCE.RM".to_string(),
        "KQ.m@CZCE.CF".to_string(),
        "KQ.m@CZCE.OI".to_string(),
        "KQ.m@CZCE.SR".to_string(),
        "KQ.m@CZCE.UR".to_string(),
        "KQ.m@CZCE.PF".to_string(),
        "KQ.m@CZCE.AP".to_string(),
        "KQ.m@CZCE.SF".to_string(),
        "KQ.m@CZCE.PX".to_string(),
        "KQ.m@CZCE.CJ".to_string(),
        "KQ.m@CZCE.PK".to_string(),
        "KQ.m@CZCE.SM".to_string(),
        "KQ.m@CZCE.CY".to_string(),
        "KQ.m@CZCE.RS".to_string(),
        // DCE 大连商品交易所
        "KQ.m@DCE.m".to_string(),
        "KQ.m@DCE.p".to_string(),
        "KQ.m@DCE.i".to_string(),
        "KQ.m@DCE.v".to_string(),
        "KQ.m@DCE.y".to_string(),
        "KQ.m@DCE.eg".to_string(),
        "KQ.m@DCE.c".to_string(),
        "KQ.m@DCE.pp".to_string(),
        "KQ.m@DCE.l".to_string(),
        "KQ.m@DCE.cs".to_string(),
        "KQ.m@DCE.a".to_string(),
        "KQ.m@DCE.eb".to_string(),
        "KQ.m@DCE.jm".to_string(),
        "KQ.m@DCE.b".to_string(),
        "KQ.m@DCE.pg".to_string(),
        "KQ.m@DCE.jd".to_string(),
        "KQ.m@DCE.j".to_string(),
        "KQ.m@DCE.lh".to_string(),
        "KQ.m@DCE.rr".to_string(),
        "KQ.m@DCE.fb".to_string(),
        "KQ.m@DCE.bb".to_string(),
        // GFEX 广州期货交易所
        "KQ.m@GFEX.si".to_string(),
        "KQ.m@GFEX.lc".to_string(),
        // INE 上海国际能源交易中心
        "KQ.m@INE.lu".to_string(),
        "KQ.m@INE.sc".to_string(),
        "KQ.m@INE.nr".to_string(),
        "KQ.m@INE.bc".to_string(),
        "KQ.m@INE.ec".to_string(),
        // CFFEX 中国金融期货交易所
        "KQ.m@CFFEX.T".to_string(),
        "KQ.m@CFFEX.TF".to_string(),
        "KQ.m@CFFEX.TS".to_string(),
        "KQ.m@CFFEX.TL".to_string(),
        "KQ.m@CFFEX.IF".to_string(),
        "KQ.m@CFFEX.IC".to_string(),
        "KQ.m@CFFEX.IH".to_string(),
        "KQ.m@CFFEX.IM".to_string(),
    ]
}

/// 期货品种名称映射
pub fn future_name_map() -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("AO".to_string(), "氧化铝".to_string());
    map.insert("PX".to_string(), "对二甲苯".to_string());
    map.insert("EC".to_string(), "欧线集运".to_string());
    map.insert("LC".to_string(), "碳酸锂".to_string());
    map.insert("PG".to_string(), "LPG".to_string());
    map.insert("EB".to_string(), "苯乙烯".to_string());
    map.insert("CS".to_string(), "玉米淀粉".to_string());
    map.insert("C".to_string(), "玉米".to_string());
    map.insert("V".to_string(), "PVC".to_string());
    map.insert("J".to_string(), "焦炭".to_string());
    map.insert("BB".to_string(), "胶合板".to_string());
    map.insert("M".to_string(), "豆粕".to_string());
    map.insert("A".to_string(), "豆一".to_string());
    map.insert("PP".to_string(), "聚丙烯".to_string());
    map.insert("P".to_string(), "棕榈油".to_string());
    map.insert("FB".to_string(), "纤维板".to_string());
    map.insert("B".to_string(), "豆二".to_string());
    map.insert("JD".to_string(), "鸡蛋".to_string());
    map.insert("JM".to_string(), "焦煤".to_string());
    map.insert("L".to_string(), "塑料".to_string());
    map.insert("I".to_string(), "铁矿石".to_string());
    map.insert("Y".to_string(), "豆油".to_string());
    map.insert("RR".to_string(), "粳米".to_string());
    map.insert("EG".to_string(), "乙二醇".to_string());
    map.insert("LH".to_string(), "生猪".to_string());
    map.insert("CJ".to_string(), "红枣".to_string());
    map.insert("UR".to_string(), "尿素".to_string());
    map.insert("TA".to_string(), "PTA".to_string());
    map.insert("OI".to_string(), "菜油".to_string());
    map.insert("MA".to_string(), "甲醇".to_string());
    map.insert("RS".to_string(), "菜籽".to_string());
    map.insert("ZC".to_string(), "动力煤".to_string());
    map.insert("LR".to_string(), "晚籼稻".to_string());
    map.insert("PM".to_string(), "普麦".to_string());
    map.insert("SR".to_string(), "白糖".to_string());
    map.insert("RI".to_string(), "早籼稻".to_string());
    map.insert("SF".to_string(), "硅铁".to_string());
    map.insert("WH".to_string(), "强麦".to_string());
    map.insert("JR".to_string(), "粳稻".to_string());
    map.insert("SM".to_string(), "锰硅".to_string());
    map.insert("FG".to_string(), "玻璃".to_string());
    map.insert("CF".to_string(), "棉花".to_string());
    map.insert("RM".to_string(), "菜粕".to_string());
    map.insert("PF".to_string(), "短纤".to_string());
    map.insert("AP".to_string(), "苹果".to_string());
    map.insert("CY".to_string(), "棉纱".to_string());
    map.insert("ER".to_string(), "早籼稻".to_string());
    map.insert("ME".to_string(), "甲醇".to_string());
    map.insert("RO".to_string(), "菜油".to_string());
    map.insert("TC".to_string(), "动力煤".to_string());
    map.insert("WS".to_string(), "强麦".to_string());
    map.insert("WT".to_string(), "硬麦".to_string());
    map.insert("SA".to_string(), "纯碱".to_string());
    map.insert("PK".to_string(), "花生".to_string());
    map.insert("SS".to_string(), "不锈钢".to_string());
    map.insert("AL".to_string(), "沪铝".to_string());
    map.insert("CU".to_string(), "沪铜".to_string());
    map.insert("ZN".to_string(), "沪锌".to_string());
    map.insert("AG".to_string(), "白银".to_string());
    map.insert("RB".to_string(), "螺纹钢".to_string());
    map.insert("SN".to_string(), "沪锡".to_string());
    map.insert("NI".to_string(), "沪镍".to_string());
    map.insert("WR".to_string(), "线材".to_string());
    map.insert("FU".to_string(), "燃油".to_string());
    map.insert("AU".to_string(), "黄金".to_string());
    map.insert("PB".to_string(), "沪铅".to_string());
    map.insert("RU".to_string(), "橡胶".to_string());
    map.insert("BR".to_string(), "合成橡胶".to_string());
    map.insert("HC".to_string(), "热轧卷板".to_string());
    map.insert("BU".to_string(), "沥青".to_string());
    map.insert("SP".to_string(), "纸浆".to_string());
    map.insert("NR".to_string(), "20号胶".to_string());
    map.insert("SC".to_string(), "原油".to_string());
    map.insert("LU".to_string(), "低硫燃料油".to_string());
    map.insert("BC".to_string(), "国际铜".to_string());
    map.insert("SCTAS".to_string(), "原油TAS指令".to_string());
    map.insert("SI".to_string(), "工业硅".to_string());
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_kline() {
        let mut row = HashMap::new();
        row.insert("symbol".to_string(), "KQ.m@SHFE.rb".to_string());
        row.insert("datetime".to_string(), "1609459200000000000".to_string()); // 2021-01-01 00:00:00 in nanoseconds
        row.insert("open".to_string(), "4000".to_string());
        row.insert("close".to_string(), "4050".to_string());
        row.insert("high".to_string(), "4080".to_string());
        row.insert("low".to_string(), "3990".to_string());
        row.insert("volume".to_string(), "1000".to_string());

        let df = vec![row];
        let freq = Freq::F1;
        let bars = format_kline(df, freq);

        assert_eq!(bars.len(), 1);
        assert_eq!(bars[0].symbol, "KQ.m@SHFE.rb");
        assert_eq!(bars[0].open, 4000.0);
    }
}