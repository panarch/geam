# Rust Embedding: Structured Data

This example passes structured data between Rust and Gleam. Rust sends a `Vec`
of Tuple rows, receives a List of Results, and passes that same List back to
another Gleam function without rebuilding it.

## Read The Example

1. [geam_rust_embedding_data.gleam](gleam/src/geam_rust_embedding_data.gleam)
   validates rows and totals the accepted quantities using Gleam Lists,
   Tuples, and Results.
2. [main.rs](src/main.rs) supplies Rust-native inputs, reads the returned List,
   and borrows it for the second call.
3. [runs.rs](tests/runs.rs) fixes the expected binary output.

The two calls share one loaded and sealed module. `&reviewed` refers to the
retained List owned by that module, so Geam does not materialize and reconstruct
all of its items before the `total` call.

## Run

With Geam, Rust, and Gleam installed, run from the repository root:

```sh
cd examples/embedding/data
geam embedding check
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

Continue with [a Gleam package](../package) to use `gleam_stdlib` and regenerate
the Rust bindings.
