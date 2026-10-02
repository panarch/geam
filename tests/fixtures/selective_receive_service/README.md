# Caller-defined selective receive

This fixture is an ordinary external Rust provider using
`geam::gleam_erlang::service::ProcessCall::receive_with`. Enable Geam's
`provider,gleam-erlang` features. Its schema-2 metadata declares a plain
component and `requires-services = ["gleam_erlang"]`; the host composes the
original stdlib and Erlang components with this provider. It creates no mailbox
or execution service of its own.

The project pins original `gleam_erlang` 1.3.0 and `gleam_stdlib` 1.0.3.
References originate from `gleam/erlang/reference.new`; delivery targets the
original `gleam/erlang/process.self` Pid. The provider retains an
input through a generated retained `Key` owner and captures its native view in a
`Send` matcher with no `Clone` implementation and a non-`Sync` `Cell`.
`NativeValues::equal` distinguishes two opaque References with identical message
tags. The matcher returns owned fields; restoration uses the original
producer's Reference adapter, without a replacement schema or hidden host API.
`Key` also exercises retained source equality, hashing and inspection.

Source assertions fix `[tcp(B), user, tcp(A)]` after selecting A, combined
`tcp/closed/passive/error` FIFO, malformed candidates, mismatched identities,
zero-deadline matches/timeouts, and deadline-free observation. The `next`
operation observes exact remaining positions through the ordinary receive API.
The provider's fixture integration test compares repeated execution and retained
inspection after execution/state drop. Built-in owner tests separately cover
bounded scans, arrival cursors, receiver errors, cancellation and matcher/output
release. The fixture's coverage denominator is its provider package.

Matchers synchronously borrow a candidate and its native comparison context.
`None` leaves the message queued; `Some(output)` consumes the first match.
Output must be owned and `Send + 'static`. Neither captures nor output require
`Clone` or `Sync`. Keep matching bounded and read-only; the service supplies no
host effects or source callback invocation. Dropping a receive or ending the
scope releases its state through the original cleanup. Retained output/aliases
keep their own lifetimes; cancellation does not roll back consumption.

This is a mailbox contract, not a Glisten, socket or Erlang-node implementation.
An I/O provider owns delivery quiescing and any transfer loop.

From the repository root:

```sh
cargo fetch --manifest-path tests/fixtures/selective_receive_service/provider/Cargo.toml --locked
cargo test --manifest-path tests/fixtures/selective_receive_service/provider/Cargo.toml --locked
cargo fmt --manifest-path tests/fixtures/selective_receive_service/provider/Cargo.toml --all --check
cargo clippy --manifest-path tests/fixtures/selective_receive_service/provider/Cargo.toml --all-targets --locked -- -D warnings
gleam format --check tests/fixtures/selective_receive_service/project/src
cargo build --package geam --bin geam --locked
cargo fetch --manifest-path tests/fixtures/selective_receive_service/embedding/Cargo.toml --locked
(cd tests/fixtures/selective_receive_service/embedding && ../../../../target/debug/geam embedding sync)
cargo run --manifest-path tests/fixtures/selective_receive_service/embedding/Cargo.toml --locked
cargo test --package geam --test prepared_embedding --locked -- selective_receive
cargo test --package geam --test standalone_build --locked -- --test-threads=1
```

Sync regenerates the ignored `embedding/src/geam_bindings/program.rs` before
formatting or compiling the embedding. The typed application calls the same
source twice in each dynamic/prepared execution and checks exact output. The
prepared test regenerates, actually compiles, relocates and repeatedly executes
the binary with no source tree and an empty PATH. The complete standalone test
includes the same project/provider in debug and release applications and checks
source-free execution. Cargo and Gleam locks belong to their separate workspaces;
downloaded sources, build products and generated program data stay out of Git.

Fresh independent coverage:

```sh
cargo llvm-cov clean --manifest-path tests/fixtures/selective_receive_service/provider/Cargo.toml --workspace
cargo llvm-cov --manifest-path tests/fixtures/selective_receive_service/provider/Cargo.toml --no-report --locked
cargo llvm-cov report --manifest-path tests/fixtures/selective_receive_service/provider/Cargo.toml --package geam-selective-receive-service-fixture --summary-only --fail-under-lines 100 --fail-under-regions 100
```
