use crate::translators::google::error::GoogleError;
use html_escape::decode_html_entities;
use regex::Regex;
use reqwest::blocking::Client as ClientSync;
use reqwest::Client;
use reqwest::Client as ClientAsync;
use urlencoding::encode;
pub async fn send_async_request(
    client: &ClientAsync,
    target_language: &str,
    source_language: &str,
    text: &str,
    timeout: u64,
) -> Result<String, GoogleError> {
    let escaped_text = encode(text);
    let url = format!(
        "https://translate.google.com/m?tl={}&sl={}&q={}",
        target_language, source_language, escaped_text
    );
    let response = client
        .get(&url)
        .timeout(std::time::Duration::from_secs(timeout))
        .send()
        .await?;

    let result_html = response.text().await?;
    let pattern = Regex::new(r#"(?s)class="(?:t0|result-container)">(.*?)<"#).unwrap();
    if let Some(captures) = pattern.captures(&result_html) {
        Ok(decode_html_entities(&captures[1]).to_string())
    } else {
        Err(GoogleError::InvalidRequest(result_html))
    }
}

pub fn send_sync_request(
    client: &ClientSync,
    target_language: &str,
    source_language: &str,
    text: &str,
    timeout: u64,
) -> Result<String, GoogleError> {
    let escaped_text = encode(text);
    let url = format!(
        "https://translate.google.com/m?tl={}&sl={}&q={}",
        target_language, source_language, escaped_text
    );
    let response = client
        .get(&url)
        .timeout(std::time::Duration::from_secs(timeout))
        .send()?;

    let result_html = response.text()?;
    let pattern = Regex::new(r#"(?s)class="(?:t0|result-container)">(.*?)<"#).unwrap();
    if let Some(captures) = pattern.captures(&result_html) {
        Ok(decode_html_entities(&captures[1]).to_string())
    } else {
        Err(GoogleError::InvalidRequest(result_html))
    }
}
