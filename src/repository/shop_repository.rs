use postgrest::Postgrest;
use serde_json::Value;

use crate::domain::entities::shop::{Shop, Brand, Category};
use crate::domain::dto::pagenation::{Pagenation, PagenationResult};

pub struct ShopRepository {
    client: Postgrest,
}

impl ShopRepository {
    pub fn new(client: Postgrest) -> Self {
        Self { client }
    }

    // Shop CRUD
    pub async fn find_shop_by_id(&self, id: i64) -> Result<Option<Shop>, Box<dyn std::error::Error>> {
        let response = self.client
            .from("shops")
            .select("*")
            .eq("id", &id.to_string())
            .single()
            .execute()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let shop: Shop = serde_json::from_str(&text)?;
            Ok(Some(shop))
        } else {
            Ok(None)
        }
    }

    pub async fn find_shops_paginated(&self, pagination: Pagenation) -> Result<PagenationResult<Shop>, Box<dyn std::error::Error>> {
        let offset = (pagination.page - 1) * pagination.limit;

        let response = self.client
            .from("shops")
            .select("*")
            .order("created_at.desc")
            .range(offset as usize, (offset + pagination.limit - 1) as usize)
            .execute()
            .await?;

        let shops: Vec<Shop> = if response.status().is_success() {
            let text = response.text().await?;
            serde_json::from_str(&text)?
        } else {
            Vec::new()
        };

        let count_response = self.client
            .from("shops")
            .select("count")
            .execute()
            .await?;

        let total: u64 = if count_response.status().is_success() {
            let text = count_response.text().await?;
            let count_result: Value = serde_json::from_str(&text)?;
            count_result.as_array()
                .and_then(|arr| arr.get(0))
                .and_then(|obj| obj.get("count"))
                .and_then(|c| c.as_u64())
                .unwrap_or(0)
        } else {
            0
        };

        let total_pages = (total as f64 / pagination.limit as f64).ceil() as u32;

        Ok(PagenationResult {
            data: shops,
            total,
            page: pagination.page,
            limit: pagination.limit,
            total_pages,
            has_next: pagination.page < total_pages,
            has_prev: pagination.page > 1,
        })
    }

    pub async fn create_shop(&self, shop: Shop) -> Result<Shop, Box<dyn std::error::Error>> {
        let response = self.client
            .from("shops")
            .insert(serde_json::to_string(&shop)?)
            .execute()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let created_shop: Shop = serde_json::from_str(&text)?;
            Ok(created_shop)
        } else {
            Err(format!("Failed to create shop: {}", response.status()).into())
        }
    }

    // Brand CRUD
    pub async fn find_brand_by_id(&self, id: i64) -> Result<Option<Brand>, Box<dyn std::error::Error>> {
        let response = self.client
            .from("brands")
            .select("*")
            .eq("id", &id.to_string())
            .single()
            .execute()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let brand: Brand = serde_json::from_str(&text)?;
            Ok(Some(brand))
        } else {
            Ok(None)
        }
    }

    pub async fn find_brands_paginated(&self, pagination: Pagenation) -> Result<PagenationResult<Brand>, Box<dyn std::error::Error>> {
        let offset = (pagination.page - 1) * pagination.limit;

        let response = self.client
            .from("brands")
            .select("*")
            .order("created_at.desc")
            .range(offset as usize, (offset + pagination.limit - 1) as usize)
            .execute()
            .await?;

        let brands: Vec<Brand> = if response.status().is_success() {
            let text = response.text().await?;
            serde_json::from_str(&text)?
        } else {
            Vec::new()
        };

        let count_response = self.client
            .from("brands")
            .select("count")
            .execute()
            .await?;

        let total: u64 = if count_response.status().is_success() {
            let text = count_response.text().await?;
            let count_result: Value = serde_json::from_str(&text)?;
            count_result.as_array()
                .and_then(|arr| arr.get(0))
                .and_then(|obj| obj.get("count"))
                .and_then(|c| c.as_u64())
                .unwrap_or(0)
        } else {
            0
        };

        let total_pages = (total as f64 / pagination.limit as f64).ceil() as u32;

        Ok(PagenationResult {
            data: brands,
            total,
            page: pagination.page,
            limit: pagination.limit,
            total_pages,
            has_next: pagination.page < total_pages,
            has_prev: pagination.page > 1,
        })
    }

    // Category CRUD
    pub async fn find_category_by_id(&self, id: i64) -> Result<Option<Category>, Box<dyn std::error::Error>> {
        let response = self.client
            .from("categories")
            .select("*")
            .eq("id", &id.to_string())
            .single()
            .execute()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let category: Category = serde_json::from_str(&text)?;
            Ok(Some(category))
        } else {
            Ok(None)
        }
    }

    pub async fn find_categories_by_parent(&self, parent_id: Option<i64>) -> Result<Vec<Category>, Box<dyn std::error::Error>> {
        let mut query = self.client
            .from("categories")
            .select("*")
            .order("name.asc");

        query = match parent_id {
            Some(pid) => query.eq("parent_id", &pid.to_string()),
            None => query.is("parent_id", "null"),
        };

        let response = query.execute().await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let categories: Vec<Category> = serde_json::from_str(&text)?;
            Ok(categories)
        } else {
            Ok(Vec::new())
        }
    }
}