# Rust Embedding Examples

These examples build a Rust-hosted Gleam application one feature at a time.
Read them in order when learning the API, or run any example independently;
each is a complete Cargo application with its Gleam source, Rust code, and
tests.

| Example | Adds |
| --- | --- |
| [`first_call`](first_call) | Call one scalar Gleam function from Rust |
| [`data`](data) | Pass nested Lists, Tuples, and Results, then reuse a returned List |
| [`package`](package) | Call a function from `gleam_stdlib` |
| [`io`](io) | Route Gleam IO through Rust and capture Echo separately |
| [`provider`](provider) | Call Gleam code backed by a configured Rust provider |
| [`application`](application) | Combine packages, IO, a provider, structured data, and repeated calls |

Start with [`first_call`](first_call). Its README follows the files created by
`geam embedding init` through the first generated function call.

## Complete Application

[`application`](application) puts every stage into one inventory workflow. Rust
sends rows to Gleam, Gleam normalizes and validates them with a Rust provider,
and Rust prints the accepted items, rejection reasons, and summary.

The returned List is passed back to Gleam to calculate a total and find the
first valid row, so the example also shows repeated calls with the same loaded
module. The internal Gleam module uses an opaque `Stock` type and converts it to
a Tuple before returning it through the generated Rust API.

Start with [main.rs](application/src/main.rs) for preparation and input/output,
then [inventory.rs](application/src/inventory.rs) for the typed calls and result
handling. Tests verify exact values, repeated calls, captured IO, and Echo.

With Geam, Rust, and Gleam installed, run from the repository root:

```sh
(cd examples/embedding/application && geam embedding check)
cargo test --manifest-path examples/embedding/application/Cargo.toml --locked
cargo run --quiet --manifest-path examples/embedding/application/Cargo.toml --locked
```

For a new application, start with `geam embedding init`; after writing Gleam,
use `geam embedding sync` and the usual Cargo commands. See [Rust
embedding](../../docs/embedding.md) for the complete first-call workflow,
project layout, staged examples, and runtime inputs.

## Manual Embedding API

[`manual.rs`](manual.rs) loads the [`manual`](manual) Gleam project without a
`main`, binds several public scalar functions from its selected root into a
shared execution, and calls their typed handles repeatedly from Rust:

```sh
cargo run --example rust_embedding --locked
```

This advanced example shows how to select a project and declare callable
function signatures by hand. Use it when the Rust application intentionally
owns those choices. For the normal application workflow, start with
`geam embedding init` and let generated bindings connect packages and
providers.
