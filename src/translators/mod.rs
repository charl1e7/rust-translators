#[cfg(feature = "google")]
pub mod google;
mod translator;

pub use translator::translator::Translator;
