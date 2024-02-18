#[derive(thiserror::Error, Debug)]
pub enum HttpError {
    #[error("ReqwestClientError: {0}")]
    ReqwestClient(#[from] reqwest::Error),
    #[error("HttpReqNotOkay: {code:?} {{response:?}}")]
    NotOkay { code: u16, response: String },
    #[error("HttpInvalidHeader: {0}")]
    InvalidHeader(#[from] reqwest::header::InvalidHeaderValue),
    #[error("DeserializeJsonError: {0}")]
    DeserializeJson(#[from] serde_json::Error),
}

pub async fn get_with_plain<U: reqwest::IntoUrl, V: AsRef<str> + serde::Serialize>(
    url: U,
    headers: Option<&std::collections::HashMap<V, V>>,
    query: Option<&[(V, V)]>,
) -> Result<String, HttpError> {
    let client = reqwest::Client::new();
    let mut request_builder = client.get(url.as_str());

    if let Some(q) = query {
        request_builder = request_builder.query(q);
    }

    if let Some(headers) = headers {
        for (k, v) in headers {
            request_builder = request_builder.header(
                k.as_ref(),
                reqwest::header::HeaderValue::from_str(v.as_ref())?,
            );
        }
    }

    let response = request_builder.send().await?;
    if response.status() == reqwest::StatusCode::OK {
        Ok(response.text().await?)
    } else {
        let status = response.status().as_u16();
        let body = response.text().await?;
        tracing::error!(
            message = "http::get failed",
            url = url.as_str(),
            status = status,
            err_body = body
        );
        Err(HttpError::NotOkay {
            code: status,
            response: body,
        })
    }
}

pub async fn post<T, U: reqwest::IntoUrl, V: AsRef<str> + serde::Serialize, B: serde::Serialize>(
    url: U,
    body: &B,
    query: Option<&[(V, V)]>,
    headers: Option<&std::collections::HashMap<V, V>>,
) -> Result<T, HttpError>
where
    T: serde::de::DeserializeOwned,
{
    let client = reqwest::Client::new();
    let body = serde_json::to_vec(body)?;
    let mut request_builder = client.post(url.as_str()).body(body);

    if let Some(q) = query {
        request_builder = request_builder.query(q);
    }

    if let Some(headers) = headers {
        for (k, v) in headers {
            request_builder = request_builder.header(
                k.as_ref(),
                reqwest::header::HeaderValue::from_str(v.as_ref())?,
            );
        }
    }

    let response = request_builder.send().await?;
    if response.status() == reqwest::StatusCode::OK {
        Ok(response.json().await?)
    } else {
        let status = response.status().as_u16();
        let body = response.text().await?;
        tracing::error!(
            message = "http::post failed",
            url = url.as_str(),
            status = status,
            err_body = body
        );
        Err(HttpError::NotOkay {
            code: status,
            response: body,
        })
    }
}
