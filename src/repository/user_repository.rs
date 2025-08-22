use postgrest::Postgrest;
use serde_json::Value;

use crate::domain::entities::user::{
    Profile, ShopSubscription, BrandSubscription, CategorySubscription
};
use crate::domain::dto::pagenation::{Pagenation, PagenationResult};

pub struct UserRepository {
    client: Postgrest,
}

impl UserRepository {
    pub fn new(client: Postgrest) -> Self {
        Self { client }
    }

    // 인증된 사용자 토큰으로 클라이언트 생성
    pub fn with_user_token(base_client: Postgrest, user_token: &str) -> Self {
        let client = base_client
            .insert_header("Authorization", format!("Bearer {}", user_token));
        
        Self { client }
    }

    // Profile CRUD
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

    pub async fn create_profile(&self, profile: Profile) -> Result<Profile, Box<dyn std::error::Error>> {
        let response = self.client
            .from("profiles")
            .insert(serde_json::to_string(&profile)?)
            .execute()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let created_profile: Profile = serde_json::from_str(&text)?;
            Ok(created_profile)
        } else {
            Err(format!("Failed to create profile: {}", response.status()).into())
        }
    }

    pub async fn update_profile(&self, user_id: &str, profile: Profile) -> Result<Profile, Box<dyn std::error::Error>> {
        let response = self.client
            .from("profiles")
            .eq("user_id", user_id)
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

    // Shop Subscription CRUD
    pub async fn find_shop_subscriptions(&self, user_id: &str) -> Result<Vec<ShopSubscription>, Box<dyn std::error::Error>> {
        let response = self.client
            .from("shop_subscriptions")
            .select("*")
            .eq("user_id", user_id)
            .order("created_at.desc")
            .execute()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let subscriptions: Vec<ShopSubscription> = serde_json::from_str(&text)?;
            Ok(subscriptions)
        } else {
            Ok(Vec::new())
        }
    }

    pub async fn create_shop_subscription(&self, subscription: ShopSubscription) -> Result<ShopSubscription, Box<dyn std::error::Error>> {
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
            Err(format!("Failed to create shop subscription: {}", response.status()).into())
        }
    }

    pub async fn delete_shop_subscription(&self, user_id: &str, shop_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.client
            .from("shop_subscriptions")
            .eq("user_id", user_id)
            .eq("shop_id", shop_id)
            .delete()
            .execute()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(format!("Failed to delete shop subscription: {}", response.status()).into())
        }
    }

    // Brand Subscription CRUD
    pub async fn find_brand_subscriptions(&self, user_id: &str) -> Result<Vec<BrandSubscription>, Box<dyn std::error::Error>> {
        let response = self.client
            .from("brand_subscriptions")
            .select("*")
            .eq("user_id", user_id)
            .order("created_at.desc")
            .execute()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let subscriptions: Vec<BrandSubscription> = serde_json::from_str(&text)?;
            Ok(subscriptions)
        } else {
            Ok(Vec::new())
        }
    }

    pub async fn create_brand_subscription(&self, subscription: BrandSubscription) -> Result<BrandSubscription, Box<dyn std::error::Error>> {
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
            Err(format!("Failed to create brand subscription: {}", response.status()).into())
        }
    }

    // Category Subscription CRUD
    pub async fn find_category_subscriptions(&self, user_id: &str) -> Result<Vec<CategorySubscription>, Box<dyn std::error::Error>> {
        let response = self.client
            .from("category_subscriptions")
            .select("*")
            .eq("user_id", user_id)
            .order("created_at.desc")
            .execute()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let subscriptions: Vec<CategorySubscription> = serde_json::from_str(&text)?;
            Ok(subscriptions)
        } else {
            Ok(Vec::new())
        }
    }

}