use crate::config::SupabaseConfig;
use crate::repository::RepositoryFactory;
use crate::domain::entities::discount::DiscountInfo;

#[derive(Clone)]
pub struct DiscountService {
    factory: RepositoryFactory,
}

impl DiscountService {
    pub fn new(config: SupabaseConfig) -> Self {
        Self {
            factory: RepositoryFactory::new(config),
        }
    }

    // 기본 할인 조회만 남김
    pub async fn get_discount_by_id(&self, discount_id: i64) -> Result<Option<DiscountInfo>, Box<dyn std::error::Error>> {
        log::info!("💰 Getting discount by ID: {}", discount_id);
        let repo = self.factory.public_discount_repo();
        repo.find_by_id(discount_id).await
    }
}