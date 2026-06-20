# Upstream Gleam Reference

Geam's first front-end milestone is intentionally close to Gleam, but it is not a
copy of the full Gleam compiler front-end. This document records the upstream
baseline, what was referenced or adapted, and what was intentionally left out.

## Baseline

Geam's initial front-end reference is:

```text
Repository: https://github.com/gleam-lang/gleam
Release:    v1.17.0
Commit:     afc1b7d956b433e638d52dbd06470f53a0b26f6a
Published:  2026-06-02
```

The baseline is release-based rather than `main`-based so parser, AST, and test
comparisons are made against a published Gleam toolchain.

## Referenced Upstream Areas

The initial Geam parser milestone referenced these Gleam front-end areas:

```text
compiler-core/src/ast.rs
compiler-core/src/ast/untyped.rs
compiler-core/src/ast/typed.rs
compiler-core/src/ast/constant.rs
compiler-core/src/parse.rs
compiler-core/src/parse/lexer.rs
compiler-core/src/parse/token.rs
compiler-core/src/parse/error.rs
compiler-core/src/parse/tests.rs
compiler-core/src/parse/snapshots/
```

The most important upstream ideas are:

- A generic AST shape that can represent phase-specific data without storing
  untyped and typed information in the same node.
- A hand-written parser with expression precedence handling.
- Source spans on AST nodes.
- Snapshot-oriented parser tests.

## Adapted In Geam

Geam currently adapts the shape and intent of Gleam's front-end rather than the
entire implementation.

- Geam defines its own untyped AST instead of importing the full Gleam AST.
- Geam owns its lexer, token model, parser, and parse error types while keeping
  their shape close enough to compare against the referenced Gleam front-end.
- Parser and lexer behavior is locked with unit tests and `insta` snapshots.

This document intentionally avoids mirroring Geam's current internal file tree.
Use the repository itself as the source of truth for exact module and test file
locations.

The public parser API is intentionally small:

```rust
pub fn parse_module(
    path: camino::Utf8PathBuf,
    src: &str,
) -> Result<geam::ast::UntypedModule, geam::parse::ParseError>
```

## Intentionally Not Imported

Geam does not import or preserve the full Gleam compiler front-end. These areas
are intentionally excluded from the first milestone:

- Typed AST internals and type inference data.
- LSP node lookup helpers.
- Erlang and JavaScript target metadata.
- Code generation metadata.
- Compiler-specific structures such as `Inferred`, `Purity`, `Implementations`,
  `Names`, and unused-definition tracking.
- Module constants and constant evaluation.
- Full Gleam parser acceptance followed by a later reject pass.
- Core IR, runtime execution, host module resolution, and CLI behavior.

The first parser milestone rejects unsupported syntax at the parser boundary
where practical. Later semantic/profile checks may refine this boundary, but
Geam should not silently accept the full Gleam language as its untyped AST.

## Current Language Boundary

The first Geam parser accepts:

- Imports, functions, custom types, and type aliases.
- Expression statements and `let` assignment.
- Literals, variables, lists, tuples, calls, binary operators, pipelines, case
  expressions, field access, tuple index, and boolean/int negation.
- Basic patterns, constructor patterns, tuple/list patterns, string-prefix
  patterns, alias patterns, alternative case patterns, and syntactic case guards.

It rejects:

- Module constants, attributes, `external`, `target`, and `opaque`.
- `use`, `assert`, and `let assert`.
- Anonymous function literals.
- `todo`, `panic`, and `echo`.
- Bit arrays, record update, and list tail/spread syntax.

## Sync Policy

When updating Geam to a newer Gleam baseline, record:

- Old and new Gleam release tags.
- Old and new commit hashes.
- Front-end files compared.
- Geam AST/parser changes made.
- Accept/reject fixture changes.
- License or provenance changes, if any.

The README should keep the current upstream release and commit hash visible.
This document should explain the practical differences between Geam and the
referenced Gleam version.
