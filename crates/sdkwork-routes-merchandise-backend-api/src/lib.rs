use axum::Router;
use sdkwork_merchandise_service_host::MerchandiseServiceHost;
use sdkwork_web_core::HttpRouteManifest;
use std::sync::Arc;

use crate::http_route_manifest::backend_route_manifest;

pub mod http_route_manifest;
pub mod routes;
pub mod web_bootstrap;

pub use routes::build_merchandise_backend_router_with_framework;

pub fn gateway_route_manifest() -> HttpRouteManifest {
    backend_route_manifest()
}

pub async fn gateway_mount(host: Arc<MerchandiseServiceHost>) -> Router {
    build_merchandise_backend_router_with_framework(host).await
}
