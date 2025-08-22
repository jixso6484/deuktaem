// Phase 1-4: 완전한 서비스 레이어
pub mod discount_service;
pub mod shop_service;
pub mod product_service;
pub mod user_service;
pub mod notification_service;
pub mod monitoring_service;

pub use discount_service::*;
pub use shop_service::*;
pub use product_service::*;
pub use user_service::*;
pub use notification_service::*;
pub use monitoring_service::*;