# Supabase 다국어 테이블 마이그레이션 가이드

## 환경 정보
- **Project Reference**: vclkfwzyofntncglpcez
- **Access Token**: sbp_7bb72db9471673a10d11d81af173db07810e5400
- **Project URL**: https://vclkfwzyofntncglpcez.supabase.co

## 테이블 생성 방법

### 방법 1: Supabase CLI 사용 (권장)

```bash
# 1. Supabase CLI 설치
npm install -g supabase

# 2. 로그인
supabase login --token sbp_7bb72db9471673a10d11d81af173db07810e5400

# 3. 프로젝트 연결
supabase link --project-ref vclkfwzyofntncglpcez

# 4. 마이그레이션 실행
supabase db push ./migrations/create_i18n_tables.sql
```

### 방법 2: Supabase Dashboard 사용 (가장 간단)

1. [SQL Editor 열기](https://supabase.com/dashboard/project/vclkfwzyofntncglpcez/sql/new)
2. `migrations/create_i18n_tables.sql` 파일 내용 복사
3. SQL Editor에 붙여넣기
4. 'Run' 버튼 클릭

### 방법 3: PowerShell 스크립트 (Windows)

```powershell
# PowerShell에서 실행
$sql = Get-Content -Path ".\migrations\create_i18n_tables.sql" -Raw
$body = @{
    query = $sql
} | ConvertTo-Json

$headers = @{
    "apikey" = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InZjbGtmd3p5b2ZudG5jZ2xwY2V6Iiwicm9sZSI6ImFub24iLCJpYXQiOjE3NTQzNjg2ODIsImV4cCI6MjA2OTk0NDY4Mn0.MoxBlXh3HGQ20HIabDFx3UJi41_yClp-tkGuYARxw3o"
    "Authorization" = "Bearer sbp_7bb72db9471673a10d11d81af173db07810e5400"
    "Content-Type" = "application/json"
}

# 주의: 이 방법은 Supabase가 직접 SQL 실행 API를 제공하는 경우에만 작동
# 실제로는 Dashboard나 CLI 사용을 권장
```

## 생성될 테이블 목록

### 신규 테이블
1. **languages** - 지원 언어 목록
2. **product_translations** - 상품 번역
3. **discount_event_translations** - 할인 이벤트 번역
4. **discount_info_translations** - 할인 정보 번역
5. **notification_translations** - 알림 번역
6. **api_metrics** - API 메트릭 (누락된 테이블)

### 기본 지원 언어
- `ko` - 한국어
- `en` - English
- `ja` - 日本語
- `zh` - 中文
- `es` - Español
- `fr` - Français
- `de` - Deutsch

## 크롤링 시스템 연동

크롤링 시스템에서 데이터 삽입 예시:

```sql
-- 상품 번역 데이터 삽입
INSERT INTO product_translations (product_id, locale, name, description)
VALUES 
    (2, 'ko', '나이키 에어맥스', 'AI로 번역된 설명...'),
    (2, 'en', 'Nike Air Max', 'AI translated description...'),
    (2, 'ja', 'ナイキ エアマックス', 'AI翻訳された説明...');

-- 할인 이벤트 번역 데이터 삽입
INSERT INTO discount_event_translations (event_id, locale, title, description)
VALUES 
    (1, 'ko', '블랙 프라이데이 대할인', '연말 최대 할인!'),
    (1, 'en', 'Black Friday Sale', 'Year-end mega sale!'),
    (1, 'ja', 'ブラックフライデーセール', '年末大セール！');
```

## 알림 시스템 다국어 지원

사용자의 언어 설정에 따라 알림을 보낼 때:

```rust
// Rust 예시 코드
async fn send_notification(user_id: &str, notification_id: &str) {
    // 1. 사용자 프로필에서 언어 설정 가져오기
    let profile = get_user_profile(user_id).await;
    let locale = profile.language.unwrap_or("en".to_string());
    
    // 2. 해당 언어의 알림 번역 가져오기
    let translation = get_notification_translation(notification_id, &locale).await;
    
    // 3. 번역된 메시지로 알림 전송
    send_push_notification(user_id, &translation.message).await;
}
```

## 마이그레이션 후 확인

테이블이 제대로 생성되었는지 확인:

```sql
-- 테이블 목록 확인
SELECT table_name 
FROM information_schema.tables 
WHERE table_schema = 'public' 
AND table_name LIKE '%translation%';

-- 언어 데이터 확인
SELECT * FROM languages;
```

## 주의사항

1. **외래 키 제약**: 번역 테이블들은 원본 테이블의 ID를 참조하므로, 원본 데이터가 먼저 존재해야 합니다.
2. **유니크 제약**: 각 번역 테이블은 (entity_id, locale) 조합이 유니크합니다.
3. **자동 업데이트**: updated_at 컬럼은 트리거를 통해 자동으로 업데이트됩니다.
4. **인덱스**: 성능 최적화를 위한 인덱스가 자동으로 생성됩니다.