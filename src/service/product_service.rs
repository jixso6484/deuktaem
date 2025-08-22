use crate::config::SupabaseConfig;
use crate::repository::RepositoryFactory;
use crate::domain::entities::product::Product;

#[derive(Clone)]
pub struct ProductService {
    factory: RepositoryFactory,
}

impl ProductService {
    pub fn new(config: SupabaseConfig) -> Self {
        Self {
            factory: RepositoryFactory::new(config),
        }
    }

    // 기본 기능만 남김 - Repository에 실제로 존재하는 메서드만 사용
    pub async fn get_product_by_id(&self, product_id: i64) -> Result<Option<Product>, Box<dyn std::error::Error>> {
        log::info!("📦 Getting product by ID: {}", product_id);
        let repo = self.factory.public_product_repo();
        repo.find_by_id(product_id).await
    }

    // 상품 클릭 기록 (간단한 버전)
    pub async fn record_product_click(&self, product_id: i64) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("🖱️ 상품 클릭 기록: {}", product_id);
        // 나중에 DB 함수 호출로 구현
        Ok(())
    }
}