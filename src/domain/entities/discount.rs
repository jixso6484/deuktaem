use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscountInfo {
    pub id: i64,  // 실제 DB 스키마에 맞춤
    pub product_id: i64,
    pub original_price: f64,
    pub discount_price: f64,
    pub discount_rate: f64,
    pub start_at: DateTime<Utc>,
    pub end_at: DateTime<Utc>,
    pub info_url: Option<String>, // 실제 DB 필드명
    pub thumbnail_url: Option<String>,
    pub click_count: Option<i32>, // 실제 DB에 존재
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscountEvent {
    pub id: i64,  // BIGSERIAL
    pub title: String,
    pub description: Option<String>,
    pub event_type: String,
    pub discount_rate: Option<f64>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub banner_image_url: Option<String>,
    pub is_featured: bool,
    pub status: String,
    pub shop_id: Option<String>,
    pub brand_id: Option<String>,
    pub category_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}