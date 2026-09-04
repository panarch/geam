# Host Provider Examples

A host provider is the companion Rust crate that implements target-specific
functions declared by a Gleam package. These examples let you read the public
Gleam API, its Rust implementation, and the application that connects them as
one complete flow.

Read the examples in order when learning provider authoring. Each stage is an
independently runnable Gleam project and Rust provider crate.

| Stage | Example | Adds |
| --- | --- | --- |
| First provider | [`text_tools`](text_tools/README.md) | Implement scalar functions across three Gleam modules |
| Value boundary | [`value_types`](value_types/README.md) | Map scalars, tuples, Lists, custom types, Result, and Option |
| Opaque Rust value | [`tag_set`](tag_set/README.md) | Keep a persistent `TagSet` payload owned by Rust |
| Process state | [`request_ids`](request_ids/README.md) | Read and mutate fresh state for each execution |
| Configuration | [`feature_flags`](feature_flags/README.md) | Initialize shared state from explicit TOML input |
| Custom value behavior | [`run_metrics`](run_metrics/README.md) | Define equality, hashing, and inspection for an external value |
| Gleam callback | [`call_tracing`](call_tracing/README.md) | Invoke a typed Gleam function and re-enter the same provider |
| Retained Gleam value | [`generic_box`](generic_box/README.md) | Store a generic source value across provider calls |
| Published pair | [`text_pattern`](text_pattern/README.md) | Pair a Hex package with a crates.io provider while keeping its Erlang implementation |

Start with [`text_tools`](text_tools/README.md), then follow the next-example
link in each README through [`text_pattern`](text_pattern/README.md). The
[provider guide](../../docs/host-providers.md) explains why the Gleam package
and Rust crate are separate before walking through the same first connection.

## Run the first provider

With Gleam, Rust, and Geam installed, run from the repository root:

```sh
cd examples/provider/text_tools/project
geam provider add --path ../provider
geam prepare
geam run
```

The example imports three modules from the Gleam package and checks calls such
as `upper("Geam") == "GEAM"` and
`join("geam", "-", "provider") == "geam-provider"`. A successful run is
silent because its assertions pass.

## Find the feature you need

After the first provider works, use this table when you need one particular
host boundary:

| Example | Provider state | Runtime configuration | Rust-owned value |
| --- | --- | --- | --- |
| [`text_tools`](text_tools/README.md) | None | None | None |
| [`value_types`](value_types/README.md) | None | None | None |
| [`tag_set`](tag_set/README.md) | None | None | Generated equality, hashing, and inspection |
| [`request_ids`](request_ids/README.md) | `Default` | None | None |
| [`feature_flags`](feature_flags/README.md) | Configured | Required | None |
| [`run_metrics`](run_metrics/README.md) | None | None | Custom equality, hashing, and inspection |
| [`call_tracing`](call_tracing/README.md) | `Default` | None | None |
| [`generic_box`](generic_box/README.md) | None | None | Retained generic Gleam value |
| [`text_pattern`](text_pattern/README.md) | None | None | Regex payload with custom behavior |

## Example layout

Every example keeps the two packages side by side:

```text
project/   Gleam application and local Gleam package
provider/  separately buildable and testable Rust provider crate
```

The application adds and imports the Gleam package. `geam provider add` selects
the companion Rust crate for the checkout. The provider's Cargo metadata names
the Gleam package and compatible versions so Geam can verify the pair before
it runs.

These checkouts select local paths explicitly. A published application instead
selects the crate and version documented by its Gleam package or provider.
Provider metadata, rather than the Cargo package name, identifies the target
Gleam package and compatible versions.

`feature_flags` is the only staged example that needs runtime configuration:

```sh
geam run --provider-config example_feature_flags=config/feature_flags.toml
```

Generated project `Cargo.toml`, `Cargo.lock`, and `build/` files are ignored in
these checkouts. Each provider's own `Cargo.lock` is tracked so the Rust crate
can be formatted, tested, linted, and packaged with `--locked` independently.
