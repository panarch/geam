# Rust Embedding: External Provider

This example lets embedded Gleam call a Rust provider. The Gleam package
declares a regular-expression API, while the selected
`geam-example-text-pattern` crate implements its bodyless Erlang externals for
Geam.

## Read The Example

1. [gleam.toml](gleam/gleam.toml) selects the local reference Gleam package.
2. [geam_rust_embedding_provider.gleam](gleam/src/geam_rust_embedding_provider.gleam)
   uses the provider's opaque Pattern and custom CompileError internally.
   Neither can currently appear in a generated Rust function signature, so the
   public function returns `Result(Bool, String)` to Rust.
3. [Cargo.toml](Cargo.toml) selects the matching provider crate explicitly.
4. [main.rs](src/main.rs) supplies provider configuration and calls the
   generated function handle.

For an ordinary registry dependency, add the provider directly to the Rust
application with Cargo, then run `geam embedding sync`. Sync verifies its
metadata and package-version range before generating bindings. This repository
uses a path declaration so the example tests the current package and provider
sources directly. The generated API requests provider configuration; this
provider has no settings, so the example supplies an empty value.

## Run

With Geam, Rust, and Gleam installed, run from the repository root:

```sh
cd examples/embedding/provider
geam embedding check
cargo test --locked
cargo run --quiet --locked
```

The application prints:

```text
matched: true
```

Continue with the [embedding application](../application), which combines
provider-backed validation, stdlib IO, structured data, and repeated calls in
one workflow.
