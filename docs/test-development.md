# Test Development

This guide collects practical ways to construct tests while implementing Geam
behavior and to diagnose difficult coverage gaps. For suite layout and commands,
see [testing.md](testing.md). For acceptance rules, see
[review-policy.md](review-policy.md).

These are working heuristics rather than additional policy. Coverage is both a
diagnostic signal and an acceptance gate, but it does not decide the production
design or make an otherwise unclear test useful.

## Test Roles

Tests are usually easiest to understand when each has one primary role.

- An owner unit test proves one protocol, branch, formatter grammar, typed
  relation, or lifecycle beside the module that owns it.
- A source-backed owner example keeps complete Gleam source and its exact result
  beside an owner that must be reached through production compilation or
  lowering.
- A fixture integration test documents source-visible behavior through the
  complete public pipeline.
- A compile-fail test fixes a public Rust type-system restriction that has no
  runtime representation.

Integration coverage may execute an owner without documenting its exact
contract. Conversely, a narrow owner probe may answer a reachability question
without replacing a readable source example.

## Choose The Strategy

An uncovered path does not always mean that another test is missing. A coverage
gap can point to different kinds of work:

- A public behavior gap is best expressed as a complete source example or
  fixture.
- A structural gap appears when an owner represents impossible states, repeats
  the same decision across family wrappers, or instantiates unrelated generic
  paths. Clarifying ownership may remove the gap without adding a test.
- A reachability gap remains when the structure looks sound but it is unclear
  which concrete branch or monomorph an existing scenario exercises. A small
  owner probe can answer that question directly.

These strategies are complementary rather than sequential. Obvious structural
findings are worth addressing before searching for more fixtures, while a probe
can help determine whether a suspected branch is valid or merely
representational.

One way to assess a structural change is to ask whether it remains useful
without its coverage effect. Good signals include a narrower phase contract,
one clear correctness owner, removal of an impossible variant, or elimination
of duplicated dispatch. A miss that only moves into a new abstraction has not
been structurally resolved.

Refreshing the report after a structural correction reveals whether exact
reachability gaps remain. Those residual gaps are good candidates for an
owner-outward probe.

## Diagnose And Probe Coverage

Use the detailed coverage report described in [testing.md](testing.md) to
identify:

- the uncovered file and owner;
- the exact line and region;
- the generic instantiation, when present;
- the missing branch outcome;
- the test binary or target that instantiated it.

Line, region, and instantiation coverage answer different questions. A line can
run while one expression outcome remains uncovered, and the same source line
can run through a different generic monomorph from the one in the report. New
closures, helpers, return-family wrappers, and test binaries can each create
another generated copy instead of covering the intended one.

For typed generic paths, source spelling may not reveal the concrete owner.
Compiler visitation, scheme-local parameter numbering, parameter layout, and
specialized return shape can all affect the instantiated function. Tracing those
values before changing a source case often prevents a nearby but irrelevant
path from being exercised.

A useful bottom-up loop is:

1. Identify the function, branch, type relation, or monomorph that owns the gap.
2. Exercise it with the smallest owner-local unit or temporary probe that can
   establish reachability.
3. Run focused coverage and confirm that the exact target moved.
4. If the state is valid, connect it outward to the nearest meaningful owner or
   source-backed behavior.
5. Remove the diagnostic probe once the retained test records the same path.

The initial probe may have little semantic value; it provides a binary answer
about reachability. If it requires a malformed plan, an impossible runtime
state, or data rejected by an earlier phase, the representation or owner
boundary may be the actual problem.

When line coverage is complete but a region remains, assertion and closure
shapes are also worth checking. A `matches!`, `let ... else panic!`, or generic
selector can introduce an unexecuted region even when the production behavior
is covered. Comparing a complete borrowed view or exact result may state the
contract more directly.

Repeated attempts that do not move the exact target are a signal to revisit the
hypothesis, concrete instantiation, or phase representation instead of varying
more source syntax. Returning to the previous baseline after a rejected
experiment keeps the next result interpretable.

## Experiments And Scenario Locality

Source and expected behavior are easiest to review together. A focused scenario
usually needs only the declarations, providers, or operations it exercises.
Broad setup can silently instantiate unrelated generic paths and make coverage
movement difficult to interpret.

Shared helpers are useful for stable compile, lower, run, and assertion
plumbing. When a helper hides the source, expected result, selected semantic
variant, or scenario-specific traversal, inlining that setup often makes the
test owner clearer.

When an end-to-end result is wrong, inspecting the nearest intermediate owner
can narrow the fault before the scenario is expanded. Naming values by role,
such as `source_index` and `target_index`, also exposes accidental cross-owner
reads.

For long investigations, a lightweight temporary work log can prevent a failed
hypothesis from being retried after interruption or context loss. Recording the
target, hypothesis, observed result, and next decision is often enough; the form
can follow the work.

Coverage work may be pointing at the wrong layer when it starts to require
test-only production visibility, raw runtime states, fallback paths, new errors,
or generic helpers used only by tests. The corresponding acceptance boundaries
belong to [review-policy.md](review-policy.md).

A particularly clean outcome covers the exact target, retains a test that
documents a real owner contract or public behavior, and removes any purely
diagnostic probe.
