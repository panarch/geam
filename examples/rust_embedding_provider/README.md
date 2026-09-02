# Rust Embedding: External Provider

This example adds one external Rust provider to the hosted embedding lifecycle.
The Gleam package declares a regular-expression API, while the selected
`geam-example-text-pattern` crate implements its bodyless Erlang externals for
Geam.

## Read The Example

1. [gleam.toml](gleam/gleam.toml) selects the local reference Gleam package.
2. [geam_rust_embedding_provider.gleam](gleam/src/geam_rust_embedding_provider.gleam)
   keeps the provider's opaque Pattern and custom CompileError inside Gleam and
   exposes `Result(Bool, String)` to Rust.
3. [Cargo.toml](Cargo.toml) selects the matching provider crate explicitly.
4. [main.rs](src/main.rs) supplies provider configuration and calls the
   generated function handle.

For an ordinary registry dependency, `geam embedding sync` verifies provider
metadata and asks for approval before adding its exact Cargo declaration. This
repository uses path declarations so the example tests the current package and
provider sources directly. Empty configuration is still explicit because the
provider owns its initialization contract.

## Run

From the repository root:

```sh
cargo build --package geam --bin geam --locked
cd examples/rust_embedding_provider
../../target/debug/geam embedding check
cargo test --locked
cargo run --quiet --locked
```

The application prints:

```text
matched: true
```

Continue with the [complete embedding application](../rust_embedding_application),
which combines provider-backed validation, stdlib IO, structured retained data,
and repeated calls in one workflow.
