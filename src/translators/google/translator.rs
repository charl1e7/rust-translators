use crate::translators::google::error::GoogleError;
use crate::translators::google::requests::{send_async_request, send_sync_request};
use crate::Translator;
use reqwest::blocking::Client as ClientSync;
use reqwest::Client as ClientAsync;
use std::thread;
use tokio::time::{sleep, Duration};

/// Translates text from one language to another using Google Translate.
///
/// # Examples
///
/// A simple async example with tokio:
///
/// ```no_run ignore
/// use translators::{GoogleTranslator, Translator};
/// // tokio = { version = "xxx", features = ["rt-multi-thread"] }
///
/// #[tokio::main]
/// async fn main() {
///     let google_trans = GoogleTranslator::default();
///     let translated_text = google_trans
///         .translate_async("Hello, world!", "", "es")
///         .await
///         .unwrap();
///     println!("{}", translated_text);
/// }
/// ```
///
/// A simple sync example:
///
/// ```no_run ignore
/// use translators::{GoogleTranslator, Translator};
///
/// fn main() {
///     let google_trans = GoogleTranslator::default();
///     let translated_text = google_trans
///         .translate_sync("Hello, world!", "", "es")
///         .unwrap();
///     println!("{}", translated_text);
/// }
/// ```
///
/// # Customizing the Translator
///
/// You can customize the request timeout and delay:
///
/// ```no_run ignore
/// let trans = GoogleTranslator {
///     timeout: 35, // ms
///     delay: 300, // ms
/// };
/// ```
pub struct GoogleTranslator {
    /// How long to wait for a request in seconds
    pub timeout: u64,
    /// Delay before sending a new request in milliseconds
    pub delay: u64,
    pub proxy_adress: Option<String>,
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
        let mut result = String::new();
        const TEXT_LIMIT: usize = 5000;

        for chunk in text.as_bytes().chunks(TEXT_LIMIT) {
            let chunk_str = std::str::from_utf8(chunk)?;

            let translated_chunk = send_async_request(
                &target_language,
                &source_language,
                chunk_str,
                self.timeout,
                self.proxy_adress.as_deref(),
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
        let mut result = String::new();
        const TEXT_LIMIT: usize = 5000;

        for chunk in text.as_bytes().chunks(TEXT_LIMIT) {
            let chunk_str = std::str::from_utf8(chunk)?;

            let translated_chunk = send_sync_request(
                &target_language,
                &source_language,
                chunk_str,
                self.timeout,
                self.proxy_adress.as_deref(),
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
            proxy_adress: None,
        }
    }
}
