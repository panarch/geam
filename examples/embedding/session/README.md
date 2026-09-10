# Retain a Gleam session

Gleam owns the `Session` type and its private fields, including a function.
Rust keeps the returned handle and passes it to later calls in the same execution
scope. Cloning the handle shares the original value; `next` returns a new session.

```gleam
pub opaque type Session {
  Session(total: Int, advance: fn(Int) -> Int)
}
```

The corresponding Rust calls are:

```rust
let original = scope.call(&functions.start, (40.into(),)).await?;
let next = scope.call(&functions.next, (&original,)).await?;
let total = scope.call(&functions.total, (next,)).await?;
```

Run from the repository root with Geam installed:

```sh
cd examples/embedding/session
geam embedding check
cargo test --locked
cargo run --quiet --locked
```

Expected output:

```text
original: 40
next: 42
```

[`src/main.rs`](src/main.rs) contains the host setup and generated bindings.
[`gleam/src/geam_rust_embedding_session.gleam`](gleam/src/geam_rust_embedding_session.gleam)
contains the complete source. The package uses neither `geam/future` nor a native
provider.

Back to the [embedding examples](../README.md).
