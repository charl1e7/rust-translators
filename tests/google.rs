use std::time::{Duration, Instant};

use translators::{GoogleTranslator, Translator};

// "Hi"
const TEXT_CHUNKS: u128 = 4;

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
        .text_limit(5000usize)
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
        .text_limit(5000usize)
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

#[tokio::test]
async fn test_async_delay() {
    let (res_time, res_abs) = res_time().await;
    let translator = GoogleTranslator::builder()
        .timeout(35usize)
        .delay(1000usize)
        .text_limit(2usize)
        // .proxy_address("http://user:password@0.0.0.0:80")
        .build();
    let text = "Hi".repeat(TEXT_CHUNKS as usize);
    let source_lang = "en";
    let target_lang = "fr";
    let start_time = Instant::now();
    match translator
        .translate_async(&text, source_lang, target_lang)
        .await
    {
        Ok(_result) => {
            let elapsed_time = start_time.elapsed().as_millis();
            let eq =
                (TEXT_CHUNKS * 1000) + ((res_time.as_millis() - res_abs.as_millis()) * TEXT_CHUNKS);
            assert!(
                elapsed_time >= eq,
                "Execution time {elapsed_time} was less than {eq} seconds {res_abs:?}"
            );
        }
        Err(err) => {
            eprintln!("Google translation error: {:?}", err);
            assert!(false);
        }
    }
}

#[tokio::test]
async fn test_async_conc() {
    let (res_time, res_abs) = res_time().await;
    let translator = GoogleTranslator::builder()
        .timeout(35usize)
        .max_concurrency(1usize)
        .text_limit(2usize)
        // .proxy_address("http://user:password@0.0.0.0:80")
        .build();
    let text = "Hi".repeat(TEXT_CHUNKS as usize);
    let source_lang = "en";
    let target_lang = "fr";
    let start_time = Instant::now();
    match translator
        .translate_async(&text, source_lang, target_lang)
        .await
    {
        Ok(_result) => {
            let elapsed_time = start_time.elapsed().as_millis();
            let eq = TEXT_CHUNKS * (res_time.as_millis() - res_abs.as_millis());
            assert!(
                elapsed_time > eq,
                "Execution time {elapsed_time} was less than {eq} seconds {res_abs:?}"
            );
        }
        Err(err) => {
            eprintln!("Google translation error: {:?}", err);
            assert!(false);
        }
    }
}

async fn res_time() -> (Duration, Duration) {
    let translator = GoogleTranslator::default();
    let text = "HiHiHiHi";
    let source_lang = "en";
    let target_lang = "fr";
    let start_time = Instant::now();
    let res1 = match translator
        .translate_async(text, source_lang, target_lang)
        .await
    {
        Ok(_result) => start_time.elapsed(),
        Err(err) => {
            eprintln!("Google translation error: {:?}", err);
            panic!("err res");
        }
    };

    let start_time = Instant::now();
    let res2 = match translator
        .translate_async(text, source_lang, target_lang)
        .await
    {
        Ok(_result) => start_time.elapsed(),
        Err(err) => {
            eprintln!("Google translation error: {:?}", err);
            panic!("err res");
        }
    };
    let diff = res1.abs_diff(res2);
    (
        res1,
        diff + Duration::from_millis((diff.as_millis() as f64 * 0.3) as u64)
            + Duration::from_millis(50),
    )
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
