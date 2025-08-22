use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// 기본 Translation 구조체 추가
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Translation {
    pub id: String,
    pub entity_type: String, // shop, product, category, etc.
    pub entity_id: String,
    pub locale: String,
    pub field_name: String,
    pub field_value: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Language {
    pub code: String,
    pub name: String,
    pub native_name: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopTranslation {
    pub id: String,
    pub shop_id: String,
    pub locale: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandTranslation {
    pub id: String,
    pub brand_id: String,
    pub locale: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryTranslation {
    pub id: String,
    pub category_id: String,
    pub locale: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductTranslation {
    pub id: String,
    pub product_id: String,
    pub locale: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscountEventTranslation {
    pub id: String,
    pub event_id: String,
    pub locale: String,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscountInfoTranslation {
    pub id: String,
    pub discount_info_id: String,
    pub locale: String,
    pub description: Option<String>,
    pub terms_conditions: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationTranslation {
    pub id: String,
    pub notification_id: String,
    pub locale: String,
    pub title: Option<String>,
    pub message: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMetric {
    pub id: String,
    pub endpoint: String,
    pub method: String,
    pub status_code: i32,
    pub response_time: f64,
    pub error_message: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}