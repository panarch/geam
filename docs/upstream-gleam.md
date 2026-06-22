# Upstream Gleam Compiler Boundary

Geam does not own a separate source language front-end. It relies on Gleam's
published compiler front-end, then starts Geam-specific work at the typed AST
boundary. This document records the upstream baseline, what is used directly,
and where Geam's runtime-specific boundary begins.

## Baseline

Geam's compiler boundary reference is:

```text
Repository: https://github.com/gleam-lang/gleam
Release:    v1.17.0
Commit:     afc1b7d956b433e638d52dbd06470f53a0b26f6a
Published:  2026-06-02
```

The baseline is release-based rather than `main`-based so typed AST and
compiler-boundary behavior are compared against a published Gleam toolchain.

## Used Gleam Areas

The first compiler-boundary milestone depends on `gleam-core` from the baseline
checkout. The primary compiler areas used by Geam are:

```text
compiler-core/src/parse.rs
compiler-core/src/parse/error.rs
compiler-core/src/analyse.rs
compiler-core/src/type_.rs
compiler-core/src/ast.rs
compiler-core/src/ast/untyped.rs
compiler-core/src/ast/typed.rs
```

The boundary wrapper also uses Gleam support APIs required to run analyse in the
same shape as Gleam itself:

```text
compiler-core/src/build.rs
compiler-core/src/config.rs
compiler-core/src/line_numbers.rs
compiler-core/src/uid.rs
compiler-core/src/warning.rs
```

The Geam wrapper parses source text, inserts Gleam's prelude interface, assigns
the caller-provided module name, and runs `ModuleAnalyzerConstructor::infer_module`
to produce a Gleam `TypedModule`.

## Geam Boundary

Geam intentionally does not define a source AST or source-language compiler. Its
current boundary is:

```text
source text -> Gleam TypedModule -> Geam ModulePlan -> Geam runtime Value
```

Geam-specific profile validation belongs in the lowering phase from Gleam's typed
AST into Geam's runtime representation. That phase rejects unsupported execution
semantics before evaluation instead of accepting a program and failing inside the
runtime.

The earlier Geam-owned parser and analyse prototype has been removed. The active
direction is to rely on Gleam's typed AST and build Geam profile validation,
lowering, and execution after that boundary.

This document intentionally avoids mirroring Geam's current internal file tree.
Use the repository itself as the source of truth for exact module and test file
locations.

The current public compiler-boundary API is:

```rust
pub fn compile_typed_module(
    module_name: impl Into<ecow::EcoString>,
    path: impl Into<camino::Utf8PathBuf>,
    src: &str,
) -> Result<gleam_core::ast::TypedModule, geam::frontend::FrontendError>
```

The current public execution APIs are:

```rust
pub fn plan_module(
    module: gleam_core::ast::TypedModule,
) -> Result<geam::ModulePlan, geam::PlanError>

pub fn run_main(plan: &geam::ModulePlan) -> Result<geam::Value, geam::RuntimeError>
```

## Intentionally Out Of Scope

These areas are intentionally excluded from the first compiler-boundary
milestone:

- LSP node lookup helpers.
- Erlang and JavaScript target metadata.
- Code generation metadata.
- Project compilation, package resolution, module loading, dependency graph
  analysis, and artifact writing.
- Imports, host bindings, broader Gleam profile support, and CLI behavior.

## Current Source Boundary

Source acceptance follows Gleam `v1.17.0` parser and analyse rules. Geam's
smaller execution profile is enforced by typed-AST planning, not by forking
Gleam's parser or type inferencer.

## Sync Policy

When updating Geam to a newer Gleam baseline, record:

- Old and new Gleam release tags.
- Old and new commit hashes.
- Compiler-boundary files compared.
- Geam compiler-boundary wrapper or lowering changes made.
- Profile/lowering fixture changes.
- License or provenance changes, if any.

The README keeps the current upstream release and commit hash visible. This
document explains the practical differences between Geam and the referenced
Gleam version.
