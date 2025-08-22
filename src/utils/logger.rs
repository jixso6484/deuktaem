use tracing::{info, warn, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_logger() {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "duk=info,tower_http=debug".into());

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer()
            .with_target(true)
            .with_thread_ids(true)
            .with_file(true)
            .with_line_number(true)
        )
        .init();

    info!("🚀 Logger initialized");
}

// 성능 측정용 매크로
#[macro_export]
macro_rules! log_execution_time {
    ($operation:expr, $block:block) => {{
        let start = std::time::Instant::now();
        let result = $block;
        let duration = start.elapsed();
        tracing::info!(
            operation = $operation,
            duration_ms = duration.as_millis(),
            "⏱️ Operation completed"
        );
        result
    }};
}

// 사용자 액션 로깅
pub fn log_user_action(user_id: &str, action: &str, details: Option<&str>) {
    info!(
        user_id = user_id,
        action = action,
        details = details.unwrap_or(""),
        "👤 User action"
    );
}

// API 요청 로깅
pub fn log_api_request(method: &str, path: &str, user_id: Option<&str>, ip: Option<&str>) {
    info!(
        method = method,
        path = path,
        user_id = user_id.unwrap_or("anonymous"),
        ip = ip.unwrap_or("unknown"),
        "🌐 API request"
    );
}

// 데이터베이스 작업 로깅
pub fn log_db_operation(operation: &str, table: &str, duration_ms: u128, affected_rows: Option<u64>) {
    info!(
        operation = operation,
        table = table,
        duration_ms = duration_ms,
        affected_rows = affected_rows.unwrap_or(0),
        "🗃️ Database operation"
    );
}

// 에러 로깅 (상세 정보 포함)
pub fn log_error_with_context(error: &dyn std::error::Error, context: &str, user_id: Option<&str>) {
    error!(
        error = %error,
        context = context,
        user_id = user_id.unwrap_or("unknown"),
        "❌ Error occurred"
    );
}

// 보안 이벤트 로깅
pub fn log_security_event(event_type: &str, user_id: Option<&str>, ip: Option<&str>, details: &str) {
    warn!(
        event_type = event_type,
        user_id = user_id.unwrap_or("unknown"),
        ip = ip.unwrap_or("unknown"),
        details = details,
        "🔐 Security event"
    );
}

// 비즈니스 메트릭 로깅
pub fn log_business_metric(metric_name: &str, value: f64, labels: Option<serde_json::Value>) {
    info!(
        metric = metric_name,
        value = value,
        labels = ?labels,
        "📊 Business metric"
    );
}

// 실시간 이벤트 로깅
pub fn log_realtime_event(event_type: &str, user_id: &str, data: Option<serde_json::Value>) {
    info!(
        event_type = event_type,
        user_id = user_id,
        data = ?data,
        "🔴 Realtime event"
    );
}