//! 输入输出相关的工具函数
//!
//! 包括 JSON、pickle 等格式的读写功能

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

/// 保存数据为 JSON 格式
pub fn save_json<T>(data: &T, file_path: &str) -> Result<(), Box<dyn std::error::Error>>
where
    T: Serialize,
{
    let mut file = File::create(file_path)?;
    let json_content = serde_json::to_string_pretty(data)?;
    file.write_all(json_content.as_bytes())?;
    Ok(())
}

/// 从 JSON 文件读取数据
pub fn read_json<T>(file_path: &str) -> Result<T, Box<dyn std::error::Error>>
where
    T: for<'de> Deserialize<'de>,
{
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let data: T = serde_json::from_str(&contents)?;
    Ok(data)
}

/// 一个简单的 ZIP 创建函数（占位符实现）
pub fn make_zip(_source_dir: &str, _file_zip: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 在实际实现中，这将使用 zip 库来创建 ZIP 文件
    println!("打包目录为zip文件功能待实现");
    Ok(())
}

/// IO 模块的测试函数
pub fn test_io() -> Result<(), Box<dyn std::error::Error>> {
    let test_data = vec!["hello", "world", "rust"];
    let file_path = "test_io.json";
    
    save_json(&test_data, file_path)?;
    let loaded_data: Vec<String> = read_json(file_path)?;
    
    // 清理测试文件
    std::fs::remove_file(file_path)?;
    
    println!("IO test completed with data: {:?}", loaded_data);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_save_and_read_json() {
        let test_data = vec!["test", "data", "here"];
        let file_path = "test_save_and_read_json.json";

        // Save data
        save_json(&test_data, file_path).expect("Failed to save JSON");

        // Read data back
        let loaded_data: Vec<String> = read_json(file_path).expect("Failed to read JSON");

        assert_eq!(test_data, loaded_data);

        // Clean up
        let _ = fs::remove_file(file_path);
    }
}