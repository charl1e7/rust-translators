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

#[tokio::test]
async fn test_async_builder() {
    let translator = GoogleTranslator::builder()
        .timeout(35usize)
        .delay(0usize)
        .max_concurrency(2usize)
        // .proxy_address("http://user:password@0.0.0.0:80")
        .build();
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
fn test_sync_builder() {
    let translator = GoogleTranslator::builder()
        .timeout(35usize)
        .delay(0usize)
        .max_concurrency(2usize)
        // .proxy_address("http://user:password@0.0.0.0:80")
        .build();
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
//     let translator = GoogleTranslator::new(GoogleTranslatorConfig {
//         timeout: 35, // How long to wait for a request
//         delay: 0, // delay between each request
//         proxy_address: Some("http://0.0.0.0:8080".to_string()), // or https or socks4 or socks5
//     });
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
//     let translator = GoogleTranslator::new(GoogleTranslatorConfig {
//         timeout: 35, // How long to wait for a request
//         delay: 0, // delay between each request
//         proxy_address: Some("http://0.0.0.0:8080".to_string()), // or https or socks4 or socks5
//     });
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
