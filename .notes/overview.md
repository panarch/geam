# Geam Overview Context

Geam is not a new general-purpose language or a Gleam fork. It is a
Rust-embedded execution runtime for a supported profile of typed Gleam code.

The core idea is:

> Gleam ecosystem as the authoring layer, Geam as the embedded execution layer.

Geam should let users write `.gleam` source with normal Gleam syntax, parser,
type analysis, formatter, LSP support, and project conventions. Geam starts
after Gleam has produced a typed module.

## Current Pipeline

The active pipeline is:

```text
Gleam source
-> Gleam typed module
-> Geam ExecutionPlan
-> Geam runtime Value
```

Geam does not currently own a subset source AST, parser, or type inferencer.
The earlier independent parser/analyse prototype was removed because it made
the project duplicate Gleam's hardest front-end work before runtime execution
could be validated.

## Positioning

Geam is closer to a "Gleam profile runner for Rust embedding" than a Gleam
replacement. A valid Gleam program is not necessarily Geam-compatible. The Geam
planner rejects typed Gleam programs outside the current executable profile
before runtime.

Current public description:

> Geam runs a supported subset of typed Gleam code inside Rust programs.

The name is pronounced like Korean "kim" (`/kim/`, romanized `gim`).

## Core Value Notes

The likely core value is not "a new language" or "a Gleam replacement". The
stronger positioning is:

```text
typed functional scripting for Rust, powered by Gleam
```

Why this can be attractive:

- Most embeddable scripting options are dynamically typed, or require a heavier
  host/runtime boundary.
- Gleam gives a friendly functional syntax with formatter, LSP, type checking,
  pattern matching, and useful diagnostics.
- Geam can let Rust programs embed a typed extension/configuration/logic layer
  while reusing Gleam's existing authoring experience.
- If the runtime boundary works well, Gleam code could run in environments where
  Rust programs run, not only in Erlang/BEAM or JavaScript environments.

This should stay as internal positioning until the execution profile, host
function boundary, and stdlib story are clearer. Public messaging should avoid
claiming full Gleam compatibility or a native Rust backend until those claims are
true.

## Execution Boundary

`ExecutionPlan` is the Geam-owned executable representation. It is an opaque
validated value, not public raw AST data assembled by callers.

Current boundary rule:

```text
Gleam source validity    = Gleam parser/analyse responsibility
Geam profile validity    = Geam planner responsibility
Runtime structural safety = guaranteed by ExecutionPlan construction
```

Runtime code assumes it receives a valid `ExecutionPlan`. Structural execution
failures should be rejected by planning as `PlanError`, not represented as
runtime errors.

## Current Profile

The current execution profile is intentionally small:

```text
function-only modules
zero-arity main entry point
Int / String / Float / Bool / Nil values
Tuple values and tuple index projection
let bindings
local variables
block expressions with local scope
local function calls
integer arithmetic
integer comparison
equality / inequality
string concatenation
boolean and integer negation
boolean short-circuit operators
Bool, Int, Float, and String subject case expressions
direct local-function pipelines
top-level function references as values
function-valued let aliases and calls
function-typed arguments and calls
function-returning function values
anonymous functions
capturing closures
use callback syntax
tail-call execution for direct local-function calls
```

Unsupported Gleam features should be added incrementally. Each feature should
come with planner accept/reject tests, runtime tests when executable behavior is
introduced, and line coverage kept at 100%.

## Current Tuple Branch Notes

- Tuple value support currently passes `cargo fmt --check`, `cargo test`,
  `cargo test --locked`, `cargo clippy --all-targets -- -D warnings`,
  `cargo llvm-cov --summary-only --fail-under-lines 100`, and `git diff
  --check`.
- Tuple projection has one reviewed runtime invariant boundary:
  `ExecutionError::tuple_index_family_mismatch`. It is only for typed tuple-index
  plan evaluation when runtime tuple contents do not match the planner-selected
  element family. Tuple index validation and typed-AST margin handling remain
  planner responsibilities.

Current function-value support is intentionally narrow. Geam accepts references
to current-module top-level functions as values, can bind those values to locals,
can pass them to function-typed arguments, and can call those function-valued
locals or arguments. Function-returning function values are supported as
registered top-level function references, including public `main` returning a
function value. Anonymous functions, captures, function-value pipelines,
imports, and externals remain outside the current profile.

## Later Direction

Rust host integration is still a later milestone. The likely host model remains:

```text
Rust crate / host app
-> exports selected APIs as Geam modules
-> provides generated or handwritten Gleam stubs/mocks
-> Geam source imports those modules
-> Geam runtime calls registered Rust functions
```

Cargo remains responsible for Rust dependency management. Geam should not
dynamically import arbitrary Rust crates at runtime.

Initial host crossing types should stay narrow:

```text
Int
Float
Bool
String
List<T>
Option<T>
Result<T, E>
limited records / variants
opaque host handles
```

Avoid exposing Rust lifetimes, arbitrary generics, trait objects, async runtimes,
or long-lived callback ownership in early milestones.
