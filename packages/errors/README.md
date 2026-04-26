# @phenotype/errors

Typed TypeScript interface for the shared Phenotype error contract.

The source of truth for cross-language parity is:

- `../../contracts/errors/error-codes.json`
- `../../contracts/errors/error-envelope.schema.json`
- `../../contracts/errors/fixtures/`

Rust services should use `phenotype-error-core`. TypeScript services should use
`@phenotype/errors`. Both interfaces must pass fixture parity tests before
adding or changing a wire error code.
