use postgrest::Postgrest;
use serde_json::Value;

use crate::domain::entities::product::Product;
use crate::domain::dto::pagenation::{Pagenation, PagenationResult};

pub struct ProductRepository {
    client: Postgrest,
}

impl ProductRepository {
    pub fn new(client: Postgrest) -> Self {
        Self { client }
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<Product>, Box<dyn std::error::Error>> {
        let response = self.client
            .from("products")
            .select("*")
            .eq("id", &id.to_string())
            .single()
            .execute()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let product: Product = serde_json::from_str(&text)?;
            Ok(Some(product))
        } else {
            Ok(None)
        }
    }

    // 나라별 상품 조회 - shipping_regions 테이블과 조인하여 구현
    pub async fn find_by_country(&self, country: &str, pagination: Pagenation) -> Result<PagenationResult<Product>, Box<dyn std::error::Error>> {
        let offset = (pagination.page - 1) * pagination.limit;

        // 임시로 전체 상품을 반환 (PostgREST 조인 문법 확인 후 개선 필요)
        let response = self.client
            .from("products")
            .select("*")
            .eq("is_deleted", "false")
            .order("created_at.desc")
            .range(offset as usize, (offset + pagination.limit - 1) as usize)
            .execute()
            .await?;

        let products: Vec<Product> = if response.status().is_success() {
            let text = response.text().await?;
            serde_json::from_str(&text)?
        } else {
            Vec::new()
        };

        // 카운트 쿼리
        let count_response = self.client
            .from("products")
            .select("count")
            .eq("is_deleted", "false")
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
            data: products,
            total,
            page: pagination.page,
            limit: pagination.limit,
            total_pages,
            has_next: pagination.page < total_pages,
            has_prev: pagination.page > 1,
        })
    }

    // 인기 상품 조회 (클릭 수 기준)
    pub async fn find_popular_products(&self, pagination: Pagenation) -> Result<PagenationResult<Product>, Box<dyn std::error::Error>> {
        let offset = (pagination.page - 1) * pagination.limit;

        let response = self.client
            .from("products")
            .select("*")
            .eq("is_deleted", "false")
            .order("click_count.desc")  // 클릭 수 기준 내림차순
            .range(offset as usize, (offset + pagination.limit - 1) as usize)
            .execute()
            .await?;

        let products: Vec<Product> = if response.status().is_success() {
            let text = response.text().await?;
            serde_json::from_str(&text)?
        } else {
            Vec::new()
        };

        let count_response = self.client
            .from("products")
            .select("count")
            .eq("is_deleted", "false")
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
            data: products,
            total,
            page: pagination.page,
            limit: pagination.limit,
            total_pages,
            has_next: pagination.page < total_pages,
            has_prev: pagination.page > 1,
        })
    }

    // 전체 상품 조회 (나라별 필터링 없음)
    pub async fn find_all_paginated(&self, pagination: Pagenation) -> Result<PagenationResult<Product>, Box<dyn std::error::Error>> {
        let offset = (pagination.page - 1) * pagination.limit;

        let response = self.client
            .from("products")
            .select("*")
            .eq("is_deleted", "false")
            .order("created_at.desc")
            .range(offset as usize, (offset + pagination.limit - 1) as usize)
            .execute()
            .await?;

        let products: Vec<Product> = if response.status().is_success() {
            let text = response.text().await?;
            serde_json::from_str(&text)?
        } else {
            Vec::new()
        };

        let count_response = self.client
            .from("products")
            .select("count")
            .eq("is_deleted", "false")
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
            data: products,
            total,
            page: pagination.page,
            limit: pagination.limit,
            total_pages,
            has_next: pagination.page < total_pages,
            has_prev: pagination.page > 1,
        })
    }

    pub async fn find_by_shop_id(&self, shop_id: &str, pagination: Pagenation) -> Result<PagenationResult<Product>, Box<dyn std::error::Error>> {
        let offset = (pagination.page - 1) * pagination.limit;

        let response = self.client
            .from("products")
            .select("*")
            .eq("shop_id", shop_id)
            .eq("is_deleted", "false")
            .order("created_at.desc")
            .range(offset as usize, (offset + pagination.limit - 1) as usize)
            .execute()
            .await?;

        let products: Vec<Product> = if response.status().is_success() {
            let text = response.text().await?;
            serde_json::from_str(&text)?
        } else {
            Vec::new()
        };

        let count_response = self.client
            .from("products")
            .select("count")
            .eq("shop_id", shop_id)
            .eq("is_deleted", "false")
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
            data: products,
            total,
            page: pagination.page,
            limit: pagination.limit,
            total_pages,
            has_next: pagination.page < total_pages,
            has_prev: pagination.page > 1,
        })
    }

    pub async fn find_by_brand_id(&self, brand_id: &str, pagination: Pagenation) -> Result<PagenationResult<Product>, Box<dyn std::error::Error>> {
        let offset = (pagination.page - 1) * pagination.limit;

        let response = self.client
            .from("products")
            .select("*")
            .eq("brand_id", brand_id)
            .eq("is_deleted", "false")
            .order("created_at.desc")
            .range(offset as usize, (offset + pagination.limit - 1) as usize)
            .execute()
            .await?;

        let products: Vec<Product> = if response.status().is_success() {
            let text = response.text().await?;
            serde_json::from_str(&text)?
        } else {
            Vec::new()
        };

        let count_response = self.client
            .from("products")
            .select("count")
            .eq("brand_id", brand_id)
            .eq("is_deleted", "false")
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
            data: products,
            total,
            page: pagination.page,
            limit: pagination.limit,
            total_pages,
            has_next: pagination.page < total_pages,
            has_prev: pagination.page > 1,
        })
    }

    pub async fn find_by_category_id(&self, category_id: &str, pagination: Pagenation) -> Result<PagenationResult<Product>, Box<dyn std::error::Error>> {
        let offset = (pagination.page - 1) * pagination.limit;

        let response = self.client
            .from("products")
            .select("*")
            .eq("category_id", category_id)
            .eq("is_deleted", "false")
            .order("created_at.desc")
            .range(offset as usize, (offset + pagination.limit - 1) as usize)
            .execute()
            .await?;

        let products: Vec<Product> = if response.status().is_success() {
            let text = response.text().await?;
            serde_json::from_str(&text)?
        } else {
            Vec::new()
        };

        let count_response = self.client
            .from("products")
            .select("count")
            .eq("category_id", category_id)
            .eq("is_deleted", "false")
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
            data: products,
            total,
            page: pagination.page,
            limit: pagination.limit,
            total_pages,
            has_next: pagination.page < total_pages,
            has_prev: pagination.page > 1,
        })
    }

    pub async fn create(&self, product: Product) -> Result<Product, Box<dyn std::error::Error>> {
        let response = self.client
            .from("products")
            .insert(serde_json::to_string(&product)?)
            .execute()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let created_product: Product = serde_json::from_str(&text)?;
            Ok(created_product)
        } else {
            Err(format!("Failed to create product: {}", response.status()).into())
        }
    }

    pub async fn update(&self, id: &str, product: Product) -> Result<Product, Box<dyn std::error::Error>> {
        let response = self.client
            .from("products")
            .eq("id", id)
            .update(serde_json::to_string(&product)?)
            .execute()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let updated_product: Product = serde_json::from_str(&text)?;
            Ok(updated_product)
        } else {
            Err(format!("Failed to update product: {}", response.status()).into())
        }
    }

    pub async fn soft_delete(&self, id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.client
            .from("products")
            .eq("id", id)
            .update(r#"{"is_deleted": true}"#)
            .execute()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(format!("Failed to delete product: {}", response.status()).into())
        }
    }

    // 상품 클릭 수 증가
    pub async fn increment_click_count(&self, id: i64) -> Result<(), Box<dyn std::error::Error>> {
        // Supabase RPC 호출로 클릭 카운트 증가
        let response = self.client
            .rpc("increment_product_click", &format!(r#"{{"product_id": {}}}"#, id))
            .execute()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            // RPC가 없는 경우 대체 방법으로 처리
            let current_product = self.find_by_id(id).await?;
            if let Some(product) = current_product {
                let new_click_count = product.click_count.unwrap_or(0) + 1;
                let response = self.client
                    .from("products")
                    .eq("id", &id.to_string())
                    .update(&format!(r#"{{"click_count": {}}}"#, new_click_count))
                    .execute()
                    .await?;

                if response.status().is_success() {
                    Ok(())
                } else {
                    Err(format!("Failed to increment click count: {}", response.status()).into())
                }
            } else {
                Err("Product not found".into())
            }
        }
    }
}