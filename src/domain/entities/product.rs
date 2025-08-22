use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: i64,  // 실제 DB 스키마에 맞춤
    pub shop_id: i64,
    pub brand_id: Option<i64>,
    pub category_id: Option<i64>,
    pub name: String,
    pub sku: Option<String>,
    pub click_count: Option<i32>, // 실제 DB에 존재
    pub is_deleted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}