#[cfg(feature = "tokio-async")]
use crate::translators::google::requests::send_async_request;
use crate::translators::google::requests::send_sync_request;
use crate::translators::translator;

use macon::Builder;
use std::time::Duration;
/// Google Translate.
///
/// # Dependencies:
/// Add to your dependency:
/// ```no_run ignore
/// [dependencies]
/// // "tokio-async" only for async, remove if you only need sync
/// translators = { version = "0.2.1", features = ["google", "tokio-async"] }
/// // only for async:
/// tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
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
///     // How long to wait for a request in sec
///     .timeout(35 as usize)
///     // delay between requests if the limit is exceeded
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
    /// How long to wait for a request in seconds.
    pub timeout: usize,
    /// Delay between requests if the limit is exceeded.
    pub delay: usize,
    /// Proxy address for reqwest.
    pub proxy_address: Option<String>,
    #[cfg(feature = "tokio-async")]
    /// How many requests can be handled concurrently.
    pub max_concurrency: Option<usize>,
    /// Limits on the maximum number of chars.
    /// Set if the translator has changed their limits.
    pub text_limit: usize,
}

impl translator::Translator for GoogleTranslator {
    #[cfg(feature = "tokio-async")]
    fn translate_async<'a>(
        &'a self,
        text: &'a str,
        source_language: &'a str,
        target_language: &'a str,
    ) -> impl std::future::Future<Output = Result<String, translator::Error>> + Send + 'a {
        async move {
            let mut builder = reqwest::Client::builder();
            if let Some(proxy) = &self.proxy_address {
                builder = builder.proxy(reqwest::Proxy::all(proxy)?);
            }
            builder = builder.timeout(Duration::from_secs(self.timeout as u64));
            builder = builder.user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36");
            let client = builder.build()?;

            let chunks = split_chunks(text, self.text_limit);

            if self.delay > 0 {
                let mut result = String::new();
                for chunk in chunks {
                    let chunk_str = &text[chunk.start..chunk.end];
                    let translated = send_async_request(
                        &client,
                        target_language,
                        source_language,
                        chunk_str,
                    ).await?;
                    result.push_str(&translated);
                    tokio::time::sleep(Duration::from_millis(self.delay as u64)).await;
                }
                return Ok(result);
            }

            use futures::stream::{self, StreamExt};
            let concurrency = self.max_concurrency.unwrap_or(10);

            let results: Vec<Result<String, translator::Error>> = stream::iter(chunks)
                .map(|chunk| {
                    let client = client.clone();
                    let chunk_str = text[chunk.start..chunk.end].to_string();
                    let tl = target_language.to_string();
                    let sl = source_language.to_string();
                    async move {
                        send_async_request(&client, &tl, &sl, &chunk_str).await
                    }
                })
                .buffered(concurrency)
                .collect()
                .await;

            let mut final_result = String::new();
            for res in results {
                final_result.push_str(&res?);
            }

            Ok(final_result)
        }
    }

    fn translate_sync(
        &self,
        text: &str,
        source_language: &str,
        target_language: &str,
    ) -> Result<String, translator::Error> {
        let mut builder = reqwest::blocking::Client::builder();
        if let Some(proxy) = &self.proxy_address {
            builder = builder.proxy(reqwest::Proxy::all(proxy)?);
        }
        builder = builder.timeout(Duration::from_secs(self.timeout as u64));
        builder = builder.user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36");
        let client = builder.build()?;

        let mut result = String::new();
        let chunks = split_chunks(text, self.text_limit);
        for chunk in chunks {
            let chunk_str = &text[chunk.start..chunk.end];
            let translated_chunk = send_sync_request(
                &client,
                target_language,
                source_language,
                chunk_str,
            )?;

            if self.delay > 0 {
                std::thread::sleep(Duration::from_millis(self.delay as u64));
            }

            result.push_str(&translated_chunk);
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

fn split_chunks(text: &str, text_limit: usize) -> Vec<Chunk> {
    let delimiters = &[
        ' ', '\n', '\r', '\t', '.', ',', ':', ';', '"', '(', ')', '[', ']', '{', '}', '/',
    ];
    let mut chunks = vec![];
    let mut start_index = 0;
    let mut chars_count = 0;
    let mut last_delimiter_pos = None;

    for (char_i, char) in text.char_indices() {
        chars_count += 1;

        if delimiters.contains(&char) {
            last_delimiter_pos = Some((char_i + char.len_utf8(), chars_count));
        }

        if chars_count == text_limit {
            let (end, reset_count) = match last_delimiter_pos {
                Some((delimiter_pos, delimiter_count)) => (delimiter_pos, delimiter_count),
                None => (char_i + char.len_utf8(), chars_count),
            };

            chunks.push(Chunk {
                start: start_index,
                end,
            });

            start_index = end;
            chars_count -= reset_count;
            last_delimiter_pos = None;
        }
    }

    if start_index < text.len() {
        chunks.push(Chunk {
            start: start_index,
            end: text.len(),
        });
    }

    chunks
}

#[derive(Default, Debug)]
struct Chunk {
    start: usize,
    end: usize,
}