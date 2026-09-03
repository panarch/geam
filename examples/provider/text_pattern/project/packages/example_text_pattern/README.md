# example_text_pattern

Regular expressions for Gleam, with an Erlang implementation using OTP's `re`
module and a matching Rust provider for
[Geam](https://github.com/panarch/geam), a Rust runtime for Gleam.

Both runtimes use the same `Pattern`, `CompileError`, and four functions below.
Each keeps its native regex semantics; read Runtime Semantics below before
switching engines. There is no JavaScript implementation.

## Run on Erlang

With Gleam and Erlang/OTP installed, add the published Hex package to your
project. Rust and Geam are not required. For local development without
publishing either package, use the
[checkout instructions](https://github.com/panarch/geam/blob/main/examples/provider/text_pattern/README.md).

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
  assert pattern.replace_all(words, "Geam + Gleam 2026", "<word>")
    == "<word> + <word> 2026"

  let assert Error(pattern.CompileError(message)) = pattern.compile("(")
  assert message != ""
}
```

```sh
gleam run --target erlang
```

The example succeeds when every assertion passes; it prints no application
output.

## Run on Geam

The same source also runs with the
[geam-example-text-pattern provider](https://github.com/panarch/geam/tree/main/examples/provider/text_pattern/provider),
which uses Rust's `regex` crate. This workflow requires Geam and Rust/Cargo,
plus the same-version Rust provider published on crates.io:

```sh
geam prepare
geam run
```

On the first `prepare`, review and approve the matching
`geam-example-text-pattern` candidate in an interactive terminal. This approves
native Rust code. Geam records the selected dependency and Cargo lock, then
builds and checks the runner. No explicit `geam provider add` or provider
configuration is needed for this workflow.

Geam uses the Rust provider, not the packaged Erlang implementation. See the
[standalone guide](https://github.com/panarch/geam/blob/main/docs/standalone.md)
for provider approval, managed files, and entry module selection.

## API

- `compile(source)` returns `Ok(Pattern)` or `Error(CompileError(message))`.
- `is_match(pattern, text)` checks whether any part of the text matches.
- `find_all(pattern, text)` returns non-overlapping matches in source order.
- `replace_all(pattern, text, replacement)` replaces every match using the
  engine's replacement syntax.

## Runtime Semantics

| Runtime | Engine | Whole-match replacement | First capture replacement |
| --- | --- | --- | --- |
| Erlang | [`re`](https://www.erlang.org/doc/apps/stdlib/re.html), with `unicode` and `ucp` | `"&"` | `"\\1"` in a Gleam string |
| Geam | Rust [`regex`](https://docs.rs/regex/latest/regex/) | `"$0"` | `"$1"` |

Patterns and replacement strings are passed to the selected engine without
translation. Regex features, zero-length match behavior, resource limits, and
compile-error messages follow that engine. For example, Erlang accepts
`Geam(?=-)` lookahead; Rust `regex` rejects it. Use literal replacements such as
`"<word>"` when sharing the example across runtimes.

`Pattern` is opaque. Equality and inspection are not portable between runtimes.
On Geam, patterns compiled from identical source text compare equal, and
inspection shows `Pattern("[A-Za-z]+")` rather than Rust's regex internals. On
Erlang, the value is OTP's opaque compiled pattern; use the API to inspect its
matching behavior rather than relying on its representation or equality.

For the paired Rust implementation and its authoring macros, read the
[provider README](https://github.com/panarch/geam/blob/main/examples/provider/text_pattern/provider/README.md).
