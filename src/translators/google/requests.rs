use crate::translators::translator;
use html_escape::decode_html_entities;
use regex::Regex;
use reqwest::blocking::Client as ClientSync;
#[cfg(feature = "tokio-async")]
use reqwest::Client as ClientAsync;
use urlencoding::encode;
use std::sync::LazyLock;

static TRANSLATION_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?s)class="(?:t0|result-container)">(.*?)<"#).unwrap()
});

#[cfg(feature = "tokio-async")]
pub async fn send_async_request(
    client: &ClientAsync,
    target_language: &str,
    source_language: &str,
    text: &str,
) -> Result<String, translator::Error> {
    let url = prepare_url(target_language, source_language, text);
    
    let response = client.get(&url).send().await?.error_for_status()?;
    let result_html = response.text().await?;

    get_translated_text(&result_html)
}

pub fn send_sync_request(
    client: &ClientSync,
    target_language: &str,
    source_language: &str,
    text: &str,
) -> Result<String, translator::Error> {
    let url = prepare_url(target_language, source_language, text);
    
    let response = client.get(&url).send()?.error_for_status()?;
    let result_html = response.text()?;

    get_translated_text(&result_html)
}

fn prepare_url(target_language: &str, source_language: &str, text: &str) -> String {
    let escaped_text = encode(text);
    format!(
        "https://translate.google.com/m?tl={}&sl={}&q={}",
        target_language, source_language, escaped_text
    )
}

fn get_translated_text(html: &str) -> Result<String, translator::Error> {
    if let Some(captures) = TRANSLATION_REGEX.captures(html) {
        Ok(decode_html_entities(&captures[1]).to_string())
    } else {
        Err(translator::Error::InvalidRequest(
            "Translation pattern not found in response. Possible CAPTCHA or block.".to_string(),
        ))
    }
}