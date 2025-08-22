#!/bin/bash

# Supabase 프로젝트 정보
SUPABASE_PROJECT_REF="vclkfwzyofntncglpcez"
SUPABASE_ACCESS_TOKEN="sbp_7bb72db9471673a10d11d81af173db07810e5400"
SUPABASE_DB_URL="https://vclkfwzyofntncglpcez.supabase.co"

echo "=== Supabase 다국어 테이블 생성 ==="
echo ""
echo "다음 방법 중 하나를 선택하세요:"
echo ""
echo "방법 1: Supabase CLI 사용 (권장)"
echo "----------------------------------------"
echo "1. Supabase CLI 설치:"
echo "   npm install -g supabase"
echo ""
echo "2. 로그인:"
echo "   supabase login --token ${SUPABASE_ACCESS_TOKEN}"
echo ""
echo "3. 프로젝트 연결:"
echo "   supabase link --project-ref ${SUPABASE_PROJECT_REF}"
echo ""
echo "4. 마이그레이션 실행:"
echo "   supabase db push ./migrations/create_i18n_tables.sql"
echo ""
echo ""
echo "방법 2: Supabase Dashboard 사용"
echo "----------------------------------------"
echo "1. https://supabase.com/dashboard/project/${SUPABASE_PROJECT_REF}/sql/new 접속"
echo "2. migrations/create_i18n_tables.sql 파일 내용 복사"
echo "3. SQL Editor에 붙여넣기"
echo "4. 'Run' 버튼 클릭"
echo ""
echo ""
echo "방법 3: API를 통한 직접 실행"
echo "----------------------------------------"
echo "curl 명령어로 직접 실행 (Windows에서는 Git Bash 또는 WSL 필요):"
echo ""

cat << 'EOF'
# SQL 파일 내용을 변수에 저장
SQL_CONTENT=$(cat ./migrations/create_i18n_tables.sql)

# API 호출
curl -X POST "https://vclkfwzyofntncglpcez.supabase.co/rest/v1/rpc/exec_sql" \
  -H "apikey: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InZjbGtmd3p5b2ZudG5jZ2xwY2V6Iiwicm9sZSI6ImFub24iLCJpYXQiOjE3NTQzNjg2ODIsImV4cCI6MjA2OTk0NDY4Mn0.MoxBlXh3HGQ20HIabDFx3UJi41_yClp-tkGuYARxw3o" \
  -H "Authorization: Bearer sbp_7bb72db9471673a10d11d81af173db07810e5400" \
  -H "Content-Type: application/json" \
  -d "{\"query\": \"$SQL_CONTENT\"}"
EOF

echo ""
echo ""
echo "=== 참고사항 ==="
echo "- 테이블 생성 후 크롤링 시스템에서 AI로 번역된 데이터를 삽입할 수 있습니다"
echo "- 각 translation 테이블은 locale 컬럼으로 언어를 구분합니다"
echo "- 지원 언어: ko, en, ja, zh, es, fr, de"