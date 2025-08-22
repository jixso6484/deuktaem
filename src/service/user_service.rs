use crate::config::SupabaseConfig;
use crate::repository::RepositoryFactory;
use crate::domain::entities::user::{Profile, ShopSubscription, BrandSubscription, CategorySubscription};
use crate::domain::dto::pagenation::Pagenation;

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
    pub async fn get_profile(&self, user_token: &str, user_id: &str) -> Result<Option<Profile>, Box<dyn std::error::Error>> {
        log::info!("👤 Getting profile for user: {}", user_id);
        let repo = self.factory.authenticated_user_repo(user_token);
        repo.find_profile_by_user_id(user_id).await
    }

    pub async fn create_profile(&self, user_token: &str, profile: Profile) -> Result<Profile, Box<dyn std::error::Error>> {
        log::info!("👤 Creating profile for user: {}", profile.user_id);
        let repo = self.factory.authenticated_user_repo(user_token);
        repo.create_profile(profile).await
    }

    pub async fn update_profile(&self, user_token: &str, user_id: &str, profile: Profile) -> Result<Profile, Box<dyn std::error::Error>> {
        log::info!("👤 Updating profile for user: {}", user_id);
        let repo = self.factory.authenticated_user_repo(user_token);
        repo.update_profile(user_id, profile).await
    }

    // 매장 구독 관리
    pub async fn get_shop_subscriptions(&self, user_token: &str, user_id: &str) -> Result<Vec<ShopSubscription>, Box<dyn std::error::Error>> {
        log::info!("🏪 Getting shop subscriptions for user: {}", user_id);
        let repo = self.factory.authenticated_user_repo(user_token);
        repo.find_shop_subscriptions(user_id).await
    }

    pub async fn subscribe_to_shop(&self, user_token: &str, subscription: ShopSubscription) -> Result<ShopSubscription, Box<dyn std::error::Error>> {
        log::info!("🏪 User {} subscribing to shop: {}", subscription.user_id, subscription.shop_id);
        let repo = self.factory.authenticated_user_repo(user_token);
        repo.create_shop_subscription(subscription).await
    }

    pub async fn unsubscribe_from_shop(&self, user_token: &str, user_id: &str, shop_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("🏪 User {} unsubscribing from shop: {}", user_id, shop_id);
        let repo = self.factory.authenticated_user_repo(user_token);
        repo.delete_shop_subscription(user_id, shop_id).await
    }

    // 브랜드 구독 관리
    pub async fn get_brand_subscriptions(&self, user_token: &str, user_id: &str) -> Result<Vec<BrandSubscription>, Box<dyn std::error::Error>> {
        log::info!("🏷️ Getting brand subscriptions for user: {}", user_id);
        let repo = self.factory.authenticated_user_repo(user_token);
        repo.find_brand_subscriptions(user_id).await
    }

    pub async fn subscribe_to_brand(&self, user_token: &str, subscription: BrandSubscription) -> Result<BrandSubscription, Box<dyn std::error::Error>> {
        log::info!("🏷️ User {} subscribing to brand: {}", subscription.user_id, subscription.brand_id);
        let repo = self.factory.authenticated_user_repo(user_token);
        repo.create_brand_subscription(subscription).await
    }

    // 카테고리 구독 관리
    pub async fn get_category_subscriptions(&self, user_token: &str, user_id: &str) -> Result<Vec<CategorySubscription>, Box<dyn std::error::Error>> {
        log::info!("📂 Getting category subscriptions for user: {}", user_id);
        let repo = self.factory.authenticated_user_repo(user_token);
        repo.find_category_subscriptions(user_id).await
    }

    // 관리자 기능
    pub async fn admin_get_all_profiles(&self, admin_token: &str, pagination: Pagenation) -> Result<Vec<Profile>, Box<dyn std::error::Error>> {
        log::info!("👑 Admin getting all profiles (page: {})", pagination.page);
        // 실제로는 paginated query 구현 필요
        Ok(Vec::new()) // 임시
    }
}