use std::{
    fs,
    time::Instant,
};

use translators::{GoogleTranslator, Translator};

const TEXT_CHUNKS: u128 = 4;

#[tokio::test]
async fn test_async() {
    let translator = GoogleTranslator::default();
    let text = "Hello, world!";
    let source_lang = "en";
    let target_lang = "fr";

    match translator.translate_async(text, source_lang, target_lang).await {
        Ok(result) => assert_eq!(result, "Bonjour le monde!"),
        Err(err) => panic!("Google translation error: {:?}", err),
    }
}

#[test]
fn test_sync() {
    let translator = GoogleTranslator::default();
    let text = "Hello, world!";
    let source_lang = "en";
    let target_lang = "fr";

    match translator.translate_sync(text, source_lang, target_lang) {
        Ok(result) => assert_eq!(result, "Bonjour le monde!"),
        Err(err) => panic!("Google translation error: {:?}", err),
    }
}

#[tokio::test]
async fn test_async_builder() {
    let translator = GoogleTranslator::builder()
        .timeout(35usize)
        .delay(0usize)
        .max_concurrency(2usize)
        .text_limit(5000usize)
        .build();
    let text = "Hello, world!";
    let source_lang = "en";
    let target_lang = "fr";

    match translator.translate_async(text, source_lang, target_lang).await {
        Ok(result) => assert_eq!(result, "Bonjour le monde!"),
        Err(err) => panic!("Google translation error: {:?}", err),
    }
}

#[test]
fn test_sync_builder() {
    let translator = GoogleTranslator::builder()
        .timeout(35usize)
        .delay(0usize)
        .max_concurrency(2usize)
        .text_limit(5000usize)
        .build();
    let text = "Hello, world!";
    let source_lang = "en";
    let target_lang = "fr";

    match translator.translate_sync(text, source_lang, target_lang) {
        Ok(result) => assert_eq!(result, "Bonjour le monde!"),
        Err(err) => panic!("Google translation error: {:?}", err),
    }
}

#[tokio::test]
async fn test_async_delay() {
    let translator = GoogleTranslator::builder()
        .timeout(35usize)
        .delay(1000usize)
        .text_limit(2usize)
        .build();
    let text = "Hi".repeat(TEXT_CHUNKS as usize);
    let source_lang = "en";
    let target_lang = "fr";
    
    let start_time = Instant::now();
    match translator.translate_async(&text, source_lang, target_lang).await {
        Ok(_) => {
            let elapsed_time = start_time.elapsed().as_millis();
            let min_expected_time = TEXT_CHUNKS * 1000;
            
            assert!(
                elapsed_time >= min_expected_time,
                "Execution time {elapsed_time}ms was less than expected delay {min_expected_time}ms. Delay logic might be broken."
            );
        }
        Err(err) => panic!("Google translation error: {:?}", err),
    }
}

#[tokio::test]
async fn test_async_conc() {
    let translator_conc = GoogleTranslator::builder()
        .timeout(35usize)
        .text_limit(2usize)
        .build();
    let text = "Hi".repeat(TEXT_CHUNKS as usize);
    let source_lang = "en";
    let target_lang = "fr";
    
    let start_conc = Instant::now();
    let _ = translator_conc.translate_async(&text, source_lang, target_lang).await.unwrap();
    let conc_time = start_conc.elapsed().as_millis();

    let translator_seq = GoogleTranslator::builder()
        .timeout(35usize)
        .max_concurrency(1usize) 
        .text_limit(2usize)
        .build();
        
    let start_seq = Instant::now();
    let _ = translator_seq.translate_async(&text, source_lang, target_lang).await.unwrap();
    let seq_time = start_seq.elapsed().as_millis();

    assert!(
        seq_time > conc_time,
        "Sequential execution ({seq_time}ms) should take longer than concurrent ({conc_time}ms). Concurrency limit might be broken."
    );
}

#[tokio::test]
async fn test_file() {
    let translator = GoogleTranslator::builder()
        .timeout(35usize)
        .max_concurrency(10usize) 
        .build();
    let text = fs::read_to_string("./tests/input.txt").unwrap();
    let source_lang = "fr";
    let target_lang = "en";
    match translator.translate_async(&text, source_lang, target_lang).await {
        Ok(result) => {
            assert!(
                (result.chars().count() as isize - text.chars().count() as isize).abs()
                    < (text.chars().count() as f64 * 0.1) as isize,
            );
        }
        Err(err) => panic!("Google translation error: {:?}", err),
    }
}