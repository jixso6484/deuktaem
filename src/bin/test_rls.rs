use postgrest::Postgrest;
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    
    let supabase_url = env::var("SUPABASE_URL")?;
    let anon_key = env::var("SUPABASE_ANON_KEY")?;
    let service_key = env::var("SUPABASE_SERVICE_KEY")?;
    
    println!("🛡️ RLS 정책 테스트 시작...\n");

    // Anon 클라이언트 (RLS 적용)
    let anon_client = Postgrest::new(&supabase_url)
        .insert_header("apikey", &anon_key)
        .insert_header("Authorization", format!("Bearer {}", anon_key));

    println!("📖 공개 읽기 테스트:");
    
    // 공개 테이블들 테스트
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

    println!("\n🔒 사용자 전용 테이블 테스트:");
    
    // 사용자 전용 테이블들
    let user_tables = vec!["profiles", "shop_subscriptions"];
    
    for table in &user_tables {
        let response = anon_client
            .from(table)
            .select("*")
            .limit(1)
            .execute()
            .await?;
            
        let status_code = response.status().as_u16();
        println!("  - {} 조회: {} {}", 
            table, 
            status_code,
            if status_code >= 400 || response.text().await? == "[]" { "✅ (보호됨)" } else { "❌" }
        );
    }

    println!("\n👑 관리자 권한 테스트:");
    
    let service_client = Postgrest::new(&supabase_url)
        .insert_header("apikey", &service_key)
        .insert_header("Authorization", format!("Bearer {}", service_key));

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

    println!("\n✅ RLS 테스트 완료!");
    
    Ok(())
}