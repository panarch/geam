# Planner / Runtime Boundary Notes

Current execution direction:

```text
Gleam source
-> Gleam typed module
-> Geam planner
-> Geam ExecutionPlan
-> Geam runtime
```

Gleam remains responsible for parsing and type analysis. Geam starts from
Gleam's typed AST, validates the supported execution profile, lowers it into a
small Rust-owned plan, and executes that plan.

## Current Invocation Boundary

The current planner-built plan resolves executable function entry points before
runtime:

- `ExecutionPlan` stores the zero-arity `main` function directly.
- Non-main functions remain available for local calls, including calls back to
  `main`.
- Local calls store typed runtime function IDs, not function names.
- Local call arity is checked in the planner.
- Runtime does not expose `run_function(name, args)`.
- Runtime no longer has `MissingFunction`, `ArityMismatch`, `UnboundLocal`, or
  `TypeMismatch` errors for planner-built plans.

This means `MissingFunction` and `ArityMismatch` are not Gleam runtime
semantics in the Geam pipeline. Wrong source-level arity should be rejected by
Gleam analyse before Geam planning. Invalid typed-AST shapes should be rejected
at the planner boundary, not deferred to runtime.

## Validated ExecutionPlan

`ExecutionPlan` is now an opaque validated value:

- Raw fields are private.
- Public callers inspect through read-only accessors.
- Runtime-only function tables live behind the plan boundary.
- Runtime/cache details are excluded from `ExecutionPlan` `Debug` and
  `PartialEq` surfaces.
- Planner tests compare the canonical plan shape, not runtime cache layout.

This is the current compromise:

- The public plan value is inspectable enough for review and tests.
- Callers cannot assemble invalid raw plans directly.
- Runtime code can use direct indexing internally because the planner constructs
  the required table and local-id invariants.

## Error Taxonomy

`PlanError` is split by boundary class:

1. Unsupported profile:
   valid Gleam typed AST, but Geam v0 does not support lowering/executing that
   feature yet.

2. Invalid typed-AST margin:
   shapes that should not come from normal Gleam source plus analyse, but can
   appear when a test or caller manually mutates a `TypedModule`.

Current shape:

```rust
PlanError::UnsupportedExpression { ... }
PlanError::UnsupportedFunction { ... }
PlanError::UnsupportedPipeline { ... }

PlanError::InvalidTypedAst {
    reason: InvalidTypedAstReason,
}
```

Rules:

- `reject_profile_*` tests are valid source-based cases and expect
  `Unsupported*` errors.
- `reject_margin_*` tests are synthetic typed-AST cases and expect
  `InvalidTypedAst`.
- Do not reintroduce free-form static string reasons.

## Review Heuristic: Derived Surfaces

Internal runtime/cache structs should not derive `Clone`, `PartialEq`, or `Eq`
mechanically unless the derived behavior is part of the reviewed plan surface.

Reason:

- Derived equality can make unused runtime/cache fields look meaningful because
  they participate in whole-struct comparisons.
- Derived clone/debug/equality can hide fields that are carried forward but no
  longer needed by execution.
- Line coverage will not catch this class of issue because construction and
  comparison lines can still execute.

Current rule of thumb:

- Public or inspectable `ExecutionPlan` shape may derive comparison traits when
  tests intentionally compare it as the canonical plan shape.
- Private runtime cache structs should derive only what execution actually
  needs.
- `Debug` can remain more permissive for development, but should not pull
  runtime cache details into the canonical plan debug surface.
- When adding a field, classify it first:
  - planner output review surface
  - runtime cache/detail
  - temporary lowering state

Runtime cache/detail fields should not leak into expected-plan comparison unless
that is an intentional review surface.

## Function Value Profile State

The current profile intentionally supports only current-module top-level function
references as values, function-valued local aliases, function-typed arguments,
and calls through those aliases or arguments.

Function value aliases are runtime-backed local values:

- Primitive locals and function aliases share one planner binding map, so Gleam
  shadowing semantics are preserved.
- `let f = add_one` emits a typed function-local step and stores the function
  value in the runtime frame.
- Calls through function aliases read a typed function value from the runtime
  frame before invoking the resolved runtime function.
- Function-typed arguments can receive supported top-level function values and
  function-valued locals, and can be called through the validated function type.
- Function values store exact private callee `ParamLocal` slots and derive the
  public `FunctionType` signature from those slots.
- Function-value call lowering validates argument expressions against the
  derived signature, while runtime binding uses the evaluated `FunctionValue`
  params as the source of truth.

Later function-value profile work:

- Anonymous functions.
- Capturing closures.
- Function-value pipelines.
- Imported or external function values.
- Rust-side invocation of returned `Value::Function`.

Function-returning function direction:

- Recursive `FunctionType` metadata is already represented with boxed return
  types.
- A recursive `FunctionType` is only metadata. It does not by itself make the
  executable plan shape panic-free because runtime still needs a concrete return
  family to select the typed function table.
- Function-returning execution should allow runtime tagged dispatch only as
  validated routing. Runtime must not turn unexpected function tags into
  structural runtime errors.
- The planner must construct call sites with known return shape. If a mismatch
  can be reached, it belongs at the planner boundary as `PlanError`.
- Runtime implementation should keep using typed expression families where
  possible. Any remaining projection or tag-routing point needs explicit review
  to confirm it is routing over a planner-validated shape, not validation moved
  into runtime.
- Removing the remaining projection requires the plan/local/frame/runtime value
  families to carry the nested function return family, not just the public
  `FunctionType` metadata.

Labelled argument note:

- Gleam v1.17.0 accepts labelled calls when the callee declares labelled
  parameters, for example `fn add(left x: Int) { ... }` called as
  `add(left: 1)`.
- Geam currently rejects labelled function parameters before labelled call
  lowering is reachable.
- When labelled function parameters enter the profile, labelled call-site
  arguments must be reclassified from typed-AST margin handling to a real
  profile boundary, or implemented with Gleam-compatible labelled argument
  semantics.

Current early-stage API principle:

- Public API compatibility is not a priority at this pre-public stage. Prefer
  stronger internal plan shape over preserving current accessor signatures.

Runtime id table note:

- Planner-assigned runtime ids are now carried by `ReturnExpr`, and runtime
  tables should treat those ids as the source of truth.
- `RuntimePlan` may sort functions into typed runtime tables by those ids, but
  hand-built internal `ExecutionPlan::new` tests can still create duplicate or
  sparse ids if they bypass the planner/DSL allocator.
- If direct plan construction starts making those invalid shapes likely, prefer
  routing tests through the DSL allocator or adding a crate-internal plan
  fixture builder rather than reintroducing runtime id allocation rules in
  `RuntimePlan`.
