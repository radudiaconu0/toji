use serde_json::Value;
use crate::app::App;
#[derive(Debug, Clone)]
pub struct Redis {
    pub host: String,
    pub port: u16,
    pub db: i64,
    pub username: Option<String>,
    pub key_prefix: String,
    pub name: String,
    pub sentinels: Option<Vec<RedisSentinel>>,
    pub cluster: Option<Vec<ClusterNode>>,
}
#[derive(Debug, Clone)]
pub struct ClusterNode {
    pub host: String,
    pub port: u16,
}
#[derive(Debug, Clone)]
pub struct RedisSentinel {
    host: String,
    port: u16,
}
#[derive(Debug, Clone)]
pub struct Adapter {
    pub driver: String,
    pub(crate) redis: RedisAdapter,
    pub(crate) cluster: ClusterAdapter,
    pub(crate) nats: NatsAdapter,
}
#[derive(Debug, Clone)]
pub struct RedisAdapter {
    pub request_timeout: i64,
    pub(crate) prefix: String,
    pub(crate) redis_pub_options: Value,
    pub(crate) redis_sub_options: Value,
    pub(crate) cluster_mode: bool,
}
#[derive(Debug, Clone, Copy)]
pub struct ClusterAdapter {
    pub request_timeout: i64,
}
#[derive(Debug, Clone)]
pub struct NatsAdapter {
    pub request_timeout: i64,
    pub(crate) prefix: String,
    pub(crate) servers: Vec<String>,
    pub(crate) user: Option<String>,
    pub(crate) password: Option<String>,
    pub(crate) token: Option<String>,
    pub(crate) timeout: i64,
    pub(crate) nodes_number: Option<i64>,
}
#[derive(Debug)]
pub struct AppManager {
    pub(crate) driver: String,
    pub(crate) array: ArrayAppManager,
    pub(crate) cache: CacheAppManager,
    pub(crate) mysql: MySQLAppManager,
}
#[derive(Debug)]
pub struct ArrayAppManager {
    pub(crate) apps: Vec<App>,
}
#[derive(Debug, Clone, Copy)]
pub struct CacheAppManager {
    pub(crate) enabled: bool,
    pub(crate) ttl: i64,
}
#[derive(Debug, Clone)]
pub struct MySQLAppManager {
    pub(crate) table: String,
    pub(crate) version: String,
}
#[derive(Debug, Clone)]
pub struct Prometheus {
    pub(crate) prefix: String,
}

#[derive(Debug, Clone)]
pub struct Metrics {
    pub(crate) enabled: bool,
    pub(crate) driver: String,
    pub(crate) host: String,
    pub(crate) prometheus: Prometheus,
    pub(crate) port: u16,
}

#[derive(Debug)]
pub struct Options {
    pub(crate) adapter: Adapter,
    pub(crate) app_manager: AppManager,
    pub(crate) debug: bool,
    pub(crate) port: u16,
    pub(crate) metrics: Metrics,
}

impl Options {
    pub fn new() -> Self {
        Options {
            adapter: Adapter {
                driver: "".to_string(),
                redis: RedisAdapter {
                    request_timeout: 0,
                    prefix: "".to_string(),
                    redis_pub_options: Default::default(),
                    redis_sub_options: Default::default(),
                    cluster_mode: false,
                },
                cluster: ClusterAdapter { request_timeout: 0 },
                nats: NatsAdapter {
                    request_timeout: 0,
                    prefix: "".to_string(),
                    servers: vec![],
                    user: None,
                    password: None,
                    token: None,
                    timeout: 0,
                    nodes_number: None,
                },
            },
            app_manager: AppManager {
                driver: "".to_string(),
                array: ArrayAppManager { apps: vec![] },
                cache: CacheAppManager {
                    enabled: false,
                    ttl: 0,
                },
                mysql: MySQLAppManager {
                    table: "".to_string(),
                    version: "".to_string(),
                },
            },
            debug: true,
            port: 6001,
            metrics: Metrics {
                enabled: false,
                driver: String::from("prometheus"),
                host: String::from("127.0.0.1"),
                prometheus: Prometheus {
                    prefix: "echoxide_".to_string(),
                },
                port: 9601,
            },
        }
    }
}