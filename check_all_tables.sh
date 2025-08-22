#!/bin/bash
URL="https://vclkfwzyofntncglpcez.supabase.co/rest/v1"
KEY="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InZjbGtmd3p5b2ZudG5jZ2xwY2V6Iiwicm9sZSI6ImFub24iLCJpYXQiOjE3NTQzNjg2ODIsImV4cCI6MjA2OTk0NDY4Mn0.MoxBlXh3HGQ20HIabDFx3UJi41_yClp-tkGuYARxw3o"

echo "🔍 모든 테이블 구조 확인 중..."

# 시도할 테이블 목록
TABLES=("shops" "products" "discount_infos" "brands" "categories" "users" "profiles" "shop_subscriptions" "brand_subscriptions" "category_subscriptions" "notifications" "notification_settings" "languages" "product_translations" "discount_event_translations")

for table in "${TABLES[@]}"; do
    echo ""
    echo "📋 $table 테이블:"
    response=$(curl -s -X GET "$URL/$table?limit=1" -H "apikey: $KEY" -H "Authorization: Bearer $KEY")
    
    if [[ $response == *"\"code\":"* ]] && [[ $response == *"\"message\":"* ]]; then
        echo "  ❌ 테이블 없음 또는 접근 불가"
    elif [[ $response == "[]" ]]; then
        echo "  ✅ 테이블 존재, 데이터 없음"
    else
        echo "  ✅ 테이블 존재, 샘플 데이터:"
        echo "  $response" | head -c 200
        echo "..."
    fi
done

echo ""
echo "✅ 모든 테이블 확인 완료!"