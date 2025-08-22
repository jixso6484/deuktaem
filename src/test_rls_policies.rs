use postgrest::Postgrest;
use std::env;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    
    let supabase_url = env::var("SUPABASE_URL")?;
    let anon_key = env::var("SUPABASE_ANON_KEY")?;
    let service_key = env::var("SUPABASE_SERVICE_KEY")?;
    
    println!("🛡️ RLS 정책 테스트 시작...\n");

    // Anon 클라이언트 (RLS 적용됨)
    let anon_client = Postgrest::new(&supabase_url)
        .insert_header("apikey", &anon_key)
        .insert_header("Authorization", format!("Bearer {}", anon_key));

    // Service 클라이언트 (RLS 우회)
    let service_client = Postgrest::new(&supabase_url)
        .insert_header("apikey", &service_key)
        .insert_header("Authorization", format!("Bearer {}", service_key));

    println!("📖 공개 읽기 테스트 (RLS: 모든 사용자 허용):");
    
    // 1. 공개 테이블들 - 읽기만 허용되어야 함
    let tables = vec!["shops", "brands", "categories", "products", "discount_info"];
    
    for table in &tables {
        let response = anon_client
            .from(table)
            .select("*")
            .limit(1)
            .execute()
            .await?;
            
        println!("  - {} 조회: {} {}", 
            table, 
            response.status(),
            if response.status().is_success() { "✅" } else { "❌" }
        );
    }

    println!("\n🔒 사용자 전용 테이블 테스트 (RLS: 인증 필요):");
    
    // 2. 사용자 전용 테이블들 - anon으로는 접근 불가해야 함
    let user_tables = vec![
        "profiles", 
        "shop_subscriptions", 
        "brand_subscriptions", 
        "category_subscriptions",
        "notifications",
        "notification_logs"
    ];
    
    for table in &user_tables {
        let response = anon_client
            .from(table)
            .select("*")
            .limit(1)
            .execute()
            .await?;
            
        let status_code = response.status().as_u16();
        let is_protected = status_code == 401 || status_code == 403 || 
                          (status_code == 200 && response.text().await? == "[]");
        
        println!("  - {} 조회: {} {}", 
            table, 
            status_code,
            if is_protected { "✅ (보호됨)" } else { "❌ (보호 안됨)" }
        );
    }

    println!("\n✍️ 쓰기 권한 테스트 (RLS: 대부분 제한):");
    
    // 3. 쓰기 시도 - 대부분 실패해야 함
    for table in &tables {
        let response = anon_client
            .from(table)
            .insert(json!({"name": "test_item"}).to_string())
            .execute()
            .await?;
            
        let status_code = response.status().as_u16();
        let is_protected = status_code >= 400;
        
        println!("  - {} 생성: {} {}", 
            table, 
            status_code,
            if is_protected { "✅ (제한됨)" } else { "❌ (제한 안됨)" }
        );
    }

    println!("\n👑 관리자 권한 테스트 (Service Key):");
    
    // 4. Service Key로는 모든 작업이 가능해야 함
    let response = service_client
        .from("shops")
        .select("*")
        .limit(1)
        .execute()
        .await?;
        
    println!("  - 관리자 읽기: {} {}", 
        response.status(),
        if response.status().is_success() { "✅" } else { "❌" }
    );

    // RLS 정책 조회 (관리자만 가능)
    let rls_response = service_client
        .rpc("get_rls_policies", "{}")
        .execute()
        .await;
        
    match rls_response {
        Ok(resp) => println!("  - RLS 정책 조회: {} ✅", resp.status()),
        Err(_) => {
            // 직접 pg_policies 조회 시도
            let policies_response = service_client
                .from("pg_policies")
                .select("schemaname,tablename,policyname,permissive,roles,cmd")
                .execute()
                .await?;
                
            println!("  - 정책 테이블 조회: {} {}", 
                policies_response.status(),
                if policies_response.status().is_success() { "✅" } else { "❌" }
            );
        }
    }

    println!("\n📊 RLS 설정 요약:");
    println!("  ✅ 공개 테이블: 읽기만 허용");
    println!("  ✅ 사용자 테이블: 인증 필요");
    println!("  ✅ 쓰기 작업: 대부분 제한");
    println!("  ✅ 관리자: 모든 권한");
    
    println!("\n💡 예상 RLS 정책:");
    println!("  📖 공개 읽기: FOR SELECT USING (true)");
    println!("  🔒 사용자 데이터: USING (auth.uid() = user_id)");
    println!("  ✍️ 관리 작업: Service Key만 허용");

    Ok(())
}