#[derive(serde::Deserialize, Debug)]
pub struct ChatReq {
    query: String,
}
#[tracing::instrument(name = "gemini-chat", skip_all, fields (query = ?req.query))]
pub async fn chat(
    axum::extract::State(ctx): axum::extract::State<service::Ctx>,
    axum::Json(req): axum::Json<ChatReq>,
) -> String {
    crate::gemini::apis::gen_text(ctx.gemini_key.as_str(), req.query.as_str())
        .await
        .expect("something went wrong");
    "".to_string()
}
