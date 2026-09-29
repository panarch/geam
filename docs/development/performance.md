# Performance Improvement

This guide describes how to choose performance work, investigate costs, and
decide whether to keep a change. Start with algorithms, data structures,
ownership, and repeated work. Inspect generated code when a specific unresolved
bottleneck warrants it.

These are investigation practices. [Testing](testing.md) and
[review policy](review-policy.md) define the required verification and acceptance
rules. The [benchmark guide](https://github.com/panarch/geam/blob/main/benchmarks/README.md)
documents preparation, measurement, and evidence formats.

## Start With Structural Costs

Trace the work performed by the target operation and how it scales with input
size, nesting, and repetition. Look for:

- Repeated scans or calculations that can be eliminated or shared.
- Intermediate values constructed only to be inspected or immediately consumed.
- Copies and allocations caused by ownership or representation boundaries.
- Setup and dispatch paid by every operation for behavior only some paths use.

Describe the work removed and the costs introduced. Reusing storage can remove
allocations while adding per-call access and retaining capacity until execution
ends. Moving work out of a loop can increase setup or memory costs for small
inputs. Include these effects in the design before choosing comparison cases.

Separate a visible cost from its contribution to elapsed time. An allocation
on every call is a fact; it being the dominant cost is a hypothesis. Source
counts can justify a focused experiment. Profiling or targeted observation can
help prioritize uncertain candidates without requiring assembly analysis first.

A disappointing result is a reason to revisit the structural trade-off. It does
not automatically make compiler layout or inlining the next investigation.
Prefer a clear opportunity to remove work over repeatedly tuning the generated
code of an unsuccessful design.

## Bound Each Experiment

Before editing, record enough to answer:

- What cost has been observed, and what remains a hypothesis?
- What single question will this change answer?
- What observation would support or reject that explanation?
- What time or candidate-build limit bounds the experiment, including its
  measurement and verification costs?

For example, when removing a repeated copy, verify that the copy is gone and
measure workloads that exercise it. Fewer copies establish the mechanism;
elapsed time establishes whether the replacement is useful. If time increases,
investigate the new work before assuming the removed copy was expensive enough
to justify further tuning.

Keep changes that test different explanations separate. Preserve the baseline
and rejected results so a later experiment has an interpretable comparison.
Use the benchmark records to identify the source, build, executable, and
measurement conditions; a label such as "before" is insufficient.

## Expand Validation In Stages

1. Review the affected contracts and run focused correctness tests. Check exact
   results and relevant error, ownership, and lifetime behavior. Use
   [test development](test-development.md) to choose the owner and scenario.
   A faster computation with changed behavior is not a valid candidate.
2. Compare a bounded set of representative workloads using the actual release
   execution path. Choose target cases and controls before seeing results.
   Include input sizes or shapes that expose the proposed trade-off. A shared
   runtime change needs controls beyond the operation it aims to improve.
   Smoke runs establish execution correctness, not a performance benefit.
3. If the candidate remains useful, expand to the full performance comparison
   and the required owner, consumer, coverage, and quality checks. Preserve
   individual regressions and memory costs when judging the result. A favorable
   subset does not establish that a shared runtime change is ready to adopt.

Use comparable builds and measurement conditions as described in the benchmark
guide. Do not profile, compile, or run tests on the measurement host during
timing. Diagnostic instrumentation and timings remain separate evidence.

Reject an unsuccessful candidate before repeatedly paying for full acceptance
and coverage runs. Reuse completed checks whose source and relevant conditions
remain valid; refresh affected evidence after corrections. Staging changes
when checks run, not the final verification required for an accepted change.

## When To Inspect Generated Code

First trace a regression through the changed source: extra work in a common
path, repeated conversions or dispatch, changed lifetimes, initialization, and
retained storage. Use affected and unaffected cases to narrow the explanation.
A source-level comparison or a small observation of the suspected owner may
answer the question without examining instruction sequences.

Inspect assembly or compiler output when all of the following are clear:

- Execution evidence identifies an important path with an unresolved cost.
- Its algorithm, ownership, and data movement have been reviewed, and a specific
  question remains that generated code can answer.
- The answer can change the implementation or the decision to keep it.

State that question and bound the inspection to the relevant path. For example,
check whether a suspected copy remains in the optimized loop. Inspect the
actual measured release artifact and its compiler settings. Debug layout,
total static instruction counts, and smaller types do not establish the cost
of an executed path.

Do not cycle through representation changes, inline hints, or compiler flags
solely because a previous candidate was slower. Finding a code-generation
difference does not by itself show that it caused the regression. Retain that
uncertainty unless further evidence distinguishes the explanation.

## Stop When The Evidence Stops Improving

Stop or change direction when the hypothesis is rejected, attempts no longer
produce distinguishing observations, or the investigation budget is spent.
Extend an investigation for a concrete new question supported by what was
learned. The possibility of another implementation variant is not enough.

When results are ambiguous, decide what additional observation could resolve
them before repeating a measurement. Preserve the original run and choose the
recheck scope in advance. Do not repeat until a favorable result appears or
discard slower cases and rounds.

A failed design can be set aside without explaining every timing difference
at the instruction level. Preserve its source and evidence, record whether the
result is rejected or unresolved, and return to the baseline or another
structural candidate. Functional correctness, removal of a particular cost,
and overall performance improvement are separate conclusions.

Keep a short task record of the observation, hypothesis, experiment, result,
and next decision. It should prevent repeating a rejected idea and make any
remaining uncertainty visible. Individual experiments and measurements belong
in those records rather than this general guide.
