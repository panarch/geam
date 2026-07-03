# Upstream Gleam Notes

Recorded on 2026-06-20. Updated after the project moved from a Geam-owned
parser prototype to using Gleam's typed AST directly.

## Official Baseline

Geam's current compiler-boundary reference is the official Gleam release:

```text
repository: https://github.com/gleam-lang/gleam
release:    v1.17.0
commit:     afc1b7d956b433e638d52dbd06470f53a0b26f6a
published:  2026-06-02
```

The local Gleam checkout used for investigation is:

```text
/Users/taehoon/Workspace/rust/gleam
```

It has been checked out to detached HEAD at `v1.17.0`.

## Why Release-Based

The prior local checkout was `main` at:

```text
28e51ca0cf733fd62f3aea3d00844709f049f2c6
```

That was `v1.17.0-111-g28e51ca0c`, so it included 111 commits beyond the latest
official release. For Geam's compiler boundary, using `main` would make typed
AST and analyse behavior drift with unreleased Gleam changes.

Use the release tag and exact commit hash when comparing typed AST shape,
analyse behavior, compiler support APIs, and backend semantics.

## Sync Policy

When moving to a newer Gleam baseline, record:

```text
old release tag
old commit
new release tag
new commit
compiler-boundary files compared
intentional Geam differences
accepted fixture changes
rejected fixture changes
license notice changes, if any
```

README and `docs/upstream-gleam.md` should always contain the current official
upstream baseline. `.notes` can contain operational investigation and migration
details.

## Dependency And Publish Notes

Gleam is a Rust workspace with internal crates such as `gleam-core`,
`gleam-cli`, `gleam-language-server`, and `hexpm`, but `gleam-core` is not
currently published as a crates.io library dependency.

This creates two separate constraints for Geam:

```text
GitHub/public repository usability:
  A pinned Git dependency can make the repository cloneable and buildable
  without requiring a sibling local Gleam checkout.

crates.io publishability:
  A normal dependency must resolve through crates.io. A `git` or `path`
  dependency on `gleam-core` is not enough for publishing Geam as a crate.
```

So a pinned Git dependency is useful for public development before crates.io
publishing, but it is not a final publish strategy. Publishing Geam while it
depends on Gleam's compiler frontend requires either upstream `gleam-core`
publication, an approved vendoring/forking strategy, or another official
compiler-boundary strategy from the Gleam project.

## Target And External Notes

Gleam core analysis currently requires choosing one upstream backend target:

```text
Target::Erlang
Target::JavaScript
```

There is no upstream `Target::Geam` or target-neutral analyse mode in the
current baseline. Geam's current `frontend::compile_typed_module` uses:

```text
target: Target::Erlang
target_support: TargetSupport::Enforced
```

This is a frontend compatibility choice, not a runtime choice:

```text
Gleam frontend analyse target: Erlang
Geam execution target:          Geam ExecutionPlan/runtime
```

A module that passes Gleam's Erlang target analysis is not automatically
executable by Geam. Erlang externals, target-specific definitions, and stdlib
functions backed by Erlang/JavaScript externals still need a Geam planner or
host-binding policy.

Initial policy direction:

- Keep using `Target::Erlang` for Gleam analysis until there is a stronger
  upstream-supported option.
- Reject external-only functions at the Geam planner boundary unless Geam has a
  builtin or host binding for them.
- Treat third-party Gleam packages with Erlang/JavaScript externals as
  potentially non-executable even if Gleam analysis accepts them for the chosen
  target.
- Revisit this if upstream Gleam exposes a target-neutral frontend mode or a
  Geam-compatible target boundary.

## Current Files To Watch

```text
compiler-core/src/ast.rs
compiler-core/src/ast/untyped.rs
compiler-core/src/ast/typed.rs
compiler-core/src/parse.rs
compiler-core/src/parse/error.rs
compiler-core/src/analyse.rs
compiler-core/src/type_.rs
compiler-core/src/build.rs
compiler-core/src/config.rs
compiler-core/src/line_numbers.rs
compiler-core/src/uid.rs
compiler-core/src/warning.rs
```

Geam should not fork Gleam's parser or type inferencer in the current direction.
The active boundary is:

```text
source text -> Gleam TypedModule -> Geam ExecutionPlan -> Geam runtime Value
```

When adding new executable features, inspect how Gleam represents them in
`TypedExpr`, typed statements, typed patterns, and `type_::Type`. If runtime
semantics are target-normalized by Gleam, compare the Erlang and JavaScript
backends before implementing Geam behavior.
