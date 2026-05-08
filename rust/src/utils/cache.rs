//! 缓存管理模块
//! 
//! 提供磁盘缓存功能，支持缓存的存储、读取、删除和管理

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use anyhow::Result;

/// 磁盘缓存结构体
pub struct DiskCache {
    /// 缓存目录路径
    pub cache_path: PathBuf,
    /// 缓存内容映射
    pub data: HashMap<String, String>,
}

impl DiskCache {
    /// 创建新的磁盘缓存实例
    pub fn new(cache_path: PathBuf) -> Result<Self> {
        if !cache_path.exists() {
            fs::create_dir_all(&cache_path)?;
        }

        Ok(DiskCache {
            cache_path,
            data: HashMap::new(),
        })
    }

    /// 获取用户主目录路径
    pub fn home_path() -> Result<PathBuf> {
        Ok(dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")))
    }

    /// 获取目录大小（模拟实现）
    pub fn get_dir_size(&self, dir_path: &PathBuf) -> Result<u64> {
        // 这里只是模拟实现，实际的目录大小计算会更复杂
        Ok(0)
    }

    /// 清空缓存路径
    pub fn empty_cache_path(&self, cache_path: &PathBuf) -> Result<()> {
        if cache_path.exists() && cache_path.is_dir() {
            for entry in fs::read_dir(cache_path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    fs::remove_file(path)?;
                }
            }
        }
        Ok(())
    }

    /// 检查缓存是否存在
    pub fn is_found(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    /// 获取缓存内容
    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    /// 设置缓存内容
    pub fn set(&mut self, key: &str, value: String) {
        self.data.insert(key.to_string(), value);
    }

    /// 删除缓存文件
    pub fn remove(&mut self, key: &str) {
        self.data.remove(key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_disk_cache_creation() {
        let temp_dir = env::temp_dir().join("czsc_test_cache");
        let cache = DiskCache::new(temp_dir).unwrap();
        assert!(cache.cache_path.exists());
    }

    #[test]
    fn test_set_and_get() {
        let mut cache = DiskCache::new(env::temp_dir().join("czsc_test_cache2")).unwrap();
        cache.set("test_key", "test_value".to_string());
        assert_eq!(cache.get("test_key"), Some(&"test_value".to_string()));
    }

    #[test]
    fn test_is_found() {
        let mut cache = DiskCache::new(env::temp_dir().join("czsc_test_cache3")).unwrap();
        cache.set("test_key", "test_value".to_string());
        assert!(cache.is_found("test_key"));
        assert!(!cache.is_found("nonexistent_key"));
    }

    #[test]
    fn test_remove() {
        let mut cache = DiskCache::new(env::temp_dir().join("czsc_test_cache4")).unwrap();
        cache.set("test_key", "test_value".to_string());
        assert!(cache.is_found("test_key"));
        cache.remove("test_key");
        assert!(!cache.is_found("test_key"));
    }
}