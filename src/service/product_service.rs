use crate::config::SupabaseConfig;
use crate::repository::RepositoryFactory;
use crate::domain::entities::product::Product;
use crate::domain::dto::pagenation::{Pagenation, PagenationResult};

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

    // 나라별 상품 목록 조회 
    pub async fn get_products_by_country(&self, country: &str, pagination: Pagenation) -> Result<PagenationResult<Product>, Box<dyn std::error::Error>> {
        log::info!("🌍 Getting products for country: {}", country);
        let repo = self.factory.public_product_repo();
        repo.find_by_country(country, pagination).await
    }

    // 전체 상품 목록 조회 (나라별 필터링 없음)
    pub async fn get_all_products(&self, pagination: Pagenation) -> Result<PagenationResult<Product>, Box<dyn std::error::Error>> {
        log::info!("📦 Getting all products");
        let repo = self.factory.public_product_repo();
        repo.find_all_paginated(pagination).await
    }

    // 인기 상품 조회 (클릭 수 기준)
    pub async fn get_popular_products(&self, pagination: Pagenation) -> Result<PagenationResult<Product>, Box<dyn std::error::Error>> {
        log::info!("🔥 Getting popular products");
        let repo = self.factory.public_product_repo();
        repo.find_popular_products(pagination).await
    }

    // 상품 클릭 기록 - 클릭 수 증가
    pub async fn record_product_click(&self, product_id: i64) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("🖱️ Recording product click: {}", product_id);
        let repo = self.factory.public_product_repo();
        repo.increment_click_count(product_id).await
    }
}