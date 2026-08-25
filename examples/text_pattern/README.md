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

The provider is deliberately not a Geam built-in. Its Rust source uses the
same public authoring macros available to an independently published provider.
Select the local crate explicitly while developing and reviewing it:

```sh
cd examples/text_pattern/project
geam provider add --path ../provider
geam prepare
geam run
```

The local package needs no Geam metadata and is not published to Hex. Its
provider mapping comes from the Rust crate's Cargo metadata. The provider crate
is formatted, tested, linted, packaged, and executed in CI, but is intentionally
not published yet.

Read the matching declarations together:

- [`project/packages/example_text_pattern/src/example_text_pattern.gleam`](project/packages/example_text_pattern/src/example_text_pattern.gleam)
  declares the constructorless `Pattern`, `CompileError`, and public functions;
- [`provider/src/lib.rs`](provider/src/lib.rs) implements the same shapes with
  `#[geam::external]`, `#[geam::custom]`, and `#[geam::function]`; and
- [`provider/README.md`](provider/README.md) explains why this advanced example
  uses manual external semantics while ordinary registration remains generated.
