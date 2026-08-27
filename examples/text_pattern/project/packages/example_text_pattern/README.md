# example_text_pattern

Regular expressions for Gleam programs running on
[Geam](https://github.com/panarch/geam), a Rust runtime for Gleam. The matching
[geam-example-text-pattern provider](https://github.com/panarch/geam/tree/main/examples/text_pattern/provider)
implements this API with Rust's `regex` crate.

Run programs using this package with Geam. The package declares external
functions for the Rust provider; it does not include Erlang or JavaScript
implementations.

## Use the Published Packages

This workflow requires the Gleam package on Hex and the matching Rust provider
on crates.io, with Gleam, Geam, and Rust/Cargo installed. For local development
without publishing either package, use the
[checkout instructions](https://github.com/panarch/geam/blob/main/examples/text_pattern/README.md).

From your Gleam project directory, add the package:

```sh
gleam add example_text_pattern
```

Use the following in your project's entry module:

```gleam
import example_text_pattern as pattern

pub fn main() {
  let assert Ok(words) = pattern.compile("[A-Za-z]+")
  assert pattern.is_match(words, "Geam + Gleam 2026")
  assert pattern.find_all(words, "Geam + Gleam 2026") == ["Geam", "Gleam"]
  assert pattern.replace_all(words, "Geam + Gleam 2026", "<$0>")
    == "<Geam> + <Gleam> 2026"

  let assert Error(pattern.CompileError(message)) = pattern.compile("(")
  assert message != ""
}
```

Prepare and run it with Geam:

```sh
geam prepare
geam run
```

On the first `prepare`, review and approve the matching
`geam-example-text-pattern` candidate in an interactive terminal. This approves
native Rust code. Geam records the selected dependency and Cargo lock, then
builds and checks the runner. No explicit `geam provider add` or provider
configuration is needed for this workflow.

The example succeeds when every assertion passes; it prints no application
output. See the
[standalone guide](https://github.com/panarch/geam/blob/main/docs/standalone.md)
for provider approval, managed files, and entry module selection.

## API

- `compile(source)` returns `Ok(Pattern)` or `Error(CompileError(message))`.
- `is_match(pattern, text)` checks whether any part of the text matches.
- `find_all(pattern, text)` returns non-overlapping matches in source order.
- `replace_all(pattern, text, replacement)` replaces every match; `$0` inserts
  the complete match.

`Pattern` is opaque to Gleam. Patterns compiled from identical source text
compare equal, even when compiled separately. Inspection shows the pattern
text, for example `Pattern("[A-Za-z]+")`, rather than Rust's regex internals.

For the paired Rust implementation and its authoring macros, read the
[provider README](https://github.com/panarch/geam/blob/main/examples/text_pattern/provider/README.md).
