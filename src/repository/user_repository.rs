use postgrest::Postgrest;
use serde_json::Value;

use crate::domain::entities::user::*;
use crate::domain::dto::pagenation::{Pagenation, PagenationResult};

pub struct UserRepository {
    client: Postgrest,
}

impl UserRepository {
    pub fn new(client: Postgrest) -> Self {
        Self { client }
    }

    // 프로필 관리
    pub async fn find_profile_by_user_id(&self, user_id: &str) -> Result<Option<Profile>, Box<dyn std::error::Error>> {
        let response = self.client
            .from("profiles")
            .select("*")
            .eq("user_id", user_id)
            .single()
            .execute()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let profile: Profile = serde_json::from_str(&text)?;
            Ok(Some(profile))
        } else {
            Ok(None)
        }
    }

    pub async fn update_profile(&self, profile: Profile) -> Result<Profile, Box<dyn std::error::Error>> {
        let response = self.client
            .from("profiles")
            .eq("user_id", &profile.user_id)
            .update(serde_json::to_string(&profile)?)
            .execute()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let updated_profile: Profile = serde_json::from_str(&text)?;
            Ok(updated_profile)
        } else {
            Err(format!("Failed to update profile: {}", response.status()).into())
        }
    }

    // 상품 구독 관리
    pub async fn add_product_subscription(&self, user_id: &str, product_id: i64) -> Result<ProductSubscription, Box<dyn std::error::Error>> {
        let subscription = ProductSubscription {
            user_id: user_id.to_string(),
            product_id,
            created_at: chrono::Utc::now(),
        };

        let response = self.client
            .from("product_subscriptions")
            .insert(serde_json::to_string(&subscription)?)
            .execute()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let created_subscription: ProductSubscription = serde_json::from_str(&text)?;
            Ok(created_subscription)
        } else {
            Err(format!("Failed to add product subscription: {}", response.status()).into())
        }
    }

    pub async fn remove_product_subscription(&self, user_id: &str, product_id: i64) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.client
            .from("product_subscriptions")
            .eq("user_id", user_id)
            .eq("product_id", &product_id.to_string())
            .delete()
            .execute()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(format!("Failed to remove product subscription: {}", response.status()).into())
        }
    }

    pub async fn find_product_subscriptions(&self, user_id: &str) -> Result<Vec<ProductSubscription>, Box<dyn std::error::Error>> {
        let response = self.client
            .from("product_subscriptions")
            .select("*")
            .eq("user_id", user_id)
            .order("created_at.desc")
            .execute()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let subscriptions: Vec<ProductSubscription> = serde_json::from_str(&text)?;
            Ok(subscriptions)
        } else {
            Ok(Vec::new())
        }
    }

    // 브랜드 구독 관리
    pub async fn add_brand_subscription(&self, user_id: &str, brand_id: i64) -> Result<BrandSubscription, Box<dyn std::error::Error>> {
        let subscription = BrandSubscription {
            user_id: user_id.to_string(),
            brand_id,
            created_at: chrono::Utc::now(),
        };

        let response = self.client
            .from("brand_subscriptions")
            .insert(serde_json::to_string(&subscription)?)
            .execute()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let created_subscription: BrandSubscription = serde_json::from_str(&text)?;
            Ok(created_subscription)
        } else {
            Err(format!("Failed to add brand subscription: {}", response.status()).into())
        }
    }

    pub async fn remove_brand_subscription(&self, user_id: &str, brand_id: i64) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.client
            .from("brand_subscriptions")
            .eq("user_id", user_id)
            .eq("brand_id", &brand_id.to_string())
            .delete()
            .execute()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(format!("Failed to remove brand subscription: {}", response.status()).into())
        }
    }

    // 매장 구독 관리
    pub async fn add_shop_subscription(&self, user_id: &str, shop_id: i64, notification_enabled: bool) -> Result<ShopSubscription, Box<dyn std::error::Error>> {
        let subscription = ShopSubscription {
            user_id: user_id.to_string(),
            shop_id,
            notification_enabled,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let response = self.client
            .from("shop_subscriptions")
            .insert(serde_json::to_string(&subscription)?)
            .execute()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let created_subscription: ShopSubscription = serde_json::from_str(&text)?;
            Ok(created_subscription)
        } else {
            Err(format!("Failed to add shop subscription: {}", response.status()).into())
        }
    }

    pub async fn remove_shop_subscription(&self, user_id: &str, shop_id: i64) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.client
            .from("shop_subscriptions")
            .eq("user_id", user_id)
            .eq("shop_id", &shop_id.to_string())
            .delete()
            .execute()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(format!("Failed to remove shop subscription: {}", response.status()).into())
        }
    }

    // 내 구독 목록
    pub async fn find_all_subscriptions(&self, user_id: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        // 모든 구독을 병렬로 조회
        let product_subs = self.find_product_subscriptions(user_id).await.unwrap_or_default();
        
        let brand_response = self.client
            .from("brand_subscriptions")
            .select("*")
            .eq("user_id", user_id)
            .execute()
            .await?;
        
        let brand_subs: Vec<BrandSubscription> = if brand_response.status().is_success() {
            let text = brand_response.text().await?;
            serde_json::from_str(&text).unwrap_or_default()
        } else {
            Vec::new()
        };

        let shop_response = self.client
            .from("shop_subscriptions")
            .select("*")
            .eq("user_id", user_id)
            .execute()
            .await?;
        
        let shop_subs: Vec<ShopSubscription> = if shop_response.status().is_success() {
            let text = shop_response.text().await?;
            serde_json::from_str(&text).unwrap_or_default()
        } else {
            Vec::new()
        };

        Ok(serde_json::json!({
            "product_subscriptions": product_subs,
            "brand_subscriptions": brand_subs,
            "shop_subscriptions": shop_subs,
            "total_products": product_subs.len(),
            "total_brands": brand_subs.len(),
            "total_shops": shop_subs.len()
        }))
    }
}