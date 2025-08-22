-- 상품 테이블에 클릭수 필드 추가 (간단하게!)
ALTER TABLE products 
ADD COLUMN IF NOT EXISTS click_count INT DEFAULT 0;

-- 할인 정보에 클릭수 필드 추가 (올바른 테이블명 사용)
ALTER TABLE discount_infos 
ADD COLUMN IF NOT EXISTS click_count INT DEFAULT 0;

-- 인기 상품 조회를 위한 인덱스
CREATE INDEX IF NOT EXISTS idx_products_clicks ON products(click_count DESC, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_discount_infos_clicks ON discount_infos(click_count DESC, created_at DESC);

-- 클릭수 증가 함수 (올바른 테이블명 사용)
CREATE OR REPLACE FUNCTION increment_click_count(entity_type TEXT, entity_id BIGINT)
RETURNS VOID AS $$
BEGIN
    IF entity_type = 'product' THEN
        UPDATE products 
        SET click_count = click_count + 1
        WHERE id = entity_id;
    ELSIF entity_type = 'discount' THEN
        UPDATE discount_infos 
        SET click_count = click_count + 1
        WHERE id = entity_id;
    END IF;
END;
$$ LANGUAGE plpgsql;