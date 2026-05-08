use rs_czsc::utils::io::*;

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