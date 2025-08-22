use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use serde_json::Value;

// 알림 관련 엔티티들
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: i64,
    pub user_id: String,
    pub title: String,
    pub content: String,
    pub notification_type: String,
    pub is_read: bool,
    pub data: Option<Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSettings {
    pub user_id: String,
    pub push_enabled: bool,
    pub email_enabled: bool,
    pub sms_enabled: bool,
    pub discount_alerts: bool,
    pub price_drop_alerts: bool,
    pub new_product_alerts: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationLog {
    pub id: i64,
    pub user_id: String,
    pub notification_type: String,
    pub title: String,
    pub content: String,
    pub status: String, // sent, failed, pending
    pub error_message: Option<String>,
    pub sent_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}