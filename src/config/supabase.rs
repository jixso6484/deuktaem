use postgrest::Postgrest;
use std::env;
use dotenv::dotenv;

#[derive(Clone)]
pub struct SupabaseConfig {
    pub client: Postgrest,
    pub url: String,
    pub anon_key: String,
    pub service_key: String,
}

impl SupabaseConfig {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();

        let url = env::var("SUPABASE_URL")?;
        let anon_key = env::var("SUPABASE_ANON_KEY")?;
        let service_key = env::var("SUPABASE_SERVICE_KEY")?;

        // Service Key로 기본 클라이언트 생성 (관리자 권한)
        let client = Postgrest::new(&url)
            .insert_header("apikey", &service_key)
            .insert_header("Authorization", format!("Bearer {}", service_key));

        Ok(Self {
            client,
            url,
            anon_key,
            service_key,
        })
    }

    // 공개 읽기용 클라이언트 (RLS 적용)
    pub fn public_client(&self) -> Postgrest {
        Postgrest::new(&self.url)
            .insert_header("apikey", &self.anon_key)
            .insert_header("Authorization", format!("Bearer {}", self.anon_key))
    }

    // 인증된 사용자 클라이언트 (RLS 적용)
    pub fn authenticated_client(&self, user_token: &str) -> Postgrest {
        Postgrest::new(&self.url)
            .insert_header("apikey", &self.anon_key)
            .insert_header("Authorization", format!("Bearer {}", user_token))
    }

    // 관리자 클라이언트 (RLS 우회)
    pub fn admin_client(&self) -> &Postgrest {
        &self.client
    }

    // 연결 테스트 메소드
    pub async fn test_connection(&self) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.client
            .from("test_table")
            .select("*")
            .limit(1)
            .execute()
            .await?;

        println!("✅ Supabase 연결 성공! Status: {}", response.status());
        Ok(())
    }
}