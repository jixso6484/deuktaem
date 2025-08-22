use redis::{Client, Connection, Commands, RedisResult};
use serde::{Serialize, Deserialize};
use std::env;
use dotenv::dotenv;
use tracing::{info, warn, error};

#[derive(Clone)]
pub struct RedisCache {
    client: Client,
}

impl RedisCache {
    pub fn new() -> Result<Self, redis::RedisError> {
        dotenv().ok();
        
        // Upstash Redis는 REST API를 제공하므로 rediss:// 프로토콜 사용
        let redis_url = env::var("REDIS_URL")
            .or_else(|_| {
                // Upstash Redis URL이 있으면 Redis 프로토콜로 변환
                if let Ok(upstash_url) = env::var("UPSTASH_REDIS_REST_URL") {
                    if let Ok(token) = env::var("UPSTASH_REDIS_REST_TOKEN") {
                        // Upstash Redis는 일반적으로 rediss://를 사용
                        // 하지만 여기서는 간단한 로컬 Redis로 폴백
                        warn!("⚠️ Upstash Redis 설정 감지됨. 로컬 Redis를 사용합니다.");
                        return Ok("redis://127.0.0.1:6379".to_string());
                    }
                }
                Err(std::env::VarError::NotPresent)
            })
            .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
            
        info!("🔗 Redis 연결 시도: {}", &redis_url);
        let client = Client::open(redis_url)?;
        
        info!("🔄 Redis 클라이언트 초기화 완료");
        Ok(Self { client })
    }

    pub async fn get_connection(&self) -> Result<Connection, redis::RedisError> {
        self.client.get_connection()
    }

    // JSON 객체를 캐시에 저장
    pub async fn set_json<T>(&self, key: &str, value: &T, ttl_seconds: u64) -> RedisResult<()>
    where
        T: Serialize,
    {
        let mut conn = self.get_connection().await?;
        let serialized = serde_json::to_string(value)
            .map_err(|e| redis::RedisError::from((redis::ErrorKind::TypeError, "직렬화 실패", e.to_string())))?;
        
        let _: () = conn.set_ex(key, serialized, ttl_seconds as u64)?;
        info!("💾 캐시 저장: {} (TTL: {}초)", key, ttl_seconds);
        Ok(())
    }

    // JSON 객체를 캐시에서 가져오기
    pub async fn get_json<T>(&self, key: &str) -> RedisResult<Option<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut conn = self.get_connection().await?;
        let data: Option<String> = conn.get(key)?;
        
        match data {
            Some(json_str) => {
                match serde_json::from_str(&json_str) {
                    Ok(value) => {
                        info!("🎯 캐시 히트: {}", key);
                        Ok(Some(value))
                    }
                    Err(e) => {
                        warn!("⚠️ 캐시 역직렬화 실패 {}: {}", key, e);
                        Ok(None)
                    }
                }
            }
            None => {
                info!("📭 캐시 미스: {}", key);
                Ok(None)
            }
        }
    }

    // 문자열 값 저장
    pub async fn set_string(&self, key: &str, value: &str, ttl_seconds: u64) -> RedisResult<()> {
        let mut conn = self.get_connection().await?;
        let _: () = conn.set_ex(key, value, ttl_seconds as u64)?;
        info!("💾 캐시 저장 (문자열): {} (TTL: {}초)", key, ttl_seconds);
        Ok(())
    }

    // 문자열 값 가져오기
    pub async fn get_string(&self, key: &str) -> RedisResult<Option<String>> {
        let mut conn = self.get_connection().await?;
        let result: Option<String> = conn.get(key)?;
        
        if result.is_some() {
            info!("🎯 캐시 히트 (문자열): {}", key);
        } else {
            info!("📭 캐시 미스 (문자열): {}", key);
        }
        
        Ok(result)
    }

    // 키 삭제
    pub async fn delete(&self, key: &str) -> RedisResult<bool> {
        let mut conn = self.get_connection().await?;
        let result: i32 = conn.del(key)?;
        
        if result > 0 {
            info!("🗑️ 캐시 삭제: {}", key);
            Ok(true)
        } else {
            warn!("❌ 캐시 삭제 실패 (키 없음): {}", key);
            Ok(false)
        }
    }

    // 패턴으로 키 삭제
    pub async fn delete_pattern(&self, pattern: &str) -> RedisResult<u64> {
        let mut conn = self.get_connection().await?;
        let keys: Vec<String> = conn.keys(pattern)?;
        
        if keys.is_empty() {
            info!("🔍 패턴에 맞는 키 없음: {}", pattern);
            return Ok(0);
        }

        let deleted_count = keys.len() as u64;
        let _: () = conn.del(&keys)?;
        
        info!("🗑️ 패턴 캐시 삭제: {} ({}개)", pattern, deleted_count);
        Ok(deleted_count)
    }

    // 키 존재 확인
    pub async fn exists(&self, key: &str) -> RedisResult<bool> {
        let mut conn = self.get_connection().await?;
        let exists: i32 = conn.exists(key)?;
        Ok(exists > 0)
    }

    // TTL 확인
    pub async fn ttl(&self, key: &str) -> RedisResult<i64> {
        let mut conn = self.get_connection().await?;
        conn.ttl(key)
    }

    // 리스트에 추가 (최신 N개 유지)
    pub async fn push_to_list(&self, key: &str, value: &str, max_size: usize, ttl_seconds: u64) -> RedisResult<()> {
        let mut conn = self.get_connection().await?;
        
        // 리스트 앞쪽에 추가
        let _: () = conn.lpush(key, value)?;
        
        // 최대 크기 유지
        let _: () = conn.ltrim(key, 0, (max_size - 1) as isize)?;
        
        // TTL 설정
        let _: () = conn.expire(key, ttl_seconds as i64)?;
        
        info!("📝 리스트에 추가: {} (최대: {}개)", key, max_size);
        Ok(())
    }

    // 리스트 전체 가져오기
    pub async fn get_list(&self, key: &str) -> RedisResult<Vec<String>> {
        let mut conn = self.get_connection().await?;
        conn.lrange(key, 0, -1)
    }

    // 카운터 증가
    pub async fn increment_counter(&self, key: &str, ttl_seconds: u64) -> RedisResult<i64> {
        let mut conn = self.get_connection().await?;
        let count: i64 = conn.incr(key, 1)?;
        
        // TTL은 처음 증가할 때만 설정
        if count == 1 {
            let _: () = conn.expire(key, ttl_seconds as i64)?;
        }
        
        Ok(count)
    }

    // 연결 테스트
    pub async fn test_connection(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = self.get_connection().await?;
        let _: String = redis::cmd("PING").query(&mut conn)?;
        info!("✅ Redis 연결 테스트 성공!");
        Ok(())
    }
}

// 캐시 키 생성 헬퍼 함수들
pub fn cache_key_user(user_id: &str) -> String {
    format!("user:{}", user_id)
}

pub fn cache_key_shop(shop_id: &str) -> String {
    format!("shop:{}", shop_id)
}

pub fn cache_key_product(product_id: &str) -> String {
    format!("product:{}", product_id)
}

pub fn cache_key_discount(discount_id: &str) -> String {
    format!("discount:{}", discount_id)
}

pub fn cache_key_search(query: &str, page: u32, limit: u32) -> String {
    format!("search:{}:{}:{}", query, page, limit)
}

pub fn cache_key_popular_shops(country: &str, limit: u32) -> String {
    format!("popular:shops:{}:{}", country, limit)
}

pub fn cache_key_popular_products(category: &str, limit: u32) -> String {
    format!("popular:products:{}:{}", category, limit)
}

pub fn cache_key_user_notifications(user_id: &str) -> String {
    format!("notifications:{}", user_id)
}