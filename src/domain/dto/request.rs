use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// Shop 관련 요청 DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateShopRequest {
    pub name: String,
    pub domain: String,
    pub platform: String,
    pub logo_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateShopRequest {
    pub name: Option<String>,
    pub domain: Option<String>,
    pub platform: Option<String>,
    pub logo_url: Option<String>,
}

// Product 관련 요청 DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProductRequest {
    pub shop_id: String,
    pub brand_id: Option<String>,
    pub category_id: Option<String>,
    pub name: String,
    pub sku: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProductRequest {
    pub brand_id: Option<String>,
    pub category_id: Option<String>,
    pub name: Option<String>,
    pub sku: Option<String>,
    pub is_deleted: Option<bool>,
}

// Discount 관련 요청 DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDiscountRequest {
    pub product_id: String,
    pub shop_id: String,
    pub brand_id: Option<String>,
    pub original_price: f64,
    pub discount_price: f64,
    pub discount_rate: f64,
    pub currency: String,
    pub start_at: DateTime<Utc>,
    pub end_at: DateTime<Utc>,
    pub source_url: Option<String>,
    pub is_auto_discovered: Option<bool>,
    pub is_event_based: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDiscountRequest {
    pub original_price: Option<f64>,
    pub discount_price: Option<f64>,
    pub discount_rate: Option<f64>,
    pub currency: Option<String>,
    pub start_at: Option<DateTime<Utc>>,
    pub end_at: Option<DateTime<Utc>>,
    pub is_active: Option<bool>,
    pub source_url: Option<String>,
}

// User 관련 요청 DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProfileRequest {
    pub user_id: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub preferred_country: Option<String>,
    pub language: Option<String>,
    pub timezone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProfileRequest {
    pub avatar_url: Option<String>,
    pub preferred_country: Option<String>,
    pub language: Option<String>,
    pub timezone: Option<String>,
}

// 구독 관련 요청 DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSubscriptionRequest {
    pub user_id: String,
    pub target_id: String, // shop_id, brand_id, category_id
}

// 쿼리 파라미터 DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct ListQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub sort_by: Option<String>,
    pub order: Option<String>, // asc, desc
    pub search: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscountQuery {
    pub shop_id: Option<String>,
    pub brand_id: Option<String>,
    pub category_id: Option<String>,
    pub min_discount_rate: Option<f64>,
    pub max_discount_rate: Option<f64>,
    pub currency: Option<String>,
    pub is_active: Option<bool>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

// 다국어 지원 DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageHeader {
    pub accept_language: String, // "ko", "en", "ja" 등
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TranslationRequest {
    pub locale: String,
    pub target_id: String,
    pub content: serde_json::Value, // 번역할 내용
}