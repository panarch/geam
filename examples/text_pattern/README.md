# Text Pattern Provider Example

This example pairs an ordinary Gleam package with a separately packageable Rust
provider for [Geam](https://github.com/panarch/geam). The Gleam package includes
an Erlang `re` implementation. The Rust crate implements the same API with
`regex` through Geam's public provider authoring macros.

See the [Gleam package README](project/packages/example_text_pattern/README.md)
for installation and API usage, and the [provider README](provider/README.md)
for the matching Rust implementation and authoring macros.

```text
project/
  packages/example_text_pattern/  ordinary local Gleam package
provider/                          geam-example-text-pattern crate
```

## Run From a Checkout

### Erlang

With Gleam and Erlang/OTP installed, the example runs without Geam or Rust:

```sh
git clone https://github.com/panarch/geam.git
cd geam/examples/text_pattern/project
gleam run --target erlang
```

### Geam

With Gleam, Rust, and Cargo installed, start from the repository root to install
Geam, select the local provider, and run the same project:

```sh
cargo install --path . --locked
cd examples/text_pattern/project
geam provider add --path ../provider
geam prepare
geam run
```

No provider configuration is required. Both runtimes execute
[`text_pattern_example.gleam`](project/src/text_pattern_example.gleam), checking
compilation, matching, literal replacement, Unicode, empty results, and invalid
patterns. A successful run prints no application output. Running the command
again repeats the same checks.

This checkout uses the Gleam package as a local path dependency. Provider
selection comes from the Rust crate's Cargo metadata; the Gleam package needs no
Geam-specific metadata. The provider is not a Geam built-in.

This checkout workflow uses an explicit path selection so changes to the
provider can be tested without publishing it. The local
[Cargo patch](.cargo/config.toml) selects the same Geam checkout for provider
and runner builds. See the [standalone guide](../../docs/standalone.md) for
provider selection and managed project files.

## Runtime-Specific Examples

From `examples/text_pattern/project`, run the additional entrypoint for each
runtime:

```sh
gleam run --target erlang --module text_pattern_erlang
geam run --module text_pattern_geam
```

- [`text_pattern_erlang.gleam`](project/src/text_pattern_erlang.gleam) uses OTP's
  `&` and `\\1` capture replacements and a lookahead pattern.
- [`text_pattern_geam.gleam`](project/src/text_pattern_geam.gleam) uses Rust's
  `$0` and `$1` replacements, rejects lookahead, and preserves Geam's
  source-text pattern equality. It echoes `Pattern("[A-Za-z]+")` to stderr.

The package keeps the engines' native syntax instead of translating between
them. See its [runtime semantics](project/packages/example_text_pattern/README.md#runtime-semantics)
for the boundary shared by the public API.

## Read the Implementation

Read the matching declarations together:

- [`project/packages/example_text_pattern/src/example_text_pattern.gleam`](project/packages/example_text_pattern/src/example_text_pattern.gleam)
  declares the constructorless `Pattern`, `CompileError`, and public functions;
- [`project/packages/example_text_pattern/src/example_text_pattern_ffi.erl`](project/packages/example_text_pattern/src/example_text_pattern_ffi.erl)
  supplies the native Erlang implementation shipped in the Hex package;
- [`provider/src/lib.rs`](provider/src/lib.rs) implements the same shapes with
  `#[geam::external]`, `#[geam::custom]`, and `#[geam::function]`; and
- [`provider/README.md`](provider/README.md) explains why this advanced example
  uses manual external semantics while ordinary registration remains generated.

The [provider authoring guide](../../docs/host-providers.md) covers the API
contracts. The [examples index](../README.md) offers smaller examples of the
individual authoring patterns used here.
