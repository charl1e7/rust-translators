# translators [![Crates.io][crates-badge]][crates-url] ![License][license-badge]

`translators` is an *async/sync*, *tread-safe* library for **Google Translator** with **no API key** and **no limits**. It also
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
translators = { version = "0.1.4", features = ["google", "tokio-async"] }
tokio = { version = "x", features = ["rt-multi-thread"] }
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
translators = { version = "0.1.4", features = ["google"] }
```

### 3. Сustom config

```rust
// delete any line if you don't need it
let google_trans = GoogleTranslator::builder()
    // How long to wait for a request in seconds
    .timeout(35 as usize) 
    //How long to wait for a request in milliseconds
    .delay(120 as usize) 
    // shows how many requests can be handled concurrently
    // work only with async without delay
    .max_concurrency(2 as usize)
    // proxy
    .proxy_address("http://user:password@0.0.0.0:80")
    .build();
```

## What's New in Version 0.1.4

- **Add max concurrency in builder**
- **Fix request delay handling**

# Additional Information

For more details, guides, and advanced usage, please refer to the [examples](https://github.com/charl1e7/rust-translators/tree/main/examples) and [official documentation](https://crates.io/crates/translators).

[crates-badge]: https://img.shields.io/crates/v/translators

[crates-url]: https://crates.io/crates/translators

[license-badge]: https://img.shields.io/github/license/charl1e7/rust-translators?style=flat&color=%230096FF

# Disclaimer

The `translators` library is provided for educational and research purposes only. It is an open-source project with no affiliation or endorsement by Google or any other translation service provider.

The library is distributed "as-is" with no warranties of any kind, express or implied. The author disclaims any liability for damages arising from the use of this library, including data loss or financial loss. Usage of this library is at your own risk, and the author does not receive any financial benefit from its use.

Users are responsible for complying with third-party terms of service, including those of Google Translator or else translators.
