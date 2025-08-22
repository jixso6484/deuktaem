// API 관련 상수
pub const API_VERSION: &str = "v1";
pub const MAX_PAGE_SIZE: u32 = 100;
pub const DEFAULT_PAGE_SIZE: u32 = 20;

// 알림 관련 상수
pub const NOTIFICATION_TYPES: &[&str] = &[
    "discount_update",
    "shop_subscription", 
    "brand_subscription",
    "category_subscription",
];

pub const NOTIFICATION_SETTINGS_TYPES: &[&str] = &[
    "push",
    "discount",
    "shop", 
    "brand",
    "category",
];

// 지원 언어
pub const SUPPORTED_LANGUAGES: &[&str] = &["en", "ko", "ja", "zh"];
pub const DEFAULT_LANGUAGE: &str = "en";

// 지원 국가
pub const SUPPORTED_COUNTRIES: &[&str] = &["KR", "JP", "US", "CN"];
pub const DEFAULT_COUNTRY: &str = "KR";

// JWT 관련
pub const JWT_EXPIRY_HOURS: u64 = 24;
pub const REFRESH_TOKEN_EXPIRY_DAYS: u64 = 30;

// 캐시 관련
pub const CACHE_TTL_SECONDS: u64 = 300; // 5분
pub const POPULAR_ITEMS_CACHE_TTL: u64 = 3600; // 1시간

// 로그 레벨
pub const LOG_LEVEL_DEV: &str = "debug";
pub const LOG_LEVEL_PROD: &str = "info";

// 에러 메시지
pub const ERROR_UNAUTHORIZED: &str = "Unauthorized";
pub const ERROR_NOT_FOUND: &str = "Resource not found";
pub const ERROR_INVALID_INPUT: &str = "Invalid input";
pub const ERROR_INTERNAL_SERVER: &str = "Internal server error";

// 성공 메시지
pub const SUCCESS_CREATED: &str = "Resource created successfully";
pub const SUCCESS_UPDATED: &str = "Resource updated successfully";
pub const SUCCESS_DELETED: &str = "Resource deleted successfully";