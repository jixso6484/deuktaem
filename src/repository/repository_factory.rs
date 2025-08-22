use crate::config::SupabaseConfig;
use crate::repository::{
    DiscountRepository, ShopRepository, ProductRepository, 
    UserRepository, TranslationRepository, NotificationRepository
};

#[derive(Clone)]
pub struct RepositoryFactory {
    config: SupabaseConfig,
}

impl RepositoryFactory {
    pub fn new(config: SupabaseConfig) -> Self {
        Self { config }
    }

    // 공개 읽기용 Repository들 (RLS 적용, anon key 사용)
    pub fn public_discount_repo(&self) -> DiscountRepository {
        DiscountRepository::new(self.config.public_client())
    }

    pub fn public_shop_repo(&self) -> ShopRepository {
        ShopRepository::new(self.config.public_client())
    }

    pub fn public_product_repo(&self) -> ProductRepository {
        ProductRepository::new(self.config.public_client())
    }

    pub fn public_translation_repo(&self) -> TranslationRepository {
        TranslationRepository::new(self.config.public_client())
    }

    // 인증된 사용자용 Repository들 (RLS 적용, user token 사용)
    pub fn authenticated_user_repo(&self, user_token: &str) -> UserRepository {
        UserRepository::new(self.config.authenticated_client(user_token))
    }

    pub fn authenticated_notification_repo(&self, user_token: &str) -> NotificationRepository {
        let realtime_url = self.config.url.replace("https://", "wss://");
        NotificationRepository::new(
            self.config.authenticated_client(user_token),
            realtime_url,
            self.config.anon_key.clone()
        )
    }

    pub fn authenticated_discount_repo(&self, user_token: &str) -> DiscountRepository {
        DiscountRepository::new(self.config.authenticated_client(user_token))
    }

    pub fn authenticated_shop_repo(&self, user_token: &str) -> ShopRepository {
        ShopRepository::new(self.config.authenticated_client(user_token))
    }

    pub fn authenticated_product_repo(&self, user_token: &str) -> ProductRepository {
        ProductRepository::new(self.config.authenticated_client(user_token))
    }

    // 관리자용 Repository들 (RLS 우회, service key 사용)
    pub fn admin_discount_repo(&self) -> DiscountRepository {
        DiscountRepository::new(self.config.admin_client().clone())
    }

    pub fn admin_shop_repo(&self) -> ShopRepository {
        ShopRepository::new(self.config.admin_client().clone())
    }

    pub fn admin_product_repo(&self) -> ProductRepository {
        ProductRepository::new(self.config.admin_client().clone())
    }

    pub fn admin_user_repo(&self) -> UserRepository {
        UserRepository::new(self.config.admin_client().clone())
    }

    pub fn admin_translation_repo(&self) -> TranslationRepository {
        TranslationRepository::new(self.config.admin_client().clone())
    }

    pub fn admin_notification_repo(&self) -> NotificationRepository {
        let realtime_url = self.config.url.replace("https://", "wss://");
        NotificationRepository::new(
            self.config.admin_client().clone(),
            realtime_url,
            self.config.service_key.clone()
        )
    }
}

// 사용 예시를 위한 헬퍼 함수들
impl RepositoryFactory {
    // 공개 데이터 조회 (모든 사용자)
    pub fn for_public_access(&self) -> PublicRepositories {
        PublicRepositories {
            discount: self.public_discount_repo(),
            shop: self.public_shop_repo(),
            product: self.public_product_repo(),
            translation: self.public_translation_repo(),
        }
    }

    // 인증된 사용자 작업 (본인 데이터 수정, 구독 관리)
    pub fn for_authenticated_user(&self, user_token: &str) -> AuthenticatedRepositories {
        AuthenticatedRepositories {
            user: self.authenticated_user_repo(user_token),
            notification: self.authenticated_notification_repo(user_token),
            discount: self.authenticated_discount_repo(user_token),
            shop: self.authenticated_shop_repo(user_token),
            product: self.authenticated_product_repo(user_token),
        }
    }

    // 관리자 작업 (모든 데이터 관리)
    pub fn for_admin(&self) -> AdminRepositories {
        AdminRepositories {
            discount: self.admin_discount_repo(),
            shop: self.admin_shop_repo(),
            product: self.admin_product_repo(),
            user: self.admin_user_repo(),
            translation: self.admin_translation_repo(),
            notification: self.admin_notification_repo(),
        }
    }
}

pub struct PublicRepositories {
    pub discount: DiscountRepository,
    pub shop: ShopRepository,
    pub product: ProductRepository,
    pub translation: TranslationRepository,
}

pub struct AuthenticatedRepositories {
    pub user: UserRepository,
    pub notification: NotificationRepository,
    pub discount: DiscountRepository,
    pub shop: ShopRepository,
    pub product: ProductRepository,
}

pub struct AdminRepositories {
    pub discount: DiscountRepository,
    pub shop: ShopRepository,
    pub product: ProductRepository,
    pub user: UserRepository,
    pub translation: TranslationRepository,
    pub notification: NotificationRepository,
}