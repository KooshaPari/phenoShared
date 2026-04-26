# Research

## Existing State

- Rust workspace includes `crates/phenotype-error-core`.
- TypeScript package exists as `packages/errors` with package name `@phenotype/errors`.
- Legacy specs referenced `@helios/errors`; those references were updated to the
  package name that exists in this repository.
- Rust previously emitted envelope codes like `ERR_404`.
- TypeScript used enum codes like `NOT_FOUND`.
- Rust `ErrorEnvelope.details` was `Option<String>`.
- TypeScript details used structured `Record<string, unknown>`.

## Decision

Use one shared contract with two interfaces:

- Rust exposes typed `ErrorCode` and structured `ErrorEnvelope`.
- TypeScript exposes `ErrorCode`, `ERROR_CODES`, `PhenotypeErrorEnvelope`, and
  `HeliosAppError`.
- JSON fixtures under `contracts/errors/fixtures/` verify parity without
  introducing a redundant Rust facade crate.
