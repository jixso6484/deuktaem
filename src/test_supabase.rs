use postgrest::Postgrest;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // .env 파일 로드
    dotenv::dotenv().ok();
    
    let supabase_url = env::var("SUPABASE_URL")?;
    let service_key = env::var("SUPABASE_SERVICE_KEY")?;
    
    println!("Supabase URL: {}", supabase_url);
    println!("Service Key: {}...", &service_key[..50]);
    
    // Supabase 클라이언트 생성
    let client = Postgrest::new(supabase_url)
        .insert_header("apikey", service_key.clone())
        .insert_header("Authorization", format!("Bearer {}", service_key));
    
    // 간단한 테스트 쿼리 (테이블이 없어도 응답은 받음)
    let response = client
        .from("test_table")
        .select("*")
        .limit(1)
        .execute()
        .await?;
    
    println!("Status: {}", response.status());
    println!("Response: {}", response.text().await?);
    
    println!("✅ Supabase 연결 성공!");
    
    Ok(())
}