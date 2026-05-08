//! 基于ClickHouse的策略持仓权重管理
//!
//! 该模块实现了基于ClickHouse数据库的策略持仓权重管理功能，
//! cwc为clickhouse weights client的缩写。

use std::collections::HashMap;
use chrono::{DateTime, Utc, NaiveDateTime};
use serde::{Deserialize, Serialize};

// 这里我们使用一个简化的ClickHouse客户端接口
// 在实际应用中，您可能需要引入一个真实的ClickHouse客户端库
pub trait ClickHouseClient {
    fn command(&self, sql: &str, params: Option<&HashMap<String, String>>) -> Result<(), Box<dyn std::error::Error>>;
    fn query_df(&self, sql: &str, params: Option<&HashMap<String, String>>) -> Result<Vec<HashMap<String, String>>, Box<dyn std::error::Error>>;
    fn insert_df(&self, table: &str, data: &Vec<HashMap<String, String>>) -> Result<(), Box<dyn std::error::Error>>;
}

/// 确保时间戳具有指定时区
pub fn ensure_timestamp(value: Option<&str>, _tz: &str) -> Option<DateTime<Utc>> {
    if let Some(val) = value {
        if val.trim().is_empty() {
            return None;
        }
        
        // 尝试解析时间字符串
        if let Ok(ts) = DateTime::parse_from_rfc3339(val) {
            return Some(ts.with_timezone(&Utc));
        }
        
        if let Ok(ndt) = NaiveDateTime::parse_from_str(val, "%Y-%m-%d %H:%M:%S") {
            return Some(DateTime::<Utc>::from_naive_utc_and_offset(ndt, Utc));
        }
        
        if let Ok(date) = chrono::NaiveDate::parse_from_str(val, "%Y-%m-%d") {
            let ndt = chrono::NaiveDateTime::new(date, chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap());
            return Some(DateTime::<Utc>::from_naive_utc_and_offset(ndt, Utc));
        }
    }
    None
}

/// 将带时区的时间对象格式化为ClickHouse兼容的字符串
pub fn format_for_db(value: Option<&str>, _tz: &str) -> Option<String> {
    ensure_timestamp(value, "").map(|ts| ts.naive_utc().format("%Y-%m-%d %H:%M:%S").to_string())
}

/// 初始化数据库表
pub fn init_tables<T: ClickHouseClient>(
    db: &T,
    database: &str,
    _kwargs: &HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    // 创建数据库
    let create_db_sql = format!("CREATE DATABASE IF NOT EXISTS {}", database);
    db.command(&create_db_sql, None)?;

    // 创建metas表
    let metas_table = format!(
        "CREATE TABLE IF NOT EXISTS {}.metas (
            strategy String NOT NULL,                              -- 策略名（唯一且不能为空）
            base_freq String,                                      -- 周期
            description String,                                    -- 描述
            author String,                                         -- 作者
            outsample_sdt DateTime('Asia/Shanghai'),               -- 样本外起始时间
            create_time DateTime('Asia/Shanghai'),                 -- 策略入库时间
            update_time DateTime('Asia/Shanghai'),                 -- 策略更新时间
            heartbeat_time DateTime('Asia/Shanghai'),              -- 最后一次心跳时间
            weight_type String,                                    -- 策略上传的权重类型，ts 或 cs
            status String DEFAULT '实盘',                           -- 策略状态：实盘、废弃
            memo String                                            -- 策略备忘信息
        )
        ENGINE = ReplacingMergeTree()
        ORDER BY strategy;",
        database
    );

    // 创建weights表
    let weights_table = format!(
        "CREATE TABLE IF NOT EXISTS {}.weights (
            dt DateTime('Asia/Shanghai'),            -- 持仓权重时间
            symbol String,                           -- 符号（例如，股票代码或其他标识符）
            weight Float64,                          -- 策略持仓权重值
            strategy String,                         -- 策略名称
            update_time DateTime('Asia/Shanghai')    -- 持仓权重更新时间
        )
        ENGINE = ReplacingMergeTree()
        ORDER BY (strategy, dt, symbol);",
        database
    );

    // 创建returns表
    let returns_table = format!(
        "CREATE TABLE IF NOT EXISTS {}.returns (
            dt DateTime('Asia/Shanghai'),            -- 时间
            symbol String,                           -- 符号（例如，股票代码或其他标识符）
            returns Float64,                         -- 策略收益，从上一个 dt 到当前 dt 的收益
            strategy String,                         -- 策略名称
            update_time DateTime('Asia/Shanghai')    -- 更新时间
        )
        ENGINE = ReplacingMergeTree()
        ORDER BY (strategy, dt, symbol);",
        database
    );

    db.command(&metas_table, None)?;
    println!("metas 表创建成功！");
    
    db.command(&weights_table, None)?;
    println!("weights 表创建成功！");
    
    db.command(&returns_table, None)?;
    println!("returns 表创建成功！");

    Ok(())
}

/// 初始化最新权重视图
pub fn init_latest_weights_view<T: ClickHouseClient>(
    db: &T,
    database: &str,
    _kwargs: &HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    // 创建截面策略的最新持仓视图
    let cs_view_sql = format!(
        "CREATE VIEW IF NOT EXISTS {}.cs_latest_weights AS
        WITH latest_dates AS (
            SELECT 
                strategy,
                MAX(dt) AS latest_dt
            FROM {}.weights
            GROUP BY strategy
        )
        SELECT 
            w.dt as dt,
            w.symbol as symbol,
            w.weight as weight,
            w.strategy as strategy,
            w.update_time as update_time
        FROM {}.weights w
        JOIN latest_dates ld ON w.strategy = ld.strategy AND w.dt = ld.latest_dt
        JOIN {}.metas m ON w.strategy = m.strategy
        WHERE m.weight_type = 'cs'",
        database, database, database, database
    );
    db.command(&cs_view_sql, None)?;
    println!("cs_latest_weights 视图初始化完成");

    // 创建时序策略的最新持仓视图
    let ts_view_sql = format!(
        "CREATE VIEW IF NOT EXISTS {}.ts_latest_weights AS
        WITH latest_records AS (
            SELECT 
                strategy,
                symbol,
                MAX(dt) AS latest_dt
            FROM {}.weights
            GROUP BY strategy, symbol
        )
        SELECT 
            w.dt as dt,
            w.symbol as symbol,
            w.weight as weight,
            w.strategy as strategy,
            w.update_time as update_time
        FROM {}.weights w
        JOIN latest_records lr ON w.strategy = lr.strategy 
                              AND w.symbol = lr.symbol 
                              AND w.dt = lr.latest_dt
        JOIN {}.metas m ON w.strategy = m.strategy
        WHERE m.weight_type = 'ts'",
        database, database, database, database
    );
    db.command(&ts_view_sql, None)?;
    println!("ts_latest_weights 视图初始化完成");

    // 创建合并的最新持仓视图
    let latest_view_sql = format!(
        "CREATE VIEW IF NOT EXISTS {}.latest_weights AS
        SELECT * FROM {}.ts_latest_weights
        UNION ALL
        SELECT * FROM {}.cs_latest_weights",
        database, database, database
    );
    db.command(&latest_view_sql, None)?;
    println!("latest_weights 视图初始化完成");

    println!("所有最新持仓权重视图初始化完成");
    Ok(())
}

/// 初始化数据库，包括创建数据表和最新持仓视图
pub fn initialize<T: ClickHouseClient>(
    db: &T,
    database: &str,
    kwargs: &HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    init_tables(db, database, kwargs)?;
    init_latest_weights_view(db, database, kwargs)
}

/// 获取策略元数据
pub fn get_meta<T: ClickHouseClient>(
    strategy: &str,
    db: &T,
    database: &str,
    _tz: &chrono_tz::Tz,
) -> Result<Option<HashMap<String, String>>, Box<dyn std::error::Error>> {
    let sql = format!(
        "SELECT * FROM {}.metas final WHERE strategy = %(strategy)s",
        database
    );
    
    let mut params = HashMap::new();
    params.insert("strategy".to_string(), strategy.to_string());
    
    let result = db.query_df(&sql, Some(&params))?;
    
    if result.is_empty() {
        println!("策略 {} 不存在元数据", strategy);
        return Ok(None);
    }
    
    if result.len() > 1 {
        return Err("策略 {} 存在多条元数据，请检查".to_string().into());
    }
    
    Ok(Some(result[0].clone()))
}

/// 设置策略元数据
pub fn set_meta<T: ClickHouseClient>(
    strategy: &str,
    base_freq: &str,
    description: &str,
    author: &str,
    outsample_sdt: &str,
    weight_type: Option<&str>,
    status: Option<&str>,
    memo: Option<&str>,
    overwrite: bool,
    database: &str,
    db: &T,
    _tz: &chrono_tz::Tz,
) -> Result<(), Box<dyn std::error::Error>> {
    let weight_type = weight_type.unwrap_or("ts");
    let status = status.unwrap_or("实盘");
    let memo = memo.unwrap_or("");
    
    let existing_meta = get_meta(strategy, db, database, _tz)?;
    
    if !overwrite && existing_meta.is_some() {
        println!("策略 {} 已存在元数据，如需更新请设置 overwrite=true", strategy);
        return Ok(());
    }

    let create_time = if let Some(meta) = existing_meta {
        meta.get("create_time").unwrap_or(&Utc::now().to_rfc3339()).clone()
    } else {
        Utc::now().to_rfc3339()
    };

    let current_time = Utc::now().to_rfc3339();
    
    // 构建数据
    let mut row = HashMap::new();
    row.insert("strategy".to_string(), strategy.to_string());
    row.insert("base_freq".to_string(), base_freq.to_string());
    row.insert("description".to_string(), description.to_string());
    row.insert("author".to_string(), author.to_string());
    row.insert("outsample_sdt".to_string(), outsample_sdt.to_string());
    row.insert("create_time".to_string(), create_time);
    row.insert("update_time".to_string(), current_time.clone());
    row.insert("heartbeat_time".to_string(), current_time.clone());
    row.insert("weight_type".to_string(), weight_type.to_string());
    row.insert("status".to_string(), status.to_string());
    row.insert("memo".to_string(), memo.to_string());

    let data = vec![row];
    db.insert_df(&format!("{}.metas", database), &data)?;
    
    println!("{} set_metadata: success", strategy);
    Ok(())
}

/// 发送心跳
fn send_heartbeat<T: ClickHouseClient>(
    db: &T,
    strategy: &str,
    database: &str,
    _tz: &chrono_tz::Tz,
) -> Result<(), Box<dyn std::error::Error>> {
    let meta = get_meta(strategy, db, database, _tz)?;
    if meta.is_none() {
        println!("策略 {} 不存在元数据，无法发送心跳", strategy);
        return Ok(());
    }

    let current_time = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    let sql = format!(
        "ALTER TABLE {}.metas UPDATE heartbeat_time = '{}' WHERE strategy = '{}'",
        database, current_time, strategy
    );
    
    db.command(&sql, None)?;
    println!("策略 {} 发送心跳成功", strategy);
    Ok(())
}

/// 发布策略持仓权重
pub fn publish_weights<T: ClickHouseClient>(
    strategy: &str,
    df: &Vec<HashMap<String, String>>,  // 包含dt, symbol, weight列
    batch_size: usize,
    db: &T,
    database: &str,
    _tz: &chrono_tz::Tz,
) -> Result<(), Box<dyn std::error::Error>> {
    // 发送心跳
    send_heartbeat(db, strategy, database, _tz)?;

    // 过滤并准备数据
    let mut prepared_data = Vec::new();
    for row in df {
        if !row.contains_key("dt") || !row.contains_key("symbol") || !row.contains_key("weight") {
            continue; // 跳过缺少必要字段的行
        }
        
        let mut new_row = row.clone();
        new_row.insert("strategy".to_string(), strategy.to_string());
        new_row.insert("update_time".to_string(), Utc::now().to_rfc3339());
        
        prepared_data.push(new_row);
    }

    // 按时间排序
    prepared_data.sort_by(|a, b| {
        a.get("dt").unwrap().cmp(b.get("dt").unwrap())
    });

    // 批量插入
    for chunk in prepared_data.chunks(batch_size) {
        db.insert_df(&format!("{}.weights", database), &chunk.to_vec())?;
        
        // 发送心跳
        send_heartbeat(db, strategy, database, _tz)?;
        println!("完成批次发布，发布 {} 条信号", chunk.len());
    }

    println!("完成所有信号发布，共 {} 条", prepared_data.len());
    Ok(())
}

/// 更新策略状态
pub fn update_strategy_status<T: ClickHouseClient>(
    strategy: &str,
    status: &str,
    db: &T,
    database: &str,
    _tz: &chrono_tz::Tz,
) -> Result<(), Box<dyn std::error::Error>> {
    let valid_statuses = ["实盘", "废弃"];
    if !valid_statuses.contains(&status) {
        return Err(format!("无效的策略状态: {}，有效状态为: {:?}", status, valid_statuses).into());
    }

    let meta = get_meta(strategy, db, database, _tz)?;
    if meta.is_none() {
        println!("策略 {} 不存在，无法更新状态", strategy);
        return Ok(());
    }

    let current_time = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let sql = format!(
        "ALTER TABLE {}.metas UPDATE status = '{}', update_time = '{}' WHERE strategy = '{}'",
        database, status, current_time, strategy
    );

    db.command(&sql, None)?;
    println!("策略 {} 状态已更新为: {}", strategy, status);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // 简单的测试用ClickHouse客户端实现
    struct MockClickHouseClient;

    impl ClickHouseClient for MockClickHouseClient {
        fn command(&self, _sql: &str, _params: Option<&HashMap<String, String>>) -> Result<(), Box<dyn std::error::Error>> {
            Ok(())
        }
        
        fn query_df(&self, _sql: &str, _params: Option<&HashMap<String, String>>) -> Result<Vec<HashMap<String, String>>, Box<dyn std::error::Error>> {
            Ok(vec![])
        }
        
        fn insert_df(&self, _table: &str, _data: &Vec<HashMap<String, String>>) -> Result<(), Box<dyn std::error::Error>> {
            Ok(())
        }
    }

    #[test]
    fn test_init_tables() {
        let client = MockClickHouseClient;
        let kwargs = HashMap::new();
        let result = init_tables(&client, "test_db", &kwargs);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_for_db() {
        let tz = &chrono::FixedOffset::east_opt(8 * 3600).unwrap(); // 使用东八区时区
        let formatted = format_for_db(Some("2023-01-01 12:00:00"), "Asia/Shanghai");
        assert!(formatted.is_some());
    }
}