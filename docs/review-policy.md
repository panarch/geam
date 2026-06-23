# Review Policy

Geam is built to keep the executable boundary easy to review by both people and
AI agents. Use explicit structure over clever compression. Keep each change
small enough to review the behavior, tests, and error boundary together.

## Boundary Rules

Gleam remains the source language. Geam starts after Gleam has produced a typed
AST:

```text
Gleam source -> Gleam TypedModule -> Geam ExecutionPlan -> Geam runtime Value
```

The planner is the Geam profile validation boundary. Unsupported Gleam semantics
must be rejected while lowering into `ExecutionPlan`, not deferred to runtime.

Use these categories consistently:

- **Profile**: valid Gleam source that reaches the planner, but is outside the
  current Geam execution profile.
- **Margin**: direct or mutated Gleam typed AST shapes that cannot be produced
  by compiling valid Gleam source through Gleam's frontend, but still need
  explicit planner behavior.

Runtime code assumes it receives a valid `ExecutionPlan`. Structural execution
failures belong in plan construction as `PlanError`, not in a runtime error
enum.

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
profile validation, or invariant handling. Do not use `panic!`, `unreachable!`,
`unwrap`, or `expect` in non-test logic code.

Boundary failures must become structured errors before runtime execution. If a
case can be reached from valid Gleam source, reject it as a profile error. If it
requires a synthetic or mutated typed AST shape, reject it as `InvalidTypedAst`.

`#[cfg(test)]` helpers may use panic paths only to assert fixture shape. Keep
those panics local, visible, and covered by explicit panic tests.

## Error Rules

Planner errors make the boundary visible:

- Use `Unsupported*` errors for valid Gleam source outside the Geam profile.
- Use `InvalidTypedAst` errors for typed AST margin cases.
- Avoid free-form static string reasons for stable planner errors.
- Keep dynamic values, such as function and local names, as structured fields.
- Do not merge unsupported profile cases and invalid typed AST cases into one
  catch-all error.

## Test Rules

Planner test names must identify the source of the case:

- `plan_*`: supported lowering from Gleam source into a Geam `ExecutionPlan`.
- `reject_profile_*`: valid Gleam source that Geam's current execution profile
  intentionally rejects.
- `reject_margin_*`: synthetic typed AST margin cases.

`reject_profile_*` tests are source-based. `reject_margin_*` tests are direct
typed-AST based and expect `InvalidTypedAst`.

Unit tests live next to the module they validate. Do not use large detached
`tests.rs` files for complex modules. Integration tests document the public
execution pipeline from source fixture to runtime value.

## Helper And DSL Rules

Test helpers reduce real repetition without hiding the reviewed shape.

- Avoid single-use shallow wrapper helpers.
- Name helpers after their fixture role, not their implementation detail.
- Keep helpers in the nearest module that uses them.
- Use the crate-internal `planner::dsl` helpers for readable expected plans.
- Keep test-only panic guards visible and covered rather than scattering
  untested inline `panic!` branches.

## Coverage Rules

Geam keeps line coverage at 100%.

Coverage gaps are work. When guard paths are needed for fixture or typed-AST
shape assertions, cover them with a small repeatable pattern.

Coverage is a review aid, not a replacement for readable tests. A covered test
that hides the behavior is still not acceptable.
