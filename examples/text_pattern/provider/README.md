# geam-example-text-pattern

An example provider for [Geam](https://github.com/panarch/geam), a Rust runtime
for a supported subset of Gleam. It implements the `example_text_pattern`
Gleam package with Rust's `regex` crate and demonstrates how to expose Rust
functionality to Gleam through Geam's public provider authoring macros.

## Gleam API

- `compile(source)` returns a compiled `Pattern` or `CompileError(message)` in
  a Gleam `Result`.
- `is_match(pattern, text)` checks whether the text contains a match.
- `find_all(pattern, text)` returns the non-overlapping matches in source order.
- `replace_all(pattern, text, replacement)` replaces every match; `$0` refers to
  the complete match.

```gleam
import example_text_pattern as pattern

pub fn main() {
  let assert Ok(words) = pattern.compile("[A-Za-z]+")
  assert pattern.is_match(words, "Geam + Gleam 2026")
  assert pattern.find_all(words, "Geam + Gleam 2026") == ["Geam", "Gleam"]
  assert pattern.replace_all(words, "Geam + Gleam 2026", "<$0>")
    == "<Geam> + <Gleam> 2026"
}
```

`Pattern` is opaque to Gleam. Patterns compiled from the same source text
compare equal, and inspection shows the pattern text rather than Rust's regex
internals.

## Run the Example

The matching Gleam package is a local dependency in the Geam repository, not
a package published to Hex. Installing this Cargo crate alone does not add
the Gleam declarations to a project.

Follow the [complete example and setup instructions](https://github.com/panarch/geam/blob/main/examples/text_pattern/README.md)
to prepare and run the project with Geam. No provider configuration is required.
The [standalone guide](https://github.com/panarch/geam/blob/main/docs/standalone.md)
explains provider selection and the generated Cargo runner.

## Provider Authoring

The [Rust implementation](https://github.com/panarch/geam/blob/main/examples/text_pattern/provider/src/lib.rs)
matches the [Gleam declarations](https://github.com/panarch/geam/blob/main/examples/text_pattern/project/packages/example_text_pattern/src/example_text_pattern.gleam):

- `#[geam::provider]` declares the target Gleam package and module;
- `#[geam::external(..., manual)]` stores a compiled regular expression while
  defining equality, hashing, and inspection from the original pattern text;
- `#[geam::custom]` maps the named `CompileError` constructor;
- ordinary Rust `Result<Pattern, CompileError>` maps to Gleam `Result`; and
- `Vec<EcoString>` constructs the returned Gleam `List(String)` once.

The crate's Cargo metadata identifies the target Gleam package and compatible
versions for Geam's provider discovery. The Gleam package itself carries no
Geam-specific metadata.

See the [provider authoring guide](https://github.com/panarch/geam/blob/main/docs/host-providers.md)
for the API contracts, or the [other provider examples](https://github.com/panarch/geam/blob/main/examples/README.md)
for smaller starting points and additional authoring patterns.
