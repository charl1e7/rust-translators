use crate::translators::google::error::GoogleError;
use html_escape::decode_html_entities;
use regex::Regex;
use reqwest::blocking::Client as ClientSync;
use reqwest::Client as ClientAsync;
use reqwest::Proxy;
use urlencoding::encode;
pub async fn send_async_request(
    target_language: &str,
    source_language: &str,
    text: &str,
    timeout: u64,
    proxy_address: Option<&str>,
) -> Result<String, GoogleError> {
    let escaped_text = encode(text);
    let url = format!(
        "https://translate.google.com/m?tl={}&sl={}&q={}",
        target_language, source_language, escaped_text
    );
    let mut client = ClientAsync::builder();
    // proxy
    if let Some(proxy_address) = proxy_address {
        let proxy = Proxy::all(proxy_address)?;
        client = client.proxy(proxy);
    }
    let client = client.build()?;
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
    target_language: &str,
    source_language: &str,
    text: &str,
    timeout: u64,
    proxy_address: Option<&str>,
) -> Result<String, GoogleError> {
    println!("<<{}>>\n", text);
    let escaped_text = encode(text);
    let url = format!(
        "https://translate.google.com/m?tl={}&sl={}&q={}",
        target_language, source_language, escaped_text
    );
    let mut client = ClientSync::builder();
    if let Some(proxy_address) = proxy_address {
        let proxy = Proxy::all(proxy_address)?;
        client = client.proxy(proxy);
    }
    let client = client.build()?;
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
