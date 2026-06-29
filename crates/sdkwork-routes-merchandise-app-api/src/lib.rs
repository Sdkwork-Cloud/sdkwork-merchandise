pub mod catalog_store;
pub mod http_envelope;
pub mod http_route_manifest;
pub mod paths;
pub mod routes;
pub mod subject;
pub mod web_bootstrap;

pub use http_envelope::{
    catalog_system_response, not_found_response, success_accepted, success_list, success_resource,
    unauthorized_response, validation_response,
};
pub use catalog_store::{
    backend_catalog_router_with_postgres_pool, backend_catalog_router_with_sqlite_pool,
    build_backend_catalog_router, map_address, map_attribute,
    map_cart_item, map_category, map_price_list_item, map_sku, map_spu,
    AddCartItemBody, AttributeQueryParams, CatalogState, CategoryQueryParams, CommerceCatalogFuture,
    CommerceCatalogStore, CreateAddressBody, CreateSpuBody, SpuListQueryParams, UpdateAddressBody,
    UpdateCartItemBody, UpdateSpuBody,
};
pub use http_route_manifest::app_route_manifest;
pub use routes::{
    build_merchandise_app_router_with_framework, build_shop_app_router,
    build_shop_app_router_with_framework,
};
pub use web_bootstrap::{
    shop_app_api_public_path_prefixes, wrap_router_with_web_framework,
    wrap_router_with_web_framework_from_env,
};

use axum::Router;
use sdkwork_merchandise_service_host::ShopServiceHost;
use sdkwork_web_core::HttpRouteManifest;
use std::sync::Arc;

pub fn gateway_route_manifest() -> HttpRouteManifest {
    app_route_manifest()
}

pub async fn gateway_mount(host: Arc<ShopServiceHost>) -> Router {
    build_merchandise_app_router_with_framework(host).await
}
