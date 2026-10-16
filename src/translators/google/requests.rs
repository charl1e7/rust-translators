use crate::translators::translator;
use reqwest::blocking::Client as ClientSync;
#[cfg(feature = "tokio-async")]
use reqwest::Client as ClientAsync;
use reqwest::StatusCode;

/// Free endpoint used by Google's own apps; no API key required.
const ENDPOINT: &str = "https://translate.googleapis.com/translate_a/single";

/// Values of the `client` query parameter, tried in order.
///
/// Google blocks individual client ids from time to time (HTTP 429 /
/// "automated queries" page), so if one is rejected we fall back to the next.
const CLIENTS: &[&str] = &["at", "gtx", "dict-chrome-ex"];

#[cfg(feature = "tokio-async")]
pub async fn send_async_request(
    client: &ClientAsync,
    target_language: &str,
    source_language: &str,
    text: &str,
) -> Result<String, translator::Error> {
    let mut last_err = None;
    for api_client in CLIENTS {
        let response = client
            .post(ENDPOINT)
            .query(&query(api_client, target_language, source_language))
            .form(&[("q", text)])
            .send()
            .await?;
        let status = response.status();
        let body = response.text().await?;
        match parse_response(status, &body) {
            Ok(text) => return Ok(text),
            Err(e) if is_blocked(status) => last_err = Some(e),
            Err(e) => return Err(e),
        }
    }
    Err(last_err.expect("CLIENTS is not empty"))
}

pub fn send_sync_request(
    client: &ClientSync,
    target_language: &str,
    source_language: &str,
    text: &str,
) -> Result<String, translator::Error> {
    let mut last_err = None;
    for api_client in CLIENTS {
        let response = client
            .post(ENDPOINT)
            .query(&query(api_client, target_language, source_language))
            .form(&[("q", text)])
            .send()?;
        let status = response.status();
        let body = response.text()?;
        match parse_response(status, &body) {
            Ok(text) => return Ok(text),
            Err(e) if is_blocked(status) => last_err = Some(e),
            Err(e) => return Err(e),
        }
    }
    Err(last_err.expect("CLIENTS is not empty"))
}

fn query<'a>(
    api_client: &'a str,
    target_language: &'a str,
    source_language: &'a str,
) -> [(&'static str, &'a str); 5] {
    let source = if source_language.is_empty() {
        "auto"
    } else {
        source_language
    };
    [
        ("client", api_client),
        ("sl", source),
        ("tl", target_language),
        ("dt", "t"),
        ("ie", "UTF-8"),
    ]
}

fn is_blocked(status: StatusCode) -> bool {
    status == StatusCode::TOO_MANY_REQUESTS || status == StatusCode::FORBIDDEN
}

/// Response layout: `[[[translated, original, ...], ...], ...]`.
/// The translated text is split into segments, which are concatenated as-is.
fn parse_response(status: StatusCode, body: &str) -> Result<String, translator::Error> {
    if !status.is_success() {
        let hint = if status == StatusCode::TOO_MANY_REQUESTS {
            " (Google blocked this client/IP as automated traffic)"
        } else {
            ""
        };
        return Err(translator::Error::Status(format!(
            "HTTP {status}{hint}"
        )));
    }

    let json: serde_json::Value = serde_json::from_str(body).map_err(|e| {
        translator::Error::DecodeBody(format!(
            "Response is not valid JSON ({e}). Possible CAPTCHA or block."
        ))
    })?;

    let segments = json
        .get(0)
        .and_then(|v| v.as_array())
        .ok_or_else(|| {
            translator::Error::InvalidRequest("Unexpected translation response format".to_string())
        })?;

    Ok(segments
        .iter()
        .filter_map(|segment| segment.get(0).and_then(|v| v.as_str()))
        .collect())
}
