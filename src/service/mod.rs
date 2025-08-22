// Phase 1: 기본 서비스만 포함
pub mod discount_service;
pub mod shop_service;
pub mod product_service;

pub use discount_service::*;
pub use shop_service::*;
pub use product_service::*;