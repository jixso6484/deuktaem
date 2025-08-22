use postgrest::Postgrest;
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    
    let supabase_url = env::var("SUPABASE_URL")?;
    let service_key = env::var("SUPABASE_SERVICE_KEY")?;
    
    println!("🗄️ Supabase 테이블 구조 확인...\n");

    // Service Key로 모든 테이블 접근
    let client = Postgrest::new(&supabase_url)
        .insert_header("apikey", &service_key)
        .insert_header("Authorization", format!("Bearer {}", service_key));

    // 테이블 목록 확인
    println!("📋 테이블 목록 확인:");
    
    // information_schema로 테이블 목록 조회
    let tables_response = client
        .from("information_schema.tables")
        .select("table_name,table_schema")
        .eq("table_schema", "public")
        .execute()
        .await?;

    if tables_response.status().is_success() {
        let tables_text = tables_response.text().await?;
        println!("Public 스키마 테이블들:");
        println!("{}", tables_text);
    } else {
        println!("테이블 목록 조회 실패: {}", tables_response.status());
    }

    // 개별 테이블 확인
    let test_tables = vec![
        "users", "profiles", "shops", "brands", "categories", 
        "products", "discounts", "discount_info", "discount_events",
        "subscriptions", "shop_subscriptions", "brand_subscriptions",
        "notifications", "translations"
    ];

    println!("\n🔍 개별 테이블 확인:");
    for table in test_tables {
        let response = client
            .from(table)
            .select("*")
            .limit(1)
            .execute()
            .await?;
            
        let status = response.status();
        if status.is_success() {
            let data = response.text().await?;
            if data.trim() == "[]" {
                println!("  ✅ {} - 존재함 (데이터 없음)", table);
            } else {
                println!("  ✅ {} - 존재함 (데이터 있음)", table);
            }
        } else if status.as_u16() == 404 {
            println!("  ❌ {} - 존재하지 않음", table);
        } else {
            println!("  ⚠️ {} - 오류: {}", table, status);
        }
    }

    // 스키마 정보 확인
    println!("\n📊 컬럼 정보 확인:");
    let columns_response = client
        .from("information_schema.columns")
        .select("table_name,column_name,data_type,is_nullable")
        .eq("table_schema", "public")
        .order("table_name.asc,ordinal_position.asc")
        .execute()
        .await?;

    if columns_response.status().is_success() {
        let columns_text = columns_response.text().await?;
        println!("컬럼 정보:");
        
        // JSON 파싱해서 정리된 형태로 출력
        if let Ok(columns) = serde_json::from_str::<serde_json::Value>(&columns_text) {
            if let Some(columns_array) = columns.as_array() {
                let mut current_table = "";
                for column in columns_array {
                    if let (Some(table), Some(col_name), Some(data_type)) = (
                        column.get("table_name").and_then(|v| v.as_str()),
                        column.get("column_name").and_then(|v| v.as_str()),
                        column.get("data_type").and_then(|v| v.as_str())
                    ) {
                        if table != current_table {
                            println!("\n📋 Table: {}", table);
                            current_table = table;
                        }
                        let nullable = column.get("is_nullable")
                            .and_then(|v| v.as_str())
                            .unwrap_or("NO");
                        println!("  - {}: {} {}", col_name, data_type, 
                            if nullable == "YES" { "(nullable)" } else { "(not null)" });
                    }
                }
            }
        }
    }

    // 리얼타임 설정 확인
    println!("\n🔴 리얼타임 설정 확인:");
    let realtime_response = client
        .from("realtime.subscription")
        .select("*")
        .execute()
        .await;

    match realtime_response {
        Ok(resp) => {
            println!("리얼타임 테이블 응답: {}", resp.status());
            if resp.status().is_success() {
                let realtime_text = resp.text().await?;
                println!("리얼타임 구독: {}", realtime_text);
            }
        }
        Err(_) => println!("리얼타임 정보 접근 불가 (정상적)"),
    }

    println!("\n✅ 테이블 구조 확인 완료!");
    Ok(())
}