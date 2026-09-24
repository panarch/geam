# Application argument observation fixture

This private provider captures native process arguments once during state
initialization. Its Gleam package exposes the captured values as strings and as
Unix bytes or Windows UTF-16 code units. String conversion is a lossy test
projection; the native units are the lossless forwarding oracle. This is not an
implementation of the `argv` package.

`tests/standalone_build.rs` selects this provider only in its argument case and
checks the real CLI, generated runner, initialization, and relocated executable.
`tests/public_usage.rs` uses the public typed host boundary with caller-owned
state and verifies that returned snapshots survive later calls and state drop.

From the parent `providers` workspace, run:

```sh
cargo fetch --locked --config net.offline=false
cargo test --package geam-arguments-fixture --locked
cargo llvm-cov clean --workspace
cargo llvm-cov --package geam-arguments-fixture --no-report --locked
cargo llvm-cov report --package geam-arguments-fixture --summary-only --fail-under-lines 100 --fail-under-regions 100
```

Coverage measures this provider package, independently of its Geam dependencies.
Run commands from that directory so its local Cargo patch selects the checkout.
