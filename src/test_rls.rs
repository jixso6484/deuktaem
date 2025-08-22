use postgrest::Postgrest;
use std::env;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    
    let supabase_url = env::var("SUPABASE_URL")?;
    let service_key = env::var("SUPABASE_SERVICE_KEY")?;
    let anon_key = env::var("SUPABASE_ANON_KEY")?;
    
    println!("🔍 RLS 정책 확인 시작...\n");

    // Service Key로 테스트 (모든 권한)
    let admin_client = Postgrest::new(&supabase_url)
        .insert_header("apikey", &service_key)
        .insert_header("Authorization", format!("Bearer {}", service_key));

    // Anon Key로 테스트 (제한된 권한)
    let anon_client = Postgrest::new(&supabase_url)
        .insert_header("apikey", &anon_key)
        .insert_header("Authorization", format!("Bearer {}", anon_key));

    // 1. 테이블 읽기 권한 테스트
    println!("📖 읽기 권한 테스트:");
    
    // 상점 목록 조회 (모든 사용자 가능해야 함)
    let shops_response = anon_client
        .from("shops")
        .select("*")
        .limit(1)
        .execute()
        .await?;
    
    println!("  - 상점 목록 조회: {}", shops_response.status());
    
    // 브랜드 목록 조회
    let brands_response = anon_client
        .from("brands")
        .select("*")
        .limit(1)
        .execute()
        .await?;
    
    println!("  - 브랜드 목록 조회: {}", brands_response.status());
    
    // 카테고리 목록 조회
    let categories_response = anon_client
        .from("categories")
        .select("*")
        .limit(1)
        .execute()
        .await?;
    
    println!("  - 카테고리 목록 조회: {}", categories_response.status());
    
    // 할인 정보 조회
    let discounts_response = anon_client
        .from("discount_info")
        .select("*")
        .limit(1)
        .execute()
        .await?;
    
    println!("  - 할인 정보 조회: {}", discounts_response.status());

    // 2. 프로필 수정 권한 테스트 (인증된 사용자만)
    println!("\n👤 사용자 프로필 수정 테스트:");
    
    // 인증된 사용자 토큰으로 테스트해야 하지만, 
    // 여기서는 anon으로 시도해서 권한 오류 확인
    let profile_update_response = anon_client
        .from("profiles")
        .eq("user_id", "test-user-id")
        .update(json!({"preferred_country": "KR"}).to_string())
        .execute()
        .await?;
    
    println!("  - 프로필 수정 (비인증): {}", profile_update_response.status());

    // 3. 구독 관리 권한 테스트
    println!("\n🔔 구독 관리 테스트:");
    
    // 상점 구독 추가 시도
    let subscription_response = anon_client
        .from("shop_subscriptions")
        .insert(json!({
            "user_id": "test-user-id",
            "shop_id": "test-shop-id"
        }).to_string())
        .execute()
        .await?;
    
    println!("  - 상점 구독 추가 (비인증): {}", subscription_response.status());

    // 4. Admin 권한으로 RLS 정책 확인
    println!("\n🔧 RLS 정책 확인 (Admin):");
    
    let rls_policies_response = admin_client
        .from("pg_policies")
        .select("tablename,policyname,permissive,roles,cmd,qual")
        .execute()
        .await?;
    
    if rls_policies_response.status().is_success() {
        let policies_text = rls_policies_response.text().await?;
        println!("  - RLS 정책 조회 성공");
        
        // JSON 파싱해서 관련 정책만 출력
        if let Ok(policies) = serde_json::from_str::<serde_json::Value>(&policies_text) {
            if let Some(policies_array) = policies.as_array() {
                println!("\n📋 현재 RLS 정책들:");
                for policy in policies_array {
                    if let (Some(table), Some(policy_name), Some(cmd)) = (
                        policy.get("tablename").and_then(|v| v.as_str()),
                        policy.get("policyname").and_then(|v| v.as_str()),
                        policy.get("cmd").and_then(|v| v.as_str())
                    ) {
                        if table.contains("shop") || table.contains("profile") || 
                           table.contains("subscription") || table.contains("discount") {
                            println!("  - {}.{}: {}", table, policy_name, cmd);
                        }
                    }
                }
            }
        }
    } else {
        println!("  - RLS 정책 조회 실패: {}", rls_policies_response.status());
    }

    println!("\n✅ RLS 테스트 완료!");
    
    Ok(())
}