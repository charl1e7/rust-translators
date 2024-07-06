//! Asynchronous/synchronous text translation library
//! supporting Google Translate and DeepL,
//! without API key requirements or limits.
//!
//! Examples of usage in the [repository].
//! - [google]
//! - [deepl]
//!
//! [repository]: https://github.com/charl1e7/rust-translators/tree/main/examples
//!
//! [google]: https://docs.rs/translators/0.1.1/translators/struct.GoogleTranslator.html
//! [deepl]: https://docs.rs/translators/0.1.1/translators/struct.DeeplTranslator.html
//!
//! ## Feature flags
//!
//! By default, Google Translate is employed as the translator; to utilize others, enable them via the flag:
//!
//! - `google`: Google translate.
//! - `deepl`: Deepl,
//!

mod translators;

pub use translators::Translator;

#[cfg(feature = "google")]
pub use translators::GoogleTranslator;
