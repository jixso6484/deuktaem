import os
from dotenv import load_dotenv
import requests

load_dotenv()

url = os.getenv("SUPABASE_URL")
anon_key = os.getenv("SUPABASE_ANON_KEY")

if not url or not anon_key:
    print("❌ SUPABASE_URL 또는 SUPABASE_ANON_KEY가 설정되지 않았습니다.")
    exit(1)

headers = {
    "apikey": anon_key,
    "Authorization": f"Bearer {anon_key}",
    "Content-Type": "application/json"
}

print("🔍 Supabase 데이터베이스 테이블 구조 확인 중...")

# 1. 테이블 목록 확인
print("\n📋 테이블 목록:")
try:
    response = requests.get(
        f"{url}/rest/v1/information_schema.tables?table_schema=eq.public&select=table_name",
        headers=headers
    )
    if response.status_code == 200:
        tables = response.json()
        for table in tables:
            print(f"  • {table['table_name']}")
    else:
        print(f"❌ 테이블 목록 조회 실패: {response.status_code}")
        print(response.text)
except Exception as e:
    print(f"❌ 에러: {e}")

# 2. 각 주요 테이블의 컬럼 구조 확인
main_tables = ["shops", "products", "discount_infos", "users", "profiles"]

for table_name in main_tables:
    print(f"\n🔍 {table_name} 테이블 컬럼 구조:")
    try:
        response = requests.get(
            f"{url}/rest/v1/information_schema.columns?table_name=eq.{table_name}&table_schema=eq.public&select=column_name,data_type,is_nullable",
            headers=headers
        )
        if response.status_code == 200:
            columns = response.json()
            if columns:
                for col in columns:
                    nullable = "NULL" if col['is_nullable'] == 'YES' else "NOT NULL"
                    print(f"  • {col['column_name']}: {col['data_type']} ({nullable})")
            else:
                print(f"  ❌ {table_name} 테이블이 존재하지 않습니다.")
        else:
            print(f"  ❌ 컬럼 조회 실패: {response.status_code}")
    except Exception as e:
        print(f"  ❌ 에러: {e}")

print("\n✅ 데이터베이스 구조 확인 완료!")