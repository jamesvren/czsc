use rs_czsc::utils::cache::*;

#[test]
fn test_home_path() {
    let path = home_path();
    assert!(path.contains(".czsc"));
}

#[test]
fn test_disk_cache() {
    let cache = DiskCache::new();
    let key = "test_key";
    let value = "Hello, Cache!";
    
    // 设置缓存
    cache.set(key, value, "").unwrap();
    
    // 获取缓存
    let retrieved: String = cache.get(key, "").unwrap();
    assert_eq!(retrieved, value);
    
    // 清理
    cache.remove(key, "");
}

#[test]
fn test_cache_operations() {
    let cache = DiskCache::new();
    let key = "operation_test";
    
    // 测试不存在的键
    assert!(!cache.is_found(key, "", 3600));
    
    // 设置值
    cache.set(key, "test_value", "").unwrap();
    
    // 验证存在且能获取
    assert!(cache.is_found(key, "", 3600));
    let value: String = cache.get(key, "").unwrap();
    assert_eq!(value, "test_value");
    
    // 删除
    cache.remove(key, "");
    
    // 验证已删除
    assert!(!cache.is_found(key, "", 3600));
}