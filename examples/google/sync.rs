use translators::{GoogleTranslator, Translator};

async fn main() {
    let google_trans = GoogleTranslator::default();
    let translated_text = google_trans
        .translate_sync("Hello, world!", "", "es")
        .unwrap();
    println!("{translated_text}");
}
