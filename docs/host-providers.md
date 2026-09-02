# Author a host provider

Some Gleam packages declare functions or values that have a separate
implementation for each target. A Geam host provider supplies the Rust
implementation.

The Gleam package keeps its public API, and users keep importing the same
modules and calling the same functions. The provider is a separate Rust crate
that supplies the Rust implementation for Geam.

Use a provider when a Gleam package needs a native Rust library, access to the
host process, or a value or state that must remain in Rust.

The Gleam package and Rust provider remain separate packages:

```text
Gleam package on Hex
  declares the public Gleam API and externals

Rust provider on crates.io, Git, or a local path
  implements those externals for Geam
```

The same provider works in standalone and Rust embedding projects.

## Start with a small provider

Imagine a Gleam package with one casing function implemented outside Gleam:

```gleam
// src/example_text_tools/casing.gleam
@external(erlang, "geam_example_text_tools_casing", "upper")
pub fn upper(value: String) -> String
```

The `erlang` target in this declaration is intentional. Standard Geam workflows
analyse the Erlang-compatible source path and treat bodyless Erlang externals as
Rust provider requirements. Geam does not call the named Erlang module or
function; the provider links the Gleam declaration to Rust by its package,
module, function, and type signature. A bodyless JavaScript-only external is not
discovered as a provider requirement in this workflow.

Create an ordinary Rust library crate for the provider:

```sh
cargo new --lib geam-example-text-tools
cd geam-example-text-tools
cargo add geam --no-default-features --features provider
```

Geam re-exports its author-facing value types from `geam::provider`, so one Geam
dependency supplies the types used in provider declarations.

Declare which Gleam package and modules the crate implements:

```rust
use geam::provider::EcoString;

#[geam::provider(
    package = "example_text_tools",
    modules = [casing],
)]
pub struct Component;

#[geam::module(path = "example_text_tools/casing")]
mod casing {
    use super::EcoString;

    #[geam::function]
    fn upper(value: EcoString) -> EcoString {
        value.to_uppercase()
    }
}
```

The provider macro generates the Geam wiring for the Rust crate. Rust
compilation checks its declarations and supported Rust types. `geam prepare`
then compares the generated description with the typed Gleam package before any
provider state is initialized or application code runs.

## Tell Geam what the provider supports

Provider metadata names the exact Gleam package and the range of that package's
versions supported by the Rust implementation:

```toml
[package]
name = "geam-example-text-tools"
version = "0.1.0"

[package.metadata.geam.provider]
schema = 1
gleam-package = "example_text_tools"
gleam-version = ">= 1.0.0 and < 2.0.0"
```

The `gleam-version` field describes the target Hex package, not the Gleam
compiler. Geam verifies metadata before recording a path, Git, or registry
selection, but compatibility metadata is never treated as user approval.

For crates.io discovery, derive the Rust package name from the Gleam package by
adding `geam-` and replacing underscores with hyphens:

```text
example_text_tools -> geam-example-text-tools
```

Explicitly selected providers can use another crate name, but metadata remains
the authority for the target Gleam package.

## Try it from a Gleam project

Keep a complete Gleam application beside the provider while authoring it:

```text
example/
  project/   Gleam application and package source
  provider/  Rust provider crate
```

Select the local crate and run the same path users rely on:

```sh
cd project
geam provider add --path ../provider
geam prepare
geam run
```

This verifies the provider metadata, Gleam declarations, generated static
runner, Rust implementation, and application behavior together. Unit tests in
the provider crate remain useful for Rust-only logic, but they do not replace
the complete source-linkage check.

The repository's
[text tools example](https://github.com/panarch/geam/tree/main/examples/text_tools)
is the smallest complete provider. Read its Gleam declarations, provider
`src/lib.rs`, and application entry point together.

## Add only what the package needs

Start with scalar functions and add only the capabilities the Gleam package
actually needs:

- Use native Rust tuples, `Result`, `Option`, and Geam's List boundary for
  ordinary source values.
- Add generated custom-value mappings when Rust constructs or receives a
  source custom type.
- Add an external payload when the source value must remain opaque to Gleam.
- Add component state for process-local mutable or read-only capabilities.
- Add explicit configuration when state construction needs caller input.
- Add typed callbacks only when provider code must call a supplied Gleam
  function.
- Use retained generic or advanced storage only when an external value must own
  source values across calls.

The [provider examples](https://github.com/panarch/geam/tree/main/examples)
form an ordered learning path:

```text
text_tools
-> value_types
-> tag_set
-> request_ids
-> feature_flags
-> run_metrics
-> call_tracing
-> generic_box
-> text_pattern
```

Each example introduces one additional ownership or type boundary instead of
combining every provider capability at once.

## Keep native code explicit

Provider crates are native code. Geam verifies their package metadata and
typed linkage, but neither step is a security endorsement. A standalone user
must approve a discovered provider before Cargo receives it. An embedding
application records the provider as an ordinary Cargo dependency and reviews
the generated static composition.

Provider configuration is caller-owned TOML data supplied during standalone
execution or constructed explicitly by an embedding application. Provider
state and external values are not stored in global registries, generated source,
or package metadata.

## Release each package on its own schedule

Publish the Gleam package with Gleam's Hex tooling and the Rust provider with
Cargo. Their versions do not have to match, but the provider metadata must
declare the actual compatible Gleam package range. Test the packaged provider
with `cargo publish --locked --dry-run` and verify the public Gleam-to-provider
path before widening that range.

The published
[text-pattern package and provider](https://github.com/panarch/geam/tree/main/examples/text_pattern)
show independent Hex and crates.io packages that can also run on Erlang through
the package's separate Erlang implementation.

## Exact reference

Continue with the [host provider boundary](reference/provider-boundary.md) for
the complete Rust type mappings, custom and external values, state,
configuration, callback invocation, retained storage, generated component
contract, and runner profile. The [runtime semantics](reference/runtime-semantics.md)
document defines ownership, equality, hashing, inspection, and failure behavior
after linkage.
