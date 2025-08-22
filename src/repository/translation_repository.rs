use postgrest::Postgrest;
use serde_json::Value;

use crate::domain::entities::translation::{
    Language, ShopTranslation, BrandTranslation, CategoryTranslation,
    ProductTranslation, DiscountEventTranslation, DiscountInfoTranslation,
    NotificationTranslation, ApiMetric
};
use crate::domain::dto::pagenation::{Pagenation, PagenationResult};

pub struct TranslationRepository {
    client: Postgrest,
}

impl TranslationRepository {
    pub fn new(client: Postgrest) -> Self {
        Self { client }
    }

    // Language CRUD
    pub async fn find_active_languages(&self) -> Result<Vec<Language>, Box<dyn std::error::Error>> {
        let response = self.client
            .from("languages")
            .select("*")
            .eq("is_active", "true")
            .order("name.asc")
            .execute()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let languages: Vec<Language> = serde_json::from_str(&text)?;
            Ok(languages)
        } else {
            Ok(Vec::new())
        }
    }

    pub async fn find_language_by_code(&self, code: &str) -> Result<Option<Language>, Box<dyn std::error::Error>> {
        let response = self.client
            .from("languages")
            .select("*")
            .eq("code", code)
            .single()
            .execute()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let language: Language = serde_json::from_str(&text)?;
            Ok(Some(language))
        } else {
            Ok(None)
        }
    }

    // Shop Translation CRUD
    pub async fn find_shop_translation(&self, shop_id: &str, locale: &str) -> Result<Option<ShopTranslation>, Box<dyn std::error::Error>> {
        let response = self.client
            .from("shop_translations")
            .select("*")
            .eq("shop_id", shop_id)
            .eq("locale", locale)
            .single()
            .execute()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let translation: ShopTranslation = serde_json::from_str(&text)?;
            Ok(Some(translation))
        } else {
            Ok(None)
        }
    }

    pub async fn find_shop_translations(&self, shop_id: &str) -> Result<Vec<ShopTranslation>, Box<dyn std::error::Error>> {
        let response = self.client
            .from("shop_translations")
            .select("*")
            .eq("shop_id", shop_id)
            .execute()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let translations: Vec<ShopTranslation> = serde_json::from_str(&text)?;
            Ok(translations)
        } else {
            Ok(Vec::new())
        }
    }

    pub async fn create_shop_translation(&self, translation: ShopTranslation) -> Result<ShopTranslation, Box<dyn std::error::Error>> {
        let response = self.client
            .from("shop_translations")
            .insert(serde_json::to_string(&translation)?)
            .execute()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let created_translation: ShopTranslation = serde_json::from_str(&text)?;
            Ok(created_translation)
        } else {
            Err(format!("Failed to create shop translation: {}", response.status()).into())
        }
    }

    // Brand Translation CRUD
    pub async fn find_brand_translation(&self, brand_id: &str, locale: &str) -> Result<Option<BrandTranslation>, Box<dyn std::error::Error>> {
        let response = self.client
            .from("brand_translations")
            .select("*")
            .eq("brand_id", brand_id)
            .eq("locale", locale)
            .single()
            .execute()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let translation: BrandTranslation = serde_json::from_str(&text)?;
            Ok(Some(translation))
        } else {
            Ok(None)
        }
    }

    pub async fn create_brand_translation(&self, translation: BrandTranslation) -> Result<BrandTranslation, Box<dyn std::error::Error>> {
        let response = self.client
            .from("brand_translations")
            .insert(serde_json::to_string(&translation)?)
            .execute()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let created_translation: BrandTranslation = serde_json::from_str(&text)?;
            Ok(created_translation)
        } else {
            Err(format!("Failed to create brand translation: {}", response.status()).into())
        }
    }

    // Category Translation CRUD
    pub async fn find_category_translation(&self, category_id: &str, locale: &str) -> Result<Option<CategoryTranslation>, Box<dyn std::error::Error>> {
        let response = self.client
            .from("category_translations")
            .select("*")
            .eq("category_id", category_id)
            .eq("locale", locale)
            .single()
            .execute()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let translation: CategoryTranslation = serde_json::from_str(&text)?;
            Ok(Some(translation))
        } else {
            Ok(None)
        }
    }

    pub async fn create_category_translation(&self, translation: CategoryTranslation) -> Result<CategoryTranslation, Box<dyn std::error::Error>> {
        let response = self.client
            .from("category_translations")
            .insert(serde_json::to_string(&translation)?)
            .execute()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let created_translation: CategoryTranslation = serde_json::from_str(&text)?;
            Ok(created_translation)
        } else {
            Err(format!("Failed to create category translation: {}", response.status()).into())
        }
    }

    // Product Translation CRUD
    pub async fn find_product_translation(&self, product_id: &str, locale: &str) -> Result<Option<ProductTranslation>, Box<dyn std::error::Error>> {
        let response = self.client
            .from("product_translations")
            .select("*")
            .eq("product_id", product_id)
            .eq("locale", locale)
            .single()
            .execute()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let translation: ProductTranslation = serde_json::from_str(&text)?;
            Ok(Some(translation))
        } else {
            Ok(None)
        }
    }

    pub async fn create_product_translation(&self, translation: ProductTranslation) -> Result<ProductTranslation, Box<dyn std::error::Error>> {
        let response = self.client
            .from("product_translations")
            .insert(serde_json::to_string(&translation)?)
            .execute()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let created_translation: ProductTranslation = serde_json::from_str(&text)?;
            Ok(created_translation)
        } else {
            Err(format!("Failed to create product translation: {}", response.status()).into())
        }
    }

    // Discount Event Translation CRUD
    pub async fn find_discount_event_translation(&self, event_id: &str, locale: &str) -> Result<Option<DiscountEventTranslation>, Box<dyn std::error::Error>> {
        let response = self.client
            .from("discount_event_translations")
            .select("*")
            .eq("event_id", event_id)
            .eq("locale", locale)
            .single()
            .execute()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let translation: DiscountEventTranslation = serde_json::from_str(&text)?;
            Ok(Some(translation))
        } else {
            Ok(None)
        }
    }

    // Discount Info Translation CRUD
    pub async fn find_discount_info_translation(&self, discount_info_id: &str, locale: &str) -> Result<Option<DiscountInfoTranslation>, Box<dyn std::error::Error>> {
        let response = self.client
            .from("discount_info_translations")
            .select("*")
            .eq("discount_info_id", discount_info_id)
            .eq("locale", locale)
            .single()
            .execute()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let translation: DiscountInfoTranslation = serde_json::from_str(&text)?;
            Ok(Some(translation))
        } else {
            Ok(None)
        }
    }

    // Notification Translation CRUD
    pub async fn find_notification_translation(&self, notification_id: &str, locale: &str) -> Result<Option<NotificationTranslation>, Box<dyn std::error::Error>> {
        let response = self.client
            .from("notification_translations")
            .select("*")
            .eq("notification_id", notification_id)
            .eq("locale", locale)
            .single()
            .execute()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let translation: NotificationTranslation = serde_json::from_str(&text)?;
            Ok(Some(translation))
        } else {
            Ok(None)
        }
    }

    pub async fn create_notification_translation(&self, translation: NotificationTranslation) -> Result<NotificationTranslation, Box<dyn std::error::Error>> {
        let response = self.client
            .from("notification_translations")
            .insert(serde_json::to_string(&translation)?)
            .execute()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let created_translation: NotificationTranslation = serde_json::from_str(&text)?;
            Ok(created_translation)
        } else {
            Err(format!("Failed to create notification translation: {}", response.status()).into())
        }
    }

    // API Metrics CRUD
    pub async fn create_api_metric(&self, metric: ApiMetric) -> Result<ApiMetric, Box<dyn std::error::Error>> {
        let response = self.client
            .from("api_metrics")
            .insert(serde_json::to_string(&metric)?)
            .execute()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let created_metric: ApiMetric = serde_json::from_str(&text)?;
            Ok(created_metric)
        } else {
            Err(format!("Failed to create API metric: {}", response.status()).into())
        }
    }

    pub async fn find_api_metrics(&self, pagination: Pagenation) -> Result<PagenationResult<ApiMetric>, Box<dyn std::error::Error>> {
        let offset = (pagination.page - 1) * pagination.limit;

        let response = self.client
            .from("api_metrics")
            .select("*")
            .order("created_at.desc")
            .range(offset as usize, (offset + pagination.limit - 1) as usize)
            .execute()
            .await?;

        let metrics: Vec<ApiMetric> = if response.status().is_success() {
            let text = response.text().await?;
            serde_json::from_str(&text)?
        } else {
            Vec::new()
        };

        let count_response = self.client
            .from("api_metrics")
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
            data: metrics,
            total,
            page: pagination.page,
            limit: pagination.limit,
            total_pages,
            has_next: pagination.page < total_pages,
            has_prev: pagination.page > 1,
        })
    }

    pub async fn find_error_metrics(&self, pagination: Pagenation) -> Result<PagenationResult<ApiMetric>, Box<dyn std::error::Error>> {
        let offset = (pagination.page - 1) * pagination.limit;

        let response = self.client
            .from("api_metrics")
            .select("*")
            .gte("status_code", "400")
            .order("created_at.desc")
            .range(offset as usize, (offset + pagination.limit - 1) as usize)
            .execute()
            .await?;

        let metrics: Vec<ApiMetric> = if response.status().is_success() {
            let text = response.text().await?;
            serde_json::from_str(&text)?
        } else {
            Vec::new()
        };

        let count_response = self.client
            .from("api_metrics")
            .select("count")
            .gte("status_code", "400")
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
            data: metrics,
            total,
            page: pagination.page,
            limit: pagination.limit,
            total_pages,
            has_next: pagination.page < total_pages,
            has_prev: pagination.page > 1,
        })
    }
}