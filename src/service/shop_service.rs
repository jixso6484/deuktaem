use crate::config::SupabaseConfig;
use crate::repository::RepositoryFactory;
use crate::domain::entities::shop::{Shop, Brand, Category};
use crate::domain::dto::pagenation::{Pagenation, PagenationResult};

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

    // 매장 관리 기능들
    pub async fn get_shop_by_id(&self, shop_id: i64) -> Result<Option<Shop>, Box<dyn std::error::Error>> {
        log::info!("🏪 Getting shop by ID: {}", shop_id);
        let repo = self.factory.public_shop_repo();
        repo.find_shop_by_id(shop_id).await
    }

    pub async fn get_shops_paginated(&self, pagination: Pagenation) -> Result<PagenationResult<Shop>, Box<dyn std::error::Error>> {
        log::info!("🏪 Getting shops list with pagination");
        let repo = self.factory.public_shop_repo();
        repo.find_shops_paginated(pagination).await
    }

    // 브랜드 관리 기능들
    pub async fn get_brand_by_id(&self, brand_id: i64) -> Result<Option<Brand>, Box<dyn std::error::Error>> {
        log::info!("🏷️ Getting brand by ID: {}", brand_id);
        let repo = self.factory.public_shop_repo();
        repo.find_brand_by_id(brand_id).await
    }

    pub async fn get_brands_paginated(&self, pagination: Pagenation) -> Result<PagenationResult<Brand>, Box<dyn std::error::Error>> {
        log::info!("🏷️ Getting brands list with pagination");
        let repo = self.factory.public_shop_repo();
        repo.find_brands_paginated(pagination).await
    }

    // 카테고리 관리 기능들 (계층형)
    pub async fn get_category_by_id(&self, category_id: i64) -> Result<Option<Category>, Box<dyn std::error::Error>> {
        log::info!("📂 Getting category by ID: {}", category_id);
        let repo = self.factory.public_shop_repo();
        repo.find_category_by_id(category_id).await
    }

    pub async fn get_categories_by_parent(&self, parent_id: Option<i64>) -> Result<Vec<Category>, Box<dyn std::error::Error>> {
        log::info!("📂 Getting categories by parent: {:?}", parent_id);
        let repo = self.factory.public_shop_repo();
        repo.find_categories_by_parent(parent_id).await
    }
}