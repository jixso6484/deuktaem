-- 기본 테이블들 생성 (없을 경우를 대비해서)

-- 매장 테이블
CREATE TABLE IF NOT EXISTS shops (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) UNIQUE NOT NULL,
    description TEXT,
    logo_url VARCHAR(500),
    website_url VARCHAR(500),
    country VARCHAR(10) DEFAULT 'KR',
    is_active BOOLEAN DEFAULT true,
    is_deleted BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- 브랜드 테이블
CREATE TABLE IF NOT EXISTS brands (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) UNIQUE NOT NULL,
    logo_url VARCHAR(500),
    country VARCHAR(10) DEFAULT 'KR',
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- 카테고리 테이블
CREATE TABLE IF NOT EXISTS categories (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) UNIQUE NOT NULL,
    parent_id BIGINT REFERENCES categories(id),
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- 상품 테이블
CREATE TABLE IF NOT EXISTS products (
    id BIGSERIAL PRIMARY KEY,
    shop_id BIGINT NOT NULL REFERENCES shops(id) ON DELETE CASCADE,
    brand_id BIGINT REFERENCES brands(id),
    category_id BIGINT REFERENCES categories(id),
    name VARCHAR(500) NOT NULL,
    sku VARCHAR(100),
    original_url VARCHAR(1000),
    image_url VARCHAR(500),
    is_deleted BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- 할인 이벤트 테이블
CREATE TABLE IF NOT EXISTS discount_events (
    id BIGSERIAL PRIMARY KEY,
    title VARCHAR(500) NOT NULL,
    description TEXT,
    event_type VARCHAR(100) NOT NULL DEFAULT 'general',
    discount_rate DECIMAL(5,2),
    start_at TIMESTAMPTZ NOT NULL,
    end_at TIMESTAMPTZ NOT NULL,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- 할인 정보 테이블
CREATE TABLE IF NOT EXISTS discount_infos (
    id BIGSERIAL PRIMARY KEY,
    product_id BIGINT NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    shop_id BIGINT NOT NULL REFERENCES shops(id) ON DELETE CASCADE,
    brand_id BIGINT REFERENCES brands(id),
    event_id BIGINT REFERENCES discount_events(id),
    original_price DECIMAL(12,2) NOT NULL,
    discount_price DECIMAL(12,2) NOT NULL,
    discount_rate DECIMAL(5,2) NOT NULL,
    currency VARCHAR(10) DEFAULT 'KRW',
    start_at TIMESTAMPTZ NOT NULL,
    end_at TIMESTAMPTZ NOT NULL,
    is_active BOOLEAN DEFAULT true,
    source_url VARCHAR(1000),
    is_auto_discovered BOOLEAN DEFAULT false,
    is_event_based BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- 사용자 프로필 테이블 (Supabase auth.users 연동)
CREATE TABLE IF NOT EXISTS profiles (
    id UUID REFERENCES auth.users ON DELETE CASCADE,
    user_id UUID NOT NULL,  -- auth.users.id 참조
    username VARCHAR(50),
    full_name VARCHAR(100),
    avatar_url VARCHAR(500),
    country VARCHAR(10) DEFAULT 'KR',
    language VARCHAR(10) DEFAULT 'ko',
    timezone VARCHAR(50) DEFAULT 'Asia/Seoul',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (id)
);

-- 매장 구독 테이블
CREATE TABLE IF NOT EXISTS shop_subscriptions (
    id BIGSERIAL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    shop_id BIGINT NOT NULL REFERENCES shops(id) ON DELETE CASCADE,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, shop_id)
);

-- 브랜드 구독 테이블
CREATE TABLE IF NOT EXISTS brand_subscriptions (
    id BIGSERIAL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    brand_id BIGINT NOT NULL REFERENCES brands(id) ON DELETE CASCADE,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, brand_id)
);

-- 카테고리 구독 테이블
CREATE TABLE IF NOT EXISTS category_subscriptions (
    id BIGSERIAL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    category_id BIGINT NOT NULL REFERENCES categories(id) ON DELETE CASCADE,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, category_id)
);

-- 알림 테이블
CREATE TABLE IF NOT EXISTS notifications (
    id BIGSERIAL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    r#type VARCHAR(50) NOT NULL,
    title VARCHAR(500),
    message TEXT NOT NULL,
    data JSONB,
    is_read BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- 알림 설정 테이블
CREATE TABLE IF NOT EXISTS notification_settings (
    id BIGSERIAL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    setting_type VARCHAR(50) NOT NULL,
    is_enabled BOOLEAN DEFAULT true,
    settings JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, setting_type)
);

-- 기본 인덱스 생성
CREATE INDEX IF NOT EXISTS idx_shops_country ON shops(country);
CREATE INDEX IF NOT EXISTS idx_shops_active ON shops(is_active, is_deleted);
CREATE INDEX IF NOT EXISTS idx_products_shop ON products(shop_id);
CREATE INDEX IF NOT EXISTS idx_products_brand ON products(brand_id);
CREATE INDEX IF NOT EXISTS idx_products_category ON products(category_id);
CREATE INDEX IF NOT EXISTS idx_discount_infos_product ON discount_infos(product_id);
CREATE INDEX IF NOT EXISTS idx_discount_infos_shop ON discount_infos(shop_id);
CREATE INDEX IF NOT EXISTS idx_discount_infos_active ON discount_infos(is_active, start_at, end_at);
CREATE INDEX IF NOT EXISTS idx_notifications_user ON notifications(user_id, is_read);

-- 업데이트 트리거 적용
CREATE TRIGGER update_shops_updated_at BEFORE UPDATE ON shops
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();
    
CREATE TRIGGER update_brands_updated_at BEFORE UPDATE ON brands
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();
    
CREATE TRIGGER update_categories_updated_at BEFORE UPDATE ON categories
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();
    
CREATE TRIGGER update_products_updated_at BEFORE UPDATE ON products
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();
    
CREATE TRIGGER update_discount_events_updated_at BEFORE UPDATE ON discount_events
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();
    
CREATE TRIGGER update_discount_infos_updated_at BEFORE UPDATE ON discount_infos
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();

CREATE TRIGGER update_profiles_updated_at BEFORE UPDATE ON profiles
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();

CREATE TRIGGER update_notification_settings_updated_at BEFORE UPDATE ON notification_settings
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();