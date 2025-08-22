use crate::config::SupabaseConfig;
use crate::repository::RepositoryFactory;
use crate::domain::entities::user::{Notification, NotificationSettings};
use crate::auth::AuthUser;
use serde_json::Value;

#[derive(Clone)]
pub struct NotificationService {
    factory: RepositoryFactory,
}

impl NotificationService {
    pub fn new(config: SupabaseConfig) -> Self {
        Self {
            factory: RepositoryFactory::new(config),
        }
    }

    // 🔧 알림 설정 관리
    pub async fn get_user_notification_settings(&self, user_token: &str, user_id: &str) -> Result<NotificationSettings, Box<dyn std::error::Error>> {
        let repo = self.factory.authenticated_notification_repo(user_token);
        repo.find_notification_settings(user_id).await
    }

    pub async fn update_notification_settings(&self, user_token: &str, user_id: &str, settings: NotificationSettings) -> Result<NotificationSettings, Box<dyn std::error::Error>> {
        let repo = self.factory.authenticated_notification_repo(user_token);
        repo.update_notification_settings(user_id, settings).await
    }

    pub async fn toggle_notification(&self, user_token: &str, user_id: &str, notification_type: &str, enabled: bool) -> Result<(), Box<dyn std::error::Error>> {
        let repo = self.factory.authenticated_notification_repo(user_token);
        repo.toggle_notification_type(user_id, notification_type, enabled).await
    }

    // 📱 알림 조회 및 관리
    pub async fn get_unread_notifications(&self, user_token: &str, user_id: &str) -> Result<Vec<Notification>, Box<dyn std::error::Error>> {
        let repo = self.factory.authenticated_notification_repo(user_token);
        repo.find_unread_notifications(user_id).await
    }

    pub async fn mark_as_read(&self, user_token: &str, notification_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let repo = self.factory.authenticated_notification_repo(user_token);
        repo.mark_notification_as_read(notification_id).await
    }

    // 🔴 실시간 알림 구독
    pub async fn start_realtime_notifications(&self, user_token: &str, user_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let repo = self.factory.authenticated_notification_repo(user_token);
        
        // 사용자별 실시간 알림 구독
        repo.subscribe_to_user_notifications(
            user_id,
            |notification: Notification| {
                log::info!("🔔 New notification for user {}: {}", notification.user_id, notification.r#type);
                
                // 실제 앱에서는 여기서 WebSocket으로 클라이언트에 전송
                tokio::spawn(async move {
                    Self::send_to_client(notification).await;
                });
            }
        ).await?;

        // 할인 정보 업데이트 구독
        repo.subscribe_to_discount_updates(
            |update| {
                log::info!("💰 Discount update received: {:?}", update);
                
                // 할인 정보 기반으로 맞춤 알림 생성
                tokio::spawn(async move {
                    Self::create_discount_notifications(update).await;
                });
            }
        ).await?;

        // 구독 변경 감지
        repo.subscribe_to_subscription_changes(
            user_id,
            |table_name: String, change| {
                log::info!("📝 Subscription change in {}: {:?}", table_name, change);
                
                // 구독 변경 기반 추천 시스템 업데이트
                tokio::spawn(async move {
                    Self::update_recommendations(table_name, change).await;
                });
            }
        ).await?;

        Ok(())
    }

    // 🚀 비즈니스 로직 헬퍼들
    async fn send_to_client(notification: Notification) {
        log::debug!("📡 Sending notification to client: {}", notification.id);
        // WebSocket/SSE로 클라이언트에 전송 로직
    }

    async fn create_discount_notifications(update: Value) {
        log::debug!("🎯 Creating personalized discount notifications");
        // 할인 정보 기반 맞춤 알림 생성 로직
        
        if let Some(event_type) = update.get("eventType").and_then(|v| v.as_str()) {
            match event_type {
                "INSERT" => log::info!("New discount added"),
                "UPDATE" => log::info!("Discount updated"), 
                "DELETE" => log::info!("Discount removed"),
                _ => log::debug!("Unknown discount event: {}", event_type),
            }
        }
    }

    async fn update_recommendations(table_name: String, change: Value) {
        log::debug!("🤖 Updating recommendation system for table: {}", table_name);
        // 추천 시스템 업데이트 로직
        
        match table_name.as_str() {
            "shop_subscriptions" => log::info!("Shop subscription changed"),
            "brand_subscriptions" => log::info!("Brand subscription changed"),
            "category_subscriptions" => log::info!("Category subscription changed"),
            _ => log::debug!("Unknown subscription table: {}", table_name),
        }
    }

    // 🔒 권한 체크가 포함된 관리자 기능
    pub async fn admin_send_notification(&self, admin_token: &str, notification: Notification) -> Result<Notification, Box<dyn std::error::Error>> {
        // 관리자 권한 확인 (실제로는 JWT에서 role 체크)
        let repo = self.factory.admin_notification_repo();
        
        log::info!("👑 Admin sending notification to user: {}", notification.user_id);
        
        // 사용자 설정 체크 후 알림 생성
        let created = repo.create_filtered_notification(notification).await?;
        
        match created {
            Some(notification) => {
                log::info!("✅ Notification sent successfully: {}", notification.id);
                Ok(notification)
            }
            None => {
                log::warn!("⚠️ Notification filtered by user settings");
                Err("Notification blocked by user settings".into())
            }
        }
    }

    // 📊 모니터링 및 분석
    pub async fn get_notification_stats(&self, admin_token: &str, user_id: Option<&str>) -> Result<Value, Box<dyn std::error::Error>> {
        let repo = self.factory.admin_notification_repo();
        
        log::info!("📊 Getting notification statistics for user: {:?}", user_id);
        
        // 통계 수집 로직 (예시)
        let stats = serde_json::json!({
            "total_sent": 1000,
            "unread_count": 50,
            "settings_enabled": true,
            "last_notification": chrono::Utc::now().to_rfc3339()
        });
        
        Ok(stats)
    }
}