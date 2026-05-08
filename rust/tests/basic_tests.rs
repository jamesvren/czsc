use rs_czsc::*;

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_rawbar_creation() {
        let bar = RawBar {
            symbol: "TEST".to_string(),
            id: 1,
            dt: Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap(),
            freq: Freq::D,
            open: 100.0,
            close: 110.0,
            high: 115.0,
            low: 95.0,
            vol: 1000.0,
            amount: 100000.0,
            cache: std::collections::HashMap::new(),
        };

        assert_eq!(bar.symbol, "TEST");
        assert_eq!(bar.id, 1);
        assert_eq!(bar.open, 100.0);
        assert_eq!(bar.close, 110.0);
        assert_eq!(bar.high, 115.0);
        assert_eq!(bar.low, 95.0);
        assert_eq!(bar.vol, 1000.0);
        assert_eq!(bar.amount, 100000.0);
    }

    #[test]
    fn test_rawbar_methods() {
        let bar = RawBar {
            symbol: "TEST".to_string(),
            id: 1,
            dt: Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap(),
            freq: Freq::D,
            open: 100.0,
            close: 110.0,
            high: 115.0,
            low: 95.0,
            vol: 1000.0,
            amount: 100000.0,
            cache: std::collections::HashMap::new(),
        };

        assert_eq!(bar.upper(), 5.0);  // 115 - max(100, 110) = 115 - 110
        assert_eq!(bar.lower(), 5.0);  // min(100, 110) - 95 = 100 - 95
        assert_eq!(bar.solid(), 10.0); // abs(100 - 110) = 10
    }

    #[test]
    fn test_signal_creation() {
        let signal = Signal::new(
            "15分钟".to_string(),
            "倒0笔".to_string(),
            "方向".to_string(),
            "向上".to_string(),
            "其他".to_string(),
            "其他".to_string(),
            0,
        );

        assert_eq!(signal.k1, "15分钟");
        assert_eq!(signal.k2, "倒0笔");
        assert_eq!(signal.k3, "方向");
        assert_eq!(signal.v1, "向上");
        assert_eq!(signal.v2, "其他");
        assert_eq!(signal.v3, "其他");
        assert_eq!(signal.score, 0);
    }

    #[test]
    fn test_bar_generator() {
        let bg = BarGenerator::new("F1", &[Freq::F5], 1000);
        // Test that construction works without accessing private fields
        let base_bars = bg.get_base_bars();
        assert!(base_bars.is_empty());
    }

    #[test]
    fn test_cal_break_even_point() {
        let seq = vec![0.1, -0.05, 0.2, -0.15, 0.05];
        let result = cal_break_even_point(&seq);
        assert!(result >= 0.0);
        assert!(result <= 1.0);
    }
}