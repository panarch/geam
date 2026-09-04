# Architecture

Geam exists to run typed Gleam modules where Rust owns the execution
environment. Gleam remains the source language; Geam uses the parser and type
checker from the supported Gleam release, then applies a Rust-owned planner,
runtime, and host-capability boundary.

This lets a Gleam application use native Rust providers without maintaining a
Rust runner, and lets a Rust application call supported public Gleam functions
through generated typed bindings.

## Source And Runtime Ownership

The Gleam language and package ecosystem define:

- source syntax, parsing, and type inference;
- package names, imports, and dependency resolution;
- public function and type declarations; and
- the ordinary source implementations shipped by Gleam packages.

Geam owns:

- validation of the supported executable profile;
- typed module and execution plans;
- runtime values, instructions, and control flow;
- static linkage to Rust host providers; and
- the application-facing standalone and embedding workflows.

The primary pipeline is:

```text
resolved Gleam project or in-memory package sources
-> Gleam typed program
-> Geam module plan
-> Geam execution plan
-> Rust-owned runtime values and effects
```

Geam reuses Gleam's source-language front end rather than implementing a
separate parser, source AST, or package manager. Its parser and type checker come
from the supported Gleam release without modifications to their implementation.
Geam-specific work starts at the typed-program boundary. Packaging differences
are limited to distribution metadata and generated version identification,
while Geam owns planning and execution.

## Planning Before Execution

The compiler integration parses and analyses the complete selected module
graph. Geam then validates that graph while constructing an inspectable
`ModulePlan`.
Unsupported execution semantics are rejected at this boundary, including in
supplied dependency definitions, rather than becoming conditional runtime
errors.

Consuming the module plan produces an opaque `ExecutionPlan`. Runtime lowering
is total after profile validation; it does not introduce another unsupported
feature boundary. `ExecutionPlan::explain()` provides a human-readable view of
lowered functions, values, instructions, and control-flow edges, but its text is
not a stable serialization format.

## Plain And Hosted Programs

A plain program contains only source behavior that needs no Rust callbacks. It
uses:

```text
TypedProgram -> ModulePlan -> ExecutionPlan -> run_main
```

A hosted program combines the same source graph with exact Rust provider
schemas and implementations:

```text
HostedTypedProgram
-> HostedModulePlan
-> HostedExecution
-> run_main with caller-owned state
```

The two paths are separate Rust types. Host callbacks do not enter the plain
plan accidentally, and a hosted execution retains only implementations selected
by the specialized source closure.

## Static Provider Composition

A provider is an ordinary Rust crate exporting a generated or manually
implemented `HostProviderComponent`. Components declare concrete stores, run
state, source modules, callable schemas, and optional configuration
initialization.

The final runner combines components into one concrete profile at compile time.
There is no runtime provider registry, dynamic library lookup, or type-erased
state map. Provider discovery and approval select Cargo dependencies; Cargo
then builds the static graph.

An exact provider implementation wins for a matching source external. An
external declaration with a Gleam fallback can keep that source body when no
provider is selected. A bodyless external without a matching provider is a
planning error.

## Two Application Workflows

Standalone and embedding are two owners around the same architecture.

### Standalone

The Gleam project owns source and dependencies. Geam manages a project-local
Cargo manifest, lockfile, and generated runner. The user approves native
provider dependencies and executes the selected module through `geam run`.

### Rust Embedding

The Cargo application owns its manifest, process, and lifecycle. Geam manages a
conventional nested `gleam/` project connection and checked-in typed bindings.
Rust loads and seals the selected module, supplies capabilities and state, and
calls generated function handles.

Provider crates can serve both workflows because component registration and
runtime ownership do not change.

## Effects Stay Caller-Owned

Language Echo uses a caller-supplied `EchoSink`. The upstream `gleam/io`
functions use a separate caller-supplied `IoSink` when run through Geam.
Entropy, wall-clock access, provider configuration, and mutable state are
similarly constructed by the runner or embedding application rather than read
from process-global defaults.

This separation makes effects visible at assembly time and keeps execution
plans independent from credentials, output destinations, and mutable process
state.

Continue with [runtime semantics](runtime-semantics.md) for exact value and
execution behavior, [host providers](provider-boundary.md) for static component
contracts, and [compatibility](compatibility.md) for the currently verified
source and package surface.
