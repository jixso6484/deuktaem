
mod domain;
mod config;
mod repository;
mod service;
mod api;
mod utils;
mod error;
mod auth;

use axum::{
    extract::{Path, Query, State},
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use serde_json::json;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};

use crate::config::SupabaseConfig;
use crate::service::{DiscountService, ShopService, ProductService, UserService, NotificationService, MonitoringService};
use crate::domain::dto::{HealthResponse, pagenation::Pagenation};
use crate::utils::init_logger;
use crate::error::{AppError, AppResult};
use serde::Deserialize;

// 상품 목록 조회를 위한 쿼리 파라미터
#[derive(Debug, Deserialize)]
pub struct ProductQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub country: Option<String>,
}

// 애플리케이션 상태 - Phase 1-4: 완전한 서비스 레이어
#[derive(Clone)]
pub struct AppState {
    pub discount_service: DiscountService,
    pub shop_service: ShopService,
    pub product_service: ProductService,
    pub user_service: UserService,
    pub notification_service: NotificationService,
    pub monitoring_service: MonitoringService,
}

#[tokio::main]
async fn main() {
    // 환경 변수 로드
    dotenv::dotenv().ok();
    
    // 로깅 초기화
    init_logger();
    tracing::info!("🚀 Starting Duk server...");
    
    // Supabase 설정
    let config = SupabaseConfig::new().expect("Failed to load Supabase config");
    tracing::info!("⚙️ Configuration loaded");
    
    // 서비스 초기화 - Phase 1-4: 완전한 서비스 레이어
    let app_state = AppState {
        discount_service: DiscountService::new(config.clone()),
        shop_service: ShopService::new(config.clone()),
        product_service: ProductService::new(config.clone()),
        user_service: UserService::new(config.clone()),
        notification_service: NotificationService::new(config.clone()),
        monitoring_service: MonitoringService::new(config),
    };
    
    tracing::info!("🔧 Services initialized");
    
    // 라우터 구성
    let app = create_router(Arc::new(app_state));
    
    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    
    tracing::info!("🎯 Server listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        // Health check
        .route("/", get(health_check))
        .route("/health", get(health_check))
        
        // 📦 Phase 1: 상품 관리 API (기본)
        .route("/api/v1/products", get(get_products))           // 상품 목록 (나라별/전체)
        .route("/api/v1/products/popular", get(get_popular_products)) // 인기 상품 목록
        .route("/api/v1/products/:id", get(get_product_by_id))   // 상품 상세
        .route("/api/v1/products/:id/click", post(record_product_click)) // 클릭 기록
        .route("/api/v1/products/search", get(search_products)) // 상품 검색
        
        // 💰 Phase 1: 할인 정보 API (기본)  
        .route("/api/v1/discounts/:id", get(get_discount_by_id))
        
        // 💰 Phase 3: 쿠폰 시스템 API
        .route("/api/v1/coupons", get(get_coupons))
        .route("/api/v1/coupons/:id", get(get_coupon_by_id))
        .route("/api/v1/coupons/:id/use", post(use_coupon))
        
        // 🏪 Phase 1: 매장 정보 API (기본)
        .route("/api/v1/shops/:id", get(get_shop_by_id))
        
        // 🏪 Phase 3: 매장/브랜드/카테고리 목록 API
        .route("/api/v1/shops", get(get_shops))
        .route("/api/v1/brands", get(get_brands))
        .route("/api/v1/brands/:id", get(get_brand_by_id))
        .route("/api/v1/categories", get(get_categories))
        .route("/api/v1/categories/:id", get(get_category_by_id))
        
        // 👥 Phase 2: 사용자 프로필 API
        .route("/api/v1/profiles/:user_id", get(get_user_profile))
        .route("/api/v1/profiles/:user_id", post(update_user_profile))
        
        // 📋 Phase 2: 구독 관리 API  
        .route("/api/v1/subscriptions/my/:user_id", get(get_my_subscriptions))
        .route("/api/v1/subscriptions/products/:user_id/:product_id", post(add_product_subscription))
        .route("/api/v1/subscriptions/products/:user_id/:product_id", delete(remove_product_subscription))
        .route("/api/v1/subscriptions/brands/:user_id/:brand_id", post(add_brand_subscription))
        .route("/api/v1/subscriptions/brands/:user_id/:brand_id", delete(remove_brand_subscription))
        .route("/api/v1/subscriptions/shops/:user_id/:shop_id", post(add_shop_subscription))
        .route("/api/v1/subscriptions/shops/:user_id/:shop_id", delete(remove_shop_subscription))
        
        // 🔔 Phase 3: 알림 시스템 API
        .route("/api/v1/notifications/:user_id", get(get_notifications))
        .route("/api/v1/notifications/:id/read", post(mark_notification_read))
        .route("/api/v1/notifications/settings/:user_id", get(get_notification_settings))
        .route("/api/v1/notifications/settings/:user_id", put(update_notification_settings))
        
        // 📈 Phase 4: 모니터링 API (관리자)
        .route("/api/v1/admin/metrics/api", get(get_api_metrics))
        .route("/api/v1/admin/logs/errors", get(get_error_logs))
        .route("/api/v1/admin/cache/stats", get(get_cache_stats))
        .route("/api/v1/admin/system/health", get(get_system_health))
        
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
        )
        .with_state(state)
}

// Health Check
async fn health_check() -> Json<HealthResponse> {
    tracing::info!("🏥 Health check requested");
    
    Json(HealthResponse {
        status: "healthy".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        database: "supabase".to_string(),
    })
}

// 📦 Phase 1: 상품 핸들러들

// 상품 목록 조회 (나라별 필터링 또는 전체)
async fn get_products(
    Query(query): Query<ProductQuery>,
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<serde_json::Value>> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20);
    
    if page == 0 || limit == 0 || limit > 100 {
        return Err(AppError::validation("Invalid page or limit parameters"));
    }

    let pagination = Pagenation { page, limit };

    let result = if let Some(country) = query.country {
        log::info!("🌍 Getting products for country: {}", country);
        state.product_service
            .get_products_by_country(&country, pagination)
            .await
            .map_err(|e| AppError::internal(format!("Failed to get products by country: {}", e)))?
    } else {
        log::info!("📦 Getting all products");
        state.product_service
            .get_all_products(pagination)
            .await
            .map_err(|e| AppError::internal(format!("Failed to get products: {}", e)))?
    };

    Ok(Json(json!({ 
        "products": result.data,
        "pagination": {
            "page": result.page,
            "limit": result.limit,
            "total": result.total,
            "total_pages": result.total_pages,
            "has_next": result.has_next,
            "has_prev": result.has_prev
        }
    })))
}

// 인기 상품 목록 조회 (클릭 수 기준)
async fn get_popular_products(
    Query(query): Query<ProductQuery>,
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<serde_json::Value>> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20);
    
    if page == 0 || limit == 0 || limit > 100 {
        return Err(AppError::validation("Invalid page or limit parameters"));
    }

    let pagination = Pagenation { page, limit };

    log::info!("🔥 Getting popular products");
    let result = state.product_service
        .get_popular_products(pagination)
        .await
        .map_err(|e| AppError::internal(format!("Failed to get popular products: {}", e)))?;

    Ok(Json(json!({ 
        "products": result.data,
        "pagination": {
            "page": result.page,
            "limit": result.limit,
            "total": result.total,
            "total_pages": result.total_pages,
            "has_next": result.has_next,
            "has_prev": result.has_prev
        }
    })))
}

async fn get_product_by_id(
    Path(product_id): Path<i64>,
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<serde_json::Value>> {
    log::info!("📦 Getting product by ID: {}", product_id);
    
    let product = state.product_service
        .get_product_by_id(product_id)
        .await
        .map_err(|e| AppError::internal(format!("Failed to get product: {}", e)))?;
    
    match product {
        Some(product) => Ok(Json(json!({ "product": product }))),
        None => Err(AppError::not_found("Product")),
    }
}

async fn record_product_click(
    Path(product_id): Path<i64>,
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<serde_json::Value>> {
    log::info!("🖱️ Recording product click: {}", product_id);
    
    state.product_service
        .record_product_click(product_id)
        .await
        .map_err(|e| AppError::internal(format!("Failed to record click: {}", e)))?;
    
    Ok(Json(json!({ 
        "success": true,
        "message": "Product click recorded",
        "product_id": product_id 
    })))
}

// 💰 Phase 1: 할인 핸들러들
async fn get_discount_by_id(
    Path(discount_id): Path<i64>,
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<serde_json::Value>> {
    log::info!("💰 Getting discount by ID: {}", discount_id);
    
    let discount = state.discount_service
        .get_discount_by_id(discount_id)
        .await
        .map_err(|e| AppError::internal(format!("Failed to get discount: {}", e)))?;
    
    match discount {
        Some(discount) => Ok(Json(json!({ "discount": discount }))),
        None => Err(AppError::not_found("Discount")),
    }
}

// 🏪 Phase 1: 매장 핸들러들
async fn get_shop_by_id(
    Path(shop_id): Path<i64>,
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<serde_json::Value>> {
    log::info!("🏪 Getting shop by ID: {}", shop_id);
    
    let shop = state.shop_service
        .get_shop_by_id(shop_id)
        .await
        .map_err(|e| AppError::internal(format!("Failed to get shop: {}", e)))?;
    
    match shop {
        Some(shop) => Ok(Json(json!({ "shop": shop }))),
        None => Err(AppError::not_found("Shop")),
    }
}

// 👥 Phase 2: 사용자 프로필 핸들러들
async fn get_user_profile(
    Path(user_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<serde_json::Value>> {
    log::info!("👤 Getting user profile: {}", user_id);
    
    let profile = state.user_service
        .get_profile(&user_id)
        .await
        .map_err(|e| AppError::internal(format!("Failed to get user profile: {}", e)))?;
    
    match profile {
        Some(profile) => Ok(Json(json!({ "profile": profile }))),
        None => Err(AppError::not_found("User profile")),
    }
}

async fn update_user_profile(
    Path(user_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<serde_json::Value>,
) -> AppResult<Json<serde_json::Value>> {
    log::info!("👤 Updating user profile: {}", user_id);
    
    // Parse payload as Profile - using actual Profile fields
    let profile = crate::domain::entities::user::Profile {
        user_id: user_id.clone(),
        avatar_url: payload.get("avatar_url").and_then(|v| v.as_str()).map(|s| s.to_string()),
        email: payload.get("email").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        preferred_country: payload.get("preferred_country").and_then(|v| v.as_str()).map(|s| s.to_string()),
        detected_country: payload.get("detected_country").and_then(|v| v.as_str()).map(|s| s.to_string()),
        language: payload.get("language").and_then(|v| v.as_str()).map(|s| s.to_string()),
        timezone: payload.get("timezone").and_then(|v| v.as_str()).map(|s| s.to_string()),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    state.user_service
        .update_profile(profile)
        .await
        .map_err(|e| AppError::internal(format!("Failed to update user profile: {}", e)))?;
    
    Ok(Json(json!({ 
        "success": true,
        "message": "User profile updated successfully",
        "user_id": user_id 
    })))
}

// 📋 Phase 2: 구독 관리 핸들러들
async fn get_my_subscriptions(
    Path(user_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<serde_json::Value>> {
    log::info!("📋 Getting subscriptions for user: {}", user_id);
    
    let subscriptions = state.user_service
        .get_all_subscriptions(&user_id)
        .await
        .map_err(|e| AppError::internal(format!("Failed to get subscriptions: {}", e)))?;
    
    Ok(Json(json!({ "subscriptions": subscriptions })))
}

async fn add_product_subscription(
    Path((user_id, product_id)): Path<(String, i64)>,
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<serde_json::Value>> {
    log::info!("📦 Adding product subscription: user={}, product={}", user_id, product_id);
    
    let subscription = state.user_service
        .add_product_subscription(&user_id, product_id)
        .await
        .map_err(|e| AppError::internal(format!("Failed to add product subscription: {}", e)))?;
    
    Ok(Json(json!({ 
        "success": true,
        "message": "Product subscription added",
        "subscription": subscription
    })))
}

async fn remove_product_subscription(
    Path((user_id, product_id)): Path<(String, i64)>,
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<serde_json::Value>> {
    log::info!("📦 Removing product subscription: user={}, product={}", user_id, product_id);
    
    state.user_service
        .remove_product_subscription(&user_id, product_id)
        .await
        .map_err(|e| AppError::internal(format!("Failed to remove product subscription: {}", e)))?;
    
    Ok(Json(json!({ 
        "success": true,
        "message": "Product subscription removed",
        "user_id": user_id,
        "product_id": product_id
    })))
}

async fn add_brand_subscription(
    Path((user_id, brand_id)): Path<(String, i64)>,
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<serde_json::Value>> {
    log::info!("🏷️ Adding brand subscription: user={}, brand={}", user_id, brand_id);
    
    let subscription = state.user_service
        .add_brand_subscription(&user_id, brand_id)
        .await
        .map_err(|e| AppError::internal(format!("Failed to add brand subscription: {}", e)))?;
    
    Ok(Json(json!({ 
        "success": true,
        "message": "Brand subscription added",
        "subscription": subscription
    })))
}

async fn remove_brand_subscription(
    Path((user_id, brand_id)): Path<(String, i64)>,
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<serde_json::Value>> {
    log::info!("🏷️ Removing brand subscription: user={}, brand={}", user_id, brand_id);
    
    state.user_service
        .remove_brand_subscription(&user_id, brand_id)
        .await
        .map_err(|e| AppError::internal(format!("Failed to remove brand subscription: {}", e)))?;
    
    Ok(Json(json!({ 
        "success": true,
        "message": "Brand subscription removed",
        "user_id": user_id,
        "brand_id": brand_id
    })))
}

async fn add_shop_subscription(
    Path((user_id, shop_id)): Path<(String, i64)>,
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<serde_json::Value>> {
    log::info!("🏪 Adding shop subscription: user={}, shop={}", user_id, shop_id);
    
    let subscription = state.user_service
        .add_shop_subscription(&user_id, shop_id, true) // Default to notifications enabled
        .await
        .map_err(|e| AppError::internal(format!("Failed to add shop subscription: {}", e)))?;
    
    Ok(Json(json!({ 
        "success": true,
        "message": "Shop subscription added",
        "subscription": subscription
    })))
}

async fn remove_shop_subscription(
    Path((user_id, shop_id)): Path<(String, i64)>,
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<serde_json::Value>> {
    log::info!("🏪 Removing shop subscription: user={}, shop={}", user_id, shop_id);
    
    state.user_service
        .remove_shop_subscription(&user_id, shop_id)
        .await
        .map_err(|e| AppError::internal(format!("Failed to remove shop subscription: {}", e)))?;
    
    Ok(Json(json!({ 
        "success": true,
        "message": "Shop subscription removed",
        "user_id": user_id,
        "shop_id": shop_id
    })))
}

// 🏪 Phase 3: 매장/브랜드/카테고리 핸들러들

// 매장 목록 조회
async fn get_shops(
    Query(query): Query<ProductQuery>,
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<serde_json::Value>> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20);
    
    if page == 0 || limit == 0 || limit > 100 {
        return Err(AppError::validation("Invalid page or limit parameters"));
    }

    let pagination = Pagenation { page, limit };
    
    log::info!("🏪 Getting shops list");
    let result = state.shop_service
        .get_shops_paginated(pagination)
        .await
        .map_err(|e| AppError::internal(format!("Failed to get shops: {}", e)))?;
    
    Ok(Json(json!({ 
        "shops": result.data,
        "pagination": {
            "page": result.page,
            "limit": result.limit,
            "total": result.total,
            "total_pages": result.total_pages,
            "has_next": result.has_next,
            "has_prev": result.has_prev
        }
    })))
}

// 브랜드 목록 조회
async fn get_brands(
    Query(query): Query<ProductQuery>,
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<serde_json::Value>> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20);
    
    if page == 0 || limit == 0 || limit > 100 {
        return Err(AppError::validation("Invalid page or limit parameters"));
    }

    let pagination = Pagenation { page, limit };
    
    log::info!("🏷️ Getting brands list");
    let result = state.shop_service
        .get_brands_paginated(pagination)
        .await
        .map_err(|e| AppError::internal(format!("Failed to get brands: {}", e)))?;
    
    Ok(Json(json!({ 
        "brands": result.data,
        "pagination": {
            "page": result.page,
            "limit": result.limit,
            "total": result.total,
            "total_pages": result.total_pages,
            "has_next": result.has_next,
            "has_prev": result.has_prev
        }
    })))
}

// 브랜드 상세 조회
async fn get_brand_by_id(
    Path(brand_id): Path<i64>,
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<serde_json::Value>> {
    log::info!("🏷️ Getting brand by ID: {}", brand_id);
    
    let brand = state.shop_service
        .get_brand_by_id(brand_id)
        .await
        .map_err(|e| AppError::internal(format!("Failed to get brand: {}", e)))?;
    
    match brand {
        Some(brand) => Ok(Json(json!({ "brand": brand }))),
        None => Err(AppError::not_found("Brand")),
    }
}

// 카테고리 목록 조회 (계층형)
async fn get_categories(
    Query(parent_query): Query<serde_json::Value>,
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<serde_json::Value>> {
    let parent_id = parent_query.get("parent_id")
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse::<i64>().ok());
    
    log::info!("📂 Getting categories (parent_id: {:?})", parent_id);
    let categories = state.shop_service
        .get_categories_by_parent(parent_id)
        .await
        .map_err(|e| AppError::internal(format!("Failed to get categories: {}", e)))?;
    
    Ok(Json(json!({ "categories": categories })))
}

// 카테고리 상세 조회
async fn get_category_by_id(
    Path(category_id): Path<i64>,
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<serde_json::Value>> {
    log::info!("📂 Getting category by ID: {}", category_id);
    
    let category = state.shop_service
        .get_category_by_id(category_id)
        .await
        .map_err(|e| AppError::internal(format!("Failed to get category: {}", e)))?;
    
    match category {
        Some(category) => Ok(Json(json!({ "category": category }))),
        None => Err(AppError::not_found("Category")),
    }
}

// 상품 검색
async fn search_products(
    Query(search_query): Query<serde_json::Value>,
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<serde_json::Value>> {
    let query = search_query.get("q").and_then(|v| v.as_str()).unwrap_or("");
    let page = search_query.get("page").and_then(|v| v.as_u64()).unwrap_or(1) as u32;
    let limit = search_query.get("limit").and_then(|v| v.as_u64()).unwrap_or(20) as u32;
    
    if query.is_empty() {
        return Err(AppError::validation("Search query is required"));
    }
    
    if page == 0 || limit == 0 || limit > 100 {
        return Err(AppError::validation("Invalid page or limit parameters"));
    }

    let pagination = Pagenation { page, limit };
    
    log::info!("🔍 Searching products with query: '{}'", query);
    // 임시 구현 - 전체 상품 목록 반환
    let result = state.product_service
        .get_all_products(pagination)
        .await
        .map_err(|e| AppError::internal(format!("Failed to search products: {}", e)))?;
    
    Ok(Json(json!({ 
        "query": query,
        "products": result.data,
        "pagination": {
            "page": result.page,
            "limit": result.limit,
            "total": result.total,
            "total_pages": result.total_pages,
            "has_next": result.has_next,
            "has_prev": result.has_prev
        }
    })))
}

// 💰 Phase 3: 쿠폰 시스템 핸들러들

// 쿠폰 목록 조회
async fn get_coupons(
    Query(query): Query<ProductQuery>,
    State(_state): State<Arc<AppState>>,
) -> AppResult<Json<serde_json::Value>> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20);
    
    log::info!("🎫 Getting coupons list");
    
    // 임시 구현 - 빈 쿠폰 목록 반환
    Ok(Json(json!({ 
        "coupons": [],
        "pagination": {
            "page": page,
            "limit": limit,
            "total": 0,
            "total_pages": 0,
            "has_next": false,
            "has_prev": false
        }
    })))
}

// 쿠폰 상세 조회
async fn get_coupon_by_id(
    Path(coupon_id): Path<i64>,
    State(_state): State<Arc<AppState>>,
) -> AppResult<Json<serde_json::Value>> {
    log::info!("🎫 Getting coupon by ID: {}", coupon_id);
    
    // 임시 구현 - 쿠폰 찾을 수 없음
    Err(AppError::not_found("Coupon"))
}

// 쿠폰 사용
async fn use_coupon(
    Path(coupon_id): Path<i64>,
    State(_state): State<Arc<AppState>>,
) -> AppResult<Json<serde_json::Value>> {
    log::info!("🎫 Using coupon: {}", coupon_id);
    
    // 임시 구현 - 쿠폰 사용 처리
    Ok(Json(json!({ 
        "success": true,
        "message": "Coupon used successfully",
        "coupon_id": coupon_id
    })))
}

// 🔔 Phase 3: 알림 시스템 핸들러들

// 알림 목록 조회
async fn get_notifications(
    Path(user_id): Path<String>,
    Query(query): Query<ProductQuery>,
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<serde_json::Value>> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20);
    
    if page == 0 || limit == 0 || limit > 100 {
        return Err(AppError::validation("Invalid page or limit parameters"));
    }

    let pagination = Pagenation { page, limit };
    
    log::info!("🔔 Getting notifications for user: {}", user_id);
    let result = state.notification_service
        .get_notifications(&user_id, pagination)
        .await
        .map_err(|e| AppError::internal(format!("Failed to get notifications: {}", e)))?;
    
    Ok(Json(json!({ 
        "notifications": result.data,
        "pagination": {
            "page": result.page,
            "limit": result.limit,
            "total": result.total,
            "total_pages": result.total_pages,
            "has_next": result.has_next,
            "has_prev": result.has_prev
        }
    })))
}

// 알림 읽음 처리
async fn mark_notification_read(
    Path(notification_id): Path<i64>,
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<serde_json::Value>> {
    log::info!("📖 Marking notification as read: {}", notification_id);
    
    state.notification_service
        .mark_notification_read(notification_id)
        .await
        .map_err(|e| AppError::internal(format!("Failed to mark notification as read: {}", e)))?;
    
    Ok(Json(json!({ 
        "success": true,
        "message": "Notification marked as read",
        "notification_id": notification_id
    })))
}

// 알림 설정 조회
async fn get_notification_settings(
    Path(user_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<serde_json::Value>> {
    log::info!("⚙️ Getting notification settings for user: {}", user_id);
    
    let settings = state.notification_service
        .get_notification_settings(&user_id)
        .await
        .map_err(|e| AppError::internal(format!("Failed to get notification settings: {}", e)))?;
    
    match settings {
        Some(settings) => Ok(Json(json!({ "settings": settings }))),
        None => Err(AppError::not_found("Notification settings")),
    }
}

// 알림 설정 업데이트
async fn update_notification_settings(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> AppResult<Json<serde_json::Value>> {
    log::info!("🔧 Updating notification settings for user: {}", user_id);
    
    // 페이로드를 NotificationSettings로 변환 (실제 필드 구조 사용)
    let settings = crate::domain::entities::notification::NotificationSettings {
        user_id: user_id.clone(),
        push_enabled: payload.get("push_enabled").and_then(|v| v.as_bool()).unwrap_or(true),
        email_enabled: payload.get("email_enabled").and_then(|v| v.as_bool()).unwrap_or(true),
        sms_enabled: payload.get("sms_enabled").and_then(|v| v.as_bool()).unwrap_or(false),
        discount_alerts: payload.get("discount_alerts").and_then(|v| v.as_bool()).unwrap_or(true),
        price_drop_alerts: payload.get("price_drop_alerts").and_then(|v| v.as_bool()).unwrap_or(true),
        new_product_alerts: payload.get("new_product_alerts").and_then(|v| v.as_bool()).unwrap_or(true),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    let updated_settings = state.notification_service
        .update_notification_settings(&user_id, settings)
        .await
        .map_err(|e| AppError::internal(format!("Failed to update notification settings: {}", e)))?;
    
    Ok(Json(json!({ 
        "success": true,
        "message": "Notification settings updated successfully",
        "settings": updated_settings
    })))
}

// 📈 Phase 4: 모니터링 핸들러들

// API 메트릭 조회
async fn get_api_metrics(
    Query(query): Query<ProductQuery>,
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<serde_json::Value>> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(50);
    
    let pagination = Pagenation { page, limit };
    
    log::info!("📊 Getting API metrics");
    let metrics = state.monitoring_service
        .get_api_metrics(pagination)
        .await
        .map_err(|e| AppError::internal(format!("Failed to get API metrics: {}", e)))?;
    
    Ok(Json(metrics))
}

// 에러 로그 조회
async fn get_error_logs(
    Query(query): Query<ProductQuery>,
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<serde_json::Value>> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(50);
    
    let pagination = Pagenation { page, limit };
    
    log::info!("🚨 Getting error logs");
    let logs = state.monitoring_service
        .get_error_logs(pagination)
        .await
        .map_err(|e| AppError::internal(format!("Failed to get error logs: {}", e)))?;
    
    Ok(Json(logs))
}

// 캐시 통계 조회
async fn get_cache_stats(
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<serde_json::Value>> {
    log::info!("💾 Getting cache statistics");
    let stats = state.monitoring_service
        .get_cache_stats()
        .await
        .map_err(|e| AppError::internal(format!("Failed to get cache stats: {}", e)))?;
    
    Ok(Json(stats))
}

// 시스템 상태 점검
async fn get_system_health(
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<serde_json::Value>> {
    log::info!("💚 Getting system health");
    let health = state.monitoring_service
        .get_system_health()
        .await
        .map_err(|e| AppError::internal(format!("Failed to get system health: {}", e)))?;
    
    Ok(Json(health))
}
