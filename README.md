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
Package-qualified source-less Rust host modules can provide infallible
functions with zero through seven `BigInt`/`bool` arguments and a
`BigInt`/`bool` return through a separate hosted pipeline. Unsupported Rust
types and arities are rejected by trait resolution rather than at runtime.
Source-declared backend external functions or types are not provider linkage
surfaces.

The main public entry points are:

- `compile_typed_module`
- `compile_typed_program`
- `compile_typed_package_program`
- `compile_typed_project`
- `compile_typed_host_program`
- `plan_module`
- `plan_program`
- `plan_host_program`
- `ExecutionPlan::explain`
- `HostedExecution::explain`
- `Value::inspect`
- `run_main`
- `HostedExecution::run_main`

`run_main` takes a caller-owned `EchoSink`; Geam never selects stdout, stderr,
or a hidden output destination for the host. Ordinary and pipeline Echo both
emit through that boundary and continue with their original value.

The existing `TypedProgram -> ModulePlan -> ExecutionPlan -> run_main` path is
host-free. Rust callbacks enter only through
`HostedTypedProgram -> HostedModulePlan -> HostedExecution`; the hosted plan
nodes store callable schemas and targets, while the hosted wrapper carries
implementations as a private sidecar until `HostedExecution` retains only the
callbacks selected by specialization.

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
