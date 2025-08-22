use crate::config::SupabaseConfig;
use crate::repository::RepositoryFactory;
use crate::domain::dto::response::Pagenation;

// RLS 권한별 사용 예시
pub async fn rls_usage_examples() -> Result<(), Box<dyn std::error::Error>> {
    let config = SupabaseConfig::new()?;
    let factory = RepositoryFactory::new(config);

    // 1. 📖 공개 읽기 (모든 사용자 가능)
    println!("📖 공개 데이터 조회 예시:");
    
    let public_repos = factory.for_public_access();
    let pagination = Pagenation { page: 1, limit: 10 };

    // 상점 목록 조회 (누구나 가능)
    let shops = public_repos.shop.find_shops_paginated(pagination.clone()).await?;
    println!("  ✅ 상점 {}개 조회됨", shops.data.len());

    // 할인 정보 조회 (누구나 가능)  
    let discounts = public_repos.discount.find_all_paginated(pagination.clone()).await?;
    println!("  ✅ 할인 정보 {}개 조회됨", discounts.data.len());

    // 2. 🔐 인증된 사용자 작업 (본인 데이터만)
    println!("\n🔐 인증된 사용자 작업 예시:");
    
    let user_token = "user_jwt_token_here"; // 실제로는 JWT 토큰
    let auth_repos = factory.for_authenticated_user(user_token);

    // 프로필 조회 (본인만 가능)
    if let Some(profile) = auth_repos.user.find_profile_by_user_id("user_id").await? {
        println!("  ✅ 프로필 조회: {}", profile.email);
    }

    // 구독 목록 조회 (본인만 가능)
    let subscriptions = auth_repos.user.find_shop_subscriptions("user_id").await?;
    println!("  ✅ 구독 {}개 조회됨", subscriptions.len());

    // 상점 팔로우 (인증된 사용자만 가능)
    // let new_subscription = ShopSubscription { 
    //     id: "new_id".to_string(),
    //     user_id: "user_id".to_string(), 
    //     shop_id: "shop_id".to_string(),
    //     created_at: chrono::Utc::now(),
    //     updated_at: chrono::Utc::now(),
    // };
    // auth_repos.user.create_shop_subscription(new_subscription).await?;
    println!("  ✅ 상점 팔로우 가능");

    // 3. 👑 관리자 작업 (모든 데이터)
    println!("\n👑 관리자 작업 예시:");
    
    let admin_repos = factory.for_admin();

    // 모든 사용자 프로필 조회 (관리자만 가능)
    // let all_profiles = admin_repos.user.find_all_profiles(pagination).await?;
    // println!("  ✅ 전체 프로필 {}개 관리", all_profiles.data.len());

    // 상점 생성 (관리자만 가능)
    // let new_shop = Shop { ... };
    // admin_repos.shop.create_shop(new_shop).await?;
    println!("  ✅ 상점 생성/수정 가능");

    // API 메트릭 조회 (관리자만 가능)
    let metrics = admin_repos.translation.find_api_metrics(pagination).await?;
    println!("  ✅ API 메트릭 {}개 조회됨", metrics.data.len());

    Ok(())
}

// HTTP 요청에서 사용자 토큰 추출
pub fn extract_user_token(auth_header: Option<&str>) -> Option<String> {
    auth_header
        .and_then(|header| header.strip_prefix("Bearer "))
        .map(|token| token.to_string())
}

// 권한 확인 미들웨어 예시
pub fn check_permissions(user_token: Option<&str>, required_auth: bool) -> Result<(), &'static str> {
    match (user_token, required_auth) {
        (None, true) => Err("인증이 필요합니다"),
        (Some(_), _) => Ok(()),
        (None, false) => Ok(()),
    }
}

// 실제 API 핸들러에서 사용 예시
pub async fn example_api_handler(
    auth_header: Option<String>
) -> Result<String, Box<dyn std::error::Error>> {
    let config = SupabaseConfig::new()?;
    let factory = RepositoryFactory::new(config);

    match extract_user_token(auth_header.as_deref()) {
        // 인증된 사용자
        Some(token) => {
            let repos = factory.for_authenticated_user(&token);
            
            // 본인 알림 조회
            let notifications = repos.user.find_notifications(
                "user_id", 
                Pagenation { page: 1, limit: 20 }
            ).await?;
            
            Ok(format!("알림 {}개", notifications.data.len()))
        }
        
        // 비인증 사용자 (공개 데이터만)
        None => {
            let repos = factory.for_public_access();
            
            // 공개 상점 목록
            let shops = repos.shop.find_shops_paginated(
                Pagenation { page: 1, limit: 10 }
            ).await?;
            
            Ok(format!("공개 상점 {}개", shops.data.len()))
        }
    }
}