use axum::Router;
use sdkwork_database_sqlx::DatabasePool;
use sdkwork_merchandise_service_host::ShopServiceHost;
use sdkwork_routes_merchandise_app_api::{
    backend_catalog_router_with_postgres_pool, backend_catalog_router_with_sqlite_pool,
};
use std::sync::Arc;

use crate::web_bootstrap::wrap_router_with_web_framework_from_env;

pub fn build_merchandise_backend_router(host: Arc<ShopServiceHost>) -> Router {
    match host.database_pool() {
        DatabasePool::Postgres(pool, _) => {
            backend_catalog_router_with_postgres_pool(pool.clone())
        }
        DatabasePool::Sqlite(pool, _) => backend_catalog_router_with_sqlite_pool(pool.clone()),
    }
}

pub async fn build_merchandise_backend_router_with_framework(
    host: Arc<ShopServiceHost>,
) -> Router {
    wrap_router_with_web_framework_from_env(build_merchandise_backend_router(host)).await
}

pub use build_merchandise_backend_router_with_framework as build_shop_backend_router_with_framework;
