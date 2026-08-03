# Geam

<p align="center">
  <img src="assets/geam-mascot.svg" width="240" alt="Geam mascot">
</p>

Geam runs a supported subset of typed Gleam code inside Rust programs.
The name is pronounced like Korean "김" (/kim/, romanized "gim").

It is not a new language, a Gleam fork, a full Gleam implementation, a package
manager, or an official Gleam target. Geam is an experimental Rust-embedded
alternative execution runtime for Gleam.

Gleam already runs by lowering into another execution environment:

```text
Gleam source -> Erlang source    -> BEAM
Gleam source -> JavaScript       -> Node / Deno / Bun
Gleam source -> Geam execution plan -> Rust-embedded runtime
```

Geam keeps Gleam as the source language. It uses Gleam's parser and
analyse/infer pass, then lowers the supported executable surface of an
selected module graph into a Rust-owned plan.

```text
Resolved Gleam project or in-memory module sources
-> Gleam typed program
-> Geam module plan
-> Geam execution plan
-> Geam runtime value
```

Unsupported execution semantics are rejected while planning from Gleam's typed
AST, before runtime evaluation. The resulting `ModulePlan` owns the root entry
and every supplied module's validated definitions, and is the canonical
inspectable planner output. Consuming it produces an opaque `ExecutionPlan` for
runtime use rather than public raw AST data assembled by runtime callers. Its
lowered control flow, typed values, instructions, and edge arguments remain
inspectable through `ExecutionPlan::explain()`. The explanation is
human-readable output rather than a stable serialization format.

## Status

Geam is in an early runtime milestone. The current execution profile includes
the core Gleam value families, custom types, generics, patterns, records,
functions, constants, imports, and read-only loading of already resolved Gleam
projects. The official `gleam_stdlib` package is not built in: compatible
imported modules are compiled from the package sources resolved by Gleam.
Package-qualified source-less Rust host modules and source-backed external
function providers can supply functions with zero through seven
`BigInt`, `f64`, `EcoString`, `BitArrayValue`, `char`, `bool`, or `()`
arguments and returns through a separate hosted pipeline. Providers that never
return successfully use Rust's return-only `Infallible` marker. Unsupported
Rust types and arities are rejected by trait resolution rather than at runtime.
Provider linkage selects an exact external declaration or its Gleam fallback
during planning; ordinary Gleam functions cannot be overridden.
Source-declared constructorless external types can be linked to profile-owned
Rust payloads. Providers define Gleam equality, runtime hashing, and canonical
inspection through narrow retained-value contexts; equal payloads must hash
equally, while collisions are resolved by equality. Their public runtime
values remain opaque and self-contained. The `geam::gleam_stdlib` module
provides an explicitly composed host-provider bundle for unchanged official
`gleam_stdlib` `v1.0.3` modules that require externals; it is not injected by
project loading. The verified module set is `gleam/bit_array`, `gleam/bool`,
`gleam/bytes_tree`, `gleam/dict`, `gleam/dynamic`, `gleam/dynamic/decode`,
`gleam/float`, `gleam/function`, `gleam/int`, `gleam/io`, `gleam/list`,
`gleam/option`, `gleam/order`, `gleam/pair`, `gleam/result`, `gleam/set`,
`gleam/string`, `gleam/string_tree`, and `gleam/uri`.

The main public entry points are:

- `compile_typed_module`
- `compile_typed_program`
- `compile_typed_package_program`
- `compile_typed_project`
- `compile_typed_host_program`
- `compile_typed_host_project`
- `plan_module`
- `plan_program`
- `plan_host_program`
- `ExecutionPlan::explain`
- `HostedExecution::explain`
- `Value::inspect`
- `run_main`
- `HostedExecution::run_main`

`run_main` takes a caller-owned `EchoSink`; language Echo does not select
stdout, stderr, or a hidden output destination. Ordinary and pipeline Echo
both emit through that boundary and continue with their original value.
Official `gleam/io` functions use a separate caller-owned
`geam::gleam_stdlib::IoSink`. The default stdlib run state collects owned
stdout and stderr events, while custom profiles can project another concrete
sink without changing the hosted execution boundary.

The existing `TypedProgram -> ModulePlan -> ExecutionPlan -> run_main` path is
host-free. Rust callbacks enter only through
`HostedTypedProgram -> HostedModulePlan -> HostedExecution`; the hosted plan
nodes store callable schemas and targets, while the hosted wrapper carries
implementations as a private sidecar until `HostedExecution` retains only the
callbacks selected by specialization. A `HostProfile` defines caller-owned
run state, and `HostedExecution::run_main` borrows that state explicitly for
one run. `compile_typed_host_project` applies the same hosted boundary to the
read-only resolved-project loader without reparsing selected modules or
running Gleam CLI.

Owned scalar closures use `BigInt`, `f64`, `EcoString`, `BitArrayValue`,
`char`, `bool`, and `()`. Scoped providers use `HostCall` with typed
`HostList`, `HostTuple`, ordinary custom, and external handles; these handles
cannot escape their invocation, and compound returns are built explicitly
through the same call. An external payload can retain exact typed Gleam values
with `HostStoredValue` and restore them only through a later active
`HostCall`. It can instead retain an existential `HostStoredDynamic` together
with its exact specialized Gleam shape; a later typed decode returns `None`
when that shape does not match. Generic providers and Gleam function values
use the same typed specialization and call paths as ordinary Gleam functions.
Private transient-style containers can use this storage through immutable
persistent versions; Geam does not expose general mutable external graphs.
The official Dict provider uses this boundary for persistent hash-bucket
storage and preserves Gleam fallback bodies for operations implemented in
Gleam. Dictionary iteration order is not part of the Geam contract.

`HostedExecution::try_from_module_plan` seals the entry-reachable host ABI
before runtime construction. Provider state remains caller-owned, and nested
source panics and host failures retain the actual failed source or provider
identity. See [runtime semantics](docs/runtime-semantics.md) for the ownership,
re-entry, and sealing rules.

## Upstream

Current Gleam compiler baseline: `v1.17.0`.

Current Gleam stdlib integration baseline: `gleam_stdlib` `v1.0.3`.

See [docs/upstream-gleam.md](docs/upstream-gleam.md) for the exact commit,
compiler-boundary details, and sync policy.

## Testing

Run the test suite:

```sh
cargo test
```

See [docs/testing.md](docs/testing.md) for fixture rules, planner test naming,
coverage commands, and validation expectations.

See [docs/review-policy.md](docs/review-policy.md) for planner boundary, error,
helper, and coverage review rules.
