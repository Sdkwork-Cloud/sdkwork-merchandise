pub mod http_route_manifest;
pub mod routes;
pub mod web_bootstrap;

pub use routes::{
    build_merchandise_backend_router_with_framework, build_shop_backend_router_with_framework,
};
