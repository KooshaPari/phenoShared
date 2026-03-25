# Phenotype HTTP Adapter

A hexagonal architecture adapter implementing the `HttpClient` port for HTTP requests.

## Features

- **Async/Await**: Fully async implementation using `reqwest` and `tokio`
- **HTTP Pattern**: Implements the `HttpClient` port interface from `phenotype-port-interfaces`
- **Configurable**: Timeout, retry, and connection settings
- **Header Support**: Full header customization

## Usage

```rust
use phenotype_http_adapter::{
    ReqwestHttpClient, HttpClientConfig,
    HttpRequest,
};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let config = HttpClientConfig {
        timeout: Duration::from_secs(30),
        connect_timeout: Duration::from_secs(10),
        max_retries: 3,
        user_agent: Some("my-app/1.0".into()),
    };
    
    let client = ReqwestHttpClient::new(config).unwrap();
    
    let request = HttpRequest {
        url: "https://api.example.com/data".into(),
        method: "GET".into(),
        headers: vec![],
        body: None,
    };
    
    let response = client.request(request).await.unwrap();
    println!("Status: {}", response.status);
}
```

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                         │
│                  (ReqwestHttpClient)                        │
└─────────────────────────────┬───────────────────────────────┘
                              │ implements
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    Ports (Interfaces)                        │
│              HttpClient<T>, HttpRequest                      │
└─────────────────────────────┬───────────────────────────────┘
                              │ implemented by
                              ▼
┌─────────────────────────────────────────────────────────────┐
│              Adapters (This Crate)                           │
│           reqwest + tokio + HTTP/1.1, HTTP/2               │
└─────────────────────────────────────────────────────────────┘
```

## License

MIT
