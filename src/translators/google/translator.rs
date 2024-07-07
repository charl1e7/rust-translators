use crate::translators::google::error::GoogleError;
#[cfg(feature = "tokio-async")]
use crate::translators::google::requests::send_async_request;
use crate::translators::google::requests::send_sync_request;
use crate::Translator;
use std::thread;
use std::time::Duration;
#[cfg(feature = "tokio-async")]
use tokio::time::sleep;

/// Translates text from one language to another using Google Translate.
///
/// # Dependencies:
/// Add to your dependency:
/// ```no_run ignore
/// [dependencies]
/// translators = { version = "0.1.2", features = ["google", "tokio-async"] } // "tokio-async" only for async, remove if you only need sync
/// // only for async:
/// tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
/// ```
/// # Examples
///
/// A simple async example with tokio:
///
/// ```no_run ignore
/// use translators::{GoogleTranslator, Translator};
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
/// # Proxy
///
/// See the [reqwest documentation](https://docs.rs/reqwest/latest/reqwest/struct.Proxy.html) for how to configure the address
///
/// ```no_run ignore
/// use translators::{GoogleTranslator, Translator};
///
/// #[tokio::main]
/// async fn main() {
///     let google_trans = GoogleTranslator{
///         timeout: 35,
///         delay: 0,
///         proxy_address: Some("http://0.0.0.0:8080".to_string()), // or https or socks4 or socks5
///     };
///     let translated_text = google_trans
///         .translate_async("Hello, world!", "", "es")
///         .await
///         .unwrap();
///     println!("{}", translated_text);
/// }
/// ```
///
#[derive(Clone, Debug)]
pub struct GoogleTranslator {
    /// How long to wait for a request in seconds
    pub timeout: u64,
    /// Delay before sending a new request in milliseconds
    pub delay: u64,
    pub proxy_address: Option<String>,
}

const TEXT_LIMIT: usize = 5000;
impl Translator for GoogleTranslator {
    type Config = GoogleTranslator;
    type Error = GoogleError;
    #[cfg(feature = "tokio-async")]
    async fn translate_async(
        self: &Self,
        text: &str,
        source_language: &str,
        target_language: &str,
    ) -> Result<String, Self::Error> {
        let mut result = String::new();
        let mut start = 0;
        let mut handles = Vec::new();

        while start < text.len() {
            let mut end = start + TEXT_LIMIT;
            if end >= text.len() {
                end = text.len();
            } else {
                while !text.is_char_boundary(end) {
                    end -= 1;
                }
            }

            let chunk_str = text[start..end].to_string();
            let target_language = target_language.to_string();
            let source_language = source_language.to_string();
            let proxy_address = self.proxy_address.clone();
            let timeout = self.timeout;

            let handle = tokio::spawn(async move {
                send_async_request(
                    &target_language,
                    &source_language,
                    &chunk_str,
                    timeout,
                    proxy_address.as_deref(),
                )
                    .await
            });

            handles.push(handle);

            if self.delay > 0 {
                sleep(Duration::from_millis(self.delay)).await;
            }

            start = end;
        }

        for handle in handles {
            match handle.await {
                Ok(Ok(translated_chunk)) => result.push_str(&translated_chunk),
                Ok(Err(e)) => return Err(e),
                Err(e) => return Err(GoogleError::TokioJoinError(e.to_string())), // Обработка ошибки JoinError
            }
        }

        Ok(result)
    }

    fn translate_sync(
        self: &Self,
        text: &str,
        source_language: &str,
        target_language: &str,
    ) -> Result<String, Self::Error> {
        let mut result = String::new();
        let mut start = 0;

        while start < text.len() {
            let mut end = start + TEXT_LIMIT;
            if end >= text.len() {
                end = text.len();
            } else {
                while !text.is_char_boundary(end) {
                    end -= 1;
                }
            }

            let chunk_str = &text[start..end];
            let translated_chunk = send_sync_request(
                &target_language,
                &source_language,
                chunk_str,
                self.timeout,
                self.proxy_address.as_deref(),
            )?;

            if self.delay > 0 {
                thread::sleep(Duration::from_millis(self.delay));
            }

            result.push_str(&translated_chunk);
            start = end;
        }

        Ok(result)
    }
}

impl Default for GoogleTranslator {
    fn default() -> Self {
        GoogleTranslator {
            timeout: 35,
            delay: 0,
            proxy_address: None,
        }
    }
}
