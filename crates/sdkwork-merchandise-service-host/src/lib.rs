use sdkwork_database_sqlx::DatabasePool;
use sdkwork_merchandise_database_host::{bootstrap_shop_database_from_env, ShopDatabaseHost};
use sdkwork_merchandise_repository_sqlx::SqlxShopRepository;
use sdkwork_merchandise_service::ShopService;

pub struct ShopServiceHost {
    database: ShopDatabaseHost,
    shop_service: ShopService<SqlxShopRepository>,
}

impl ShopServiceHost {
    pub async fn new() -> Self {
        Self::from_env()
            .await
            .expect("merchandise service host bootstrap failed")
    }

    pub async fn from_env() -> Result<Self, String> {
        let database = bootstrap_shop_database_from_env().await?;
        let repository = SqlxShopRepository::new(database.pool().clone());
        Ok(Self {
            shop_service: ShopService::new(repository),
            database,
        })
    }

    pub fn shop_service(&self) -> &ShopService<SqlxShopRepository> {
        &self.shop_service
    }

    pub fn database_pool(&self) -> &DatabasePool {
        self.database.pool()
    }

    pub fn database_module(&self) -> std::sync::Arc<sdkwork_database_spi::DefaultDatabaseModule> {
        self.database.module()
    }
}

pub fn default_seed_locale() -> &'static str {
    "zh-CN"
}

pub fn default_seed_profile() -> &'static str {
    "standard"
}
