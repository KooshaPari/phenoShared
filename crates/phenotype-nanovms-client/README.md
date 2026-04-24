# phenotype-nanovms-client

A Rust client library for NanoVMs unikernel orchestration. This library provides async APIs for managing NanoVMs instances through the OPS CLI.

> **Provenance:** promoted from `bare-cua/integrations/014-nanovms-integration/nanovms-client/` into the `phenoShared` workspace so that both the bare-cua sandbox (014) and the helioscli integration (015) can depend on a single canonical crate. See `NOTICE` for attribution.

## Features

- **Tiered Sandboxing**: Support for three isolation tiers:
  - **Tier 1**: WebAssembly (WASM) sandboxes for lightweight isolation
  - **Tier 2**: gVisor-based sandboxes for enhanced security
  - **Tier 3**: Firecracker microVMs for full virtualization

- **Async/Await**: Built on tokio for efficient async operations
- **CLI Integration**: Direct integration with the OPS CLI tool
- **Mock Transport**: In-memory transport for testing
- **Session Snapshots**: Create and restore sandbox snapshots

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
nanovms-client = { path = "path/to/nanovms-client" }
```

## Prerequisites

Install OPS (the NanoVMs orchestration tool):

```bash
curl https://ops.city/get.sh -sSfL | sh
```

## Usage

### Basic Example

```rust
use phenotype_nanovms_client::{NanovmsClient, SandboxConfig, Tier};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = NanovmsClient::new();
    
    // Create a WASM sandbox
    let config = SandboxConfig::new("my-sandbox", Tier::WASM)
        .with_memory(256)
        .with_cpus(2);
    
    let sandbox = client.create_sandbox(config).await?;
    println!("Created sandbox: {}", sandbox.id);
    
    // Execute a command
    let output = client.execute(&sandbox.id, &["echo", "Hello, World!"]).await?;
    println!("stdout: {}", output.stdout);
    
    // Clean up
    client.delete_sandbox(&sandbox.id).await?;
    
    Ok(())
}
```

### Using the Builder Pattern

```rust
use phenotype_nanovms_client::NanovmsClient;

let client = NanovmsClient::builder()
    .cli_transport()
    .ops_path("/usr/local/bin/ops")
    .default_timeout(std::time::Duration::from_secs(120))
    .build()?;
```

### Working with Different Tiers

```rust
use phenotype_nanovms_client::{NanovmsClient, Tier};

let client = NanovmsClient::new();

// Tier 1: WebAssembly
let wasm = client.create_sandbox_with_tier("wasm-box", Tier::WASM).await?;

// Tier 2: gVisor
let gvisor = client.create_sandbox_with_tier("gvisor-box", Tier::Gvisor).await?;

// Tier 3: Firecracker
let fc = client.create_sandbox_with_tier("firecracker-box", Tier::Firecracker).await?;
```

### Snapshots

```rust
use phenotype_nanovms_client::NanovmsClient;

let client = NanovmsClient::new();
let sandbox = client.create_sandbox_simple("my-app").await?;

// Create a snapshot
let snapshot = client.snapshot(&sandbox.id, "v1.0").await?;
println!("Created snapshot: {} ({} bytes)", snapshot.id, snapshot.size_bytes);

// List snapshots
let snapshots = client.list_snapshots(&sandbox.id).await?;

// Restore from snapshot
let restored = client.restore_snapshot(&sandbox.id, &snapshot.id).await?;
```

### Testing with Mock Transport

```rust
use phenotype_nanovms_client::NanovmsClient;

let client = NanovmsClient::new_mock();
let sandbox = client.create_sandbox_simple("test").await?;
// Operations work without actual OPS installation
```

## API Reference

### `NanovmsClient`

The main client for interacting with nanovms.

#### Methods

- `new()` - Create a client with default CLI transport
- `new_mock()` - Create a client with mock transport for testing
- `builder()` - Get a builder for custom configuration

#### Sandbox Operations

- `create_sandbox(config)` - Create a new sandbox
- `create_sandbox_simple(name)` - Create with defaults
- `create_sandbox_with_tier(name, tier)` - Create with specific tier
- `get_sandbox(id)` - Get sandbox by ID
- `list_sandboxes()` - List all sandboxes
- `start_sandbox(id)` - Start a sandbox
- `stop_sandbox(id)` - Stop a sandbox
- `delete_sandbox(id)` - Delete a sandbox
- `wait_for_state(id, state, timeout)` - Wait for state change

#### Execution

- `execute(sandbox_id, command)` - Execute command in sandbox
- `execute_shell(sandbox_id, command)` - Execute shell command

#### Snapshots

- `snapshot(sandbox_id, name)` - Create a snapshot
- `list_snapshots(sandbox_id)` - List snapshots
- `restore_snapshot(sandbox_id, snapshot_id)` - Restore from snapshot
- `delete_snapshot(sandbox_id, snapshot_id)` - Delete a snapshot

## Architecture

The client uses a transport abstraction to communicate with nanovms:

- **CliTransport**: Wraps the OPS CLI tool
- **MockTransport**: In-memory implementation for testing
- **HttpTransport** (planned): Direct HTTP API communication
- **GrpcTransport** (planned): gRPC API communication

## Error Handling

All operations return `Result<T, NanovmsError>`:

```rust
use phenotype_nanovms_client::NanovmsError;

match client.get_sandbox("invalid-id").await {
    Ok(sandbox) => println!("Found: {}", sandbox.name),
    Err(NanovmsError::SandboxNotFound(id)) => println!("Not found: {}", id),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Feature Flags

- `cli-client` (default): Enable CLI transport
- `http-client`: Enable HTTP REST API transport
- `grpc-client`: Enable gRPC transport
- `ffi-client`: Enable FFI bindings

## License

MIT
