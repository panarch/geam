# Examples

## Rust Embedding

[`rust_embedding_application`](rust_embedding_application) is the canonical
managed workflow. It keeps a resolved Gleam project inside an independently
locked Rust application, commits the generated bindings, and composes stdlib IO
with a real external provider while leaving capabilities, configuration, state,
Echo, loading, sealing, and typed calls visible in Rust.

The inventory workflow normalizes and validates codes, consumes a Rust `Vec`
of rows, and returns a retained List of Tuple/Result values. Rust inspects
selected items and passes the same List back to calculate a total and find
the first valid row as an Option. A thin Gleam boundary keeps the domain's
opaque `Stock` type inside Gleam. The application asserts the exact values,
captured IO, and Echo before printing `total quantity: 7`.

```sh
cd examples/rust_embedding_application/gleam
gleam deps download
cd ../../..
cargo run --package geam --locked -- embedding check \
  --manifest-path examples/rust_embedding_application/Cargo.toml
cargo test --manifest-path examples/rust_embedding_application/Cargo.toml --locked
cargo run --quiet --manifest-path examples/rust_embedding_application/Cargo.toml --locked
```

See [Rust embedding](../docs/embedding.md) for the project layout and
sync/check lifecycle.

[`rust_embedding.rs`](rust_embedding.rs) loads a no-`main` Gleam project with
an imported module, binds several public scalar functions from its selected
root into a shared execution, and calls their typed handles repeatedly from
Rust:

```sh
cargo run --example rust_embedding --locked
```

This provider-free example is the manual embedding boundary. Rust selects the
project, declares exact function signatures, and seals one shared execution.
When a selected source closure requires built-in or external providers, use the
managed application so generated bindings own provider composition while Rust
keeps capabilities, configuration, mutable state, Echo, loading, sealing, and
call order explicit.

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
