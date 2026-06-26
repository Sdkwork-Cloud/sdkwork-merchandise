use sdkwork_web_core::{HttpMethod, HttpRoute, HttpRouteManifest};

const HTTP_ROUTES: &[HttpRoute] = &[
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/catalog/products",
        "merchandise",
        "merchandise.catalog.products.list",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        "/app/v3/api/catalog/products",
        "merchandise",
        "merchandise.catalog.products.create",
    ),
];

pub fn app_route_manifest() -> HttpRouteManifest {
    HttpRouteManifest::new(HTTP_ROUTES)
}
