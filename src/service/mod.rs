// Phase 1-2: 기본 서비스 + 구독 시스템
pub mod discount_service;
pub mod shop_service;
pub mod product_service;
pub mod user_service;

pub use discount_service::*;
pub use shop_service::*;
pub use product_service::*;
pub use user_service::*;