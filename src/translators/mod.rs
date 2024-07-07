#[cfg(feature = "google")]
mod google;
mod translator;

#[cfg(feature = "google")]
pub use google::translator::GoogleTranslator;
pub use translator::translator::Translator;
