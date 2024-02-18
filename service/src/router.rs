pub async fn health() -> String {
    "health".to_string()
}

pub async fn routes<S: Send + Sync + 'static + Clone>(app_ctx: service::Ctx) -> axum::Router<S> {
    axum::Router::new()
        .route(
            "/v1/api/health/",
            axum::routing::on(axum::routing::MethodFilter::GET, health),
        )
        .route(
            "/-/v1/api/chat/gemini/",
            axum::routing::on(axum::routing::MethodFilter::POST, crate::gemini::chat),
        )
        .with_state(app_ctx)
}
