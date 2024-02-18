use crate::gemini::constant::GEMINI_HOST_API;

#[derive(serde::Serialize)]
pub struct GenTextReq {
    contents: Vec<Parts>,
}

#[derive(serde::Serialize)]
pub struct Parts {
    parts: Vec<Text>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Text {
    text: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct GenTextResp {
    candidates: Vec<Candidate>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Candidate {
    content: Content,
}

#[derive(serde::Deserialize, Debug)]
pub struct Content {
    parts: Vec<Text>,
}

pub async fn gen_text(api_key: &str, query: &str) -> Result<String, Box<dyn std::error::Error>> {
    use std::str::FromStr;
    let url = reqwest::Url::from_str(
        format!(
            "{}/{}",
            GEMINI_HOST_API, "v1beta/models/gemini-pro:generateContent"
        )
        .as_str(),
    )?;
    let req = GenTextReq {
        contents: vec![Parts {
            parts: vec![Text {
                text: query.to_string(),
            }],
        }],
    };
    let response: GenTextResp =
        crate::http::post(url, &req, Some(&vec![("key", api_key)]), None).await?;
    tracing::info!(message = "response", ?response);
    Ok("".to_string())
}
