//! Asynchronous/synchronous text translation library
//! supporting Google Translate and DeepL,
//! without API key requirements or limits.
//!
//! Examples of usage in the [repository].
//! - [google-translate]
//!
//! [repository]: https://github.com/charl1e7/rust-translators/tree/main/examples
//!
//! [google-translate]: https://github.com/charl1e7/rust-translators/tree/main/examples/google
//! # Examples
//!
//! A simple async example with tokio:
//!
//! ```no_run ignore
//! use translators::{GoogleTranslator, Translator};
//!// tokio = { version = "xxx", features = ["rt-multi-thread"] }
//! #[tokio::main]
//! async fn main() {
//!       let google_trans = GoogleTranslator::default();
//!       let translated_text = google_trans
//!           .translate_async("Hello, world!", "", "es")
//!           .await
//!           .unwrap();
//!       println!("{translated_text}");
//!}
//! ```
//!
//! A simple sync example:
//!
//! ```no_run ignore
//!   use translators::{GoogleTranslator, Translator};
//!
//!   fn main() {
//!       let google_trans = GoogleTranslator::default();
//!       let translated_text = google_trans
//!           .translate_sync("Hello, world!", "", "es")
//!           .unwrap();
//!       println!("{translated_text}");
//!   }
//! ```
//!
//!
//! ## Feature flags
//!
//! By default, Google Translate is employed as the translator; to utilize others, enable them via the flag:
//!
//! - `google`: Google translate.
//! - `deepl`: WIP,
//!

mod translators;

pub use translators::Translator;

#[cfg(feature = "google")]
pub use translators::GoogleTranslator;
