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

Start with [`first_call`](first_call) and follow each README's next step through
[`application`](application). Every example keeps its file tour, run commands,
and expected output together. See [Rust embedding](../../docs/embedding.md) for
the initial project setup, synchronization workflow, and supported data.

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
