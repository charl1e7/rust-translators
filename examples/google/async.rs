use translators::{GoogleTranslator, Translator};

// tokio = { version = "x", features = ["rt-multi-thread"] }
#[tokio::main]
async fn main() {
    let google_trans = GoogleTranslator::default();
    let translated_text = google_trans
        .translate_async("Hello, world!", "", "es")
        .await
        .unwrap();
    println!("{translated_text}");
}
