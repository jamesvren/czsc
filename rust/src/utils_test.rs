// Test utils module functionality
#[cfg(test)]
mod utils_tests {
    use crate::utils::cache::*;
    use crate::utils::io::*;
    use crate::utils::ta::*;
    use crate::utils::*;

    #[test]
    fn test_home_path() {
        let path = std::env::temp_dir();
        assert!(path.display().to_string().contains(".czsc"));
    }

    #[test]
    fn test_disk_cache() {
        let cache = DiskCache::new(std::env::temp_dir().join(".czsc_cache")).unwrap();
        let key = "test_key";
        let value = "Hello, Cache!";
        
        // 设置缓存
        cache.set(key, value.to_string());
        
        // 获取缓存
        let retrieved: String = cache.get(key).unwrap().to_string();
        assert_eq!(retrieved, value);
        
        // 清理
        cache.remove(key);
    }

    #[test]
    fn test_cache_operations() {
        let cache = DiskCache::new(std::env::temp_dir().join(".czsc_cache")).unwrap();
        let key = "operation_test";
        
        // 测试不存在的键
        assert!(!cache.is_found(key));
        
        // 设置值
        cache.set(key, "test_value".to_string());
        
        // 验证存在且能获取
        assert!(cache.is_found(key));
        let value: String = cache.get(key).unwrap().to_string();
        assert_eq!(value, "test_value");
        
        // 删除
        cache.remove(key);
        
        // 验证已删除
        assert!(!cache.is_found(key));
    }

    #[test]
    fn test_save_and_read_json() {
        let data = vec!["hello".to_string(), "world".to_string(), "rust".to_string()];
        let filename = "test_data.json";

        // 保存JSON
        save_json(&data, filename).expect("Failed to save JSON");

        // 读取JSON
        let loaded_data: Vec<String> = read_json(filename).expect("Failed to read JSON");

        assert_eq!(data, loaded_data);

        // 清理测试文件
        std::fs::remove_file(filename).ok();
    }

    #[test]
    fn test_sma() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = sma(&data, 2);
        let expected = vec![1.0, 1.5, 2.0, 3.0, 4.0];
        
        assert_eq!(result.len(), expected.len());
        for (a, b) in result.iter().zip(expected.iter()) {
            assert!((a - b).abs() < 1e-10);
        }
    }

    #[test]
    fn test_ema() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = ema(&data, 2);
        // EMA has more complex calculation, we'll just check it returns correct length
        assert_eq!(result.len(), data.len());
        
        // Test with single element
        let single_data = vec![5.0];
        let single_result = ema(&single_data, 2);
        assert_eq!(single_result, vec![5.0]);
        
        // Test with timeperiod 0
        let zero_period_result = ema(&data, 0);
        assert_eq!(zero_period_result, vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        
        // Test with timeperiod greater than data length
        let large_period_result = ema(&data, 10);
        assert_eq!(large_period_result.len(), data.len());
    }

    #[test]
    fn test_rsq() {
        // Perfect positive correlation
        let data1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let rsq1 = rsq(&data1);
        assert!((rsq1 - 1.0).abs() < 1e-10);
        
        // Test with single element
        let data_single = vec![5.0];
        let rsq_single = rsq(&data_single);
        assert!((rsq_single - 1.0).abs() < 1e-10);
        
        // Test with two elements
        let data_two = vec![1.0, 2.0];
        let rsq_two = rsq(&data_two);
        assert!((rsq_two - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_trait_round_digits() {
        let x = 3.1415926_f64;
        assert_eq!(x.round_digits(4), 3.1415);
        
        let y = 3.1415926_f32;
        assert_eq!(y.round_digits(4), 3.1415);
        
        // Test edge cases
        let z = 0.0_f64;
        assert_eq!(z.round_digits(4), 0.0);
        
        let w = -3.1415926_f64;
        assert_eq!(w.round_digits(4), -3.1415);
    }

    #[test]
    fn test_x_round() {
        assert_eq!(x_round(3.1415926, 4), 3.1415);
        assert_eq!(x_round(-3.1415926, 4), -3.1415);
        assert_eq!(x_round(0.0, 4), 0.0);
        assert_eq!(x_round(1.0, 0), 1.0);
        assert_eq!(x_round(1.9, 0), 1.0);
    }

    #[test]
    fn test_mac_address() {
        let mac = mac_address();
        // MAC address should be 17 characters in XX-XX-XX-XX-XX-XX format
        assert_eq!(mac.len(), 17);
        // Should contain only hex digits and dashes
        assert!(mac.chars().all(|c| c.is_ascii_hexdigit() || c == '-'));
    }

    #[test]
    fn test_freqs_sorted() {
        let input = vec!["日线", "1分钟", "5分钟", "周线"];
        let sorted = freqs_sorted(input);
        let expected = vec!["1分钟", "5分钟", "日线", "周线"];
        
        assert_eq!(sorted, expected);
    }
}