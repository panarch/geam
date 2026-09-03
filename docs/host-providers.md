# Add Rust to a Gleam package

Most Gleam packages need no provider. Geam can run ordinary Gleam source and
its built-in package integrations directly.

A **host provider** is a companion Rust crate that implements native functions
from a Gleam package for Geam. Use one when part of the package API must run
inside the Rust host.

## One API, two packages

The Gleam package and provider have separate jobs:

| Artifact | Contains | Used for |
| --- | --- | --- |
| Gleam package, usually on Hex | Public Gleam modules, types, and external declarations | What applications add, import, and call |
| Rust provider on crates.io, Git, or a local path | Native implementations for those declarations | What Cargo compiles into the Geam host |

The provider does not replace or translate the Gleam package. Users keep
writing the same Gleam calls:

```text
Gleam application
  adds and imports example_text_tools as a Gleam dependency
  calls example_text_tools/casing.upper("Geam")
                              |
                              v
Geam links the declaration to geam-example-text-tools from Cargo
```

A package can keep separate Erlang or JavaScript implementations for the same
API. The provider adds the implementation used when Geam runs the package. The
same provider can be used by a standalone Gleam project or a Rust embedding
application.

## Follow one function from Gleam to Rust

The Gleam package declares a function without a Gleam body:

```gleam
// src/example_text_tools/casing.gleam
@external(erlang, "geam_example_text_tools_casing", "upper")
pub fn upper(value: String) -> String
```

The `erlang` target is intentional. Standard Geam workflows analyse the
Erlang-compatible source path and treat a bodyless Erlang external as a Rust
provider requirement. Geam does not call the named Erlang module or function;
it links the declaration to Rust by package, module, function, and exact type
signature. A bodyless JavaScript-only external is not available in this
workflow.

Create an ordinary Rust library crate beside the Gleam package:

```sh
cargo new --lib geam-example-text-tools
cd geam-example-text-tools
cargo add geam --no-default-features --features provider
```

Then declare the package and implement the matching module and function:

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

The macros generate provider registration and typed wiring. Rust checks the
declarations and supported Rust types at compile time. During preparation,
Geam compares that generated description with the typed Gleam declaration
before provider state is initialized or application code runs.

Geam re-exports its author-facing value types from `geam::provider`, so this
single dependency supplies types such as `EcoString`, `BigInt`, and `List`.

## Declare which Gleam versions it supports

Cargo metadata connects the crate to its Gleam package and states the package
versions implemented by this Rust code:

```toml
[package]
name = "geam-example-text-tools"
version = "0.1.0"

[package.metadata.geam.provider]
schema = 1
gleam-package = "example_text_tools"
gleam-version = ">= 1.0.0 and < 2.0.0"
```

`gleam-version` refers to the target Hex package, not the Gleam compiler. The
provider and Gleam package versions do not need to match.

For automatic crates.io discovery, derive the crate name by adding `geam-` to
the Gleam package name and replacing underscores with hyphens:

```text
example_text_tools -> geam-example-text-tools
```

An explicitly selected provider may use another crate name. Its packaged
metadata still has to identify the exact Gleam package and a compatible
version range. Compatibility metadata helps Geam verify a selection; it is
never treated as approval to add native code.

## Run the complete pair

Keep a small Gleam application beside the provider while authoring it:

```text
example/
  project/   Gleam application and local Gleam package
  provider/  Rust provider crate
```

The application imports the Gleam package and calls its API normally:

```gleam
import example_text_tools/casing

pub fn main() {
  assert casing.upper("Geam") == "GEAM"
}
```

From the Gleam project, select the local companion crate and run the same path
that a standalone user relies on:

```sh
cd project
geam provider add --path ../provider
geam prepare
geam run
```

`provider add` verifies the crate metadata and records the local selection.
`prepare` checks the Rust implementation against the Gleam declarations and
builds the generated runner. `run` executes the Gleam application with that
implementation.

The repository's [text tools example](../examples/provider/text_tools) is this
complete flow with three Gleam modules. Its entrypoint asserts results such as
`upper("Geam") == "GEAM"`; a successful run is silent because every check
passes.

Keep unit tests for Rust-only logic and use this end-to-end run to verify the
Gleam declaration, provider metadata, generated component, and application call
together.

## Grow the provider with the package

The smallest provider is a collection of ordinary Rust functions. Add other
features only when the Gleam API calls for them:

- Use Rust scalars, tuples, `Result`, `Option`, and Geam's lazy `List` boundary
  for ordinary source values.
- Map a Gleam custom type when Rust constructs or receives its constructors.
- Use an external value when an opaque payload must remain owned by Rust.
- Add component state for process-local mutable or read-only capabilities.
- Add explicit configuration when constructing that state needs caller input.
- Accept a typed callback when provider code must call a Gleam function.
- Retain a generic source value only when an external value must own it across
  calls.

The [provider examples](../examples/provider) form an executable path through
those choices, beginning with scalar functions and ending with a separately
published Hex package and provider crate.

## Use the provider from an application

Provider crates are native code. In a standalone project, Geam verifies a
registry candidate and asks for approval before Cargo receives it. In a Rust
embedding project, `geam embedding sync` performs the same selection and
records the provider as an ordinary Cargo dependency. Geam verifies metadata
and typed linkage, but neither step is a security endorsement.

Standalone applications pass provider configuration as TOML at run time.
Embedding applications construct the corresponding Rust configuration and
state values through generated bindings. External values and mutable state stay
inside the running application in both workflows.

See [standalone provider selection](standalone.md#use-a-gleam-package-with-a-rust-provider)
and [embedding package synchronization](embedding.md#use-gleam-packages-and-rust-providers)
for the consuming side of each workflow.

## Publish the pair

Publish the Gleam package with Gleam's Hex tooling and the provider with Cargo.
The Hex release contains the API that applications import. The crates.io
release contains the native code that Rust hosts compile. Test the packaged
provider with `cargo publish --locked --dry-run`, verify the public
Gleam-to-provider path, and widen `gleam-version` only when that package range
has been checked.

The final [text pattern example](../examples/provider/text_pattern) shows a Hex
package, a crates.io provider, and a separate Erlang implementation of the same
Gleam API.

## Exact reference

Continue with the [host provider boundary](reference/provider-boundary.md) for
the complete type mappings, custom and external values, state, configuration,
callbacks, retained storage, generated component contract, and runner profile.
The [runtime semantics](reference/runtime-semantics.md) document defines
ownership, equality, hashing, inspection, and failure behavior after linkage.
