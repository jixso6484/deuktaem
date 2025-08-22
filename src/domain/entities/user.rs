use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub user_id: String,
    pub avatar_url: Option<String>,
    pub email: String,
    pub preferred_country: Option<String>,
    pub detected_country: Option<String>,
    pub language: Option<String>,
    pub timezone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopSubscription {
    pub id: String,
    pub user_id: String,
    pub shop_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandSubscription {
    pub id: String,
    pub user_id: String,
    pub brand_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategorySubscription {
    pub id: String,
    pub user_id: String,
    pub category_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: String,
    pub user_id: String,
    pub actor_id: Option<String>,
    pub r#type: String,
    pub target_type: Option<String>,
    pub target_id: Option<String>,
    pub read_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationLog {
    pub id: String,
    pub user_id: String,
    pub subscription_type: String,
    pub target_id: String,
    pub message: String,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSettings {
    pub user_id: String,
    pub push_enabled: bool,
    pub discount_notifications: bool,
    pub shop_notifications: bool,
    pub brand_notifications: bool,
    pub category_notifications: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}