# Drive and cancel Gleam execution

This application runs an ordinary Gleam function that never returns. Rust waits
until the function starts, cancels its call, and then calls `double` through the
same module. Everything runs on a single-thread Tokio runtime.

```gleam
pub fn spin() -> Int {
  echo "started"
  repeat()
}

fn repeat() -> Int {
  repeat()
}
```

[`src/main.rs`](src/main.rs) selects a `HostedProject` explicitly so that even
this pure Gleam source runs through the host-driven API. The native executor
can run other Rust work between Gleam instruction budgets. Dropping the Rust
Future returned by `scope.call` requests cancellation; the enclosing
`with_execution` awaits cleanup before returning.

```sh
cd examples/embedding/execution
cargo test --locked
cargo run --quiet --locked
```

Expected output:

```text
Rust made progress while Gleam was running
after cancellation: 42
```

The application uses neither `geam/future` nor a native provider. Its Echo signal
coordinates cancellation without relying on a timer.

Back to the [embedding examples](../README.md).
