-- 지원 언어 테이블
CREATE TABLE IF NOT EXISTS languages (
    code VARCHAR(10) PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    native_name VARCHAR(100) NOT NULL,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- 기본 언어 데이터 삽입
INSERT INTO languages (code, name, native_name) VALUES
    ('ko', 'Korean', '한국어'),
    ('en', 'English', 'English'),
    ('ja', 'Japanese', '日本語'),
    ('zh', 'Chinese', '中文'),
    ('es', 'Spanish', 'Español'),
    ('fr', 'French', 'Français'),
    ('de', 'German', 'Deutsch')
ON CONFLICT (code) DO NOTHING;

-- 상품 번역 테이블
CREATE TABLE IF NOT EXISTS product_translations (
    id BIGSERIAL PRIMARY KEY,
    product_id BIGINT NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    locale VARCHAR(10) NOT NULL REFERENCES languages(code),
    name TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(product_id, locale)
);

-- 할인 이벤트 번역 테이블
CREATE TABLE IF NOT EXISTS discount_event_translations (
    id BIGSERIAL PRIMARY KEY,
    event_id BIGINT NOT NULL REFERENCES discount_events(id) ON DELETE CASCADE,
    locale VARCHAR(10) NOT NULL REFERENCES languages(code),
    title TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(event_id, locale)
);

-- 할인 정보 번역 테이블 (할인 상세 정보용)
CREATE TABLE IF NOT EXISTS discount_info_translations (
    id BIGSERIAL PRIMARY KEY,
    discount_info_id BIGINT NOT NULL REFERENCES discount_infos(id) ON DELETE CASCADE,
    locale VARCHAR(10) NOT NULL REFERENCES languages(code),
    description TEXT,
    terms_conditions TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(discount_info_id, locale)
);

-- 알림 번역 테이블
CREATE TABLE IF NOT EXISTS notification_translations (
    id BIGSERIAL PRIMARY KEY,
    notification_id BIGINT NOT NULL REFERENCES notifications(id) ON DELETE CASCADE,
    locale VARCHAR(10) NOT NULL REFERENCES languages(code),
    title TEXT,
    message TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(notification_id, locale)
);

-- API 메트릭 테이블 (누락된 테이블 생성)
CREATE TABLE IF NOT EXISTS api_metrics (
    id BIGSERIAL PRIMARY KEY,
    endpoint VARCHAR(255) NOT NULL,
    method VARCHAR(10) NOT NULL,
    status_code INT NOT NULL,
    response_time FLOAT NOT NULL,
    error_message TEXT,
    metadata JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- 인덱스 생성 (성능 최적화)
CREATE INDEX idx_product_translations_product_id ON product_translations(product_id);
CREATE INDEX idx_product_translations_locale ON product_translations(locale);

CREATE INDEX idx_discount_event_translations_event_id ON discount_event_translations(event_id);
CREATE INDEX idx_discount_event_translations_locale ON discount_event_translations(locale);

CREATE INDEX idx_discount_info_translations_info_id ON discount_info_translations(discount_info_id);
CREATE INDEX idx_discount_info_translations_locale ON discount_info_translations(locale);

CREATE INDEX idx_notification_translations_notification_id ON notification_translations(notification_id);
CREATE INDEX idx_notification_translations_locale ON notification_translations(locale);

CREATE INDEX idx_api_metrics_created_at ON api_metrics(created_at);
CREATE INDEX idx_api_metrics_endpoint ON api_metrics(endpoint);

-- Updated_at 자동 업데이트 트리거 함수
CREATE OR REPLACE FUNCTION update_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 각 테이블에 트리거 적용
CREATE TRIGGER update_languages_updated_at BEFORE UPDATE ON languages
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();

CREATE TRIGGER update_product_translations_updated_at BEFORE UPDATE ON product_translations
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();

CREATE TRIGGER update_discount_event_translations_updated_at BEFORE UPDATE ON discount_event_translations
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();

CREATE TRIGGER update_discount_info_translations_updated_at BEFORE UPDATE ON discount_info_translations
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();

CREATE TRIGGER update_notification_translations_updated_at BEFORE UPDATE ON notification_translations
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();