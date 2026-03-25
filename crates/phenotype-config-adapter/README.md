# Phenotype Config Adapter

A hexagonal architecture adapter implementing the `Config` port for configuration management.

## Features

- **Multi-Source**: Environment variables, YAML files, JSON files, defaults
- **Priority System**: Later sources override earlier ones
- **Async/Await**: Fully async implementation using `tokio`
- **Config Pattern**: Implements the `Config` port interface from `phenotype-port-interfaces`

## Usage

```rust
use phenotype_config_adapter::{ConfigAdapter, GetStr, GetBool, GetUsize};
use std::path::Path;

#[tokio::main]
async fn main() {
    let config = ConfigAdapter::new();
    
    // Load from environment with prefix
    config.load_env(Some("APP_")).await;
    
    // Or load from file
    config.load_yaml(Path::new("config.yaml")).await.unwrap();
    
    // Use the config
    let db_url = config.get_str("database_url").await.unwrap();
    let port = config.get_usize("server_port").await.unwrap();
    let debug = config.get_bool("debug").await.unwrap_or(false);
}
```

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                         │
│                   (ConfigAdapter)                            │
└─────────────────────────────┬───────────────────────────────┘
                              │ implements
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    Ports (Interfaces)                        │
│        Config<T>, GetStr, GetBool, GetUsize                  │
└─────────────────────────────┬───────────────────────────────┘
                              │ implemented by
                              ▼
┌─────────────────────────────────────────────────────────────┐
│              Adapters (This Crate)                           │
│    Env vars, YAML, JSON, HashMap + tokio::sync::RwLock     │
└─────────────────────────────────────────────────────────────┘
```

## License

MIT
