//! 策略持仓权重管理 - Redis客户端
//!
//! 该模块实现了基于Redis的策略持仓权重管理功能，
//! rwc为redis weights client的缩写。

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json;

/// Redis权重客户端
pub struct RedisWeightsClient {
    pub strategy_name: String,
    pub key_prefix: String,
    pub heartbeat_prefix: String,
    pub send_heartbeat: bool,
    pub heartbeat_thread: Option<thread::JoinHandle<()>>,
    // 模拟Redis连接
    pub redis_connection: Arc<Mutex<HashMap<String, String>>>,
}

impl RedisWeightsClient {
    /// 创建新的Redis权重客户端
    pub fn new(
        strategy_name: String,
        redis_url: Option<String>,
        connection_pool: Option<String>,
        send_heartbeat: Option<bool>,
        kwargs: HashMap<String, String>,
    ) -> Self {
        let key_prefix = kwargs.get("key_prefix").unwrap_or(&"Weights".to_string()).clone();
        let heartbeat_prefix = kwargs.get("heartbeat_prefix").unwrap_or(&"heartbeat".to_string()).clone();
        let send_heartbeat = send_heartbeat.unwrap_or(true);
        
        // 模拟Redis连接
        let redis_connection = Arc::new(Mutex::new(HashMap::new()));
        
        let mut client = RedisWeightsClient {
            strategy_name,
            key_prefix,
            heartbeat_prefix,
            send_heartbeat,
            heartbeat_thread: None,
            redis_connection,
        };
        
        // 如果需要发送心跳，启动心跳线程
        if send_heartbeat {
            let strategy_name = client.strategy_name.clone();
            let key_prefix = client.key_prefix.clone();
            let heartbeat_prefix = client.heartbeat_prefix.clone();
            let redis_conn = client.redis_connection.clone();
            
            let handle = thread::spawn(move || {
                loop {
                    let key = format!("{}:{}:{}", key_prefix, heartbeat_prefix, strategy_name);
                    let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
                    
                    let mut conn = redis_conn.lock().unwrap();
                    conn.insert(key, timestamp);
                    drop(conn); // 释放锁
                    
                    thread::sleep(Duration::from_secs(15));
                }
            });
            
            client.heartbeat_thread = Some(handle);
        }
        
        client
    }
    
    /// 设置策略元数据
    pub fn set_metadata(&self, base_freq: &str, description: &str, author: &str, outsample_sdt: &str, kwargs: HashMap<String, String>) {
        let key = format!("{}:META:{}", self.key_prefix, self.strategy_name);
        let overwrite = kwargs.get("overwrite").map(|s| s == "true").unwrap_or(false);
        
        let mut conn = self.redis_connection.lock().unwrap();
        
        if conn.contains_key(&key) && !overwrite {
            println!("已存在 {} 的元数据，如需覆盖请设置 overwrite=true", self.strategy_name);
            return;
        }
        
        if conn.contains_key(&key) && overwrite {
            conn.remove(&key);
            println!("删除 {} 的元数据，重新写入", self.strategy_name);
        }
        
        let outsample_sdt_formatted = chrono::NaiveDate::parse_from_str(outsample_sdt, "%Y%m%d")
            .map(|date| date.format("%Y%m%d").to_string())
            .unwrap_or_else(|_| outsample_sdt.to_string());
        
        let mut meta = HashMap::new();
        meta.insert("name".to_string(), self.strategy_name.clone());
        meta.insert("base_freq".to_string(), base_freq.to_string());
        meta.insert("key_prefix".to_string(), self.key_prefix.clone());
        meta.insert("description".to_string(), description.to_string());
        meta.insert("author".to_string(), author.to_string());
        meta.insert("outsample_sdt".to_string(), outsample_sdt_formatted);
        meta.insert("update_time".to_string(), Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());
        meta.insert("kwargs".to_string(), serde_json::to_string(&kwargs).unwrap_or_else(|_| "{}".to_string()));
        
        // 存储元数据
        for (k, v) in meta {
            conn.insert(format!("{}:{}", key, k), v);
        }
        
        // 添加到策略名称集合
        conn.insert(format!("{}:StrategyNames", self.key_prefix), self.strategy_name.clone());
    }
    
    /// 更新策略最近一次更新时间
    pub fn update_last(&self, kwargs: HashMap<String, String>) {
        let key = format!("{}:LAST:{}", self.key_prefix, self.strategy_name);
        let mut conn = self.redis_connection.lock().unwrap();
        
        let mut last = HashMap::new();
        last.insert("name".to_string(), self.strategy_name.clone());
        last.insert("time".to_string(), Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());
        last.insert("kwargs".to_string(), serde_json::to_string(&kwargs).unwrap_or_else(|_| "{}".to_string()));
        
        // 存储最后更新信息
        for (k, v) in last {
            conn.insert(format!("{}:{}", key, k), v);
        }
        
        println!("更新 {} 的 last 时间", key);
    }
    
    /// 获取策略元数据
    pub fn get_metadata(&self) -> HashMap<String, String> {
        let key = format!("{}:META:{}", self.key_prefix, self.strategy_name);
        let conn = self.redis_connection.lock().unwrap();
        
        let mut meta = HashMap::new();
        for (full_key, value) in conn.iter() {
            if full_key.starts_with(&key) {
                let parts: Vec<&str> = full_key.split(':').collect();
                if parts.len() > 3 {
                    let field = parts[3];
                    meta.insert(field.to_string(), value.clone());
                }
            }
        }
        meta
    }
    
    /// 获取策略的最近一次心跳时间
    pub fn get_heartbeat_time(&self) -> Option<DateTime<Utc>> {
        let key = format!("{}:{}:{}", self.key_prefix, self.heartbeat_prefix, self.strategy_name);
        let conn = self.redis_connection.lock().unwrap();
        
        if let Some(time_str) = conn.get(&key) {
            if let Ok(time) = DateTime::parse_from_str(time_str, "%Y-%m-%d %H:%M:%S%.f%z") {
                return Some(time.with_timezone(&Utc));
            } else if let Ok(time) = DateTime::parse_from_str(time_str, "%Y-%m-%d %H:%M:%S") {
                return Some(time.with_timezone(&Utc));
            }
        }
        None
    }
    
    /// 获取所有品种上策略最近一次发布信号的时间
    pub fn get_last_times(&self, symbols: Option<Vec<String>>) -> HashMap<String, DateTime<Utc>> {
        let mut result = HashMap::new();
        
        let symbols_to_check = if let Some(sym_list) = symbols {
            sym_list
        } else {
            self.get_symbols()
        };
        
        let conn = self.redis_connection.lock().unwrap();
        
        for symbol in symbols_to_check {
            let key = format!("{}:{}:{}:LAST", self.key_prefix, self.strategy_name, symbol);
            if let Some(dt_str) = conn.get(&format!("{}:dt", key)) {
                if let Ok(dt) = DateTime::parse_from_str(dt_str, "%Y-%m-%d %H:%M:%S%.f%z") {
                    result.insert(symbol, dt.with_timezone(&Utc));
                } else if let Ok(dt) = DateTime::parse_from_str(dt_str, "%Y-%m-%d %H:%M:%S") {
                    result.insert(symbol, dt.with_timezone(&Utc));
                }
            }
        }
        
        result
    }
    
    /// 获取策略交易的品种列表
    pub fn get_symbols(&self) -> Vec<String> {
        let pattern = format!("{}:{}:*:LAST", self.key_prefix, self.strategy_name);
        let conn = self.redis_connection.lock().unwrap();
        
        let mut symbols = std::collections::HashSet::new();
        
        for key in conn.keys() {
            if key.starts_with(&format!("{}:{}:", self.key_prefix, self.strategy_name)) && 
               key.ends_with(":LAST") {
                let parts: Vec<&str> = key.split(':').collect();
                if parts.len() >= 3 {
                    symbols.insert(parts[2].to_string());  // symbol部分
                }
            }
        }
        
        symbols.into_iter().collect()
    }
    
    /// 发布单个策略持仓权重
    pub fn publish(&self, symbol: &str, dt: DateTime<Utc>, weight: f64, price: Option<f64>, ref_data: Option<HashMap<String, String>>, overwrite: Option<bool>) -> i32 {
        let overwrite = overwrite.unwrap_or(false);
        let price = price.unwrap_or(0.0);
        let ref_data = ref_data.unwrap_or_else(HashMap::new);
        
        // 检查是否允许重复写入
        if !overwrite {
            let last_times = self.get_last_times(Some(vec![symbol.to_string()]));
            if let Some(last_dt) = last_times.get(symbol) {
                if &dt <= last_dt {
                    println!("不允许重复写入，已过滤 {} {} 的重复信号", symbol, dt);
                    return 0;
                }
            }
        }
        
        let update_time = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let time_key = dt.format("%Y%m%d%H%M%S").to_string();
        let key = format!("{}:{}:{}:{}", self.key_prefix, self.strategy_name, symbol, time_key);
        
        let mut conn = self.redis_connection.lock().unwrap();
        
        // 存储权重数据
        conn.insert(format!("{}:symbol", key), symbol.to_string());
        conn.insert(format!("{}:weight", key), weight.to_string());
        conn.insert(format!("{}:dt", key), dt.format("%Y-%m-%d %H:%M:%S").to_string());
        let update_time_clone = update_time.clone();
        conn.insert(format!("{}:update_time", key), update_time_clone);
        conn.insert(format!("{}:price", key), price.to_string());
        conn.insert(format!("{}:ref", key), serde_json::to_string(&ref_data).unwrap_or_else(|_| "{}".to_string()));
        
        // 更新最后的权重数据
        let last_key = format!("{}:{}:{}:LAST", self.key_prefix, self.strategy_name, symbol);
        conn.insert(format!("{}:symbol", last_key), symbol.to_string());
        conn.insert(format!("{}:weight", last_key), weight.to_string());
        conn.insert(format!("{}:dt", last_key), dt.format("%Y-%m-%d %H:%M:%S").to_string());
        conn.insert(format!("{}:update_time", last_key), update_time);
        conn.insert(format!("{}:price", last_key), price.to_string());
        conn.insert(format!("{}:ref", last_key), serde_json::to_string(&ref_data).unwrap_or_else(|_| "{}".to_string()));
        
        // 添加到有序集合（模拟Redis ZADD）
        let zset_key = format!("{}:{}:{}", self.key_prefix, self.strategy_name, symbol);
        conn.insert(format!("{}:score:{}", zset_key, time_key), time_key);
        
        1  // 返回成功发布的数量
    }
    
    /// 获取最近的持仓权重
    pub fn get_last_weights(&self, symbols: Option<Vec<String>>, ignore_zero: Option<bool>, _lua: Option<bool>) -> Vec<HashMap<String, String>> {
        let ignore_zero = ignore_zero.unwrap_or(true);
        let symbols_to_check = if let Some(sym_list) = symbols {
            sym_list
        } else {
            self.get_symbols()
        };
        
        let mut result = Vec::new();
        let conn = self.redis_connection.lock().unwrap();
        
        for symbol in symbols_to_check {
            let key = format!("{}:{}:{}:LAST", self.key_prefix, self.strategy_name, symbol);
            
            let mut weight_data = HashMap::new();
            weight_data.insert("symbol".to_string(), conn.get(&format!("{}:symbol", key)).unwrap_or(&"".to_string()).clone());
            weight_data.insert("weight".to_string(), conn.get(&format!("{}:weight", key)).unwrap_or(&"0".to_string()).clone());
            weight_data.insert("dt".to_string(), conn.get(&format!("{}:dt", key)).unwrap_or(&"".to_string()).clone());
            weight_data.insert("update_time".to_string(), conn.get(&format!("{}:update_time", key)).unwrap_or(&"".to_string()).clone());
            weight_data.insert("price".to_string(), conn.get(&format!("{}:price", key)).unwrap_or(&"0".to_string()).clone());
            weight_data.insert("ref".to_string(), conn.get(&format!("{}:ref", key)).unwrap_or(&"{}".to_string()).clone());
            
            if ignore_zero {
                let weight_val = weight_data.get("weight").unwrap_or(&"0".to_string()).parse::<f64>().unwrap_or(0.0);
                if weight_val == 0.0 {
                    continue;  // 跳过权重为0的品种
                }
            }
            
            result.push(weight_data);
        }
        
        // 按时间和symbol排序
        result.sort_by(|a, b| {
            let empty_str = "".to_string();
            let dt_a = a.get("dt").unwrap_or(&empty_str);
            let dt_b = b.get("dt").unwrap_or(&empty_str);
            let sym_a = a.get("symbol").unwrap_or(&empty_str);
            let sym_b = b.get("symbol").unwrap_or(&empty_str);
            
            dt_a.cmp(dt_b).then_with(|| sym_a.cmp(sym_b))
        });
        
        result
    }
    
    /// 清除所有策略记录
    pub fn clear_all(&self, with_human: Option<bool>) {
        let with_human = with_human.unwrap_or(true);
        
        if with_human {
            // 在真实环境中，这应该是用户确认的，但在这里我们简单模拟
            println!("模拟用户确认删除操作...");
        }
        
        let mut conn = self.redis_connection.lock().unwrap();
        
        // 找到所有匹配该策略的键
        let keys_to_delete: Vec<String> = conn.keys()
            .filter(|key| key.starts_with(&format!("{}:{}", self.key_prefix, self.strategy_name)))
            .cloned()
            .collect();
        
        // 删除所有匹配的键
        for key in keys_to_delete {
            conn.remove(&key);
        }
        
        // 删除元数据和最后记录
        conn.remove(&format!("{}:META:{}", self.key_prefix, self.strategy_name));
        conn.remove(&format!("{}:LAST:{}", self.key_prefix, self.strategy_name));
        conn.remove(&format!("{}:{}:{}", self.key_prefix, self.heartbeat_prefix, self.strategy_name));
        
        // 从策略名称集合中移除
        conn.remove(&format!("{}:StrategyNames", self.key_prefix));
        
        println!("{} 删除了 {} 条记录", self.strategy_name, conn.len());
    }
}

/// 删除策略所有记录
pub fn clear_strategy(strategy_name: &str, redis_url: Option<String>, connection_pool: Option<String>, key_prefix: Option<&str>, kwargs: HashMap<String, String>) {
    let key_prefix = key_prefix.unwrap_or("Weights");
    let with_human = kwargs.get("with_human").map(|s| s == "true").unwrap_or(true);
    
    let rwc = RedisWeightsClient::new(
        strategy_name.to_string(),
        redis_url,
        connection_pool,
        Some(false),  // 不发送心跳
        [(String::from("key_prefix"), String::from(key_prefix))].iter().cloned().collect(),
    );
    
    rwc.clear_all(Some(with_human));
}

/// 获取策略的持仓权重
pub fn get_strategy_weights(
    strategy_name: &str, 
    redis_url: Option<String>, 
    connection_pool: Option<String>, 
    key_prefix: Option<&str>, 
    kwargs: HashMap<String, String>
) -> Vec<HashMap<String, String>> {
    let key_prefix = key_prefix.unwrap_or("Weights");
    let symbols: Option<Vec<String>> = kwargs.get("symbols")
        .and_then(|s| serde_json::from_str(s).ok());
    let sdt = kwargs.get("sdt").map(|s| s.as_str());
    let edt = kwargs.get("edt").map(|s| s.as_str());
    let only_last = kwargs.get("only_last").map(|s| s == "true").unwrap_or(false);
    
    let rwc = RedisWeightsClient::new(
        strategy_name.to_string(),
        redis_url,
        connection_pool,
        Some(false),  // 不发送心跳
        [(String::from("key_prefix"), String::from(key_prefix))].iter().cloned().collect(),
    );
    
    if only_last {
        // 保留每个品种最近一次权重
        return rwc.get_last_weights(symbols, Some(false), None);
    }
    
    // 获取所有权重
    let mut all_weights = Vec::new();
    if let Some(syms) = symbols {
        for symbol in syms {
            // 模拟获取特定品种的历史权重
            // 这里只是一个简化的实现
        }
    } else {
        // 获取所有品种的权重
        all_weights = rwc.get_last_weights(None, Some(false), None);
    }
    
    // 应用时间过滤
    if sdt.is_some() || edt.is_some() {
        // 时间过滤逻辑
        all_weights.retain(|weight| {
            let empty_str = "".to_string();
        let dt_str = weight.get("dt").unwrap_or(&empty_str);
            if let Ok(dt) = DateTime::parse_from_str(dt_str, "%Y-%m-%d %H:%M:%S") {
                let utc_dt = dt.with_timezone(&Utc);
                let sdt_ok = sdt.map(|s| {
                    if let Ok(sdt_dt) = DateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
                        utc_dt >= sdt_dt.with_timezone(&Utc)
                    } else {
                        true
                    }
                }).unwrap_or(true);
                
                let edt_ok = edt.map(|e| {
                    if let Ok(edt_dt) = DateTime::parse_from_str(e, "%Y-%m-%d %H:%M:%S") {
                        utc_dt <= edt_dt.with_timezone(&Utc)
                    } else {
                        true
                    }
                }).unwrap_or(true);
                
                sdt_ok && edt_ok
            } else {
                true
            }
        });
    }
    
    all_weights
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_redis_weights_client_creation() {
        let kwargs = HashMap::new();
        let client = RedisWeightsClient::new(
            "test_strategy".to_string(),
            None,
            None,
            Some(false),
            kwargs,
        );
        
        assert_eq!(client.strategy_name, "test_strategy");
        assert_eq!(client.key_prefix, "Weights");
    }
    
    #[test]
    fn test_set_metadata() {
        let kwargs = HashMap::new();
        let client = RedisWeightsClient::new(
            "test_strategy".to_string(),
            None,
            None,
            Some(false),
            kwargs,
        );
        
        client.set_metadata("daily", "test description", "test author", "20230101", HashMap::new());
        
        let metadata = client.get_metadata();
        assert_eq!(metadata.get("author").unwrap(), "test author");
    }
    
    #[test]
    fn test_publish_weight() {
        let kwargs = HashMap::new();
        let client = RedisWeightsClient::new(
            "test_strategy".to_string(),
            None,
            None,
            Some(false),
            kwargs,
        );
        
        let dt = Utc::now();
        let result = client.publish("TEST", dt, 1.0, Some(100.0), None, None);
        
        assert_eq!(result, 1);
    }
}