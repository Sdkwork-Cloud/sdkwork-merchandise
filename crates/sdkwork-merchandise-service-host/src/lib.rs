use sdkwork_database_sqlx::{process_shared_database_pool, DatabasePool};
use sdkwork_merchandise_database_host::bootstrap_merchandise_database_from_env;

pub struct MerchandiseServiceHost {
    database_pool: DatabasePool,
}

impl MerchandiseServiceHost {
    pub async fn new() -> Self {
        Self::from_env()
            .await
            .expect("merchandise service host bootstrap failed")
    }

    pub async fn from_env() -> Result<Self, String> {
        if let Some(database_pool) = process_shared_database_pool() {
            return Ok(Self { database_pool });
        }

        let database = bootstrap_merchandise_database_from_env().await?;
        Ok(Self {
            database_pool: database.pool().clone(),
        })
    }

    pub fn database_pool(&self) -> &DatabasePool {
        &self.database_pool
    }
}

pub fn default_seed_locale() -> &'static str {
    "zh-CN"
}

pub fn default_seed_profile() -> &'static str {
    "standard"
}
