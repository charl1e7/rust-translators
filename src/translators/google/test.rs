use crate::{GoogleTranslator, Translator};

#[tokio::test]
async fn test_translate_async() {
    let translator = GoogleTranslator::default();
    let text = "Hello, world!";
    let source_lang = "en";
    let target_lang = "fr";

    match translator
        .translate_async(text, source_lang, target_lang)
        .await
    {
        Ok(result) => {
            assert_eq!(result, "Bonjour le monde!");
        }
        Err(err) => {
            eprintln!("Google translation error: {:?}", err);
            assert!(false);
        }
    }
}
