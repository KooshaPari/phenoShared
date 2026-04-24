//! Basic sandbox example
//!
//! This example demonstrates how to create a sandbox,
//! execute commands, and clean up.

use std::time::Duration;

use phenotype_nanovms_client::{NanovmsClient, SandboxConfig, Tier, SandboxExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("=== nanovms-client Basic Example ===\n");

    // Create a mock client (uses in-memory transport, no OPS required)
    let client = NanovmsClient::new_mock();

    // Example 1: Create a simple WASM sandbox
    println!("1. Creating WASM sandbox...");
    let wasm_sandbox = client.create_sandbox_simple("wasm-demo").await?;
    println!("   Created: {} (ID: {})", wasm_sandbox.name, wasm_sandbox.id);
    println!("   Tier: {:?}", wasm_sandbox.tier);
    println!("   State: {:?}", wasm_sandbox.state);

    // Example 2: Create a sandbox with custom configuration
    println!("\n2. Creating Firecracker sandbox with custom config...");
    let config = SandboxConfig::new("firecracker-demo", Tier::Firecracker)
        .with_memory(1024)
        .with_cpus(2)
        .with_env("APP_ENV", "production")
        .with_env("LOG_LEVEL", "debug")
        .with_timeout(Duration::from_secs(120))
        .with_label("team", "backend")
        .with_label("project", "demo");

    let fc_sandbox = client.create_sandbox(config).await?;
    println!("   Created: {} (ID: {})", fc_sandbox.name, fc_sandbox.id);
    println!("   Memory: {} MB", fc_sandbox.config.memory_mb);
    println!("   CPUs: {}", fc_sandbox.config.cpus);
    println!("   Env: {:?}", fc_sandbox.config.env);

    // Example 3: Execute commands
    println!("\n3. Executing commands in WASM sandbox...");
    
    // Execute a simple command
    let output = client.execute(&wasm_sandbox.id, &["echo", "Hello from nanovms!"]).await?;
    println!("   Command: echo Hello from nanovms!");
    println!("   Exit code: {}", output.exit_code);
    println!("   Output: {}", output.stdout);

    // Execute a shell command
    let output = client.execute_shell(&wasm_sandbox.id, "pwd && ls -la").await?;
    println!("\n   Shell command: pwd && ls -la");
    println!("   Exit code: {}", output.exit_code);
    println!("   Output: {}", output.stdout);

    // Example 4: Manage lifecycle
    println!("\n4. Managing sandbox lifecycle...");
    
    // Stop the sandbox
    let stopped = client.stop_sandbox(&wasm_sandbox.id).await?;
    println!("   Stopped sandbox: {} -> {:?}", stopped.name, stopped.state);
    
    // Start it again
    let started = client.start_sandbox(&wasm_sandbox.id).await?;
    println!("   Started sandbox: {} -> {:?}", started.name, started.state);

    // Example 5: Snapshots
    println!("\n5. Working with snapshots...");
    
    // Create a snapshot
    let snapshot = client.snapshot(&fc_sandbox.id, "v1.0").await?;
    println!("   Created snapshot: {} (ID: {})", snapshot.name, snapshot.id);
    println!("   Size: {} bytes", snapshot.size_bytes);
    println!("   Created at: {}", snapshot.created_at);

    // List snapshots
    let snapshots = client.list_snapshots(&fc_sandbox.id).await?;
    println!("   Total snapshots for {}: {}", fc_sandbox.name, snapshots.len());

    // Restore from snapshot (simulated)
    let restored = client.restore_snapshot(&fc_sandbox.id, &snapshot.id).await?;
    println!("   Restored sandbox: {} from snapshot {}", restored.name, snapshot.name);

    // Example 6: List all sandboxes
    println!("\n6. Listing all sandboxes...");
    let all_sandboxes = client.list_sandboxes().await?;
    for sandbox in &all_sandboxes {
        println!("   - {} (ID: {}, Tier: {:?}, State: {:?})",
            sandbox.name, sandbox.id, sandbox.tier, sandbox.state);
    }

    // Example 7: Use extension methods
    println!("\n7. Using SandboxExt methods...");
    
    // Create and use extension methods
    let ext_sandbox = client.create_sandbox_with_tier("ext-demo", Tier::Gvisor).await?;
    println!("   Created: {}", ext_sandbox.name);
    
    // Execute via extension
    let output = ext_sandbox.execute(&client, &["uname", "-a"]).await?;
    println!("   Executed uname: exit_code={}", output.exit_code);
    
    // Create snapshot via extension
    let ext_snapshot = ext_sandbox.snapshot(&client, "ext-snapshot").await?;
    println!("   Created snapshot: {}", ext_snapshot.name);

    // Example 8: Clean up
    println!("\n8. Cleaning up...");
    
    client.delete_sandbox(&wasm_sandbox.id).await?;
    println!("   Deleted WASM sandbox: {}", wasm_sandbox.id);
    
    client.delete_sandbox(&fc_sandbox.id).await?;
    println!("   Deleted Firecracker sandbox: {}", fc_sandbox.id);
    
    ext_sandbox.delete(&client).await?;
    println!("   Deleted GVisor sandbox: {}", ext_sandbox.id);

    println!("\n=== Example Complete ===");

    Ok(())
}
