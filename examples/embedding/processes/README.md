# Gleam Process Service

A Gleam process keeps a counter alive between Rust calls. Rust retains its
`Pid` and typed `Subject`, sends requests through Gleam functions, and waits for
the process to stop. The application's Tokio runtime drives the execution.

```sh
cd examples/embedding/processes
geam embedding check
cargo test --locked
cargo run --quiet --locked
```

```text
first: 20
second: 42
stopped: true
```

The [Gleam service](gleam/src/geam_rust_embedding_processes.gleam) owns its
mailbox and counter state. [Rust](src/main.rs) starts it once, retains an alias
of its Subject, and makes several calls inside one execution scope. Generated
opaque handles keep their Gleam types and cannot leave that scope. Retaining a
handle does not keep a terminated process alive.

The same Gleam source has a standalone entry:

```sh
cd gleam
geam run
```

It prints `20` and `42`. Geam owns the executor in this mode.
