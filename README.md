# Duk - 종합 쇼핑 플랫폼 API

Rust + Supabase + Redis로 구현된 고성능 쇼핑 플랫폼 백엔드

## 🚀 주요 기능

- **상품 관리**: 다중 플랫폼 상품 정보 통합 관리
- **할인 추적**: 실시간 할인 정보 및 이벤트 관리  
- **인기 상품**: 클릭수 기반 상품/브랜드 랭킹
- **구독 시스템**: 매장/브랜드/카테고리/상품 구독 관리
- **쿠폰 관리**: 쿠폰 발급 및 사용 추적
- **알림 시스템**: 실시간 푸시 알림 및 큐 관리
- **다국어 지원**: 상품/브랜드/카테고리 다국어 번역
- **캐싱 최적화**: Redis 기반 성능 최적화
- **모니터링**: API 성능 및 에러 로그 추적

## 🛠 기술 스택

- **Rust** - Axum, Tokio, SQLx
- **Supabase** - PostgreSQL + Auth + RLS
- **Redis** - 캐싱 (Upstash 지원)
- **로깅** - tracing, log

## 📊 데이터베이스 구조

### 완전한 데이터베이스 스키마 (41개 테이블)

#### 🏪 매장/브랜드/카테고리 (7개 테이블) ✅ 확인됨
```sql
-- 매장 관리 (실제 컬럼 구조 확인됨)
shops (7 columns)               
  id: BIGSERIAL (i64)
  name: TEXT
  domain: TEXT  
  platform: TEXT
  logo_url: TEXT (nullable)
  created_at: TIMESTAMPTZ
  updated_at: TIMESTAMPTZ

brands (5 columns)              
  id: BIGSERIAL (i64)
  name: TEXT
  image_url: TEXT (nullable)
  created_at: TIMESTAMPTZ
  updated_at: TIMESTAMPTZ

categories (6 columns)          
  id: BIGSERIAL (i64)
  name: TEXT
  parent_id: BIGINT (nullable, FK to categories.id)
  icon: TEXT (nullable)
  created_at: TIMESTAMPTZ
  updated_at: TIMESTAMPTZ

-- 다국어 지원 (추가 확인 필요)
languages (6 columns)          -- 지원 언어
shop_translations (7 columns)  -- 매장 번역
brand_translations (7 columns) -- 브랜드 번역
category_translations (7 columns) -- 카테고리 번역
```

#### 🛍️ 상품 관리 (4개 테이블) ✅ 확인됨
```sql
products (10 columns)
  id: BIGSERIAL (i64)
  shop_id: BIGINT (FK to shops.id)
  brand_id: BIGINT (nullable, FK to brands.id)
  category_id: BIGINT (nullable, FK to categories.id)
  name: TEXT
  sku: TEXT (nullable)
  click_count: INTEGER (default 0)
  is_deleted: BOOLEAN (default false)
  created_at: TIMESTAMPTZ
  updated_at: TIMESTAMPTZ

active_products (9 columns)    -- 활성 상품 뷰 (추가 확인 필요)
popular_products (6 columns)   -- 인기 상품 랭킹 (추가 확인 필요)
top_popular_products (6 columns) -- 최상위 인기 상품 (추가 확인 필요)
```

#### 💰 할인/쿠폰 시스템 (9개 테이블) ✅ 부분 확인됨
```sql
-- 할인 정보 (실제 컬럼 구조 확인됨)
discount_infos (12 columns)
  id: BIGSERIAL (i64)
  product_id: BIGINT (FK to products.id)
  original_price: NUMERIC
  discount_price: NUMERIC
  discount_rate: NUMERIC
  start_at: TIMESTAMPTZ
  end_at: TIMESTAMPTZ
  info_url: TEXT (nullable)
  thumbnail_url: TEXT (nullable)
  click_count: INTEGER (default 0)
  created_at: TIMESTAMPTZ
  updated_at: TIMESTAMPTZ

-- 추가 확인 필요 테이블들
discount_events (15 columns)   -- 할인 이벤트
discount_event_products (5 columns) -- 이벤트-상품 연결
active_events (23 columns)     -- 활성 이벤트 상세
discount_overview (9 columns)  -- 할인 요약
discount_shipping (3 columns)  -- 할인 배송
coupons (11 columns)           -- 쿠폰 정보 (빈 테이블)
coupon_overview (10 columns)   -- 쿠폰 요약  
coupon_shipping (3 columns)    -- 쿠폰 배송
```

#### 👥 사용자/구독 관리 (7개 테이블) ✅ 부분 확인됨
```sql
-- 사용자 관리 (실제 컬럼 구조 확인됨)
profiles (8 columns)
  user_id: UUID (FK to auth.users)
  avatar_url: TEXT (nullable)
  email: TEXT
  preferred_country: TEXT (nullable)
  detected_country: TEXT (nullable)
  language: TEXT (nullable)
  timezone: TEXT (nullable)
  created_at: TIMESTAMPTZ
  updated_at: TIMESTAMPTZ

user_roles (2 columns)         -- 사용자 권한 (추가 확인 필요)

-- 구독 시스템 (빈 테이블들)
product_subscriptions (3 columns)  -- 상품 구독
brand_subscriptions (3 columns)    -- 브랜드 구독
category_subscriptions (5 columns) -- 카테고리 구독
shop_subscriptions (5 columns)     -- 매장 구독
shipping_regions (5 columns)   -- 배송 지역
```

#### 🔔 알림 시스템 (5개 테이블)
```sql
notifications (8 columns)      -- 알림 정보 (id, user_id, title, content, type, is_read, data, created_at)
notification_queue (11 columns) -- 알림 큐
notification_logs (8 columns)  -- 알림 로그
notification_settings (8 columns) -- 사용자별 푸시 알림 설정
notification_translations (7 columns) -- 알림 번역
```

#### 🌐 다국어 지원 (4개 테이블)
```sql
product_translations (7 columns)     -- 상품 번역 (id, product_id, locale, name, description, created_at, updated_at)
discount_info_translations (7 columns) -- 할인 정보 번역
discount_event_translations (7 columns) -- 할인 이벤트 번역
[shop_translations, brand_translations, category_translations 위에 포함]
```

#### 📈 모니터링/로깅 (6개 테이블)
```sql
-- 시스템 로그
system_logs (14 columns)       -- 시스템 로그 (id, level, message, context, user_id, ip_address, user_agent, created_at, ...)
error_log_summary (5 columns)  -- 에러 로그 요약

-- 성능 모니터링
api_metrics (8 columns)        -- API 메트릭 (id, endpoint, method, status_code, response_time, user_id, ip, created_at)
api_performance_summary (6 columns) -- API 성능 요약

-- 캐시 모니터링
cache_logs (9 columns)         -- 캐시 로그
cache_hit_rate_summary (4 columns) -- 캐시 히트율 요약
```

### 🔗 주요 테이블 관계
```
users (Supabase Auth)
  ├── profiles (1:1)
  ├── user_roles (1:1)
  ├── product_subscriptions (1:N)
  ├── brand_subscriptions (1:N)
  ├── shop_subscriptions (1:N)
  └── notification_settings (1:1)

shops
  ├── products (1:N)
  ├── shop_translations (1:N)
  └── shop_subscriptions (1:N)

products
  ├── discount_infos (1:N)
  ├── product_translations (1:N)
  ├── product_subscriptions (1:N)
  └── brands, categories (N:1)

discount_infos
  ├── discount_info_translations (1:N)
  └── products (N:1)
```

## 🏃 실행 방법

```bash
# 환경변수 설정 (.env 파일)
SUPABASE_URL=your_supabase_url
SUPABASE_ANON_KEY=your_supabase_anon_key  
SUPABASE_SERVICE_KEY=your_supabase_service_key
REDIS_URL=redis://localhost:6379

# 실행
cargo run
```

## 📡 구현 필요한 API 엔드포인트

### 🛍️ 상품 관리 API
```
GET    /api/v1/products                     # 상품 목록 조회 (필터링, 페이징)
GET    /api/v1/products/{id}                # 상품 상세 정보
GET    /api/v1/products/popular/{country}   # 인기 상품 (국가별)
POST   /api/v1/products/{id}/click          # 상품 클릭 수 증가
GET    /api/v1/products/active              # 활성 상품 목록
GET    /api/v1/products/search              # 상품 검색 (키워드, 카테고리)
```

### 🏪 매장/브랜드 API
```
GET    /api/v1/shops                        # 매장 목록
GET    /api/v1/shops/{id}                   # 매장 상세 정보
GET    /api/v1/brands                       # 브랜드 목록
GET    /api/v1/brands/{id}                  # 브랜드 상세 정보
GET    /api/v1/categories                   # 카테고리 목록 (계층형)
```

### 💰 할인/쿠폰 API
```
GET    /api/v1/discounts                    # 할인 정보 목록
GET    /api/v1/discounts/{id}               # 할인 상세 정보
GET    /api/v1/discounts/events/active      # 진행중인 할인 이벤트
POST   /api/v1/discounts/{id}/click         # 할인 클릭 수 증가
GET    /api/v1/coupons                      # 쿠폰 목록
GET    /api/v1/coupons/{id}                 # 쿠폰 상세 정보
POST   /api/v1/coupons/{id}/use             # 쿠폰 사용
```

### 👥 구독 관리 API  
```
POST   /api/v1/subscriptions/products       # 상품 구독 추가
DELETE /api/v1/subscriptions/products/{id}  # 상품 구독 해제
POST   /api/v1/subscriptions/brands         # 브랜드 구독 추가
DELETE /api/v1/subscriptions/brands/{id}    # 브랜드 구독 해제
POST   /api/v1/subscriptions/shops          # 매장 구독 추가
DELETE /api/v1/subscriptions/shops/{id}     # 매장 구독 해제
POST   /api/v1/subscriptions/categories     # 카테고리 구독 추가
DELETE /api/v1/subscriptions/categories/{id} # 카테고리 구독 해제
GET    /api/v1/subscriptions/my             # 내 구독 목록
```

### 🔔 알림 API
```
GET    /api/v1/notifications               # 내 알림 목록
POST   /api/v1/notifications/{id}/read     # 알림 읽음 처리
PUT    /api/v1/notifications/settings      # 알림 설정 변경
GET    /api/v1/notifications/settings      # 알림 설정 조회
POST   /api/v1/notifications/test          # 테스트 알림 발송
```

### 🌐 다국어 API
```
GET    /api/v1/translations/products/{id}   # 상품 다국어 정보
GET    /api/v1/translations/brands/{id}     # 브랜드 다국어 정보  
GET    /api/v1/translations/categories/{id} # 카테고리 다국어 정보
GET    /api/v1/languages                    # 지원 언어 목록
```

### 👤 사용자 API
```
GET    /api/v1/profiles/me                 # 내 프로필 조회
PUT    /api/v1/profiles/me                 # 내 프로필 수정
GET    /api/v1/profiles/stats              # 사용자 활동 통계
```

### 📈 모니터링 API (관리자)
```
GET    /api/v1/admin/metrics/api           # API 성능 메트릭
GET    /api/v1/admin/logs/errors           # 에러 로그 요약
GET    /api/v1/admin/cache/stats           # 캐시 통계
GET    /api/v1/admin/system/health         # 시스템 상태 점검
```

## 🔧 구현 우선순위

### Phase 1: 핵심 기능
1. 상품 조회/검색 API
2. 인기 상품 API (클릭 기반)
3. 할인 정보 API
4. 기본 Redis 캐싱

### Phase 2: 구독 시스템
1. 상품/브랜드/매장 구독 API
2. 알림 시스템 기본 구현
3. 사용자 프로필 관리

### Phase 3: 고급 기능
1. 쿠폰 시스템
2. 다국어 지원
3. 실시간 알림 (WebSocket)
4. 관리자 모니터링 대시보드

### Phase 4: 최적화
1. 캐싱 전략 고도화
2. API 성능 모니터링
3. 로그 분석 시스템
4. 부하 테스트 및 최적화
