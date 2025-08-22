use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// 쿠폰 시스템 엔티티들
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coupon {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub discount_type: String,  // percentage, fixed_amount
    pub discount_value: f64,
    pub min_order_amount: Option<f64>,
    pub max_discount_amount: Option<f64>,
    pub usage_limit: Option<i32>,
    pub used_count: i32,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub is_active: bool,
    pub shop_id: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouponUsage {
    pub id: i64,
    pub coupon_id: i64,
    pub user_id: String,
    pub order_amount: f64,
    pub discount_amount: f64,
    pub used_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouponValidationResult {
    pub is_valid: bool,
    pub discount_amount: f64,
    pub error_message: Option<String>,
}