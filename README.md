# translators [![Crates.io][crates-badge]][crates-url] ![License][license-badge]

`translators` is an *async/sync* library for **Google Translator** with **no API key** and **no limits**. It also
includes support for **proxy**.

**Questions**:

* **There are no limits?**  Yes, I tested the translation of a book exceeding 1 million characters and uploaded the raw
  string into a single function.

**Features flags**

* `google` - add support google-translate
* `tokio-async` - add support async

# Examples

* [google-translate](https://github.com/charl1e7/rust-translators/tree/main/examples/google)

### 1. Async example

```rust
use translators::{GoogleTranslator, Translator};

#[tokio::main]
async fn main() {
    let google_trans = GoogleTranslator::default();
    let res = google_trans
        .translate_async("Hello, world!", "", "es")
        .await
        .unwrap();
    println!("{res}");
}
```

Add to the dependency:

```rust
[dependencies]
translators = { version = "0.1.3", features = ["google", "tokio-async"] }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

### 2. Sync example

```rust
use translators::{GoogleTranslator, Translator};

fn main() {
    let google_trans = GoogleTranslator::default();
    let res = google_trans
        .translate_sync("Hello, world!", "", "es")
        .unwrap();
    println!("{res}");
}
```

Add to the dependency:

```rust
[dependencies]
translators = { version = "0.1.3", features = ["google"] }
```

### 3. Proxy and custom config

```rust
let google_trans = GoogleTranslator::builder()
    .timeout(35 as u64) // How long to wait for a request in seconds
    .delay(120 as u64) //How long to wait for a request in milliseconds
    .proxy_address("http://user:password@0.0.0.0:80") // delete the line if you don't need proxy
    .build();
```

## What's New in Version 0.1.3

- **Fix separation of words**
- **Add google builder**
- **New feature flag "all"**

# Additional Information

For more details, guides, and advanced usage, please refer to the [examples](https://github.com/charl1e7/rust-translators/tree/main/examples) and [official documentation](https://crates.io/crates/translators).

[crates-badge]: https://img.shields.io/crates/v/translators

[crates-url]: https://crates.io/crates/translators

[license-badge]: https://img.shields.io/github/license/charl1e7/rust-translators?style=flat&color=%230096FF
