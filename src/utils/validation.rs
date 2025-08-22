use crate::utils::constants::*;
use crate::error::AppError;

// 페이지네이션 검증
pub fn validate_pagination(page: u32, limit: u32) -> Result<(u32, u32), AppError> {
    if page == 0 {
        return Err(AppError::validation("Page must be greater than 0"));
    }
    
    if limit == 0 {
        return Err(AppError::validation("Limit must be greater than 0"));
    }
    
    if limit > MAX_PAGE_SIZE {
        return Err(AppError::validation(format!("Limit cannot exceed {}", MAX_PAGE_SIZE)));
    }
    
    Ok((page, limit))
}

// 언어 코드 검증
pub fn validate_language(language: &str) -> Result<String, AppError> {
    if language.is_empty() {
        return Ok(DEFAULT_LANGUAGE.to_string());
    }
    
    if SUPPORTED_LANGUAGES.contains(&language) {
        Ok(language.to_string())
    } else {
        Err(AppError::validation(format!("Unsupported language: {}. Supported: {:?}", language, SUPPORTED_LANGUAGES)))
    }
}

// 국가 코드 검증
pub fn validate_country(country: &str) -> Result<String, AppError> {
    if country.is_empty() {
        return Ok(DEFAULT_COUNTRY.to_string());
    }
    
    if SUPPORTED_COUNTRIES.contains(&country) {
        Ok(country.to_string())
    } else {
        Err(AppError::validation(format!("Unsupported country: {}. Supported: {:?}", country, SUPPORTED_COUNTRIES)))
    }
}

// 알림 타입 검증
pub fn validate_notification_type(notification_type: &str) -> Result<String, AppError> {
    if NOTIFICATION_TYPES.contains(&notification_type) {
        Ok(notification_type.to_string())
    } else {
        Err(AppError::validation(format!("Invalid notification type: {}. Supported: {:?}", notification_type, NOTIFICATION_TYPES)))
    }
}

// 알림 설정 타입 검증
pub fn validate_notification_setting_type(setting_type: &str) -> Result<String, AppError> {
    if NOTIFICATION_SETTINGS_TYPES.contains(&setting_type) {
        Ok(setting_type.to_string())
    } else {
        Err(AppError::validation(format!("Invalid notification setting type: {}. Supported: {:?}", setting_type, NOTIFICATION_SETTINGS_TYPES)))
    }
}

// UUID 검증
pub fn validate_uuid(id: &str) -> Result<String, AppError> {
    if uuid::Uuid::parse_str(id).is_ok() {
        Ok(id.to_string())
    } else {
        Err(AppError::validation(format!("Invalid UUID format: {}", id)))
    }
}

// 이메일 검증 (간단한 버전)
pub fn validate_email(email: &str) -> Result<String, AppError> {
    if email.is_empty() {
        return Err(AppError::validation("Email cannot be empty".to_string()));
    }
    
    if !email.contains('@') {
        return Err(AppError::validation("Invalid email format".to_string()));
    }
    
    Ok(email.to_string())
}

// 검색 쿼리 검증
pub fn validate_search_query(query: &str) -> Result<String, AppError> {
    if query.is_empty() {
        return Err(AppError::validation("Search query cannot be empty".to_string()));
    }
    
    if query.len() < 2 {
        return Err(AppError::validation("Search query must be at least 2 characters".to_string()));
    }
    
    if query.len() > 100 {
        return Err(AppError::validation("Search query cannot exceed 100 characters".to_string()));
    }
    
    Ok(query.trim().to_string())
}

// 정렬 옵션 검증
pub fn validate_sort_option(sort: &str, allowed_fields: &[&str]) -> Result<(String, String), AppError> {
    if sort.is_empty() {
        return Ok(("created_at".to_string(), "desc".to_string()));
    }
    
    let parts: Vec<&str> = sort.split(':').collect();
    if parts.len() != 2 {
        return Err(AppError::validation("Sort format must be 'field:direction' (e.g., 'name:asc')".to_string()));
    }
    
    let field = parts[0];
    let direction = parts[1];
    
    if !allowed_fields.contains(&field) {
        return Err(AppError::validation(format!("Invalid sort field: {}. Allowed: {:?}", field, allowed_fields)));
    }
    
    if !["asc", "desc"].contains(&direction) {
        return Err(AppError::validation("Sort direction must be 'asc' or 'desc'".to_string()));
    }
    
    Ok((field.to_string(), direction.to_string()))
}