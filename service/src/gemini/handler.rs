use axum::response::IntoResponse;

#[derive(serde::Deserialize, Debug)]
pub struct ChatReq {
    topic: Option<String>,
    query: String,
}

#[derive(serde::Serialize)]
pub struct ChatResp {
    answer: String,
}

#[tracing::instrument(name = "gemini-chat", skip_all, fields(query = req.query))]
pub async fn chat(
    axum::extract::State(ctx): axum::extract::State<service::Ctx>,
    axum::Json(req): axum::Json<ChatReq>,
) -> axum::response::Response {
    let answer = crate::gemini::apis::gen_text(ctx.gemini_key.as_str(), req.query.as_str())
        .await
        .expect("something went wrong");
    if let Some(topic) = req.topic {
        store(
            ctx.root.as_path(),
            topic.as_str(),
            req.query.as_str(),
            answer.as_str(),
        )
        .await
        .expect("");
    }

    (
        axum::http::StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "application/json")],
        axum::Json(ChatResp { answer }),
    )
        .into_response()
}

async fn store(root: &camino::Utf8Path, topic: &str, q: &str, ans: &str) -> std::io::Result<()> {
    let path = root.join(format!("{}.md", topic.trim_matches('/')));
    let content = format!("## {}\n\n{}\n\n\n", q, ans);
    service::fs::write(path.as_path(), content.as_bytes()).await?;
    Ok(())
}
