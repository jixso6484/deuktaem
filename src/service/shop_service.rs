use crate::config::SupabaseConfig;
use crate::repository::RepositoryFactory;
use crate::domain::entities::shop::Shop;

#[derive(Clone)]
pub struct ShopService {
    factory: RepositoryFactory,
}

impl ShopService {
    pub fn new(config: SupabaseConfig) -> Self {
        Self {
            factory: RepositoryFactory::new(config),
        }
    }

    // 기본 매장 조회만 남김
    pub async fn get_shop_by_id(&self, shop_id: i64) -> Result<Option<Shop>, Box<dyn std::error::Error>> {
        log::info!("🏪 Getting shop by ID: {}", shop_id);
        let repo = self.factory.public_shop_repo();
        repo.find_shop_by_id(shop_id).await
    }
}