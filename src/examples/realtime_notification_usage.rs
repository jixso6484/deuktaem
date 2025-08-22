use crate::config::SupabaseConfig;
use crate::repository::RepositoryFactory;
use crate::domain::dto::pagenation::Pagenation;
use crate::domain::entities::user::Notification;

// 🔴 Supabase Realtime 알림 시스템 사용 예시
pub async fn realtime_notification_examples() -> Result<(), Box<dyn std::error::Error>> {
    let config = SupabaseConfig::new()?;
    let factory = RepositoryFactory::new(config);

    println!("🔴 Supabase Realtime 알림 시스템 시작...\n");

    // 1. 인증된 사용자의 알림 Repository 생성
    let user_token = "user_jwt_token_here"; // 실제 JWT 토큰
    let user_id = "user_uuid_here";

    let notification_repo = factory.authenticated_notification_repo(user_token);

    // 2. 기본 알림 CRUD 작업
    println!("📱 기본 알림 작업:");
    
    // 읽지 않은 알림 조회
    let unread_notifications = notification_repo.find_unread_notifications(user_id).await?;
    println!("  - 읽지 않은 알림: {}개", unread_notifications.len());

    // 페이지네이션으로 알림 목록 조회
    let pagination = Pagenation { page: 1, limit: 10 };
    let notifications_page = notification_repo.find_notifications(user_id, pagination).await?;
    println!("  - 알림 목록: {}개 (총 {}개)", 
        notifications_page.data.len(), 
        notifications_page.total
    );

    // 3. 🔴 실시간 알림 구독
    println!("\n🔴 실시간 알림 구독 시작:");
    
    // 사용자별 실시간 알림 구독
    notification_repo.subscribe_to_user_notifications(
        user_id,
        |notification: Notification| {
            println!("🔔 새 알림 도착!");
            println!("  타입: {}", notification.r#type);
            println!("  생성일: {}", notification.created_at);
            
            // 실제 앱에서는 여기서 UI 업데이트, 푸시 알림 등을 처리
            // notify_frontend(notification);
            // send_push_notification(notification);
        }
    ).await?;

    // 4. 🔴 할인 정보 실시간 구독
    println!("\n💰 실시간 할인 정보 구독:");
    
    notification_repo.subscribe_to_discount_updates(
        |discount_update| {
            println!("💰 할인 정보 업데이트!");
            
            if let Some(event_type) = discount_update.get("eventType").and_then(|v| v.as_str()) {
                match event_type {
                    "INSERT" => println!("  새로운 할인 정보 추가됨"),
                    "UPDATE" => println!("  할인 정보 업데이트됨"),
                    "DELETE" => println!("  할인 정보 삭제됨"),
                    _ => println!("  할인 정보 변경: {}", event_type),
                }
            }

            // 실제 앱에서는 사용자별 맞춤 알림 생성
            // create_personalized_notifications(discount_update);
        }
    ).await?;

    // 5. 🔴 구독 변경 실시간 감지
    println!("\n👥 사용자 구독 변경 감지:");
    
    notification_repo.subscribe_to_subscription_changes(
        user_id,
        |table_name: String, change| {
            println!("📝 구독 변경 감지: {}", table_name);
            
            if let Some(event_type) = change.get("eventType").and_then(|v| v.as_str()) {
                match event_type {
                    "INSERT" => println!("  새로운 구독 추가됨"),
                    "DELETE" => println!("  구독 취소됨"),
                    _ => println!("  구독 변경: {}", event_type),
                }
            }

            // 실제 앱에서는 추천 시스템 업데이트 등
            // update_recommendation_system(table_name, change);
        }
    ).await?;

    println!("\n✅ 실시간 구독 설정 완료!");
    println!("📡 WebSocket 연결이 활성화되어 실시간 업데이트를 수신합니다.");

    // 실제 앱에서는 이 지점에서 서버가 계속 실행되면서 실시간 이벤트를 처리
    // tokio::time::sleep(Duration::from_secs(60)).await; // 1분간 대기

    Ok(())
}

// 🔧 실제 앱에서 사용할 헬퍼 함수들

/// 프론트엔드에 실시간 알림 전송
pub async fn notify_frontend(notification: Notification) {
    // WebSocket이나 Server-Sent Events로 프론트엔드에 전송
    println!("📡 프론트엔드로 알림 전송: {}", notification.id);
}

/// 모바일 푸시 알림 전송
pub async fn send_push_notification(notification: Notification) {
    // FCM, APNS 등으로 푸시 알림 전송
    println!("📱 푸시 알림 전송: {}", notification.r#type);
}

/// 맞춤형 알림 생성
pub async fn create_personalized_notifications(discount_update: serde_json::Value) {
    // 사용자의 구독 정보를 기반으로 개인화된 알림 생성
    println!("🎯 맞춤형 알림 생성 중...");
    
    // 예시 로직:
    // 1. 할인 상품의 카테고리/브랜드 확인
    // 2. 해당 카테고리/브랜드를 구독한 사용자 조회
    // 3. 각 사용자에게 맞춤형 알림 생성
}

/// 추천 시스템 업데이트
pub async fn update_recommendation_system(table_name: String, change: serde_json::Value) {
    // 사용자의 구독 변경에 따라 추천 알고리즘 업데이트
    println!("🤖 추천 시스템 업데이트: {}", table_name);
}

// 🎯 실제 사용 예시 - API 핸들러에서

/// WebSocket 엔드포인트 예시
pub async fn websocket_handler(user_token: String) -> Result<(), Box<dyn std::error::Error>> {
    let config = SupabaseConfig::new()?;
    let factory = RepositoryFactory::new(config);
    
    // 사용자별 실시간 알림 채널 구독
    let notification_repo = factory.authenticated_notification_repo(&user_token);
    
    // 사용자 ID 추출 (JWT 토큰에서)
    let user_id = extract_user_id_from_token(&user_token)?;
    
    // 실시간 구독 시작
    notification_repo.subscribe_to_user_notifications(
        &user_id,
        |notification| {
            // WebSocket으로 클라이언트에 즉시 전송
            tokio::spawn(async move {
                notify_frontend(notification).await;
            });
        }
    ).await?;

    Ok(())
}

/// JWT 토큰에서 사용자 ID 추출 (예시)
fn extract_user_id_from_token(token: &str) -> Result<String, Box<dyn std::error::Error>> {
    // 실제로는 JWT 라이브러리를 사용해서 토큰 검증 및 파싱
    Ok("user-uuid-from-jwt".to_string())
}

// 📊 모니터링 및 분석

/// 실시간 이벤트 모니터링
pub async fn monitor_realtime_events() -> Result<(), Box<dyn std::error::Error>> {
    let config = SupabaseConfig::new()?;
    let factory = RepositoryFactory::new(config);
    
    // 관리자용 알림 Repository로 모든 이벤트 모니터링
    let admin_notification_repo = factory.admin_notification_repo();
    
    // 모든 할인 정보 변경 모니터링
    admin_notification_repo.subscribe_to_discount_updates(
        |update| {
            // 분석용 로그 수집
            println!("📊 할인 이벤트 로깅: {:?}", update);
            
            // 메트릭 수집, 알람 시스템 등
            // collect_metrics(update);
            // check_business_rules(update);
        }
    ).await?;

    Ok(())
}