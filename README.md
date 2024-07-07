[crates-badge]: https://img.shields.io/crates/v/translators
[crates-url]: https://crates.io/crates/translators
[license-badge]: https://img.shields.io/github/license/charl1e7/rust-translators?style=flat&color=%230096FF


# translators [![Crates.io][crates-badge]][crates-url] ![License][license-badge]

`translators` is an async/sync library for Google Translator and Deepl without an API key and limits.

### Async example
```rust
use translators::{GoogleTranslator, Translator};

#[tokio::main]
async fn main() {
    let trans = GoogleTranslator::default();
    let a = trans
        .translate_async("Hello, world!", "", "es")
        .await
        .unwrap();
    println!("{a}");
}
```

Add to the dependency:
```rust
[dependencies]
translators = { version = "0.1.2", features = ["google"] }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

### Sync example
```rust
use translators::{GoogleTranslator, Translator};

fn main() {
    let trans = GoogleTranslator::default();
    let a = trans
        .translate_sync("Hello, world!", "", "es")
        .unwrap();
    println!("{a}");
}
```

Add to the dependency:
```rust
[dependencies]
translators = { version = "0.1.2", features = ["google"] }
```


## Additional Information

For more details, guides, and advanced usage, please refer to the [examples](https://github.com/charl1e7/rust-translators/tree/main/examples) and [official documentation](https://crates.io/crates/translators).


