use crate::config::SupabaseConfig;
use crate::repository::{
    DiscountRepository, ShopRepository, ProductRepository, UserRepository
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

    // 인증된 사용자용 Repository들 (RLS 적용, user token 사용)
    pub fn authenticated_user_repo(&self, user_token: &str) -> UserRepository {
        UserRepository::new(self.config.authenticated_client(user_token))
    }
}

