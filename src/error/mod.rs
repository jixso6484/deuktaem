use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::json;
use tracing::error;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("Authentication error: {0}")]
    Authentication(String),
    
    #[error("Authorization error: {0}")]
    Authorization(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Conflict: {0}")]
    Conflict(String),
    
    #[error("Rate limit exceeded")]
    RateLimit,
    
    #[error("External service error: {0}")]
    ExternalService(String),
    
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    #[error("Internal server error: {0}")]
    InternalServer(String),
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Authentication(_) => StatusCode::UNAUTHORIZED,
            AppError::Authorization(_) => StatusCode::FORBIDDEN,
            AppError::Validation(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::RateLimit => StatusCode::TOO_MANY_REQUESTS,
            AppError::ExternalService(_) => StatusCode::BAD_GATEWAY,
            AppError::Configuration(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InternalServer(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn error_code(&self) -> &str {
        match self {
            AppError::Database(_) => "DATABASE_ERROR",
            AppError::Authentication(_) => "AUTHENTICATION_ERROR",
            AppError::Authorization(_) => "AUTHORIZATION_ERROR",
            AppError::Validation(_) => "VALIDATION_ERROR",
            AppError::NotFound(_) => "NOT_FOUND",
            AppError::Conflict(_) => "CONFLICT",
            AppError::RateLimit => "RATE_LIMIT_EXCEEDED",
            AppError::ExternalService(_) => "EXTERNAL_SERVICE_ERROR",
            AppError::Configuration(_) => "CONFIGURATION_ERROR",
            AppError::InternalServer(_) => "INTERNAL_SERVER_ERROR",
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status_code();
        let error_code = self.error_code();
        let message = self.to_string();

        // 에러 로깅
        error!(
            error = %self,
            status_code = %status,
            error_code = error_code,
            "API error occurred"
        );

        let body = Json(json!({
            "error": {
                "code": error_code,
                "message": message,
                "timestamp": chrono::Utc::now().to_rfc3339()
            }
        }));

        (status, body).into_response()
    }
}

// From 구현들
impl From<Box<dyn std::error::Error>> for AppError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        AppError::InternalServer(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Validation(format!("JSON parsing error: {}", err))
    }
}

// 헬퍼 함수들
impl AppError {
    pub fn authentication<T: Into<String>>(msg: T) -> Self {
        AppError::Authentication(msg.into())
    }

    pub fn authorization<T: Into<String>>(msg: T) -> Self {
        AppError::Authorization(msg.into())
    }

    pub fn validation<T: Into<String>>(msg: T) -> Self {
        AppError::Validation(msg.into())
    }

    pub fn not_found<T: Into<String>>(resource: T) -> Self {
        AppError::NotFound(format!("{} not found", resource.into()))
    }

    pub fn conflict<T: Into<String>>(msg: T) -> Self {
        AppError::Conflict(msg.into())
    }

    pub fn internal<T: Into<String>>(msg: T) -> Self {
        AppError::InternalServer(msg.into())
    }
}

pub type AppResult<T> = Result<T, AppError>;