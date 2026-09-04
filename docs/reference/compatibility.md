# Compatibility

Geam uses the parser and type checker from the supported Gleam release without
changing their language behavior, then implements a separate, smaller execution
profile in Rust. Source acceptance follows that compiler behavior; executable
compatibility is decided while Geam plans the typed module graph.

Packages and modules beyond the verified list may also work. End-to-end
compatibility is established when their source and required providers pass the
integration path described here.

## Toolchain Baseline

| Component | Supported baseline |
| --- | --- |
| Rust | `1.96` or newer, 64-bit target |
| Gleam compiler and CLI | `v1.18.1` |
| Compiler crate | `geam-gleam-core 1.18.1-geam.2` |
| `gleam_stdlib` | `v1.0.3` |
| `gleam_http` | `v4.3.0` |
| `gleam_json` | `v3.1.0` |
| `gleam_time` | `v1.8.0` |

`geam-gleam-core` packages the matching compiler source for this integration.
Geam pins it exactly and updates the baseline deliberately. It does not follow
Gleam `main` or silently reinterpret a project with another compiler release.

## Source Profile

The current execution profile includes the core Gleam value families,
constants, functions, imports, pattern matching, records, custom types,
generics, and read-only loading of resolved Gleam projects. Every supplied
module body is validated from the typed AST before executable lowering.

Project loading always selects and analyses the Erlang-target import closure,
regardless of the package's default build target. This selects the source path;
Geam does not execute BEAM code. Ordinary Gleam bodies and available fallback
bodies are planned for the Rust runtime. A bodyless Erlang external must bind to
a built-in or selected Rust provider.

A bodyless JavaScript-only external is unavailable on this path. If selected
Gleam code calls it, Gleam analysis fails before provider validation.
If it is unreachable, Geam omits it from the planned module. A JavaScript native
implementation does not become a Rust provider merely because it is present in
the package.

Provider call signatures support zero through seven source arguments. Direct
owned closures cover `Int`, `Float`, `String`, `BitArray`, `UtfCodepoint`,
`Bool`, and `Nil`; scoped providers add typed compound, custom, external,
generic, retained, and callback forms documented in the
[provider boundary](provider-boundary.md).

Generated Rust function bindings currently support this recursive data grammar:

```text
Scalar | Tuple(Data...) | Result(Data, Data) | Option(Data) | List(Data)
```

Records, domain custom types, external values, callbacks, and generic types
cannot currently appear in generated Rust function signatures. Gleam modules
may still use them internally. Rust can reach such logic through generated
bindings only when the same-name root module exposes a public function with
supported arguments and return values. See [Rust embedding](embedding-boundary.md)
for the exact recursive type map.

## Verified Package Integrations

### Gleam Standard Library

Geam verifies all 19 public modules in `gleam_stdlib v1.0.3` against unchanged
upstream package source:

```text
gleam/bit_array        gleam/bool          gleam/bytes_tree
gleam/dict             gleam/dynamic       gleam/dynamic/decode
gleam/float            gleam/function      gleam/int
gleam/io               gleam/list          gleam/option
gleam/order            gleam/pair          gleam/result
gleam/set              gleam/string        gleam/string_tree
gleam/uri
```

Modules without mandatory externals execute directly from source. The explicit
stdlib provider supplies bodyless bit-array, dictionary, dynamic, numeric, IO,
string, string-tree, and URI operations. Randomness and IO remain caller-owned
capabilities; project loading does not inject global defaults.

### Gleam HTTP

Geam verifies all five `gleam_http v4.3.0` modules and their unchanged public
functions and data types. The package needs no HTTP-specific provider.

Compatibility covers HTTP values and transformations, methods and schemes,
cookies, URI-backed requests and responses, content disposition, and multipart
parsing. It does not provide a network client, server, socket, transport, or
service runtime.

### Gleam JSON

Geam verifies unchanged `gleam_json v3.1.0` through a separate explicit JSON
provider and the stdlib provider. It covers JSON construction, encoding,
parsing, `Dynamic` conversion, and upstream decode behavior without introducing
a generic JSON runtime value.

The Erlang implementation path is selected. A JavaScript-only external does not
become a Rust callback merely because it is present in the package.

### Gleam Time

Geam verifies all three `gleam_time v1.8.0` modules. Duration arithmetic,
calendar conversion, and RFC3339 behavior remain in unchanged upstream package
source. The Time provider supplies only wall-clock time and the current local
UTC offset through caller-owned state.

This integration does not provide timezone history, a monotonic clock, timers,
sleep, or an implicit system-clock fallback after provider failure.

## Runtime And Deployment Limits

- Geam currently requires a 64-bit Rust target.
- Standalone and embedding load selected Gleam source and resolved package
  sources; they do not compile a BEAM or JavaScript artifact.
- The current Rust embedding workflow reads its nested `gleam/` project at
  application initialization. A copied executable alone is not self-contained.
- Provider components are statically linked Rust dependencies. Geam does not
  load arbitrary dynamic libraries or choose providers at runtime.
- Source effects exist only when the selected profile supplies their explicit
  capability or provider.
- Runtime inspection and `ExecutionPlan::explain()` are human-readable
  diagnostics, not stable serialization formats.

For the exact upstream release, commit, mirror package, and update procedure,
see the [upstream synchronization record](../development/upstream-gleam.md).
For value, equality, IO, error, and control-flow behavior, see
[runtime semantics](runtime-semantics.md).
