use std::{error::Error, fmt::{Debug, Display}};

pub trait Translator: Clone + Default + Debug + Send + Sync {
    type Error: Debug + Display + Error + Clone + PartialEq + Eq;

    #[cfg(feature = "tokio-async")]
    async fn translate_async(
        &self,
        text: &str,
        target_language: &str,
        source_language: &str,
    ) -> Result<String, Self::Error>;

    fn translate_sync(
        &self,
        text: &str,
        target_language: &str,
        source_language: &str,
    ) -> Result<String, Self::Error>;
}
