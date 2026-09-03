# Rust Embedding: Caller-Owned IO

This example calls `gleam/io` from embedded Gleam while the Rust application
owns the output destination. The Gleam function both prints a message and
returns it, making the effect and the ordinary return value visible separately.

## Read The Example

1. [geam_rust_embedding_io.gleam](gleam/src/geam_rust_embedding_io.gleam)
   calls `io.println` and returns the same String.
2. [main.rs](src/main.rs) supplies deterministic stdlib state, calls the
   function, and routes collected stdout and stderr events.
3. [runs.rs](tests/runs.rs) verifies both the emitted IO and returned value.

`GleamStdlibRunState` keeps randomness and IO caller-owned. This example uses a
fixed seed because it does not need system entropy. Standard-library IO events
are collected in that state; Gleam's language-level `echo` output remains a
separate call argument.

## Run

With Geam, Rust, and Gleam installed, run from the repository root:

```sh
cd examples/embedding/io
geam embedding check
cargo test --locked
cargo run --quiet --locked
```

The application prints:

```text
Hello, Rust!
returned: Hello, Rust!
```

Continue with [an external provider](../provider) to connect a
Gleam package to a separately packaged Rust implementation.
