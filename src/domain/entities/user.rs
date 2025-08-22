use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// 사용자 프로필 (profiles 테이블)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub user_id: String,  // UUID from auth.users
    pub avatar_url: Option<String>,
    pub email: String,
    pub preferred_country: Option<String>,
    pub detected_country: Option<String>,
    pub language: Option<String>,
    pub timezone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// 구독 관련 엔티티들
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductSubscription {
    pub user_id: String,
    pub product_id: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopSubscription {
    pub user_id: String,
    pub shop_id: i64,
    pub notification_enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandSubscription {
    pub user_id: String,
    pub brand_id: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategorySubscription {
    pub user_id: String,
    pub category_id: i64,
    pub min_discount_rate: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}