# geam-example-text-pattern

`geam-example-text-pattern` implements the ordinary
`example_text_pattern` Gleam package with Rust's `regex` crate. The provider
uses Geam's public authoring macros; its source contains only component
identity, source type declarations, source-visible semantics, and function
bodies.

The complete mapping lives in [`src/lib.rs`](src/lib.rs):

- `#[geam::provider]` declares the target Gleam package and module;
- `#[geam::external(..., manual)]` stores a compiled regular expression while
  defining equality, hashing, and inspection from the original pattern text;
- `#[geam::custom]` maps the named `CompileError` constructor;
- ordinary Rust `Result<Pattern, CompileError>` maps to Gleam `Result`; and
- `Vec<EcoString>` constructs the returned Gleam `List(String)` once.

The crate remains independently packageable. Its Cargo metadata is the
crates.io discovery contract; the Gleam package itself carries no Geam
metadata. The repository-local Cargo patch selects the current checkout until
the next lockstep Geam release publishes the authoring surface.

See the [complete example](../README.md) for the matching Gleam declaration and
standalone commands.
