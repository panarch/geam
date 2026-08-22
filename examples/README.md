# Provider Authoring Examples

These examples pair ordinary Gleam packages with independently locked Rust
provider crates. Start with the smallest authoring surface and move toward the
low-level SDK only when the macro surface does not yet cover a capability.

| Example | State | Configuration | External semantics | Purpose |
| --- | --- | --- | --- | --- |
| [`text_tools`](text_tools/README.md) | None | None | None | One provider implementing three Gleam modules |
| [`value_types`](value_types/README.md) | None | None | None | Scalar and recursive tuple mapping |
| [`tag_set`](tag_set/README.md) | None | None | Generated | Stateless persistent external value |
| [`request_ids`](request_ids/README.md) | `Default` | None | None | Mutable and read-only state access |
| [`feature_flags`](feature_flags/README.md) | Configured | Required | None | Explicit configuration and shared state |
| [`run_metrics`](run_metrics/README.md) | None | None | Manual | Specialized equality, hashing, and inspection |
| [`text_pattern`](text_pattern/README.md) | Explicit low-level component | None | Manual | Capabilities outside the current macro surface |

The recommended reading order is:

```text
text_tools -> value_types -> tag_set -> request_ids -> feature_flags -> run_metrics -> text_pattern
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
