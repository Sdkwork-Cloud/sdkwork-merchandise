use std::sync::Arc;

use async_trait::async_trait;
use sdkwork_contract_service::CommerceServiceError;
use sdkwork_database_id::{IdGenerator, SnowflakeIdGenerator};
use sdkwork_database_sqlx::DatabasePool;
use sdkwork_merchandise_service::{
    CreateSingleSkuMerchandiseCommand, SingleSkuMerchandiseListQuery, SingleSkuMerchandisePage,
    SingleSkuMerchandiseRepositoryPort, SkuRecord, UpdateSingleSkuMerchandiseCommand,
};

mod postgres;
mod shared;
mod sqlite;

#[cfg(test)]
mod tests;

#[derive(Clone)]
pub struct SqlxSingleSkuMerchandiseRepository {
    pool: DatabasePool,
    id_generator: Arc<dyn IdGenerator>,
}

impl SqlxSingleSkuMerchandiseRepository {
    /// Creates the repository with an ID provider owned by the composition root.
    pub fn new(pool: DatabasePool, id_generator: Arc<dyn IdGenerator>) -> Self {
        Self { pool, id_generator }
    }

    /// Convenience constructor for hosts that already own a configured,
    /// collision-free Snowflake node id.
    pub fn with_snowflake_node_id(
        pool: DatabasePool,
        node_id: u16,
    ) -> Result<Self, CommerceServiceError> {
        let generator = SnowflakeIdGenerator::new(node_id)
            .map_err(|error| CommerceServiceError::storage(error.to_string()))?;
        Ok(Self::new(pool, Arc::new(generator)))
    }

    pub fn database_pool(&self) -> &DatabasePool {
        &self.pool
    }

    pub fn id_generator(&self) -> &dyn IdGenerator {
        self.id_generator.as_ref()
    }
}

#[async_trait]
impl SingleSkuMerchandiseRepositoryPort for SqlxSingleSkuMerchandiseRepository {
    async fn list_skus(
        &self,
        query: SingleSkuMerchandiseListQuery,
    ) -> Result<SingleSkuMerchandisePage, CommerceServiceError> {
        match &self.pool {
            DatabasePool::Sqlite(pool, _) => sqlite::list(pool, query).await,
            DatabasePool::Postgres(pool, _) => postgres::list(pool, query).await,
        }
    }

    async fn create_single_sku(
        &self,
        command: CreateSingleSkuMerchandiseCommand,
    ) -> Result<SkuRecord, CommerceServiceError> {
        match &self.pool {
            DatabasePool::Sqlite(pool, _) => {
                sqlite::create(pool, self.id_generator.as_ref(), command).await
            }
            DatabasePool::Postgres(pool, _) => {
                postgres::create(pool, self.id_generator.as_ref(), command).await
            }
        }
    }

    async fn update_single_sku(
        &self,
        command: UpdateSingleSkuMerchandiseCommand,
    ) -> Result<SkuRecord, CommerceServiceError> {
        match &self.pool {
            DatabasePool::Sqlite(pool, _) => sqlite::update(pool, command).await,
            DatabasePool::Postgres(pool, _) => postgres::update(pool, command).await,
        }
    }
}
