use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shop {
    pub id: i64,  // 실제 DB 스키마에 맞춤
    pub name: String,
    pub domain: String,
    pub platform: String,
    pub logo_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Brand {
    pub id: i64,  // BIGSERIAL
    pub name: String,
    pub image_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: i64,  // 실제 DB에서는 BIGSERIAL
    pub name: String,
    pub parent_id: Option<i64>,  // 실제 DB에서는 i64
    pub icon: Option<String>,    // 실제 DB 필드
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}