use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: String,
    pub email: String,
    pub role: Option<String>,
}

/// JWT 토큰에서 사용자 정보 추출
pub fn extract_user_from_token(token: &str) -> Result<AuthUser, Box<dyn std::error::Error>> {
    // 실제로는 JWT 라이브러리로 토큰 검증 및 파싱
    // 여기서는 간단한 예시
    if token.is_empty() {
        return Err("Token is empty".into());
    }

    // 토큰 검증 로직 (실제로는 Supabase public key로 검증)
    // 예시 구현
    Ok(AuthUser {
        id: "user-uuid-from-jwt".to_string(),
        email: "user@example.com".to_string(),
        role: None,
    })
}

/// 사용자 로그인 상태 확인
pub fn is_authenticated(token: Option<&str>) -> bool {
    match token {
        Some(t) if !t.is_empty() => {
            // 실제로는 JWT 토큰 유효성 검증
            extract_user_from_token(t).is_ok()
        }
        _ => false,
    }
}

/// 관리자 권한 확인
pub fn is_admin(token: Option<&str>) -> bool {
    if let Some(token) = token {
        if let Ok(user) = extract_user_from_token(token) {
            return user.role.as_deref() == Some("admin");
        }
    }
    false
}