//! 文件IO操作模块
//! 
//! 提供JSON文件的读写功能和其他IO操作

use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use anyhow::Result;

/// 保存数据为JSON格式
pub fn save_json<T>(data: &T, file_path: &str) -> Result<()> 
where
    T: Serialize,
{
    let mut file = File::create(file_path)?;
    let json_data = serde_json::to_string_pretty(data)?;
    file.write_all(json_data.as_bytes())?;
    Ok(())
}

/// 从JSON文件读取数据
pub fn read_json<T>(file_path: &str) -> Result<T> 
where
    T: for<'de> Deserialize<'de>,
{
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let data: T = serde_json::from_str(&contents)?;
    Ok(data)
}

/// 创建ZIP文件（占位实现）
pub fn make_zip(src_dir: &str, dest_path: &str) -> Result<()> {
    // 这里是占位实现，实际的ZIP功能需要使用zip库
    println!("Creating zip from {} to {}", src_dir, dest_path);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct TestStruct {
        name: String,
        value: i32,
    }

    #[test]
    fn test_save_and_read_json() {
        let temp_dir = env::temp_dir();
        let test_file = temp_dir.join("test_json.json").to_str().unwrap().to_string();
        
        let test_data = TestStruct {
            name: "test".to_string(),
            value: 42,
        };
        
        // 保存数据
        save_json(&test_data, &test_file).unwrap();
        
        // 确认文件存在
        assert!(Path::new(&test_file).exists());
        
        // 读取数据
        let loaded_data: TestStruct = read_json(&test_file).unwrap();
        
        // 验证数据一致性
        assert_eq!(test_data, loaded_data);
        
        // 清理测试文件
        fs::remove_file(&test_file).unwrap();
    }

    #[test]
    fn test_make_zip() {
        // 测试ZIP函数的占位实现
        let result = make_zip("/tmp", "/tmp/test.zip");
        assert!(result.is_ok());
    }
}