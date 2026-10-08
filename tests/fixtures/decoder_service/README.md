# Producer-owned Decoder transfer

This independently locked provider consumes
`geam::gleam_stdlib::service::Decoder<Item>` using Geam's
`provider,gleam-stdlib,gleam-erlang` features. It copies no private Decoder
schema, callback, or producer store. The project pins original `gleam_stdlib`
1.0.3 and `gleam_erlang` 1.3.0; downloaded sources and build products stay out
of Git.

The provider includes a manual typed pass-through and macro-authored generic
pass-through, lazy List selection, tuples, and `Message(Item)` fields. An explicit
`#[geam::restore] Restore<Decoder<Item>>` declaration checks a Dynamic round trip
without exposing the Decoder representation. It also
passes an original `Subject(Message(Item))` through its producer SDK. The source
constructs Int/String decoders with captured values, sends Configure/Dispatch
messages through the shared Erlang process service, and checks exact original
`decode.run` success and DecodeError results. Decoding remains a Gleam operation.
The SDK grants neither Decoder construction nor private function invocation.

The public integration target composes the real producer components and repeats
the source assertions in two fresh executions on the same loaded owner. Core and stdlib owner tests separately
cover nominal/representation grants, hidden fields, exact specialization,
original-execution restoration, and retained aliases. This fixture exercises
those public boundaries together; it is not a complete Lustre lifecycle.

From the repository root:

```sh
cargo fetch --manifest-path tests/fixtures/decoder_service/provider/Cargo.toml --locked
cargo test --manifest-path tests/fixtures/decoder_service/provider/Cargo.toml --locked
cargo fmt --manifest-path tests/fixtures/decoder_service/provider/Cargo.toml --all --check
cargo clippy --manifest-path tests/fixtures/decoder_service/provider/Cargo.toml --all-targets --locked -- -D warnings
gleam format --check tests/fixtures/decoder_service/project/src
gleam format --check tests/fixtures/decoder_service/embedding/gleam/src
cargo build --package geam --bin geam --locked
cargo fetch --manifest-path tests/fixtures/decoder_service/embedding/Cargo.toml --locked
(cd tests/fixtures/decoder_service/embedding && ../../../../target/debug/geam embedding sync)
cargo run --manifest-path tests/fixtures/decoder_service/embedding/Cargo.toml --locked
cargo test --package geam --test prepared_embedding --locked -- decoder_sdk
```

Sync regenerates the ignored `embedding/src/geam_bindings/program.rs` before
formatting or compiling the embedding. The prepared acceptance target copies
this consumer, checks repeatable generation, compiles and runs live/prepared
bindings, then builds the same project through the CLI. Relocated applications
run with no source tree and an empty PATH, with exact `decoder SDK: 42` output.
Tracked bindings and Cargo/Gleam locks belong to their independent workspaces.

Fresh independent coverage uses only the provider package as its denominator:

```sh
cargo llvm-cov clean --manifest-path tests/fixtures/decoder_service/provider/Cargo.toml --workspace
cargo llvm-cov --manifest-path tests/fixtures/decoder_service/provider/Cargo.toml --no-report --locked
cargo llvm-cov report --manifest-path tests/fixtures/decoder_service/provider/Cargo.toml --package geam-decoder-service-fixture --summary-only --fail-under-lines 100 --fail-under-regions 100
```

Workspace owns provider formatting/Clippy, Acceptance owns original-source and
compiled prepared/standalone consumption, and Coverage owns this independent
consumer gate. Production core and built-in coverage retain their separate
package owner closures.
