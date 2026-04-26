# Errors Dual-Interface Contracts

## Goal

Implement the shared contract layer for the dual-interface error strategy:

- Rust source of truth: `phenotype-error-core`
- TypeScript interface: `@phenotype/errors`
- Shared parity surface: `contracts/errors/*`

## Outcome

- Added JSON error-code and envelope contracts.
- Split `phenotype-error-core/src/lib.rs` into focused modules.
- Added typed Rust `ErrorCode` and structured `ErrorEnvelope`.
- Added TypeScript `ERROR_CODES` and fixture parity tests.
- Narrowed `packages/types` response error codes from `string` to the shared code union.

## Known Limitation

Workspace-level `cargo fmt --check` is blocked by a preexisting syntax error in
`crates/phenotype-config-core/src/lib.rs`. Focused formatting and validation for
`phenotype-error-core` pass.
