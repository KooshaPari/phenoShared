# phenotype-shared

Shared Rust crates extracted from the Phenotype project ecosystem for cross-repo reuse.

## Crates

| Crate | Description | Source |
|-------|-------------|--------|
| `phenotype-event-sourcing` | Append-only event store with SHA-256 hash chains, snapshots, and replay | AgilePlus |
| `phenotype-cache-adapter` | Two-tier LRU + concurrent cache with TTL and metrics hooks | thegent + tokenledger |
| `phenotype-policy-engine` | Policy evaluation engine with TOML-based rule definitions | thegent + civ |
| `phenotype-state-machine` | Generic finite state machine with transition guards and validation | AgilePlus |

## Usage

Add a dependency on any crate:

```toml
[dependencies]
phenotype-event-sourcing = { git = "https://github.com/phenotype-dev/phenotype-shared" }
```

## License

MIT
