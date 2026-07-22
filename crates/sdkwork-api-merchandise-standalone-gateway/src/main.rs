use sdkwork_api_merchandise_assembly::assemble_api_router;
use sdkwork_merchandise_service_host::MerchandiseServiceHost;
use sdkwork_web_bootstrap::{service_router, ServiceRouterConfig};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting SDKWork Merchandise API Server...");

    let host = Arc::new(MerchandiseServiceHost::new().await);
    let assembly = assemble_api_router(host).await;

    let business = assembly
        .router
        .layer(sdkwork_web_bootstrap::application_cors_layer_from_env(
            &["SDKWORK_MERCHANDISE_ENVIRONMENT"],
            &[
                "SDKWORK_MERCHANDISE_CORS_ALLOWED_ORIGINS",
                "SDKWORK_CORS_ALLOWED_ORIGINS",
            ],
        ));
    let app = service_router(business, ServiceRouterConfig::default().with_always_ready());

    let addr = std::env::var("SDKWORK_MERCHANDISE_APPLICATION_PUBLIC_INGRESS_BIND")
        .unwrap_or_else(|_| "0.0.0.0:18090".to_owned());
    tracing::info!("Merchandise API server listening on {addr}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("bind merchandise server");
    axum::serve(listener, app)
        .await
        .expect("serve merchandise server");
}
