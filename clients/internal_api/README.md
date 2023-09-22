<div id="top"></div>

<p align="center">
    <a href="https://github.com/lenra-io/app-lib-rust/stargazers">
        <img src="https://img.shields.io/github/stars/lenra-io/app-lib-rust.svg?style=flat-square" alt="Stars" />
    </a>
    <a href="https://github.com/lenra-io/app-lib-rust/actions">
        <img src="https://img.shields.io/github/workflow/status/lenra-io/app-lib-rust/ci?style=flat-square" alt="Build Status" />
    </a>
    
<a href="https://crates.io/crates/internal_api">
    <img src="https://img.shields.io/crates/d/internal_api?style=flat-square" alt="Downloads" />
</a>
<a href="https://crates.io/crates/internal_api">
    <img src="https://img.shields.io/crates/v/internal_api?style=flat-square" alt="Crates.io" />
</a>

</p>

InternalApi client, generated from the OpenAPI spec.

# Usage

```rust
use internal_api::InternalApiClient;
use internal_api::model::*;
#[tokio::main]
async fn main() {
    let client = InternalApiClient::from_env();
    let coll = "your coll";
    let response = client.delete_collection(coll).await.unwrap();
    println!("{:#?}", response);
}
```

This example loads configuration from environment variables, specifically:

* `INTERNAL_API_BEARER_AUTH`



# Installation

Add this to your Cargo.toml:

```toml
[dependencies]
internal_api = "0.1.0"
```


# Documentation



* [Client Library Documentation](https://docs.rs/internal_api)


You can see working examples of every API call in the `examples/` directory.

# Contributing

Contributions are welcome!

*Library created with [Libninja](https://www.libninja.com).*