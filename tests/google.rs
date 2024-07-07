use translators::{GoogleTranslator, Translator};

#[tokio::test]
async fn test_async() {
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
#[test]
fn test_sync() {
    let translator = GoogleTranslator::default();
    let text = "Hello, world!";
    let source_lang = "en";
    let target_lang = "fr";

    match translator.translate_sync(text, source_lang, target_lang) {
        Ok(result) => {
            assert_eq!(result, "Bonjour le monde!");
        }
        Err(err) => {
            eprintln!("Google translation error: {:?}", err);
            assert!(false);
        }
    }
}

// #[tokio::test]
// async fn test_async_proxy() {
//     let translator = GoogleTranslator{
//         timeout: 35,
//         delay: 0,
//         proxy_adress: Some("http://0.0.0.0:80".to_string()),
//     };
//     let text = "Hello, world!";
//     let source_lang = "en";
//     let target_lang = "fr";
//
//     match translator
//         .translate_async(text, source_lang, target_lang)
//         .await
//     {
//         Ok(result) => {
//             assert_eq!(result, "Bonjour le monde!");
//         }
//         Err(err) => {
//             eprintln!("Google translation error: {:?}", err);
//             assert!(false);
//         }
//     }
// }
// #[test]
// fn test_sync_proxy() {
//     let translator = GoogleTranslator{
//         timeout: 35,
//         delay: 0,
//         proxy_adress: Some("http://0.0.0.0:1080".to_string()),
//     };
//     let text = "Hello, world!";
//     let source_lang = "en";
//     let target_lang = "fr";
//
//     match translator.translate_sync(text, source_lang, target_lang) {
//         Ok(result) => {
//             assert_eq!(result, "Bonjour le monde!");
//         }
//         Err(err) => {
//             eprintln!("Google translation error: {:?}", err);
//             assert!(false);
//         }
//     }
// }
