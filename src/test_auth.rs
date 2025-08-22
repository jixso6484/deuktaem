use postgrest::Postgrest;
use std::env;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    
    let supabase_url = env::var("SUPABASE_URL")?;
    let anon_key = env::var("SUPABASE_ANON_KEY")?;
    
    println!("🔍 Supabase Auth 설정 확인...\n");

    // 1. Auth 엔드포인트 확인
    let auth_url = format!("{}/auth/v1/settings", supabase_url);
    let client = reqwest::Client::new();
    
    println!("📡 Auth 설정 확인:");
    let auth_response = client
        .get(&auth_url)
        .header("apikey", &anon_key)
        .send()
        .await?;
    
    println!("  - Auth 엔드포인트: {}", auth_response.status());
    
    if auth_response.status().is_success() {
        let auth_settings = auth_response.text().await?;
        println!("  - Auth 설정: {}", &auth_settings[..200.min(auth_settings.len())]);
    }

    // 2. 공개 스키마 확인 (auth 관련 테이블들)
    let postgrest_client = Postgrest::new(&supabase_url)
        .insert_header("apikey", &anon_key);

    println!("\n🗄️ 사용자 관련 테이블 확인:");
    
    // auth.users 테이블 접근 시도 (보통 접근 불가)
    let users_response = postgrest_client
        .from("auth.users")
        .select("id")
        .limit(1)
        .execute()
        .await?;
    
    println!("  - auth.users 접근: {}", users_response.status());

    // profiles 테이블 확인
    let profiles_response = postgrest_client
        .from("profiles")
        .select("*")
        .limit(1)
        .execute()
        .await?;
    
    println!("  - profiles 테이블: {}", profiles_response.status());
    
    if profiles_response.status().is_success() {
        let profiles_text = profiles_response.text().await?;
        if profiles_text.trim() == "[]" {
            println!("    (테이블 존재하지만 데이터 없음)");
        } else {
            println!("    (테이블 존재, 데이터 있음)");
        }
    }

    // 3. RLS 정책 존재 확인
    println!("\n🛡️ RLS 정책 확인:");
    
    let rls_response = postgrest_client
        .rpc("check_rls_policies", "{}")
        .execute()
        .await;
    
    match rls_response {
        Ok(resp) => println!("  - RLS 함수 호출: {}", resp.status()),
        Err(_) => println!("  - RLS 함수 없음 (정상적)"),
    }

    // 4. 테스트 사용자 생성 시도 (Auth API)
    println!("\n👤 Auth API 테스트:");
    
    let signup_url = format!("{}/auth/v1/signup", supabase_url);
    let test_signup = client
        .post(&signup_url)
        .header("apikey", &anon_key)
        .header("Content-Type", "application/json")
        .json(&json!({
            "email": "test@example.com",
            "password": "test123456"
        }))
        .send()
        .await?;
    
    println!("  - 회원가입 엔드포인트: {}", test_signup.status());
    
    if test_signup.status().as_u16() == 422 {
        println!("    (이미 존재하는 이메일이거나 정책에 의해 제한됨)");
    } else if test_signup.status().is_success() {
        println!("    (회원가입 성공 - Auth 정상 작동)");
    }

    // 5. 현재 세션 확인
    println!("\n🔐 세션 확인:");
    
    let session_url = format!("{}/auth/v1/user", supabase_url);
    let session_response = client
        .get(&session_url)
        .header("apikey", &anon_key)
        .header("Authorization", format!("Bearer {}", anon_key))
        .send()
        .await?;
    
    println!("  - 세션 확인: {}", session_response.status());
    
    if session_response.status().as_u16() == 401 {
        println!("    (인증되지 않은 상태 - 정상)");
    }

    println!("\n✅ Supabase Auth 설정 확인 완료!");
    println!("\n💡 결과 요약:");
    println!("  - Auth 엔드포인트가 응답하면 Auth 활성화됨");
    println!("  - profiles 테이블이 있으면 사용자 프로필 관리 가능");
    println!("  - 회원가입 엔드포인트가 정상 응답하면 사용자 등록 가능");
    
    Ok(())
}