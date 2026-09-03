# Host Provider: Text Pattern

This final example turns the side-by-side authoring layout into two separately
published packages. Applications add `example_text_pattern` from Hex and keep
calling its Gleam API. Geam hosts select `geam-example-text-pattern` from
crates.io to implement that API with Rust's `regex` crate.

The Hex package also ships an Erlang `re` implementation, so the same public
Gleam modules can run on Erlang without Geam. This repository releases the
example package and provider at the same version, although provider metadata is
what declares their compatibility.

## Read The Pair

1. [`project/packages/example_text_pattern/src/example_text_pattern.gleam`](project/packages/example_text_pattern/src/example_text_pattern.gleam)
   declares the constructorless `Pattern`, `CompileError`, and public API.
2. [`project/packages/example_text_pattern/src/example_text_pattern_ffi.erl`](project/packages/example_text_pattern/src/example_text_pattern_ffi.erl)
   implements that API for Erlang.
3. [`provider/src/lib.rs`](provider/src/lib.rs) implements the same Gleam shapes
   for Geam with `#[geam::external]`, `#[geam::custom]`, and
   `#[geam::function]`.
4. [`project/src/text_pattern_example.gleam`](project/src/text_pattern_example.gleam)
   runs the shared behavior against either implementation.

The [Gleam package README](project/packages/example_text_pattern/README.md)
documents its public API. The [provider crate README](provider/README.md)
explains the Rust implementation and why this advanced external value uses
custom behavior.

```text
project/
  packages/example_text_pattern/  ordinary local Gleam package
provider/                          geam-example-text-pattern crate
```

## Run From a Checkout

### Erlang

With Gleam and Erlang/OTP installed, run from the repository root without Geam
or Rust:

```sh
cd examples/provider/text_pattern/project
gleam run --target erlang
```

### Geam

With Gleam, Rust, and Geam installed, return to the repository root, select the
local provider, and run the same project:

```sh
cd examples/provider/text_pattern/project
geam provider add --path ../provider
geam prepare
geam run
```

No provider configuration is required. Both commands execute
[`text_pattern_example.gleam`](project/src/text_pattern_example.gleam), checking
compilation, matching, literal replacement, Unicode, empty results, and invalid
patterns. A successful run is silent because all assertions pass.

This checkout uses the Gleam package as a local path dependency. Provider
selection comes from the Rust crate's Cargo metadata; the Gleam package needs no
Geam-specific metadata. The provider is not a Geam built-in.

This checkout uses an explicit path selection so provider changes can be tested
before publication. See the [standalone guide](../../../docs/standalone.md) for
registry selection and managed project files.

## Runtime-Specific Examples

From `examples/provider/text_pattern/project`, run the additional entrypoint for each
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

The [provider authoring guide](../../../docs/host-providers.md) covers the API
from the first function through packaging. The [examples index](../README.md)
offers smaller examples for each provider feature combined here.
