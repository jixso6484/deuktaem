use postgrest::Postgrest;
use serde_json::Value;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;

use crate::domain::entities::user::{Notification, NotificationLog, NotificationSettings};
use crate::domain::dto::pagenation::{Pagenation, PagenationResult};

pub struct NotificationRepository {
    client: Postgrest,
    realtime_url: String,
    api_key: String,
}

impl NotificationRepository {
    pub fn new(client: Postgrest, realtime_url: String, api_key: String) -> Self {
        Self { 
            client,
            realtime_url,
            api_key,
        }
    }

    // 일반 CRUD 작업들
    pub async fn find_notifications(&self, user_id: &str, pagination: Pagenation) -> Result<PagenationResult<Notification>, Box<dyn std::error::Error>> {
        let offset = (pagination.page - 1) * pagination.limit;

        let response = self.client
            .from("notifications")
            .select("*")
            .eq("user_id", user_id)
            .order("created_at.desc")
            .range(offset as usize, (offset + pagination.limit - 1) as usize)
            .execute()
            .await?;

        let notifications: Vec<Notification> = if response.status().is_success() {
            let text = response.text().await?;
            serde_json::from_str(&text)?
        } else {
            Vec::new()
        };

        let count_response = self.client
            .from("notifications")
            .select("count")
            .eq("user_id", user_id)
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
            data: notifications,
            total,
            page: pagination.page,
            limit: pagination.limit,
            total_pages,
            has_next: pagination.page < total_pages,
            has_prev: pagination.page > 1,
        })
    }

    pub async fn find_unread_notifications(&self, user_id: &str) -> Result<Vec<Notification>, Box<dyn std::error::Error>> {
        let response = self.client
            .from("notifications")
            .select("*")
            .eq("user_id", user_id)
            .is("read_at", "null")
            .order("created_at.desc")
            .execute()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let notifications: Vec<Notification> = serde_json::from_str(&text)?;
            Ok(notifications)
        } else {
            Ok(Vec::new())
        }
    }

    pub async fn mark_notification_as_read(&self, notification_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.client
            .from("notifications")
            .eq("id", notification_id)
            .update(format!(r#"{{"read_at": "{}"}}"#, chrono::Utc::now().to_rfc3339()))
            .execute()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(format!("Failed to mark notification as read: {}", response.status()).into())
        }
    }

    pub async fn create_notification(&self, notification: Notification) -> Result<Notification, Box<dyn std::error::Error>> {
        let response = self.client
            .from("notifications")
            .insert(serde_json::to_string(&notification)?)
            .execute()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let created_notification: Notification = serde_json::from_str(&text)?;
            Ok(created_notification)
        } else {
            Err(format!("Failed to create notification: {}", response.status()).into())
        }
    }

    // 🔴 Realtime 기능들
    
    /// 사용자의 실시간 알림 구독
    pub async fn subscribe_to_user_notifications(
        &self, 
        user_id: &str,
        callback: impl Fn(Notification) + Send + 'static
    ) -> Result<(), Box<dyn std::error::Error>> {
        let ws_url = format!("{}/realtime/v1/websocket", self.realtime_url);
        
        let (ws_stream, _) = connect_async(&ws_url).await?;
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        // Supabase Realtime 인증 메시지
        let auth_message = serde_json::json!({
            "topic": "realtime:notifications",
            "event": "phx_join",
            "payload": {
                "config": {
                    "postgres_changes": [{
                        "event": "INSERT",
                        "schema": "public",
                        "table": "notifications",
                        "filter": format!("user_id=eq.{}", user_id)
                    }]
                }
            },
            "ref": "1"
        });

        ws_sender.send(Message::Text(auth_message.to_string())).await?;

        // 메시지 수신 루프
        tokio::spawn(async move {
            while let Some(message) = ws_receiver.next().await {
                match message {
                    Ok(Message::Text(text)) => {
                        if let Ok(realtime_msg) = serde_json::from_str::<Value>(&text) {
                            if let Some(event) = realtime_msg.get("event").and_then(|e| e.as_str()) {
                                if event == "postgres_changes" {
                                    if let Some(payload) = realtime_msg.get("payload") {
                                        if let Some(record) = payload.get("record") {
                                            if let Ok(notification) = serde_json::from_value::<Notification>(record.clone()) {
                                                callback(notification);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Ok(Message::Close(_)) => break,
                    Err(e) => {
                        eprintln!("WebSocket error: {}", e);
                        break;
                    }
                    _ => {}
                }
            }
        });

        Ok(())
    }

    /// 모든 활성 할인 정보 실시간 구독
    pub async fn subscribe_to_discount_updates(
        &self,
        callback: impl Fn(Value) + Send + 'static
    ) -> Result<(), Box<dyn std::error::Error>> {
        let ws_url = format!("{}/realtime/v1/websocket", self.realtime_url);
        
        let (ws_stream, _) = connect_async(&ws_url).await?;
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        let auth_message = serde_json::json!({
            "topic": "realtime:discount_info",
            "event": "phx_join",
            "payload": {
                "config": {
                    "postgres_changes": [{
                        "event": "*",
                        "schema": "public", 
                        "table": "discount_info"
                    }]
                }
            },
            "ref": "2"
        });

        ws_sender.send(Message::Text(auth_message.to_string())).await?;

        tokio::spawn(async move {
            while let Some(message) = ws_receiver.next().await {
                match message {
                    Ok(Message::Text(text)) => {
                        if let Ok(realtime_msg) = serde_json::from_str::<Value>(&text) {
                            if let Some(event) = realtime_msg.get("event").and_then(|e| e.as_str()) {
                                if event == "postgres_changes" {
                                    if let Some(payload) = realtime_msg.get("payload") {
                                        callback(payload.clone());
                                    }
                                }
                            }
                        }
                    }
                    Ok(Message::Close(_)) => break,
                    Err(e) => {
                        eprintln!("WebSocket error: {}", e);
                        break;
                    }
                    _ => {}
                }
            }
        });

        Ok(())
    }

    /// 사용자 구독 변경 실시간 감지
    pub async fn subscribe_to_subscription_changes(
        &self,
        user_id: &str,
        callback: impl Fn(String, Value) + Send + 'static // (table_name, record)
    ) -> Result<(), Box<dyn std::error::Error>> {
        let ws_url = format!("{}/realtime/v1/websocket", self.realtime_url);
        
        let (ws_stream, _) = connect_async(&ws_url).await?;
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        // 여러 구독 테이블 동시 구독
        let subscription_tables = vec!["shop_subscriptions", "brand_subscriptions", "category_subscriptions"];
        
        for (i, table) in subscription_tables.iter().enumerate() {
            let auth_message = serde_json::json!({
                "topic": format!("realtime:{}", table),
                "event": "phx_join",
                "payload": {
                    "config": {
                        "postgres_changes": [{
                            "event": "*",
                            "schema": "public",
                            "table": table,
                            "filter": format!("user_id=eq.{}", user_id)
                        }]
                    }
                },
                "ref": (i + 3).to_string()
            });

            ws_sender.send(Message::Text(auth_message.to_string())).await?;
        }

        tokio::spawn(async move {
            while let Some(message) = ws_receiver.next().await {
                match message {
                    Ok(Message::Text(text)) => {
                        if let Ok(realtime_msg) = serde_json::from_str::<Value>(&text) {
                            if let Some(event) = realtime_msg.get("event").and_then(|e| e.as_str()) {
                                if event == "postgres_changes" {
                                    if let Some(payload) = realtime_msg.get("payload") {
                                        if let Some(topic) = realtime_msg.get("topic").and_then(|t| t.as_str()) {
                                            let table_name = topic.replace("realtime:", "");
                                            callback(table_name, payload.clone());
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Ok(Message::Close(_)) => break,
                    Err(e) => {
                        eprintln!("WebSocket error: {}", e);
                        break;
                    }
                    _ => {}
                }
            }
        });

        Ok(())
    }

    // NotificationLog 관련 메소드들
    pub async fn create_notification_log(&self, log: NotificationLog) -> Result<NotificationLog, Box<dyn std::error::Error>> {
        let response = self.client
            .from("notification_logs")
            .insert(serde_json::to_string(&log)?)
            .execute()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let created_log: NotificationLog = serde_json::from_str(&text)?;
            Ok(created_log)
        } else {
            Err(format!("Failed to create notification log: {}", response.status()).into())
        }
    }

    pub async fn find_notification_logs(&self, user_id: &str, pagination: Pagenation) -> Result<PagenationResult<NotificationLog>, Box<dyn std::error::Error>> {
        let offset = (pagination.page - 1) * pagination.limit;

        let response = self.client
            .from("notification_logs")
            .select("*")
            .eq("user_id", user_id)
            .order("created_at.desc")
            .range(offset as usize, (offset + pagination.limit - 1) as usize)
            .execute()
            .await?;

        let logs: Vec<NotificationLog> = if response.status().is_success() {
            let text = response.text().await?;
            serde_json::from_str(&text)?
        } else {
            Vec::new()
        };

        let count_response = self.client
            .from("notification_logs")
            .select("count")
            .eq("user_id", user_id)
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
            data: logs,
            total,
            page: pagination.page,
            limit: pagination.limit,
            total_pages,
            has_next: pagination.page < total_pages,
            has_prev: pagination.page > 1,
        })
    }

    // 🔧 알림 설정 관리 메소드들
    
    /// 사용자 알림 설정 조회 (없으면 기본값 생성)
    pub async fn find_notification_settings(&self, user_id: &str) -> Result<NotificationSettings, Box<dyn std::error::Error>> {
        let response = self.client
            .from("notification_settings")
            .select("*")
            .eq("user_id", user_id)
            .single()
            .execute()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let settings: NotificationSettings = serde_json::from_str(&text)?;
            Ok(settings)
        } else {
            // 설정이 없으면 기본값으로 생성
            let default_settings = NotificationSettings {
                user_id: user_id.to_string(),
                push_enabled: true,
                discount_notifications: true,
                shop_notifications: true,
                brand_notifications: true,
                category_notifications: true,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };
            
            self.create_notification_settings(default_settings.clone()).await?;
            Ok(default_settings)
        }
    }

    /// 알림 설정 생성
    pub async fn create_notification_settings(&self, settings: NotificationSettings) -> Result<NotificationSettings, Box<dyn std::error::Error>> {
        let response = self.client
            .from("notification_settings")
            .insert(serde_json::to_string(&settings)?)
            .execute()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let created_settings: NotificationSettings = serde_json::from_str(&text)?;
            Ok(created_settings)
        } else {
            Err(format!("Failed to create notification settings: {}", response.status()).into())
        }
    }

    /// 알림 설정 업데이트
    pub async fn update_notification_settings(&self, user_id: &str, settings: NotificationSettings) -> Result<NotificationSettings, Box<dyn std::error::Error>> {
        let mut updated_settings = settings;
        updated_settings.updated_at = chrono::Utc::now();

        let response = self.client
            .from("notification_settings")
            .eq("user_id", user_id)
            .update(serde_json::to_string(&updated_settings)?)
            .execute()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let result: NotificationSettings = serde_json::from_str(&text)?;
            Ok(result)
        } else {
            Err(format!("Failed to update notification settings: {}", response.status()).into())
        }
    }

    /// 특정 알림 타입 토글
    pub async fn toggle_notification_type(&self, user_id: &str, notification_type: &str, enabled: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut settings = self.find_notification_settings(user_id).await?;

        match notification_type {
            "push" => settings.push_enabled = enabled,
            "discount" => settings.discount_notifications = enabled,
            "shop" => settings.shop_notifications = enabled,
            "brand" => settings.brand_notifications = enabled,
            "category" => settings.category_notifications = enabled,
            _ => return Err("Invalid notification type".into()),
        }

        self.update_notification_settings(user_id, settings).await?;
        Ok(())
    }

    /// 사용자가 특정 알림을 받을 수 있는지 확인
    pub async fn can_receive_notification(&self, user_id: &str, notification_type: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let settings = self.find_notification_settings(user_id).await?;

        let can_receive = match notification_type {
            "discount" => settings.push_enabled && settings.discount_notifications,
            "shop" => settings.push_enabled && settings.shop_notifications,
            "brand" => settings.push_enabled && settings.brand_notifications,
            "category" => settings.push_enabled && settings.category_notifications,
            _ => false,
        };

        Ok(can_receive)
    }

    /// 🔴 설정을 고려한 실시간 알림 생성 (내부 헬퍼)
    pub async fn create_filtered_notification(&self, notification: Notification) -> Result<Option<Notification>, Box<dyn std::error::Error>> {
        // 알림 타입에 따른 설정 확인
        let notification_type = match notification.r#type.as_str() {
            "discount_update" => "discount",
            "shop_subscription" => "shop", 
            "brand_subscription" => "brand",
            "category_subscription" => "category",
            _ => &notification.r#type
        };

        // 사용자 설정 확인
        if self.can_receive_notification(&notification.user_id, notification_type).await? {
            let created = self.create_notification(notification).await?;
            Ok(Some(created))
        } else {
            // 설정에 의해 필터링됨
            Ok(None)
        }
    }
}