# Phenotype Config Adapter

Configuration adapter implementing the `Config` port for hexagonal architecture.

## Features

- Multi-source configuration (env, yaml, json)
- Async/await with `tokio`
- Config pattern implementation

## Usage

```rust
use phenotype_config_adapter::{ConfigAdapter, Config};

#[tokio::main]
async fn main() {
    let config = ConfigAdapter::new();
    config.load_env(None).await;
}
```
