# Duk Database Schema

## 📋 Overview

Duk 프로젝트의 Supabase 데이터베이스 스키마 문서입니다.

## 🗄️ Tables

### 1. 상점 관련 테이블

#### `shops` - 상점 정보
```sql
CREATE TABLE shops (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    domain TEXT NOT NULL UNIQUE,
    platform TEXT NOT NULL,
    logo_url TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

#### `brands` - 브랜드 정보
```sql
CREATE TABLE brands (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    image_url TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

#### `categories` - 카테고리 정보
```sql
CREATE TABLE categories (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    parent_id TEXT REFERENCES categories(id),
    level INTEGER NOT NULL DEFAULT 0,
    path TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

### 2. 상품 관련 테이블

#### `products` - 상품 정보
```sql
CREATE TABLE products (
    id TEXT PRIMARY KEY,
    shop_id TEXT NOT NULL REFERENCES shops(id),
    brand_id TEXT REFERENCES brands(id),
    category_id TEXT REFERENCES categories(id),
    name TEXT NOT NULL,
    sku TEXT,
    is_deleted BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

### 3. 할인 관련 테이블

#### `discount_info` - 할인 정보
```sql
CREATE TABLE discount_info (
    id TEXT PRIMARY KEY,
    product_id TEXT NOT NULL REFERENCES products(id),
    shop_id TEXT NOT NULL REFERENCES shops(id),
    brand_id TEXT REFERENCES brands(id),
    original_price DECIMAL(10,2) NOT NULL,
    discount_price DECIMAL(10,2) NOT NULL,
    discount_rate DECIMAL(5,2) NOT NULL,
    currency TEXT NOT NULL DEFAULT 'KRW',
    start_at TIMESTAMPTZ NOT NULL,
    end_at TIMESTAMPTZ NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    source_url TEXT,
    is_auto_discovered BOOLEAN DEFAULT FALSE,
    is_event_based BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

#### `discount_events` - 할인 이벤트
```sql
CREATE TABLE discount_events (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    event_type TEXT NOT NULL,
    discount_rate DECIMAL(5,2),
    start_date TIMESTAMPTZ NOT NULL,
    end_date TIMESTAMPTZ NOT NULL,
    banner_image_url TEXT,
    is_featured BOOLEAN DEFAULT FALSE,
    status TEXT NOT NULL DEFAULT 'active',
    shop_id TEXT REFERENCES shops(id),
    brand_id TEXT REFERENCES brands(id),
    category_id TEXT REFERENCES categories(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

### 4. 사용자 관련 테이블

#### `profiles` - 사용자 프로필 (Auth 연동)
```sql
CREATE TABLE profiles (
    user_id UUID PRIMARY KEY REFERENCES auth.users(id),
    avatar_url TEXT,
    email TEXT NOT NULL,
    preferred_country TEXT,
    detected_country TEXT,
    language TEXT DEFAULT 'ko',
    timezone TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

#### `shop_subscriptions` - 상점 구독
```sql
CREATE TABLE shop_subscriptions (
    id TEXT PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES auth.users(id),
    shop_id TEXT NOT NULL REFERENCES shops(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, shop_id)
);
```

#### `brand_subscriptions` - 브랜드 구독
```sql
CREATE TABLE brand_subscriptions (
    id TEXT PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES auth.users(id),
    brand_id TEXT NOT NULL REFERENCES brands(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, brand_id)
);
```

#### `category_subscriptions` - 카테고리 구독
```sql
CREATE TABLE category_subscriptions (
    id TEXT PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES auth.users(id),
    category_id TEXT NOT NULL REFERENCES categories(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, category_id)
);
```

### 5. 알림 관련 테이블

#### `notifications` - 알림
```sql
CREATE TABLE notifications (
    id TEXT PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES auth.users(id),
    actor_id UUID REFERENCES auth.users(id),
    type TEXT NOT NULL,
    target_type TEXT,
    target_id TEXT,
    read_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

#### `notification_logs` - 알림 로그
```sql
CREATE TABLE notification_logs (
    id TEXT PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES auth.users(id),
    subscription_type TEXT NOT NULL,
    target_id TEXT NOT NULL,
    message TEXT NOT NULL,
    is_read BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

### 6. 다국어 지원 테이블

#### `languages` - 지원 언어
```sql
CREATE TABLE languages (
    code TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    native_name TEXT NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

#### Translation 테이블들
```sql
-- 상점 번역
CREATE TABLE shop_translations (
    id TEXT PRIMARY KEY,
    shop_id TEXT NOT NULL REFERENCES shops(id),
    locale TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(shop_id, locale)
);

-- 브랜드 번역
CREATE TABLE brand_translations (
    id TEXT PRIMARY KEY,
    brand_id TEXT NOT NULL REFERENCES brands(id),
    locale TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(brand_id, locale)
);

-- 카테고리 번역
CREATE TABLE category_translations (
    id TEXT PRIMARY KEY,
    category_id TEXT NOT NULL REFERENCES categories(id),
    locale TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(category_id, locale)
);

-- 상품 번역
CREATE TABLE product_translations (
    id TEXT PRIMARY KEY,
    product_id TEXT NOT NULL REFERENCES products(id),
    locale TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(product_id, locale)
);

-- 할인 이벤트 번역
CREATE TABLE discount_event_translations (
    id TEXT PRIMARY KEY,
    event_id TEXT NOT NULL REFERENCES discount_events(id),
    locale TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(event_id, locale)
);

-- 할인 정보 번역
CREATE TABLE discount_info_translations (
    id TEXT PRIMARY KEY,
    discount_info_id TEXT NOT NULL REFERENCES discount_info(id),
    locale TEXT NOT NULL,
    description TEXT,
    terms_conditions TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(discount_info_id, locale)
);

-- 알림 번역
CREATE TABLE notification_translations (
    id TEXT PRIMARY KEY,
    notification_id TEXT NOT NULL REFERENCES notifications(id),
    locale TEXT NOT NULL,
    title TEXT,
    message TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(notification_id, locale)
);
```

### 7. 시스템 관련 테이블

#### `api_metrics` - API 메트릭
```sql
CREATE TABLE api_metrics (
    id TEXT PRIMARY KEY,
    endpoint TEXT NOT NULL,
    method TEXT NOT NULL,
    status_code INTEGER NOT NULL,
    response_time DECIMAL(8,3) NOT NULL,
    error_message TEXT,
    metadata JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

## 🔒 Row Level Security (RLS) 정책

### 공개 읽기 테이블
- `shops`, `brands`, `categories`, `products`, `discount_info`, `discount_events`
- 모든 사용자가 읽기 가능

```sql
-- 예시: shops 테이블
ALTER TABLE shops ENABLE ROW LEVEL SECURITY;
CREATE POLICY "Public read access" ON shops FOR SELECT USING (true);
```

### 사용자 전용 테이블
- `profiles`, `*_subscriptions`, `notifications`, `notification_logs`
- 본인 데이터만 접근 가능

```sql
-- 예시: profiles 테이블
ALTER TABLE profiles ENABLE ROW LEVEL SECURITY;
CREATE POLICY "Users can view own profile" ON profiles 
FOR SELECT USING (auth.uid() = user_id);
CREATE POLICY "Users can update own profile" ON profiles 
FOR UPDATE USING (auth.uid() = user_id);
```

### 관리자 전용 테이블
- `*_translations`, `api_metrics`
- Service Key로만 접근 가능

## 🔴 Realtime 설정

### 구독 가능한 테이블
- `notifications` - 실시간 알림
- `discount_info` - 실시간 할인 정보
- `*_subscriptions` - 실시간 구독 변경

```sql
-- Realtime 활성화
ALTER PUBLICATION supabase_realtime ADD TABLE notifications;
ALTER PUBLICATION supabase_realtime ADD TABLE discount_info;
ALTER PUBLICATION supabase_realtime ADD TABLE shop_subscriptions;
ALTER PUBLICATION supabase_realtime ADD TABLE brand_subscriptions;
ALTER PUBLICATION supabase_realtime ADD TABLE category_subscriptions;
```

## 📊 인덱스

### 성능 최적화 인덱스
```sql
-- 상품 조회 최적화
CREATE INDEX idx_products_shop_id ON products(shop_id);
CREATE INDEX idx_products_brand_id ON products(brand_id);
CREATE INDEX idx_products_category_id ON products(category_id);
CREATE INDEX idx_products_is_deleted ON products(is_deleted);

-- 할인 정보 조회 최적화
CREATE INDEX idx_discount_info_product_id ON discount_info(product_id);
CREATE INDEX idx_discount_info_shop_id ON discount_info(shop_id);
CREATE INDEX idx_discount_info_is_active ON discount_info(is_active);
CREATE INDEX idx_discount_info_start_end ON discount_info(start_at, end_at);

-- 구독 조회 최적화
CREATE INDEX idx_shop_subscriptions_user_id ON shop_subscriptions(user_id);
CREATE INDEX idx_brand_subscriptions_user_id ON brand_subscriptions(user_id);
CREATE INDEX idx_category_subscriptions_user_id ON category_subscriptions(user_id);

-- 알림 조회 최적화
CREATE INDEX idx_notifications_user_id ON notifications(user_id);
CREATE INDEX idx_notifications_created_at ON notifications(created_at DESC);
CREATE INDEX idx_notifications_read_at ON notifications(read_at);

-- 번역 조회 최적화
CREATE INDEX idx_shop_translations_shop_locale ON shop_translations(shop_id, locale);
CREATE INDEX idx_brand_translations_brand_locale ON brand_translations(brand_id, locale);
CREATE INDEX idx_category_translations_category_locale ON category_translations(category_id, locale);
CREATE INDEX idx_product_translations_product_locale ON product_translations(product_id, locale);
```

## 🔧 트리거 함수

### 자동 updated_at 업데이트
```sql
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- 각 테이블에 트리거 적용
CREATE TRIGGER update_shops_updated_at BEFORE UPDATE ON shops
FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_brands_updated_at BEFORE UPDATE ON brands
FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- (다른 테이블들도 동일하게 적용)
```

## 🚀 초기 데이터

### 기본 언어 설정
```sql
INSERT INTO languages (code, name, native_name, is_active) VALUES
('ko', 'Korean', '한국어', true),
('en', 'English', 'English', true),
('ja', 'Japanese', '日本語', true),
('zh', 'Chinese', '中文', true);
```

## 📝 Migration 순서

1. **기본 테이블 생성** (shops, brands, categories, products)
2. **사용자 관련 테이블 생성** (profiles, subscriptions)
3. **할인 관련 테이블 생성** (discount_info, discount_events)
4. **알림 테이블 생성** (notifications, notification_logs)
5. **번역 테이블 생성** (모든 *_translations)
6. **시스템 테이블 생성** (api_metrics, languages)
7. **RLS 정책 적용**
8. **인덱스 생성**
9. **트리거 함수 적용**
10. **Realtime 설정**
11. **초기 데이터 삽입**