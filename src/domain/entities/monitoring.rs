use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use serde_json::Value;

// 모니터링 및 로깅 엔티티들
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMetric {
    pub id: i64,
    pub endpoint: String,
    pub method: String,
    pub status_code: i32,
    pub response_time_ms: i32,
    pub user_id: Option<String>,
    pub ip_address: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemLog {
    pub id: i64,
    pub level: String,    // info, warn, error, debug
    pub message: String,
    pub context: Option<Value>,
    pub user_id: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub request_id: Option<String>,
    pub module: Option<String>,
    pub function: Option<String>,
    pub line_number: Option<i32>,
    pub stack_trace: Option<String>,
    pub tags: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheLog {
    pub id: i64,
    pub cache_key: String,
    pub operation: String,  // get, set, delete, expire
    pub hit: bool,
    pub ttl_seconds: Option<i32>,
    pub size_bytes: Option<i32>,
    pub execution_time_ms: i32,
    pub metadata: Option<Value>,
    pub created_at: DateTime<Utc>,
}

// 성능 분석을 위한 집계 구조체들
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiPerformanceSummary {
    pub endpoint: String,
    pub method: String,
    pub total_requests: i64,
    pub avg_response_time_ms: f64,
    pub error_rate: f64,
    pub success_rate: f64,
    pub p95_response_time_ms: f64,
    pub p99_response_time_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheHitRateSummary {
    pub cache_key_pattern: String,
    pub total_operations: i64,
    pub hit_count: i64,
    pub miss_count: i64,
    pub hit_rate: f64,
    pub avg_response_time_ms: f64,
}