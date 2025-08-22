use crate::config::SupabaseConfig;

pub async fn check_database_structure() -> Result<(), Box<dyn std::error::Error>> {
    let config = SupabaseConfig::new()?;
    
    println!("🔍 데이터베이스 구조 확인 중...");
    
    // 테이블 목록 확인
    let tables_response = config.client
        .from("information_schema.tables")
        .select("table_name")
        .eq("table_schema", "public")
        .execute()
        .await?;

    println!("📊 응답 상태: {}", tables_response.status());
    
    if let Ok(body) = tables_response.text().await {
        println!("📋 테이블 목록:");
        println!("{}", body);
    }
    
    Ok(())
}