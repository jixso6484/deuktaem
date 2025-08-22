use crate::config::SupabaseConfig;
use crate::repository::RepositoryFactory;
use crate::domain::entities::user::*;
use crate::error::{AppError, AppResult};

#[derive(Clone)]
pub struct UserService {
    factory: RepositoryFactory,
}

impl UserService {
    pub fn new(config: SupabaseConfig) -> Self {
        Self {
            factory: RepositoryFactory::new(config),
        }
    }

    // 프로필 관리
    pub async fn get_profile(&self, user_id: &str) -> AppResult<Option<Profile>> {
        log::info!("👤 Getting profile for user: {}", user_id);
        let repo = self.factory.authenticated_user_repo(user_id);
        repo.find_profile_by_user_id(user_id)
            .await
            .map_err(|e| AppError::internal(format!("Failed to get profile: {}", e)))
    }

    pub async fn update_profile(&self, profile: Profile) -> AppResult<Profile> {
        log::info!("👤 Updating profile for user: {}", profile.user_id);
        let repo = self.factory.authenticated_user_repo(&profile.user_id);
        repo.update_profile(profile)
            .await
            .map_err(|e| AppError::internal(format!("Failed to update profile: {}", e)))
    }

    // 구독 관리
    pub async fn add_product_subscription(&self, user_id: &str, product_id: i64) -> AppResult<ProductSubscription> {
        log::info!("📦➕ Adding product subscription - User: {}, Product: {}", user_id, product_id);
        let repo = self.factory.authenticated_user_repo(user_id);
        repo.add_product_subscription(user_id, product_id)
            .await
            .map_err(|e| AppError::internal(format!("Failed to add product subscription: {}", e)))
    }

    pub async fn remove_product_subscription(&self, user_id: &str, product_id: i64) -> AppResult<()> {
        log::info!("📦➖ Removing product subscription - User: {}, Product: {}", user_id, product_id);
        let repo = self.factory.authenticated_user_repo(user_id);
        repo.remove_product_subscription(user_id, product_id)
            .await
            .map_err(|e| AppError::internal(format!("Failed to remove product subscription: {}", e)))
    }

    pub async fn add_brand_subscription(&self, user_id: &str, brand_id: i64) -> AppResult<BrandSubscription> {
        log::info!("🏷️➕ Adding brand subscription - User: {}, Brand: {}", user_id, brand_id);
        let repo = self.factory.authenticated_user_repo(user_id);
        repo.add_brand_subscription(user_id, brand_id)
            .await
            .map_err(|e| AppError::internal(format!("Failed to add brand subscription: {}", e)))
    }

    pub async fn remove_brand_subscription(&self, user_id: &str, brand_id: i64) -> AppResult<()> {
        log::info!("🏷️➖ Removing brand subscription - User: {}, Brand: {}", user_id, brand_id);
        let repo = self.factory.authenticated_user_repo(user_id);
        repo.remove_brand_subscription(user_id, brand_id)
            .await
            .map_err(|e| AppError::internal(format!("Failed to remove brand subscription: {}", e)))
    }

    pub async fn add_shop_subscription(&self, user_id: &str, shop_id: i64, notification_enabled: bool) -> AppResult<ShopSubscription> {
        log::info!("🏪➕ Adding shop subscription - User: {}, Shop: {}, Notifications: {}", user_id, shop_id, notification_enabled);
        let repo = self.factory.authenticated_user_repo(user_id);
        repo.add_shop_subscription(user_id, shop_id, notification_enabled)
            .await
            .map_err(|e| AppError::internal(format!("Failed to add shop subscription: {}", e)))
    }

    pub async fn remove_shop_subscription(&self, user_id: &str, shop_id: i64) -> AppResult<()> {
        log::info!("🏪➖ Removing shop subscription - User: {}, Shop: {}", user_id, shop_id);
        let repo = self.factory.authenticated_user_repo(user_id);
        repo.remove_shop_subscription(user_id, shop_id)
            .await
            .map_err(|e| AppError::internal(format!("Failed to remove shop subscription: {}", e)))
    }

    pub async fn get_all_subscriptions(&self, user_id: &str) -> AppResult<serde_json::Value> {
        log::info!("📋 Getting all subscriptions for user: {}", user_id);
        let repo = self.factory.authenticated_user_repo(user_id);
        repo.find_all_subscriptions(user_id)
            .await
            .map_err(|e| AppError::internal(format!("Failed to get subscriptions: {}", e)))
    }
}