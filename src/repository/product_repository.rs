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
}