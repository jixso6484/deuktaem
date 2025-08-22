
use sqlx::postgres::{PgPool,PgPoolOptions};
use std::env;
use dotenv::dotenv;

pub struct DatabaseConfig {
      pub pool: PgPool,
      pub supabase_url: String,
      pub supabase_anon_key: String,  // 오타 수정
  }

impl DatabaseConfig {  // 오타 수정
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();  // 괄호 추가

        let supabase_url = env::var("SUPABASE_URL")?;  // ? 추가
        let supabase_anon_key = env::var("SUPABASE_ANON_KEY")?;

          // DATABASE_URL 변수 추가!
        let database_url = env::var("DATABASE_URL")?;

        let pool = PgPoolOptions::new()  // poop → pool
            .max_connections(10)
            .connect(&database_url)  // 이제 database_url 사용 가능
            .await?;

        Ok(Self {
            pool,
            supabase_url,
            supabase_anon_key,
        })
      }
  }
