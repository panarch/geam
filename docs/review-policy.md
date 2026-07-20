# Review Policy

Geam is built to keep the executable boundary easy to review by both people and
AI agents. Use explicit structure over clever compression. Keep each change
small enough to review the behavior, tests, and error boundary together.

## Boundary Rules

Gleam remains the source language. Geam starts after Gleam has produced a typed
AST:

```text
Gleam source -> Gleam TypedModule -> Geam ModulePlan -> Geam ExecutionPlan -> Geam runtime Value
```

The planner is the Geam profile validation boundary. Unsupported Gleam semantics
must be rejected while lowering into `ModulePlan`, not deferred to execution
lowering or runtime.

Use these categories consistently:

- **Profile**: valid Gleam source that reaches the planner, but is outside the
  current Geam execution profile.
- **Margin**: direct or mutated Gleam typed AST shapes that cannot be produced
  by compiling valid Gleam source through Gleam's frontend, but still need
  explicit planner behavior.

`ModulePlan` is the canonical inspectable planner output. Converting it with
`ExecutionPlan::from_module_plan` is an infallible, consuming ownership
lowering, not another validation boundary. Runtime code assumes it receives a
valid `ExecutionPlan`. Structural execution failures belong in ModulePlan
planning as `PlanError`, not in execution lowering or a runtime error enum.

The two plan layers own independent executable node families. `ModulePlan`
owns canonical expressions, steps, returns, arguments, captures, ids, and frame
layouts for planner review. `ExecutionPlan` owns runtime-only equivalents.
Production runtime code must not import module-plan nodes, and execution node
definitions must not import them outside the consuming lowering modules.
Source spans/sites and immutable value/function type metadata are the narrow
shared domains; runtime `Value` and evaluated captures are not plan data.

`ExecutionError` separates two allowed domains:

- Source-reachable execution stops accepted by the Geam profile use
  `ExecutionError::Panic(Panic)`, with `PanicKind` as the source-level tag.
  Cover them with execution-error fixtures that compile and plan successfully
  before failing at runtime. Do not add speculative `PanicKind` variants.
- Runtime invariant failures that Rust cannot encode in the current plan shape
  use `ExecutionError::Invariant(InvariantError)`. Adding an invariant
  kind requires explicit design review.

Runtime tag dispatch is allowed only for planner-validated recursive plan
shapes, and only as execution routing. It must not become validation, fallback
behavior, or a source-visible semantic difference.

The following execution invariants are approved:

- `InvariantError::FunctionReturnFamilyMismatch` is only for a typed
  function call or return path whose evaluated target cannot serve the
  planner-selected function family.
- `InvariantError::TupleIndexFamilyMismatch` is only for typed
  tuple-index plan evaluation when the runtime tuple value lacks the
  planner-selected element or that element has a different value family.
- `InvariantError::ListIndexOutOfBounds` is only for typed list-index
  plan evaluation when the runtime list value lacks the planner-selected
  element. Source-reachable list matching must guard list-index projections
  with the planner-selected length condition.
- `InvariantError::CustomFieldFamilyMismatch` is only for a
  planner-selected custom field projection or binding whose runtime field has
  a different exact value type.

Planner-established type, shape, and discriminant relationships must be
preserved structurally through lowering. Runtime must not revalidate them
unless this policy explicitly lists the boundary as an approved execution
invariant. Source-level refutable matches remain normal control flow, not
execution invariant failures.

List item-family identity must be preserved by the execution list type graph,
family-specific frame/function boundaries, and RC-backed typed runtime handles.
It must not be recovered through runtime family checks or represented as an
execution error. Runtime list storage uses reference counting rather than a
tracing collector; features that can create cyclic evaluated value graphs stay
outside the profile until they have a separate ownership design.
Profile boundaries, list-match length guards, and typed-AST margins remain
planner responsibilities.

## Ownership Rules

Determine ownership from construction, mutation, lifetime, dependencies, and
actual callers before assigning a semantic role.

- Phase-local builders, interners, and accumulators belong to that phase.
- Data that survives a phase belongs to the downstream domain that stores and
  reads it. Final models must not depend on transformation-only types.
- Module paths and visibility must match actual production callers. Do not move
  phase-local types across boundaries merely to bypass privacy.
- Treat a one-caller type that mirrors a final type and is immediately consumed
  as a design smell. Make it a real final substructure or keep it phase-local
  behind a narrow constructor.
- A plausible role description cannot justify structure that conflicts with
  these facts. File size justifies a module split, not an ownership change.

## Plan Construction Rules

Plan construction is not a validation layer. Reaching a `ModulePlan` or plan
node constructor means the planner has already accepted a runtime-executable
shape. Reaching `ExecutionPlan` means the accepted ModulePlan has been consumed
into execution-owned nodes and function tables.

Generic function and constant templates are validated once in `ModulePlan`.
Execution lowering must remain total from the validated template and publish a
closed executable plan in which every retained reference has a matching runtime
entry. It must not re-plan a body, interpret a template at runtime, or recover a
bare type parameter through a generic runtime payload or downcast. A successful
runtime value may preserve parameter metadata only when its payload is
representable without fabricating a parameter value. Constant evaluation
strategy must preserve source evaluation order, control flow, and function
identity.

Do not use `Option` or `Result` in internal plan constructors to represent
unsupported profile features, typed-AST margin cases, or runtime executability
checks. Reject those cases before constructing plan data.

`ModulePlan` and `ExecutionPlan` shapes must not contain state for features that
are outside the current Geam profile. If a feature is profile-out, its storage,
ids, frame slots, and executable variants must also stay out unless they are
required by an accepted source path.

Treat over-wide execution plan state as a blocking design issue, even when no
current source fixture executes incorrectly. The plan model is the validation
boundary; unused executable shape creates future margin and review ambiguity.

When adding or changing fields on a type with custom `Debug`, `PartialEq`,
`Eq`, `Hash`, or ordering implementations, update or explicitly justify every
affected implementation. Only derived/cache fields may be omitted, and owning
unit tests must prove both included fields and intentionally omitted fields.

## Gleam Compatibility Rules

For any Gleam source that Geam accepts, observable runtime behavior must match
Gleam semantics.

If Gleam defines target-independent behavior, Geam follows that behavior. If
Gleam normalizes backend differences, Geam follows the normalized Gleam behavior
rather than Rust's default behavior.

The planner may reject valid Gleam source as outside the current Geam profile,
but it must not accept source and then execute it with different semantics.

Preserve Gleam evaluation semantics, including expression evaluation order,
short-circuit behavior, case clause ordering, first-match behavior, shadowing,
and block scope.

When Gleam's typed AST already encodes an invariant, Geam should preserve that
shape instead of weakening it into a wider internal representation.

Target-specific externals or backend-dependent behavior must be rejected until
Geam has an explicit compatibility rule for that surface.

## Panic Rules

Production Geam logic must not use explicit panic paths for control flow,
profile validation, or recoverable invariant handling. Do not use
`panic!`, `unreachable!`, `unwrap`, or `expect` in non-test logic code.

Boundary failures must become structured errors before runtime execution. If a
case can be reached from valid Gleam source, reject it as a profile error. If it
requires a synthetic or mutated typed AST shape, reject it as `InvalidTypedAst`.

`#[cfg(test)]` helpers may use panic paths only to assert fixture shape. Keep
those panics local, visible, and covered by explicit panic tests.

## Error Rules

Errors make boundaries visible:

- Use `Unsupported*` errors for valid Gleam source outside the Geam profile.
- Use `InvalidTypedAst` errors for typed AST margin cases.
- Avoid free-form static string reasons for stable planner errors.
- Keep dynamic values, such as function and local names, as structured fields.
- Do not merge unsupported profile cases and invalid typed AST cases into one
  catch-all error.
- Stable error variants should represent one boundary condition. Do not use one
  variant for multiple distinct profile, typed-AST, host, or runtime boundaries
  merely because the broad feature family is similar.
- Stable error variants must correspond to a reachable production boundary.
  Test-only references do not justify keeping an error variant.
- When feature scope changes, re-audit newly added error variants and remove any
  that are only test-referenced or speculative.
- Use `Option` for lookups or partial conversions when the caller owns the
  boundary meaning of absence. If all callers translate `None` into the same
  failure boundary, return `Result` or a structured reason instead.
- Do not erase a boundary-carrying `Result` into `Option` with `.ok()`. If the
  caller needs a failure boundary, propagate a structured `Result`; if failure
  should be impossible after prior typed matching, make the construction
  infallible by shape instead.
- Keep fallibility meaningful: an internal helper may return `Result` only when
  it creates or propagates a real failure boundary.
- When the accepted profile grows into an area that was previously rejected by a
  broad `Unsupported*` variant, revisit that variant. Rename or split it if the
  old name also describes behavior that is now supported.

## Import Rules

Do not use wildcard imports or re-exports anywhere. The only exception is a
parent facade module re-exporting child modules as its intentional surface.

## Module Split Rules

Child modules must not become shared utility surfaces for sibling modules. If
sibling modules need a helper, keep it in the parent facade or in a child module
that owns that helper's domain.

## Test Rules

Planner test names must identify the source of the case:

- `plan_*`: supported lowering from Gleam source into a Geam `ModulePlan`.
- `reject_profile_*`: valid Gleam source that Geam's current execution profile
  intentionally rejects.
- `reject_margin_*`: synthetic typed AST margin cases.

`reject_profile_*` tests are source-based. `reject_margin_*` tests are direct
typed-AST based and expect `InvalidTypedAst`.

Planner accept tests should compare the expected `ModulePlan` whenever the
plan shape is meaningful. Avoid `is_ok()` for planner accept coverage unless the
exact shape is irrelevant to the test.

When result-typed plan shape is the point of a planner test, include that shape
in the test name. Prefer explicit names such as `function_valued_block` over
short names that hide the reviewed result family.

Unit tests live next to the module they validate. Do not use large detached
`tests.rs` files for complex modules. Integration tests are fixture-based and
document accepted Gleam source running through the public execution pipeline to
a runtime value.

Planner rejection coverage belongs in the owning planner unit test unless it is
represented by a dedicated fixture-based integration case.

Only add public planner API integration tests when the public boundary itself is
the reviewed behavior, not to cover planner implementation branches that belong
to an owning planner unit test.

## Helper And DSL Rules

Test helpers may reduce real repetition and fixture setup, but they must not
hide the reviewed shape or create a second constructor API for plan/runtime
shapes.

- Avoid single-use shallow wrapper helpers.
- Name helpers after their fixture role, not their implementation detail.
- Keep helpers in the nearest module that uses them.
- Do not layer test-only constructor helpers. A test-only helper that builds a
  plan/runtime value must construct the reviewed shape directly from production
  constructors, not by calling another test-only helper that supplies hidden
  defaults, sentinel values, dummy ids, empty spans, or unknown sites.
- Use the crate-internal `planner::dsl` helpers for readable expected plans.
- Expected-plan DSL is an oracle, not a lowering path: helpers may reduce
  constructor noise, but must not infer, hide, or call planner lowering for the
  behavior under test.
- Keep test-only panic guards visible and covered rather than scattering
  untested inline `panic!` branches.

## Coverage Rules

Geam keeps full-scope region coverage and line coverage at 100%.

Coverage gaps are work. When guard paths are needed for fixture or typed-AST
shape assertions, cover them with a small repeatable pattern.

Coverage is a review aid, not a replacement for readable tests. A covered test
that hides the behavior is still not acceptable.
