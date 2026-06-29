use axum::response::{IntoResponse, Response};
use axum::Json;
use sdkwork_utils_rust::http_api::{
    PageInfo, PageMode, SdkWorkApiResponse, SdkWorkCommandData, SdkWorkPageData, SdkWorkResourceData,
};
use sdkwork_web_core::{
    problem_response, ProblemCorrelation, WebFrameworkError, WebFrameworkErrorKind,
};
use serde::Serialize;
use serde_json::{json, Value};

pub fn resolve_trace_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

pub fn success_item<T: Serialize>(trace_id: impl Into<String>, item: T) -> Json<Value> {
    Json(
        serde_json::to_value(SdkWorkApiResponse::success(
            SdkWorkResourceData { item },
            trace_id,
        ))
        .unwrap_or_else(|_| json!({ "code": 0, "data": { "item": null }, "traceId": "" })),
    )
}

pub fn success_page<T: Serialize>(
    trace_id: impl Into<String>,
    items: Vec<T>,
) -> Json<Value> {
    let item_count = items.len();
    Json(
        serde_json::to_value(SdkWorkApiResponse::success(
            SdkWorkPageData {
                items,
                page_info: PageInfo {
                    mode: PageMode::Offset,
                    page: Some(1),
                    page_size: Some(item_count as i32),
                    total_items: Some(item_count.to_string()),
                    total_pages: Some(1),
                    next_cursor: None,
                    has_more: Some(false),
                },
            },
            trace_id,
        ))
        .unwrap_or_else(|_| {
            json!({ "code": 0, "data": { "items": [], "pageInfo": { "mode": "offset" } }, "traceId": "" })
        }),
    )
}

pub fn success_command(trace_id: impl Into<String>) -> Json<Value> {
    Json(
        serde_json::to_value(SdkWorkApiResponse::success(
            SdkWorkCommandData::accepted(),
            trace_id,
        ))
        .unwrap_or_else(|_| json!({ "code": 0, "data": { "accepted": true }, "traceId": "" })),
    )
}

pub fn success_resource<T: Serialize>(item: T) -> Response {
    success_item(resolve_trace_id(), item).into_response()
}

pub fn success_list<T: Serialize>(items: Vec<T>) -> Response {
    success_page(resolve_trace_id(), items).into_response()
}

pub fn success_accepted() -> Response {
    success_command(resolve_trace_id()).into_response()
}

fn problem_for(kind: WebFrameworkErrorKind, message: impl Into<String>) -> Response {
    let trace_id = resolve_trace_id();
    problem_response(
        &WebFrameworkError {
            kind,
            message: message.into(),
            retry_after_seconds: None,
        },
        ProblemCorrelation::from(Some(trace_id.as_str())),
    )
}

pub fn unauthorized_response(message: impl Into<String>) -> Response {
    problem_for(WebFrameworkErrorKind::MissingCredentials, message)
}

pub fn validation_response(message: impl Into<String>) -> Response {
    problem_for(WebFrameworkErrorKind::BadRequest, message)
}

pub fn not_found_response(message: impl Into<String>) -> Response {
    problem_for(WebFrameworkErrorKind::NotFound, message)
}

pub fn catalog_system_response(context: &str, error: sdkwork_contract_service::CommerceServiceError) -> Response {
    problem_for(
        WebFrameworkErrorKind::DependencyUnavailable,
        format!("{context}: {}", error.message()),
    )
}
