//! Asynchronous/synchronous text translation library
//! supporting Google Translate and DeepL,
//! without API key requirements or limits.
//! It also includes support for [proxy].
//!
//! Examples of usage in the [repository].
//! - [google]
//! - [deepl] (Soon)
//!
//! [repository]: https://github.com/charl1e7/rust-translators/tree/main/examples
//! [google]: https://docs.rs/translators/0.1.2/translators/struct.GoogleTranslator.html
//! [deepl]: https://docs.rs/translators/0.1.2/translators/struct.DeeplTranslator.html
//! [proxy]: https://docs.rs/translators/0.1.2/translators/struct.GoogleTranslator.html#proxy
//!
//! ## Feature flags
//!
//! By default, Google Translate is employed as the translator; to utilize others, enable them via the flag:
//!
//! - `tokio-async` - for enable async features
//! - `google` - for enable google-translate
//! - `deepl` - for enable Deepl translate (soon)
//!

mod translators;

// general
pub use translators::translator::translator::Translator;

// google
#[cfg(feature = "google")]
pub use translators::google::error::GoogleError;
#[cfg(feature = "google")]
pub use translators::google::translator::GoogleTranslator;
#[cfg(feature = "google")]
pub use translators::google::translator::GoogleTranslatorConfig;
