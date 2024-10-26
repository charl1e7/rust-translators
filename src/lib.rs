//! `translators` is an *async/sync*, *tread-safe* library for **Google Translator**
//! with **no API key** and **no limits**.
//! It also includes support for **proxy**.
//!
//! Examples of usage in the [repository]:
//! - [google]
//!
//! [repository]: https://github.com/charl1e7/rust-translators/tree/main/examples
//! [google]: https://docs.rs/translators/0.1.4/translators/struct.GoogleTranslator.html
//! [proxy]: https://docs.rs/translators/0.1.4/translators/struct.GoogleTranslator.html#proxy
//!
//! ## Feature flags
//!
//! - `tokio-async` - for enable async features
//! - `google` - for enable google-translate
//!

mod translators;

// general trait
pub use translators::translator::translator::Translator;

// google
#[cfg(feature = "google")]
pub use translators::google::error::GoogleError;
#[cfg(feature = "google")]
pub use translators::google::translator::GoogleTranslator;
