# Phenotype Postgres Adapter

<<<<<<< HEAD
A hexagonal architecture adapter implementing the `Repository` port for PostgreSQL.

## Features

- **Connection Pooling**: Uses `deadpool-postgres` for efficient connection pooling
- **Async/Await**: Fully async implementation using `tokio`
- **Repository Pattern**: Implements the `Repository` port interface from `phenotype-port-interfaces`
- **Optimistic Locking**: Built-in version tracking for concurrent updates
- **Schema Management**: Auto-creates tables with proper indexes
=======
PostgreSQL adapter implementing the `Repository` port for hexagonal architecture.

## Features

- Connection pooling with `deadpool-postgres`
- Async/await with `tokio`
- Repository pattern implementation
>>>>>>> main

## Usage

```rust
<<<<<<< HEAD
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

=======
use phenotype_postgres_adapter::{PostgresRepository, PostgresConfig, create_pool};

#[tokio::main]
async fn main() {
    let config = PostgresConfig::default();
>>>>>>> main
    let pool = create_pool(config).await.unwrap();
    let repo = PostgresRepository::with_default_table(pool);
    repo.initialize().await.unwrap();
}
```
<<<<<<< HEAD

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
=======
>>>>>>> main
