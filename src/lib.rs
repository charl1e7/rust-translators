//! `translators` is an *async/sync*, *thread-safe* library for **Google Translator**
//! with **no API key** and **no limits**.
//! It also includes support for **proxy**.
//!
//! Examples of usage in the [repository].
//!
//! [repository]: https://github.com/charl1e7/rust-translators/tree/main/examples
//! [google]: https://docs.rs/translators/0.1.5/translators/struct.GoogleTranslator.html
//! [proxy]: https://docs.rs/translators/0.1.5/translators/struct.GoogleTranslator.html#proxy
//!
//! ## Feature flags
//!
//! - `all`
//! - `tokio-async` - for enable async features
//! - `google` - for enable google-translate
//!

mod translators;

// general
pub use translators::translator::Error;
pub use translators::translator::Translator;

#[cfg(feature = "google")]
pub use translators::google::translator::GoogleTranslator;
