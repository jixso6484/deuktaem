use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

// 간단한 인메모리 캐시 구현
#[derive(Clone)]
pub struct SimpleCache {
    data: Arc<RwLock<HashMap<String, CacheEntry>>>,
}

#[derive(Clone)]
struct CacheEntry {
    value: String,
    expires_at: Instant,
}

impl SimpleCache {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn set<T: Serialize>(&self, key: &str, value: &T, ttl: Duration) -> Result<(), Box<dyn std::error::Error>> {
        let serialized = serde_json::to_string(value)?;
        let entry = CacheEntry {
            value: serialized,
            expires_at: Instant::now() + ttl,
        };

        let mut data = self.data.write().unwrap();
        data.insert(key.to_string(), entry);
        Ok(())
    }

    pub fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Option<T> {
        let data = self.data.read().unwrap();
        
        if let Some(entry) = data.get(key) {
            if entry.expires_at > Instant::now() {
                if let Ok(value) = serde_json::from_str(&entry.value) {
                    return Some(value);
                }
            }
        }
        None
    }

    pub fn delete(&self, key: &str) {
        let mut data = self.data.write().unwrap();
        data.remove(key);
    }

    pub fn clear(&self) {
        let mut data = self.data.write().unwrap();
        data.clear();
    }

    pub fn size(&self) -> usize {
        let data = self.data.read().unwrap();
        data.len()
    }
}

// 캐시 키 생성 헬퍼들
pub fn cache_key_product(product_id: i64) -> String {
    format!("product:{}", product_id)
}

pub fn cache_key_products_by_country(country: &str, page: u32, limit: u32) -> String {
    format!("products:country:{}:{}:{}", country, page, limit)
}

pub fn cache_key_popular_products(page: u32, limit: u32) -> String {
    format!("products:popular:{}:{}", page, limit)
}

pub fn cache_key_shop(shop_id: i64) -> String {
    format!("shop:{}", shop_id)
}

pub fn cache_key_discount(discount_id: i64) -> String {
    format!("discount:{}", discount_id)
}

pub fn cache_key_user_subscriptions(user_id: &str) -> String {
    format!("subscriptions:{}", user_id)
}