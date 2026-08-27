# Text Pattern Provider Example

This example pairs an ordinary Gleam package with a separately packageable Rust
provider for [Geam](https://github.com/panarch/geam). The Gleam package declares
only its public types and Erlang externals. The Rust crate implements those
externals with `regex` through Geam's public provider authoring macros.

See the [provider README](provider/README.md) for the crate overview, Gleam API,
and a short usage example.

```text
project/
  packages/example_text_pattern/  ordinary local Gleam package
provider/                          geam-example-text-pattern crate
```

## Run From a Checkout

With Rust and Cargo installed, clone Geam and install the CLI from the same
checkout:

```sh
git clone https://github.com/panarch/geam.git
cd geam
cargo install --path . --locked
```

From the repository root, select the local provider and run the Gleam project:

```sh
cd examples/text_pattern/project
geam provider add --path ../provider
geam prepare
geam run
```

No provider configuration is required. A successful run checks regex compilation,
matching, replacement, invalid-pattern errors, and pattern equality, and echoes
`Pattern("[A-Za-z]+")` to stderr. Running `geam run` again repeats the same checks.

The Gleam package is a checked-in path dependency, not a Hex package. Provider
selection comes from the Rust crate's Cargo metadata; the Gleam package needs no
Geam-specific metadata. The provider is not a Geam built-in.

This checkout workflow uses an explicit path selection so changes to the
provider can be tested without publishing it. The local
[Cargo patch](.cargo/config.toml) selects the same Geam checkout for provider
and runner builds. See the [standalone guide](../../docs/standalone.md) for
provider selection and managed project files.

## Read the Implementation

Read the matching declarations together:

- [`project/packages/example_text_pattern/src/example_text_pattern.gleam`](project/packages/example_text_pattern/src/example_text_pattern.gleam)
  declares the constructorless `Pattern`, `CompileError`, and public functions;
- [`provider/src/lib.rs`](provider/src/lib.rs) implements the same shapes with
  `#[geam::external]`, `#[geam::custom]`, and `#[geam::function]`; and
- [`provider/README.md`](provider/README.md) explains why this advanced example
  uses manual external semantics while ordinary registration remains generated.

The [provider authoring guide](../../docs/host-providers.md) covers the API
contracts. The [examples index](../README.md) offers smaller examples of the
individual authoring patterns used here.
