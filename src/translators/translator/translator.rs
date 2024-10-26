use std::fmt::Debug;

pub trait Translator: Clone + Default + Debug + Send + Sync {
    type Error;

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
