use postgrest::Postgrest;
use serde_json::Value;

use crate::domain::entities::discount::DiscountInfo;
use crate::domain::dto::pagenation::{Pagenation, PagenationResult};

pub struct DiscountRepository {
    client: Postgrest,
}

impl DiscountRepository {
    pub fn new(client: Postgrest) -> Self {
        Self { client }
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<DiscountInfo>, Box<dyn std::error::Error>> {
        let response = self.client
            .from("discount_infos")
            .select("*")
            .eq("id", &id.to_string())
            .single()
            .execute()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let discount: DiscountInfo = serde_json::from_str(&text)?;
            Ok(Some(discount))
        } else {
            Ok(None)
        }
    }

    pub async fn find_all_paginated(&self, pagination: Pagenation) -> Result<PagenationResult<DiscountInfo>, Box<dyn std::error::Error>> {
        let offset = (pagination.page - 1) * pagination.limit;

        // 데이터 조회
        let response = self.client
            .from("discount_infos")
            .select("*")
            .order("created_at.desc")
            .range(offset as usize, (offset + pagination.limit - 1) as usize)
            .execute()
            .await?;

        let discounts: Vec<DiscountInfo> = if response.status().is_success() {
            let text = response.text().await?;
            serde_json::from_str(&text)?
        } else {
            Vec::new()
        };

        // 전체 개수 조회
        let count_response = self.client
            .from("discount_infos")
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
            data: discounts,
            total,
            page: pagination.page,
            limit: pagination.limit,
            total_pages,
            has_next: pagination.page < total_pages,
            has_prev: pagination.page > 1,
        })
    }
}