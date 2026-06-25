use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;

pub fn shop_health_router() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/ready", get(ready_check))
}

async fn health_check() -> Response {
    (
        StatusCode::OK,
        Json(json!({ "status": "ok", "service": "sdkwork-merchandise" })),
    )
        .into_response()
}

async fn ready_check() -> Response {
    (
        StatusCode::OK,
        Json(json!({ "status": "ready", "service": "sdkwork-merchandise" })),
    )
        .into_response()
}
