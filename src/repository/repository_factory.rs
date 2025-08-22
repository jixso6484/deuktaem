use crate::config::SupabaseConfig;
use crate::repository::{
    DiscountRepository, ShopRepository, ProductRepository
};

#[derive(Clone)]
pub struct RepositoryFactory {
    config: SupabaseConfig,
}

impl RepositoryFactory {
    pub fn new(config: SupabaseConfig) -> Self {
        Self { config }
    }

    // Phase 1: 기본 공개 Repository들만
    pub fn public_discount_repo(&self) -> DiscountRepository {
        DiscountRepository::new(self.config.public_client())
    }

    pub fn public_shop_repo(&self) -> ShopRepository {
        ShopRepository::new(self.config.public_client())
    }

    pub fn public_product_repo(&self) -> ProductRepository {
        ProductRepository::new(self.config.public_client())
    }
}

