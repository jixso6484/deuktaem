use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// 공통 응답 구조
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub success: bool,
    pub data: Vec<T>,
    pub pagination: PaginationInfo,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationInfo {
    pub page: u32,
    pub limit: u32,
    pub total: u64,
    pub total_pages: u32,
    pub has_next: bool,
    pub has_prev: bool,
}

// Shop 관련 응답 DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct ShopResponse {
    pub id: String,
    pub name: String,
    pub domain: String,
    pub platform: String,
    pub logo_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub translations: Option<Vec<ShopTranslationResponse>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShopTranslationResponse {
    pub locale: String,
    pub name: String,
    pub description: Option<String>,
}

// Product 관련 응답 DTO  
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductResponse {
    pub id: String,
    pub shop_id: String,
    pub brand_id: Option<String>,
    pub category_id: Option<String>, 
    pub name: String,
    pub sku: Option<String>,
    pub is_deleted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub translations: Option<Vec<ProductTranslationResponse>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductDetailResponse {
    pub id: String,
    pub shop: Option<ShopResponse>,
    pub brand: Option<BrandResponse>,
    pub category: Option<CategoryResponse>,
    pub name: String,
    pub sku: Option<String>,
    pub is_deleted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub translations: Option<Vec<ProductTranslationResponse>>,
    pub current_discount: Option<DiscountInfoResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductTranslationResponse {
    pub locale: String,
    pub name: String,
    pub description: Option<String>,
}

// Brand 관련 응답 DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct BrandResponse {
    pub id: String,
    pub name: String,
    pub image_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub translations: Option<Vec<BrandTranslationResponse>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BrandTranslationResponse {
    pub locale: String,
    pub name: String,
    pub description: Option<String>,
}

// Category 관련 응답 DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryResponse {
    pub id: String,
    pub name: String,
    pub parent_id: Option<String>,
    pub level: i32,
    pub path: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub translations: Option<Vec<CategoryTranslationResponse>>,
    pub children: Option<Vec<Box<CategoryResponse>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryTranslationResponse {
    pub locale: String,
    pub name: String,
    pub description: Option<String>,
}

// Discount 관련 응답 DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct DiscountInfoResponse {
    pub id: String,
    pub product_id: String,
    pub shop_id: String,
    pub brand_id: Option<String>,
    pub original_price: f64,
    pub discount_price: f64,
    pub discount_rate: f64,
    pub currency: String,
    pub start_at: DateTime<Utc>,
    pub end_at: DateTime<Utc>,
    pub is_active: bool,
    pub source_url: Option<String>,
    pub is_auto_discovered: bool,
    pub is_event_based: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub translations: Option<Vec<DiscountInfoTranslationResponse>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscountInfoDetailResponse {
    pub id: String,
    pub product: Option<ProductResponse>,
    pub shop: Option<ShopResponse>,
    pub brand: Option<BrandResponse>,
    pub original_price: f64,
    pub discount_price: f64,
    pub discount_rate: f64,
    pub currency: String,
    pub start_at: DateTime<Utc>,
    pub end_at: DateTime<Utc>,
    pub is_active: bool,
    pub source_url: Option<String>,
    pub is_auto_discovered: bool,
    pub is_event_based: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub translations: Option<Vec<DiscountInfoTranslationResponse>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscountInfoTranslationResponse {
    pub locale: String,
    pub description: Option<String>,
    pub terms_conditions: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscountEventResponse {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub event_type: String,
    pub discount_rate: Option<f64>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub banner_image_url: Option<String>,
    pub is_featured: bool,
    pub status: String,
    pub shop: Option<ShopResponse>,
    pub brand: Option<BrandResponse>,
    pub category: Option<CategoryResponse>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub translations: Option<Vec<DiscountEventTranslationResponse>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscountEventTranslationResponse {
    pub locale: String,
    pub title: String,
    pub description: Option<String>,
}

// User 관련 응답 DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileResponse {
    pub user_id: String,
    pub avatar_url: Option<String>,
    pub email: String,
    pub preferred_country: Option<String>,
    pub detected_country: Option<String>,
    pub language: Option<String>,
    pub timezone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub subscriptions: Option<UserSubscriptionsResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSubscriptionsResponse {
    pub shops: Vec<ShopResponse>,
    pub brands: Vec<BrandResponse>,
    pub categories: Vec<CategoryResponse>,
}

// Notification 관련 응답 DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationResponse {
    pub id: String,
    pub user_id: String,
    pub actor_id: Option<String>,
    pub r#type: String,
    pub target_type: Option<String>,
    pub target_id: Option<String>,
    pub read_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub translations: Option<Vec<NotificationTranslationResponse>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationTranslationResponse {
    pub locale: String,
    pub title: Option<String>,
    pub message: String,
}

// 통계 관련 응답 DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardStatsResponse {
    pub total_shops: u64,
    pub total_products: u64,
    pub total_discounts: u64,
    pub active_discounts: u64,
    pub total_users: u64,
    pub recent_activity: Vec<RecentActivityResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecentActivityResponse {
    pub activity_type: String,
    pub description: String,
    pub timestamp: DateTime<Utc>,
    pub metadata: Option<serde_json::Value>,
}

// 건강 체크 응답
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
    pub version: String,
    pub database: String,
}