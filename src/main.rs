
mod domain;
mod config;
mod repository;
mod service;
mod api;
mod utils;
mod error;
mod auth;

use axum::{
    extract::{Path, State},
    response::Json,
    routing::{get, post},
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
use crate::service::{DiscountService, ShopService, ProductService};
use crate::domain::dto::HealthResponse;
use crate::utils::init_logger;
use crate::error::{AppError, AppResult};

// 애플리케이션 상태 - Phase 1에서는 기본 서비스만 사용
#[derive(Clone)]
pub struct AppState {
    pub discount_service: DiscountService,
    pub shop_service: ShopService,
    pub product_service: ProductService,
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
    
    // 서비스 초기화 - Phase 1 기본 서비스만
    let app_state = AppState {
        discount_service: DiscountService::new(config.clone()),
        shop_service: ShopService::new(config.clone()),
        product_service: ProductService::new(config),
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
        .route("/api/v1/products/:id", get(get_product_by_id))
        .route("/api/v1/products/:id/click", post(record_product_click))
        
        // 💰 Phase 1: 할인 정보 API (기본)  
        .route("/api/v1/discounts/:id", get(get_discount_by_id))
        
        // 🏪 Phase 1: 매장 정보 API (기본)
        .route("/api/v1/shops/:id", get(get_shop_by_id))
        
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
