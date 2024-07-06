use std::thread;
use crate::translators::google::error::GoogleError;
use crate::translators::google::requests::{send_async_request, send_sync_request};
use crate::translators::translator::translator::Translator;
use tokio::time::{sleep, Duration};
use reqwest::Client as ClientAsync;
use reqwest::blocking::Client as ClientSync;
pub struct GoogleTranslator {
    pub timeout: u64,
    pub delay: u64,
}
impl Translator for GoogleTranslator {
    type Config = GoogleTranslator;
    type Error = GoogleError;

    async fn translate_async(
        self: Self,
        text: &str,
        source_language: &str,
        target_language: &str,
    ) -> Result<String, Self::Error> {
        let client = ClientAsync::new();
        let mut result = String::new();
        const TEXT_LIMIT: usize = 5000;

        for chunk in text.as_bytes().chunks(TEXT_LIMIT) {
            let chunk_str = std::str::from_utf8(chunk)?;

            let translated_chunk = send_async_request(
                &client,
                &target_language,
                &source_language,
                chunk_str,
                self.timeout,
            )
            .await?;
            if self.delay > 0 {
                sleep(Duration::from_millis(self.delay)).await;
            }

            result.push_str(&translated_chunk);
        }

        Ok(result)
    }
    fn translate_sync(
        self: Self,
        text: &str,
        source_language: &str,
        target_language: &str,
    ) -> Result<String, Self::Error> {
        let client = ClientSync::new();
        let mut result = String::new();
        const TEXT_LIMIT: usize = 5000;

        for chunk in text.as_bytes().chunks(TEXT_LIMIT) {
            let chunk_str = std::str::from_utf8(chunk)?;

            let translated_chunk = send_sync_request(
                &client,
                &target_language,
                &source_language,
                chunk_str,
                self.timeout,
            )?;
            if self.delay > 0 {
                thread::sleep(Duration::from_millis(self.delay));
            }

            result.push_str(&translated_chunk);
        }

        Ok(result)
    }
    fn default() -> Self {
        GoogleTranslator {
            timeout: 35,
            delay: 0,
        }
    }
}
