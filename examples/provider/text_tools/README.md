# Host Provider: Text Tools

This is the smallest complete provider. A Gleam package declares six text
functions, and one companion Rust crate implements them for Geam. It uses only
scalar arguments and returns, with no provider state, configuration, or opaque
external values.

## Read The Example

1. [`project/packages/example_text_tools/src`](project/packages/example_text_tools/src)
   contains the public Gleam API across three modules.
2. [`provider/src/lib.rs`](provider/src/lib.rs) implements the same package,
   module paths, functions, and signatures in Rust.
3. [`project/src/text_tools_example.gleam`](project/src/text_tools_example.gleam)
   imports the Gleam modules and checks every call through the provider.

The package API is ordinary Gleam:

```gleam
// example_text_tools
pub fn join(left: String, separator: String, right: String) -> String
pub fn surround(value: String, left: String, right: String) -> String

// example_text_tools/casing
pub fn upper(value: String) -> String
pub fn lower(value: String) -> String

// example_text_tools/checks
pub fn starts_with(value: String, prefix: String) -> Bool
pub fn ends_with(value: String, suffix: String) -> Bool
```

The provider declares the package once, lists its Rust modules, and gives each
module the exact Gleam path it implements:

```rust
#[geam::provider(
    package = "example_text_tools",
    modules = [text_tools, casing, checks],
)]
pub struct Component;

#[geam::module(path = "example_text_tools")]
mod text_tools { /* ... */ }

#[geam::module(path = "example_text_tools/casing")]
mod casing { /* ... */ }

#[geam::module(path = "example_text_tools/checks")]
mod checks { /* ... */ }
```

## Run

With Gleam, Rust, and Geam installed, run from the repository root:

```sh
cd examples/provider/text_tools/project
geam provider add --path ../provider
geam prepare
geam run
```

The entrypoint checks results including `upper("Geam") == "GEAM"`,
`surround("ready", "[", "]") == "[ready]"`, and both Boolean predicates. A
successful run is silent because all assertions pass.

Continue with [value types](../value_types/README.md) to map Gleam scalars,
tuples, Lists, custom types, Result, and Option to provider signatures.
