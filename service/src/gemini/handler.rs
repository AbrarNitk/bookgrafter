#[derive(serde::Deserialize, Debug)]
pub struct ChatReq {
    query: String,
}
#[tracing::instrument(name = "gemini-chat", skip_all, fields (query = ?req.query))]
pub async fn chat(axum::Json(req): axum::Json<ChatReq>) -> String {
    crate::gemini::apis::gen_text("", req.query.as_str())
        .await
        .expect("something went wrong");
    "".to_string()
}
