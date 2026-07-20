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
analyse/infer pass, then lowers the supported executable surface of the
resulting typed module into a Rust-owned plan.

```text
Gleam source
-> Gleam typed module
-> Geam module plan
-> Geam execution plan
-> Geam runtime value
```

Unsupported execution semantics are rejected while planning from Gleam's typed
AST, before runtime evaluation. The resulting `ModulePlan` is the canonical
inspectable planner output. Consuming it produces an opaque `ExecutionPlan` for
runtime use rather than public raw AST data assembled by runtime callers. Its
runtime control-flow topology remains inspectable through
`ExecutionPlan::explain()`. The explanation is human-readable output rather
than a stable serialization format.

## Status

Geam is in an early runtime milestone. The current execution profile supports a
small function-only surface for integers, strings, booleans, nil, local
bindings, local calls, and basic operators.

The main public entry points are:

- `compile_typed_module`
- `plan_module`
- `ExecutionPlan::explain`
- `run_main`

## Upstream

Current Gleam baseline: `v1.17.0`.

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
