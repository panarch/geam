# Examples

## Rust Embedding

[`rust_embedding.rs`](rust_embedding.rs) compiles one no-`main` Gleam module,
binds several public scalar functions into a shared execution, and calls their
typed handles repeatedly from Rust:

```sh
cargo run --example rust_embedding --locked
```

This is the manual embedding boundary. It selects source and declares exact
Rust signatures directly; project loading and generated bindings are separate
follow-up work.

## Provider Authoring

These examples pair ordinary Gleam packages with independently locked Rust
provider crates. Start with the smallest authoring surface, then combine the
same static macros into stateful, retained, callback, and advanced providers.

| Example | State | Configuration | External semantics | Purpose |
| --- | --- | --- | --- | --- |
| [`text_tools`](text_tools/README.md) | None | None | None | One provider implementing three Gleam modules |
| [`value_types`](value_types/README.md) | None | None | None | Scalar, tuple, lazy List, custom, Result, and Option mapping |
| [`tag_set`](tag_set/README.md) | None | None | Generated | Stateless persistent external value |
| [`request_ids`](request_ids/README.md) | `Default` | None | None | Mutable and read-only state access |
| [`feature_flags`](feature_flags/README.md) | Configured | Required | None | Explicit configuration and shared state |
| [`run_metrics`](run_metrics/README.md) | None | None | Manual | Specialized equality, hashing, and inspection |
| [`call_tracing`](call_tracing/README.md) | `Default` | None | None | Typed callback invocation and same-component re-entry |
| [`generic_box`](generic_box/README.md) | None | None | Retained generic | Persistent generic source values and callback mapping |
| [`text_pattern`](text_pattern/README.md) | None | None | Manual | Regex-backed external, custom error, Result, and List output |

The recommended reading order is:

```text
text_tools -> value_types -> tag_set -> request_ids -> feature_flags -> run_metrics -> call_tracing -> generic_box -> text_pattern
```

Every example keeps the Gleam package and Rust provider separate:

```text
project/   ordinary Gleam application and local Gleam dependency
provider/  independently locked Rust provider crate
```

Run a macro-authored example with the same standalone workflow:

```sh
cd examples/text_tools/project
geam provider add --path ../provider
geam prepare
geam run
```

`feature_flags` additionally passes its checked-in configuration:

```sh
geam run --provider-config example_feature_flags=config/feature_flags.toml
```

The generated project `Cargo.toml`, `Cargo.lock`, and `build/` tree are ignored.
Each provider's own `Cargo.lock` is tracked so its Rust package can be formatted,
tested, linted, and packaged with `--locked` independently.
