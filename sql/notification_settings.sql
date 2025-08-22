-- 🔧 알림 설정 테이블 생성
CREATE TABLE IF NOT EXISTS notification_settings (
    user_id UUID PRIMARY KEY REFERENCES auth.users(id) ON DELETE CASCADE,
    push_enabled BOOLEAN NOT NULL DEFAULT true,
    discount_updates BOOLEAN NOT NULL DEFAULT true,
    subscription_changes BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- 🔐 RLS 정책 설정
ALTER TABLE notification_settings ENABLE ROW LEVEL SECURITY;

-- 사용자는 자신의 설정만 조회 가능
CREATE POLICY "Users can view own notification settings" ON notification_settings
    FOR SELECT USING (auth.uid() = user_id);

-- 사용자는 자신의 설정만 수정 가능  
CREATE POLICY "Users can update own notification settings" ON notification_settings
    FOR UPDATE USING (auth.uid() = user_id);

-- 사용자는 자신의 설정만 생성 가능
CREATE POLICY "Users can insert own notification settings" ON notification_settings
    FOR INSERT WITH CHECK (auth.uid() = user_id);

-- 사용자는 자신의 설정만 삭제 가능
CREATE POLICY "Users can delete own notification settings" ON notification_settings
    FOR DELETE USING (auth.uid() = user_id);

-- 🕒 업데이트 시간 자동 갱신 트리거
CREATE OR REPLACE FUNCTION update_notification_settings_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_notification_settings_updated_at
    BEFORE UPDATE ON notification_settings
    FOR EACH ROW
    EXECUTE FUNCTION update_notification_settings_updated_at();

-- 📝 테이블 및 컬럼 설명
COMMENT ON TABLE notification_settings IS '사용자별 실시간 알림 설정';
COMMENT ON COLUMN notification_settings.user_id IS '사용자 ID (auth.users 참조)';
COMMENT ON COLUMN notification_settings.push_enabled IS '푸시 알림 전체 활성화 여부';
COMMENT ON COLUMN notification_settings.discount_updates IS '할인 정보 업데이트 알림 수신 여부';
COMMENT ON COLUMN notification_settings.subscription_changes IS '구독 변경 알림 수신 여부 (브랜드/카테고리/매장)';