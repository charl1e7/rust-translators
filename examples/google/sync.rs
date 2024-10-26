use translators::{GoogleTranslator, Translator};

// translators = { version = "0.1.4", features = ["google", "tokio-async"] }
fn main() {
    let google_trans = GoogleTranslator::default();
    let translated_text = google_trans
        .translate_sync("Hello, world!", "", "es")
        .unwrap();
    println!("{translated_text}");

    let google_trans = GoogleTranslator::builder()
        .timeout(35usize)
        .delay(0usize)
        // .proxy_address("http://user:password@0.0.0.0:80")
        .build();
    let translated_text = google_trans
        .translate_sync("Hello, world!", "", "es")
        .unwrap();
    println!("{translated_text}");
}
