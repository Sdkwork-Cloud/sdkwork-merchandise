//! API assembly for sdkwork-merchandise.
//! Application bootstrap lives in `bootstrap.rs`; route inventory is in `assembly-manifest.json`.
// SDKWORK-ASSEMBLY-LIB-CUSTOM

mod bootstrap;
mod generated;

pub use bootstrap::{assemble_api_router, ApiAssembly, ApiAssemblyContext};

pub async fn assemble_api_router_from_env() -> Result<ApiAssembly, String> {
    let host = sdkwork_merchandise_service_host::MerchandiseServiceHost::from_env().await?;
    assemble_api_router(ApiAssemblyContext {
        host: std::sync::Arc::new(host),
        domain_context_injectors: Vec::new(),
        readiness_check: std::sync::Arc::new(sdkwork_web_bootstrap::AlwaysReady),
    })
    .await
}

pub fn assembly_route_count() -> usize {
    generated::ROUTE_CRATE_COUNT
}
