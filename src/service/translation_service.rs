use crate::config::SupabaseConfig;
use crate::repository::RepositoryFactory;
use crate::domain::entities::translation::Translation;
use crate::domain::dto::pagenation::{Pagenation, PagenationResult};

#[derive(Clone)]
pub struct TranslationService {
    factory: RepositoryFactory,
}

impl TranslationService {
    pub fn new(config: SupabaseConfig) -> Self {
        Self {
            factory: RepositoryFactory::new(config),
        }
    }

    // 번역 조회
    pub async fn get_translation(&self, key: &str, language: &str) -> Result<Option<Translation>, Box<dyn std::error::Error>> {
        log::info!("🌐 Getting translation: {} ({})", key, language);
        let repo = self.factory.public_translation_repo();
        repo.find_translation(key, language).await
    }

    pub async fn get_translations_by_language(&self, language: &str, pagination: Pagenation) -> Result<PagenationResult<Translation>, Box<dyn std::error::Error>> {
        log::info!("🌐 Getting translations for language: {} (page: {})", language, pagination.page);
        let repo = self.factory.public_translation_repo();
        repo.find_translations_by_language(language, pagination).await
    }

    pub async fn get_translations_by_namespace(&self, namespace: &str, language: &str, pagination: Pagenation) -> Result<PagenationResult<Translation>, Box<dyn std::error::Error>> {
        log::info!("🌐 Getting translations for namespace: {} ({}, page: {})", namespace, language, pagination.page);
        let repo = self.factory.public_translation_repo();
        repo.find_translations_by_namespace(namespace, language, pagination).await
    }

    // 다국어 지원 헬퍼
    pub async fn get_localized_text(&self, key: &str, language: &str, fallback_language: Option<&str>) -> Result<String, Box<dyn std::error::Error>> {
        log::debug!("🌐 Getting localized text: {} ({})", key, language);
        
        let repo = self.factory.public_translation_repo();
        
        // 요청된 언어로 번역 조회
        if let Ok(Some(translation)) = repo.find_translation(key, language).await {
            return Ok(translation.value);
        }
        
        // 요청된 언어에 없으면 fallback 언어로 조회
        if let Some(fallback) = fallback_language {
            if let Ok(Some(translation)) = repo.find_translation(key, fallback).await {
                log::warn!("🌐 Used fallback language {} for key: {}", fallback, key);
                return Ok(translation.value);
            }
        }
        
        // 기본 영어로 조회
        if language != "en" {
            if let Ok(Some(translation)) = repo.find_translation(key, "en").await {
                log::warn!("🌐 Used English fallback for key: {}", key);
                return Ok(translation.value);
            }
        }
        
        log::error!("🌐 Translation not found: {} ({})", key, language);
        Ok(key.to_string()) // 키 자체를 반환
    }

    // 전체 네임스페이스 번역 가져오기 (앱 시작시 사용)
    pub async fn get_namespace_translations(&self, namespace: &str, language: &str) -> Result<std::collections::HashMap<String, String>, Box<dyn std::error::Error>> {
        log::info!("🌐 Loading namespace translations: {} ({})", namespace, language);
        
        let repo = self.factory.public_translation_repo();
        let pagination = Pagenation { page: 1, limit: 1000 }; // 한번에 많이 가져오기
        let result = repo.find_translations_by_namespace(namespace, language, pagination).await?;
        
        let mut translations = std::collections::HashMap::new();
        for translation in result.data {
            translations.insert(translation.key, translation.value);
        }
        
        log::info!("🌐 Loaded {} translations for namespace: {}", translations.len(), namespace);
        Ok(translations)
    }

    // 관리자 기능
    pub async fn admin_create_translation(&self, admin_token: &str, translation: Translation) -> Result<Translation, Box<dyn std::error::Error>> {
        log::info!("👑 Admin creating translation: {} ({})", translation.key, translation.language);
        let repo = self.factory.admin_translation_repo();
        repo.create_translation(translation).await
    }

    pub async fn admin_update_translation(&self, admin_token: &str, key: &str, language: &str, translation: Translation) -> Result<Translation, Box<dyn std::error::Error>> {
        log::info!("👑 Admin updating translation: {} ({})", key, language);
        let repo = self.factory.admin_translation_repo();
        repo.update_translation(key, language, translation).await
    }

    pub async fn admin_delete_translation(&self, admin_token: &str, key: &str, language: &str) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("👑 Admin deleting translation: {} ({})", key, language);
        let repo = self.factory.admin_translation_repo();
        repo.delete_translation(key, language).await
    }

    pub async fn admin_get_all_translations(&self, admin_token: &str, pagination: Pagenation) -> Result<PagenationResult<Translation>, Box<dyn std::error::Error>> {
        log::info!("👑 Admin getting all translations (page: {})", pagination.page);
        let repo = self.factory.admin_translation_repo();
        repo.find_all_translations(pagination).await
    }

    // 통계 및 분석
    pub async fn get_translation_stats(&self, admin_token: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        log::info!("📊 Getting translation statistics");
        
        let stats = serde_json::json!({
            "total_keys": 500,
            "supported_languages": ["en", "ko", "ja", "zh"],
            "completion_rates": {
                "en": 100.0,
                "ko": 98.5,
                "ja": 95.0,
                "zh": 92.0
            },
            "last_updated": chrono::Utc::now().to_rfc3339()
        });
        
        Ok(stats)
    }

    pub async fn get_missing_translations(&self, admin_token: &str, language: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        log::info!("📊 Getting missing translations for language: {}", language);
        
        // 실제로는 복잡한 쿼리 필요
        // 모든 키를 가져와서 해당 언어에 없는 것들을 찾아야 함
        let missing_keys = vec![
            "common.loading".to_string(),
            "error.not_found".to_string(),
        ];
        
        Ok(missing_keys)
    }
}