# Rust Embedding: Structured Data

This example adds recursive ordinary data to the provider-free first call.
Rust passes a `Vec` of Tuple rows to Gleam, receives a retained List of Results,
and passes that same List back to another Gleam function without rebuilding it.

## Read The Example

1. [geam_rust_embedding_data.gleam](gleam/src/geam_rust_embedding_data.gleam)
   validates rows and totals the accepted quantities using Gleam Lists,
   Tuples, and Results.
2. [main.rs](src/main.rs) supplies Rust-native inputs, reads the returned List,
   and borrows it for the second call.
3. [runs.rs](tests/runs.rs) fixes the complete application output.

The two calls share one loaded and sealed module. `&reviewed` refers to the
retained List owned by that module, so Geam does not materialize and reconstruct
all of its items before the `total` call.

## Run

From the repository root:

```sh
cargo build --package geam --bin geam --locked
cd examples/rust_embedding_data
../../target/debug/geam embedding check
cargo test --locked
cargo run --quiet --locked
```

The application prints:

```text
accepted: A-1 (3)
rejected: quantity must not be negative
accepted: C-3 (4)
total: 7
```

Continue with [a Gleam package](../rust_embedding_package) to add a locked
dependency and regenerate the host profile.
