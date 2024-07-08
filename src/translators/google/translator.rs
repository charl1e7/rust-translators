use crate::translators::google::error::GoogleError;
#[cfg(feature = "tokio-async")]
use crate::translators::google::requests::send_async_request;
use crate::translators::google::requests::send_sync_request;
use crate::Translator;
use std::time::Duration;

/// Translates text from one language to another using Google Translate.
///
/// # Dependencies:
/// Add to your dependency:
/// ```no_run ignore
/// [dependencies]
/// translators = { version = "0.1.3", features = ["google", "tokio-async"] } // "tokio-async" only for async, remove if you only need sync
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
/// # Proxy and custom config
///
/// See the [reqwest documentation](https://docs.rs/reqwest/latest/reqwest/struct.Proxy.html) for how to configure the address
/// - "http://0.0.0.0:8080"
/// - "https://0.0.0.0:8080"
/// - "socks5://0.0.0.0:8080" // also suitable for socks4
/// ``` ignore
/// use translators::{GoogleTranslator, Translator};
///
/// #[tokio::main]
/// async fn main() {
///     let google_trans = GoogleTranslator::new(GoogleTranslatorConfig {
///         timeout: 35, // How long to wait for a request
///         delay: 0, // delay between each request
///         proxy_address: Some("http://0.0.0.0:8080".to_string()), // or https or socks4 or socks5
///     });
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
    /// proxy address for reqwest
    pub proxy_address: Option<String>,
}

#[derive(Clone, Debug)]
pub struct GoogleTranslatorConfig {
    /// How long to wait for a request in seconds
    pub timeout: u64,
    /// Delay before sending a new request in milliseconds
    pub delay: u64,
    /// proxy address for reqwest
    pub proxy_address: Option<String>,
}
const TEXT_LIMIT: usize = 5000;
impl Translator for GoogleTranslator {
    type Config = GoogleTranslatorConfig;
    type Error = GoogleError;
    #[cfg(feature = "tokio-async")]
    async fn translate_async(
        &self,
        text: &str,
        source_language: &str,
        target_language: &str,
    ) -> Result<String, Self::Error> {
        let mut result = String::new();
        let mut start = 0;
        let mut handles = Vec::new();

        while start < text.len() {
            let end = start + TEXT_LIMIT;
            let end = if end < text.len() {
                text.char_indices()
                    .rev()
                    .skip_while(|&(i, _)| i > end)
                    .find(|&(_, c)| c.is_whitespace())
                    .map_or(end, |(i, _)| i)
            } else {
                text.len()
            };


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
                tokio::time::sleep(Duration::from_millis(self.delay)).await;
            }

            start = end;
        }

        for handle in handles {
            match handle.await {
                Ok(Ok(translated_chunk)) => result.push_str(&translated_chunk),
                Ok(Err(e)) => return Err(e),
                Err(e) => return Err(GoogleError::TokioJoinError(e.to_string())),
            }
        }

        Ok(result)
    }

    fn translate_sync(
        &self,
        text: &str,
        source_language: &str,
        target_language: &str,
    ) -> Result<String, Self::Error> {
        let mut result = String::new();
        let mut start = 0;

        while start < text.len() {
            let end = start + TEXT_LIMIT;
            let end = if end < text.len() {
                text.char_indices()
                    .rev()
                    .skip_while(|&(i, _)| i > end)
                    .find(|&(_, c)| c.is_whitespace())
                    .map_or(end, |(i, _)| i)
            } else {
                text.len()
            };

            let chunk_str = &text[start..end];
            let translated_chunk = send_sync_request(
                &target_language,
                &source_language,
                chunk_str,
                self.timeout,
                self.proxy_address.as_deref(),
            )?;

            if self.delay > 0 {
                std::thread::sleep(Duration::from_millis(self.delay));
            }

            result.push_str(&translated_chunk);
            start = end;
        }

        Ok(result)
    }

    fn new(config: Self::Config) -> Self {
        GoogleTranslator {
            timeout: config.timeout,
            delay: config.delay,
            proxy_address: config.proxy_address,
        }
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
