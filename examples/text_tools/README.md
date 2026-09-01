# Text Tools Provider Example

This is the smallest macro-authored provider example. It has no state,
configuration, or external values. One Rust provider crate implements three
ordinary Gleam modules using scalar arguments and return values only.

The complete Gleam API is:

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

The provider lists Rust module identifiers in component order, while each
module declares its exact Gleam module path:

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

Run it through the generated standalone runner:

```sh
cd examples/text_tools/project
geam provider add --path ../provider
geam prepare
geam run
```

A successful run produces no application output. The entrypoint imports all three modules
and executes every public function.

Read the three files under
[`project/packages/example_text_tools/src`](project/packages/example_text_tools/src),
[`provider/src/lib.rs`](provider/src/lib.rs), and
[`project/src/text_tools_example.gleam`](project/src/text_tools_example.gleam)
together.
