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

    let resp = response
        .candidates
        .into_iter()
        .flat_map(|candidate| {
            candidate
                .content
                .parts
                .into_iter()
                .map(|parts| parts.text)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .join("## Gemini multi part is coming"); // todo: may need to check the response if multiple is coming

    print_terminal(&resp);
    Ok(resp)
}

fn print_terminal(src: &str) {
    let mut skin = termimad::MadSkin::default_dark();
    skin.code_block.align = termimad::Alignment::Right;
    let (width, _) = termimad::terminal_size();
    let terminal_width = width as usize;
    let mut text = termimad::FmtText::from(&skin, src, Some(terminal_width));
    text.set_rendering_width(text.content_width());
    println!("{}", text);
}
