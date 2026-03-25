# Phenotype Redis Adapter

A hexagonal architecture adapter implementing the `Cache` port for Redis.

## Features

- **Connection Pooling**: Uses `redis` crate with `ConnectionManager` for efficient connections
- **Async/Await**: Fully async implementation using `tokio`
- **Cache Pattern**: Implements the `Cache` port interface from `phenotype-port-interfaces`
- **TTL Support**: Built-in TTL management for cache entries
- **Serialization**: Stores data as raw bytes for flexibility

## Usage

```rust
use phenotype_redis_adapter::{
    RedisCache, RedisConfig, create_pool,
};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let config = RedisConfig::default();
    let pool = create_pool(config).await.unwrap();
    
    // Create cache with default TTL of 5 minutes
    let cache = RedisCache::with_default_ttl(pool, Duration::from_secs(300));
    
    // Use the cache
    cache.set("key", b"value".to_vec()).await.unwrap();
    let value = cache.get("key").await.unwrap();
}
```

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                         │
│                    (RedisCache, Commands)                    │
└─────────────────────────────┬───────────────────────────────┘
                              │ implements
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    Ports (Interfaces)                        │
│           Cache<T>, Get<T>, Set<T>, Delete<T>                │
└─────────────────────────────┬───────────────────────────────┘
                              │ implemented by
                              ▼
┌─────────────────────────────────────────────────────────────┐
│              Adapters (This Crate)                           │
│         redis + tokio + ConnectionManager                   │
└─────────────────────────────────────────────────────────────┘
```

## License

MIT
