use axum::routing::get;
use axum::Router;
use sdkwork_merchandise_service_host::ShopServiceHost;
use std::sync::Arc;

use crate::web_bootstrap::wrap_router_with_web_framework_from_env;

/// Browse/open catalog HTTP is owned by `sdkwork-catalog`. Merchandise standalone app surface is health-only.
pub fn build_merchandise_app_router(_host: Arc<ShopServiceHost>) -> Router {
    Router::new().route(
        "/app/v3/api/merchandise/health",
        get(|| async { "ok" }),
    )
}

pub async fn build_merchandise_app_router_with_framework(host: Arc<ShopServiceHost>) -> Router {
    wrap_router_with_web_framework_from_env(build_merchandise_app_router(host)).await
}

// Legacy names retained for callers migrating to explicit merchandise naming.
pub use build_merchandise_app_router as build_shop_app_router;
pub use build_merchandise_app_router_with_framework as build_shop_app_router_with_framework;
