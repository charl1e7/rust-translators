#[cfg(feature = "tokio-async")]
use crate::translators::google::requests::send_async_request;
use crate::translators::google::requests::send_sync_request;
use crate::translators::translator;

use macon::Builder;
#[cfg(feature = "tokio-async")]
use std::sync::Arc;
use std::time::Duration;
#[cfg(feature = "tokio-async")]
use tokio::sync::Semaphore;
/// Translates text from one language to another using Google Translate.
///
/// # Dependencies:
/// Add to your dependency:
/// ```no_run ignore
/// [dependencies]
/// // "tokio-async" only for async, remove if you only need sync
/// translators = { version = "0.1.4", features = ["google", "tokio-async"] }
/// // only for async:
/// tokio = { version = "x", features = ["rt-multi-thread", "macros"] }
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
/// - "http://user:password@0.0.0.0:80" // basic auth
/// ``` ignore
/// // delete any line if you don't need it
/// let google_trans = GoogleTranslator::builder()
///     // How long to wait for a request in seconds
///     .timeout(35 as usize)
///     //How long to wait for a request in milliseconds
///     .delay(120 as usize)
///     // shows how many requests can be handled concurrently
///     // work only with async
///     .max_concurrency(2 as usize)
///     // proxy
///     .proxy_address("http://user:password@0.0.0.0:80")
///     .build();
/// ```
///
#[derive(Builder, Clone, Debug)]
#[builder(Default)]
pub struct GoogleTranslator {
    /// How long to wait for a request in seconds
    pub timeout: usize,
    /// Delay before sending a new request in milliseconds
    pub delay: usize,
    /// Proxy address for reqwest
    pub proxy_address: Option<String>,
    #[cfg(feature = "tokio-async")]
    /// how many requests can be handled concurrently
    pub max_concurrency: Option<usize>,
    /// limits on the maximum number of characters from the translator
    /// set if the translator has changed their limits.
    pub text_limit: usize,
}

impl translator::Translator for GoogleTranslator {
    #[cfg(feature = "tokio-async")]
    async fn translate_async(
        &self,
        text: &str,
        source_language: &str,
        target_language: &str,
    ) -> Result<String, translator::Error> {
        let mut result = String::new();
        let mut start = 0;
        let mut tasks = Vec::new();
        let semaphore = self
            .max_concurrency
            .map(|max| Arc::new(Semaphore::new(max)));

        while start < text.len() {
            let end = start + self.text_limit;
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
            let target_language = &target_language;
            let source_language = &source_language;
            let proxy_address = &self.proxy_address;
            let timeout = self.timeout;
            let semaphore = semaphore.clone();

            let task = async move {
                let _permit = match &semaphore {
                    Some(sem) => Some(sem.acquire().await.unwrap()),
                    None => None,
                };

                send_async_request(
                    &target_language,
                    &source_language,
                    &chunk_str,
                    timeout,
                    proxy_address.as_deref(),
                )
                .await
            };

            tasks.push(task);
            start = end;
        }

        // send sync req with delay
        if self.delay > 0 {
            for task in tasks {
                match task.await {
                    Ok(translated_chunk) => result.push_str(&translated_chunk),
                    Err(e) => return Err(e),
                }
                tokio::time::sleep(Duration::from_millis(self.delay as u64)).await;
            }
        // send async async
        } else {
            let results = futures::future::join_all(tasks).await;

            for res in results {
                match res {
                    Ok(translated_chunk) => result.push_str(&translated_chunk),
                    Err(e) => return Err(e),
                }
            }
        }

        Ok(result)
    }

    fn translate_sync(
        &self,
        text: &str,
        source_language: &str,
        target_language: &str,
    ) -> Result<String, translator::Error> {
        let mut result = String::new();
        let mut start = 0;

        while start < text.len() {
            let end = start + self.text_limit;
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
                std::thread::sleep(Duration::from_millis(self.delay as u64));
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
            #[cfg(feature = "tokio-async")]
            max_concurrency: None,
            text_limit: 5000,
        }
    }
}
