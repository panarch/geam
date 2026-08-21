# Text Pattern Provider Example

This example pairs an ordinary Gleam package with a separately packageable Rust
provider. The Gleam package declares only its public types and Erlang
externals. The Rust crate implements those externals with `regex` through
Geam's typed provider component API.

```text
project/
  packages/example_text_pattern/  ordinary local Gleam package
provider/                          geam-example-text-pattern crate
```

The provider is deliberately not a Geam built-in. Select the local crate
explicitly while developing and reviewing the provider:

```sh
cd examples/text_pattern/project
geam provider add --path ../provider
geam prepare
geam run
```

The local package needs no Geam metadata and is not published to Hex. Its
provider mapping comes from the Rust crate's Cargo metadata. The provider crate
is formatted, linted, packaged, and executed in CI, but is intentionally not
published yet.

[`provider/README.md`](provider/README.md) records the intended higher-level
authoring API. The current low-level implementation remains the executable
baseline until the Geam workspace and proc-macro authoring layer are complete.
After that migration, this same example can be published and used to complete
the crates.io discovery flow without `geam provider add`.
