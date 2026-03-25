# Phenotype Postgres Adapter

A hexagonal architecture adapter implementing the `Repository` port for PostgreSQL.

## Features

- **Connection Pooling**: Uses `deadpool-postgres` for efficient connection pooling
- **Async/Await**: Fully async implementation using `tokio`
- **Repository Pattern**: Implements the `Repository` port interface from `phenotype-port-interfaces`
- **Optimistic Locking**: Built-in version tracking for concurrent updates
- **Schema Management**: Auto-creates tables with proper indexes

## Usage

```rust
use phenotype_postgres_adapter::{
    PostgresRepository, PostgresConfig, create_pool,
};

#[tokio::main]
async fn main() {
    let config = PostgresConfig {
        host: "localhost".into(),
        port: 5432,
        user: "postgres".into(),
        password: "secret".into(),
        database: "app".into(),
        max_size: 16,
        ..Default::default()
    };

    let pool = create_pool(config).await.unwrap();
    let repo = PostgresRepository::with_default_table(pool);
    repo.initialize().await.unwrap();
}
```

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                         │
│              (PostgresRepository, Commands)                  │
└─────────────────────────────┬───────────────────────────────┘
                              │ implements
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    Ports (Interfaces)                        │
│           Repository<T>, FindById<T>, Save<T>                │
└─────────────────────────────┬───────────────────────────────┘
                              │ implemented by
                              ▼
┌─────────────────────────────────────────────────────────────┐
│              Adapters (This Crate)                           │
│    deadpool_postgres + tokio_postgres + PostgreSQL           │
└─────────────────────────────────────────────────────────────┘
```

## License

MIT
