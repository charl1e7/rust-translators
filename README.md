# translators [![Crates.io][crates-badge]][crates-url] ![License][license-badge]

`translators` is a fast *async/sync*, *thread-safe* library for **Google Translator** with **no API key** and **no limits**. It also
includes support for **proxy**.

**Questions**:

* **There are no limits?**  Yes, I tested the translation of a book exceeding 1 million characters and uploaded the raw string into a single function. I was able to complete the translation in 20 sec.

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
tokio = { version = "x", features = ["rt-multi-thread", "macros"] }
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

### 3. Custom config

```rust
// delete any line if you don't need it
let google_trans = GoogleTranslator::builder()
    // How long to wait for a request in seconds
    .timeout(35 as usize) 
    // delay between requests
    .delay(120 as usize) 
    // shows how many requests can be handled concurrently
    // delete=no limits
    // work only with async 
    .max_concurrency(2 as usize)
    // proxy
    .proxy_address("http://user:password@0.0.0.0:80")
    /// limits on the maximum number of characters from the translator
    /// set if the translator has changed their limits.
    .text_limit(5000)
    .build();
```

## What's New in Version 0.1.4

- **Add max concurrency**
- **Fix request delay handling**
- **Add general error for translators**
- **Add text limit in builder**

# Additional Information

For more details, guides, and advanced usage, please refer to the [examples](https://github.com/charl1e7/rust-translators/tree/main/examples) and [official documentation](https://crates.io/crates/translators).

[crates-badge]: https://img.shields.io/crates/v/translators

[crates-url]: https://crates.io/crates/translators

[license-badge]: https://img.shields.io/github/license/charl1e7/rust-translators?style=flat&color=%230096FF

# Disclaimer

The `translators` library is provided for educational and research purposes only. It is an open-source project with no affiliation or endorsement by Google or any other translation service provider.

The library is distributed "as-is" with no warranties of any kind, express or implied. The author disclaims any liability for damages arising from the use of this library, including data loss or financial loss. Usage of this library is at your own risk, and the author does not receive any financial benefit from its use.

Users are responsible for complying with third-party terms of service, including those of Google Translator or any other translation service provider.
