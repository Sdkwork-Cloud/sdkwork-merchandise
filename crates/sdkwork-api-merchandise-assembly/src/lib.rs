//! API assembly for sdkwork-merchandise.
//! Application bootstrap lives in `bootstrap.rs`; route inventory is in `assembly-manifest.json`.
// SDKWORK-ASSEMBLY-LIB-CUSTOM

mod bootstrap;
mod generated;

pub use bootstrap::{assemble_api_router, ApiAssembly, ApiAssemblyContext};

pub async fn assemble_api_router_from_env() -> Result<ApiAssembly, String> {
    let host = std::sync::Arc::new(
        sdkwork_merchandise_service_host::MerchandiseServiceHost::from_env().await?,
    );
    let readiness_check = std::sync::Arc::new(
        sdkwork_web_bootstrap::DatabasePoolReadinessCheck::new(host.database_pool().clone()),
    );
    assemble_api_router(ApiAssemblyContext {
        host,
        domain_context_injectors: Vec::new(),
        readiness_check,
    })
    .await
}

/// Builds the complete Merchandise contribution against the host process pool.
pub async fn assemble_api_router_with_pool(
    pool: sdkwork_database_sqlx::DatabasePool,
) -> Result<ApiAssembly, String> {
    let host = std::sync::Arc::new(
        sdkwork_merchandise_service_host::MerchandiseServiceHost::from_pool(pool).await?,
    );
    let readiness_check = std::sync::Arc::new(
        sdkwork_web_bootstrap::DatabasePoolReadinessCheck::new(host.database_pool().clone()),
    );
    assemble_api_router(ApiAssemblyContext {
        host,
        domain_context_injectors: Vec::new(),
        readiness_check,
    })
    .await
}

pub fn assembly_route_count() -> usize {
    generated::ROUTE_CRATE_COUNT
}
