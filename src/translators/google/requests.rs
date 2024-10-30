use crate::translators::translator;
use html_escape::decode_html_entities;
use regex::Regex;
use reqwest::blocking::Client as ClientSync;
#[cfg(feature = "tokio-async")]
use reqwest::Client as ClientAsync;
use reqwest::Proxy;
use urlencoding::encode;

#[cfg(feature = "tokio-async")]
pub async fn send_async_request(
    target_language: &str,
    source_language: &str,
    text: &str,
    timeout: usize,
    proxy_address: Option<&str>,
) -> Result<String, translator::Error> {
    // client build
    let mut client = ClientAsync::builder();
    // proxy
    if let Some(proxy_address) = proxy_address {
        let proxy = Proxy::all(proxy_address)?;
        client = client.proxy(proxy);
    }
    let client = client.build()?;

    // send req
    let url = prepare_url(target_language, source_language, text);
    let response = client
        .get(&url)
        .timeout(std::time::Duration::from_secs(timeout as u64))
        .send()
        .await?;

    let result_html = response.text().await?;

    // look for translated text
    get_translated_text(&result_html)
}
pub fn send_sync_request(
    target_language: &str,
    source_language: &str,
    text: &str,
    timeout: usize,
    proxy_address: Option<&str>,
) -> Result<String, translator::Error> {
    // client build
    let mut client = ClientSync::builder();
    // proxy
    if let Some(proxy_address) = proxy_address {
        let proxy = Proxy::all(proxy_address)?;
        client = client.proxy(proxy);
    }
    let client = client.build()?;

    // send req
    let url = prepare_url(target_language, source_language, text);
    let response = client
        .get(&url)
        .timeout(std::time::Duration::from_secs(timeout as u64))
        .send()?;
    let result_html = response.text()?;

    get_translated_text(&result_html)
}

fn prepare_url(target_language: &str, source_language: &str, text: &str) -> String {
    let escaped_text = encode(text);
    let url = format!(
        "https://translate.google.com/m?tl={}&sl={}&q={}",
        target_language, source_language, escaped_text
    );

    url
}

fn get_translated_text(html: &str) -> Result<String, translator::Error> {
    // extracting translation text
    let pattern = Regex::new(r#"(?s)class="(?:t0|result-container)">(.*?)<"#).unwrap();
    if let Some(captures) = pattern.captures(html) {
        Ok(decode_html_entities(&captures[1]).to_string())
    } else {
        Err(translator::Error::InvalidRequest("".to_string()))
    }
}
