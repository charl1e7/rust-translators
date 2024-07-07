use std::fmt::Debug;

pub trait Translator: Clone + Default + Debug + Sync + Send {
    type Config;
    type Error;

    async fn translate_async(
        self: &Self,
        text: &str,
        target_language: &str,
        source_language: &str,
    ) -> Result<String, Self::Error>;
    fn translate_sync(
        self: &Self,
        text: &str,
        target_language: &str,
        source_language: &str,
    ) -> Result<String, Self::Error>;
}
