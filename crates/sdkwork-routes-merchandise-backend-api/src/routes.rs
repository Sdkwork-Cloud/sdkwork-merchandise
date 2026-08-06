use axum::Router;
use sdkwork_database_sqlx::DatabasePool;
use sdkwork_merchandise_service_host::MerchandiseServiceHost;
use sdkwork_merchandise_web_support::backend_catalog_router_with_postgres_pool;
use std::sync::Arc;

use crate::web_bootstrap::wrap_router_with_web_framework_from_env;

pub fn build_merchandise_backend_router(host: Arc<MerchandiseServiceHost>) -> Router {
    // 服务端权威持久化仅支持 PostgreSQL（DATABASE_SPEC：authoritative-server）
    let DatabasePool::Postgres(pool, _) = host.database_pool() else {
        panic!("merchandise backend router requires a PostgreSQL database pool");
    };
    backend_catalog_router_with_postgres_pool(pool.clone())
}

pub async fn build_merchandise_backend_router_with_framework(
    host: Arc<MerchandiseServiceHost>,
) -> Router {
    wrap_router_with_web_framework_from_env(build_merchandise_backend_router(host)).await
}
