# Rust Embedding Examples

These managed examples add one embedding boundary at a time. Read them in order
when learning the API, or run any example independently; each has its own Cargo
and Gleam manifests, lockfiles, generated bindings, README, and exact-output
test.

| Example | Adds |
| --- | --- |
| [`first_call`](first_call) | Plain loading, binding, sealing, and one scalar call |
| [`data`](data) | Recursive List, Tuple, Result, and retained List reuse |
| [`package`](package) | A locked package, hosted bindings, and explicit stdlib state |
| [`io`](io) | Caller-owned stdlib IO and separate Echo output |
| [`provider`](provider) | An external provider, configuration, and opaque Gleam values |
| [`application`](application) | The complete workflow with packages, IO, a provider, retained data, and repeated calls |

Start with [`first_call`](first_call). Its README shows which files `geam
embedding init` and `sync` own, which Rust code remains handwritten, and how the
generated function handle is called.

## Complete Application

[`application`](application) is the capstone managed workflow. It keeps a
resolved Gleam project inside an independently locked Rust application, commits
the generated bindings, and composes stdlib IO with a real external provider
while leaving capabilities, configuration, state, Echo, loading, sealing, and
typed calls visible in Rust.

The inventory workflow consumes a Rust `Vec` of rows and returns a retained List
of Tuple/Result values. Rust passes the same List back to calculate a total and
find the first valid row as an Option, then prints accepted items, rejection
reasons, and the summary. The internal Gleam module uses an opaque `Stock` type,
which cannot currently appear in a generated Rust function signature. The root
module converts `Stock` to a Tuple before returning data through supported
Result, List, and Option types.

Start with [main.rs](application/src/main.rs) for preparation and input/output,
then [inventory.rs](application/src/inventory.rs) for the typed calls and result
handling. Exact values, repeated calls, captured IO, and Echo are verified in
tests rather than assertions in the entry point.

With Geam, Rust, and Gleam installed, run from the repository root:

```sh
(cd examples/embedding/application && geam embedding check)
cargo test --manifest-path examples/embedding/application/Cargo.toml --locked
cargo run --quiet --manifest-path examples/embedding/application/Cargo.toml --locked
```

Check restores missing locked Gleam sources without rewriting project files.
For a new application, start with `geam embedding init`; after writing Gleam,
use `geam embedding sync` and the usual Cargo commands. See [Rust
embedding](../../docs/embedding.md) for the complete first-call workflow,
project layout, staged examples, and caller-owned runtime state.

## Manual Embedding API

[`manual.rs`](manual.rs) loads the [`manual`](manual) Gleam project without a
`main`, binds several public scalar functions from its selected root into a
shared execution, and calls their typed handles repeatedly from Rust:

```sh
cargo run --example rust_embedding --locked
```

This is an advanced manual binding reference rather than a stage in the managed
tutorial. Rust selects the project, declares exact function signatures, and
seals one shared execution. When a selected source closure requires built-in or
external providers, use the managed application so generated bindings own
provider composition while Rust keeps capabilities, configuration, mutable
state, Echo, loading, sealing, and call order explicit.
