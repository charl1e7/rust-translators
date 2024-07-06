pub trait Translator {
    type Config;
    type Error;

    async fn translate_async(
        self: Self,
        text: &str,
        target_language: &str,
        source_language: &str,
    ) -> Result<String, Self::Error>;
    fn default() -> Self;
}
