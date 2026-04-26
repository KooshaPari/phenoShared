# Validation

## Passed

```bash
cargo test -p phenotype-error-core
cargo clippy -p phenotype-error-core -- -D warnings
cd packages/errors && bun run typecheck
cd packages/errors && bun test
cd packages/types && bun test
cargo fmt --check -p phenotype-error-core
```

## Blocked

```bash
cargo fmt --check
```

The workspace-wide formatter check is blocked before this change by a syntax
error in `crates/phenotype-config-core/src/lib.rs`.
