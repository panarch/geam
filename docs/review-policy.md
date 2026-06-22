# Review Policy

Geam is built to keep the executable boundary easy to review by both people and
AI agents. Use explicit structure over clever compression. Keep each change
small enough to review the behavior, tests, and error boundary together.

## Boundary Rules

Gleam remains the source language. Geam starts after Gleam has produced a typed
AST:

```text
Gleam source -> Gleam TypedModule -> Geam ModulePlan -> Geam runtime Value
```

The planner is the Geam profile validation boundary. Unsupported Gleam semantics
must be rejected while lowering into `ModulePlan`, not deferred to runtime.

Use these categories consistently:

- **Profile**: valid Gleam source that reaches the planner, but is outside the
  current Geam execution profile.
- **Margin**: direct or mutated Gleam typed AST shapes that cannot be produced
  by compiling valid Gleam source through Gleam's frontend, but still need
  explicit planner behavior.

Runtime code assumes it receives a valid `ModulePlan`. When a runtime error is
structurally unreachable, fix the plan structure instead of adding a runtime
check.

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

- `plan_*`: supported lowering from Gleam source into a Geam `ModulePlan`.
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
- Use the planner DSL for readable expected plans.
- Keep test-only panic guards visible and covered rather than scattering
  untested inline `panic!` branches.

## Coverage Rules

Geam keeps line coverage at 100%.

Coverage gaps are work. When guard paths are needed for fixture or typed-AST
shape assertions, cover them with a small repeatable pattern.

Coverage is a review aid, not a replacement for readable tests. A covered test
that hides the behavior is still not acceptable.
