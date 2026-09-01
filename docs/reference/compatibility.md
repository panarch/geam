# Compatibility

Geam uses Gleam's published compiler front-end but implements a separate,
smaller execution profile in Rust. Source acceptance follows Gleam parsing and
type analysis; executable compatibility is decided while Geam plans the typed
module graph.

An unchecked package or module is not automatically incompatible. It has not
been verified end to end until its source and required providers pass the
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

Geam pins its compiler crate exactly and updates the baseline deliberately. It
does not follow Gleam `main` or silently reinterpret a project with another
compiler release.

## Source Profile

The current execution profile includes the core Gleam value families,
constants, functions, imports, pattern matching, records, custom types,
generics, and read-only loading of resolved Gleam projects. Every supplied
module body is validated from the typed AST before executable lowering.

Project loading selects the Erlang-target import closure. Erlang external
declarations can bind to Geam providers or use available Gleam fallback bodies.
JavaScript-only native implementations are not Rust providers and cannot by
themselves satisfy a bodyless external in this profile.

Provider call signatures support zero through seven source arguments. Direct
owned closures cover `Int`, `Float`, `String`, `BitArray`, `UtfCodepoint`,
`Bool`, and `Nil`; scoped providers add typed compound, custom, external,
generic, retained, and callback forms documented in the
[provider boundary](provider-boundary.md).

Rust embedding intentionally exposes a narrower public function boundary:

```text
Scalar | Tuple(Data...) | Result(Data, Data) | Option(Data) | List(Data)
```

Use a small Gleam boundary module to project records, domain custom types,
external values, callbacks, or generic APIs into that ordinary-data grammar.
See [Rust embedding](embedding-boundary.md) for the exact recursive type map.

## Verified Package Integrations

### Gleam Standard Library

Geam verifies all 19 public modules in `gleam_stdlib v1.0.3` against unchanged
official source:

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
parsing, `Dynamic` conversion, and official decode behavior without introducing
a generic JSON runtime value.

The Erlang implementation path is selected. A JavaScript-only external does not
become a Rust callback merely because it is present in the package.

### Gleam Time

Geam verifies all three `gleam_time v1.8.0` modules. Duration arithmetic,
calendar conversion, and RFC3339 behavior remain in official Gleam source. The
Time provider supplies only wall-clock time and the current local UTC offset
through caller-owned state.

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
