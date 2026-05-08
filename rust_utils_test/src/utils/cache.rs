//! 缓存相关的工具函数和结构体
//!
//! 包括磁盘缓存、缓存装饰器等功能

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use serde_json;
use std::io::Write;

/// 获取用户主目录路径
pub fn home_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".czsc")
}

/// 获取目录大小（模拟实现）
pub fn get_dir_size<P: AsRef<Path>>(_path: P) -> u64 {
    // 在实际实现中，这应该遍历目录并累加文件大小
    0
}

/// 清空缓存路径
pub fn empty_cache_path() -> Result<(), Box<dyn std::error::Error>> {
    let path = home_path();
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    println!("已清空缓存文件夹：{}", path.display());
    Ok(())
}

/// 磁盘缓存结构体
#[derive(Debug)]
pub struct DiskCache {
    path: PathBuf,
}

impl DiskCache {
    /// 创建新的磁盘缓存实例
    pub fn new(path: Option<PathBuf>) -> Result<Self, Box<dyn std::error::Error>> {
        let base_path = path.unwrap_or_else(home_path);
        
        if base_path.is_file() {
            return Err("path must be a directory, not a file".into());
        }
        
        fs::create_dir_all(&base_path)?;
        
        Ok(DiskCache { path: base_path })
    }

    /// 检查缓存是否存在
    pub fn is_found(&self, k: &str, suffix: &str, ttl: i64) -> bool {
        let file_path = self.path.join(format!("{}.{}", k, suffix));
        
        if !file_path.exists() {
            println!("缓存文件不存在, {}", file_path.display());
            return false;
        }

        if ttl > 0 {
            // 检查文件修改时间是否超过 TTL
            if let Ok(metadata) = file_path.metadata() {
                if let Ok(modified) = metadata.modified() {
                    if let Ok(elapsed) = modified.elapsed() {
                        if elapsed.as_secs() > ttl as u64 {
                            println!("缓存文件已过期, {}", file_path.display());
                            let _ = fs::remove_file(&file_path); // 忽略删除错误
                            return false;
                        }
                    }
                }
            }
        }

        println!("缓存文件已找到, {}", file_path.display());
        true
    }

    /// 获取缓存内容
    pub fn get<T>(&self, k: &str, suffix: &str) -> Result<Option<T>, Box<dyn std::error::Error>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let file_path = self.path.join(format!("{}.{}", k, suffix));
        println!("正在读取缓存记录，地址：{}", file_path.display());

        if !file_path.exists() {
            println!("文件不存在, {}", file_path.display());
            return Ok(None);
        }

        match suffix {
            "json" => {
                let content = fs::read_to_string(&file_path)?;
                let data: T = serde_json::from_str(&content)?;
                Ok(Some(data))
            }
            "txt" => {
                let content = fs::read_to_string(&file_path)?;
                // 对于文本文件，需要特殊处理
                Err("txt suffix not fully implemented for generic deserialization".into())
            }
            _ => Err(format!("suffix {} not supported", suffix).into()),
        }
    }

    /// 设置缓存内容
    pub fn set<T>(&self, k: &str, v: &T, suffix: &str) -> Result<(), Box<dyn std::error::Error>>
    where
        T: Serialize,
    {
        let file_path = self.path.join(format!("{}.{}", k, suffix));
        
        if file_path.exists() {
            println!("缓存文件 {} 将被覆盖", file_path.display());
        }

        match suffix {
            "json" => {
                let content = serde_json::to_string_pretty(v)?;
                fs::write(&file_path, content)?;
            }
            "txt" => {
                // 需要将值转换为字符串
                if let Some(str_val) = v.serialize(&mut serde_json::Serializer::new(std::io::sink())).ok() {
                    fs::write(&file_path, format!("{:?}", str_val))?;
                } else {
                    return Err("Failed to serialize value as string for txt format".into());
                }
            }
            _ => return Err(format!("suffix {} not supported", suffix).into()),
        }

        println!("已写入缓存文件：{}", file_path.display());
        Ok(())
    }

    /// 删除缓存文件
    pub fn remove(&self, k: &str, suffix: &str) {
        let file_path = self.path.join(format!("{}.{}", k, suffix));
        println!("准备删除缓存文件：{}", file_path.display());
        if file_path.exists() {
            let _ = fs::remove_file(file_path); // 忽略删除错误
        }
    }
}

/// 简单的缓存测试函数
pub fn test_cache() -> Result<(), Box<dyn std::error::Error>> {
    let cache = DiskCache::new(None)?;
    let test_data = "Hello, Cache!";
    
    cache.set("test_key", &test_data, "json")?;
    let retrieved: Option<String> = cache.get("test_key", "json")?;
    
    match retrieved {
        Some(value) => {
            println!("从缓存检索到: {}", value);
            Ok(())
        }
        None => Err("未能从缓存检索到数据".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_home_path() {
        let path = home_path();
        assert!(path.is_absolute());
    }

    #[test]
    fn test_disk_cache() {
        let cache = DiskCache::new(None).expect("Failed to create cache");
        assert!(cache.path.exists());
    }
}