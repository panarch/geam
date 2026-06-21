# Geam

Geam is a Rust-embedded runtime and lowering layer for a Gleam-compatible
execution profile. Gleam remains the source language; Geam starts after Gleam
has parsed and type-checked a module, then lowers the supported typed program
surface into Geam's own runtime representation.

## Upstream Gleam Reference

Geam's compiler boundary references the latest official Gleam release at the
time the project baseline was recorded:

- Repository: https://github.com/gleam-lang/gleam
- Release: `v1.17.0`
- Commit: `afc1b7d956b433e638d52dbd06470f53a0b26f6a`
- Release date: 2026-06-02

This baseline is intentionally release-based rather than `main`-based so typed
AST and compiler-boundary behavior are compared against a published Gleam
toolchain. Future upstream sync work should record both the old and new release
tag and exact commit hash.

Milestone 1 uses the `gleam-core` compiler front-end from a local checkout of
this baseline. The expected development layout is:

```text
rust/
|-- geam/
`-- gleam/    # checked out at afc1b7d956b433e638d52dbd06470f53a0b26f6a
```

If Geam later copies or adapts Gleam source files, preserve the applicable
upstream license notices and document the intentional differences.

See [docs/upstream-gleam.md](docs/upstream-gleam.md) for the tracked upstream
reference, direct compiler dependency, Geam runtime boundary, and sync policy.

See [docs/testing.md](docs/testing.md) for test and line coverage commands.

## Runtime Direction

Geam does not define a separate source language, parser, or type checker. Its
current direction is to use Gleam's parser and analyse/infer pass as the
compiler boundary. Geam-specific validation should happen while lowering from
Gleam's typed AST into Geam's runtime representation, so unsupported execution
semantics are rejected before evaluation.

The first milestone target is:

```text
source text
-> Gleam parser
-> Gleam untyped AST
-> Gleam analyse/infer
-> Gleam typed AST
```

Core IR and execution are intentionally out of scope for this milestone. The
public Geam entry point for the current compiler-boundary milestone is:

```rust
pub fn compile_typed_module(
    module_name: impl Into<ecow::EcoString>,
    path: impl Into<camino::Utf8PathBuf>,
    src: &str,
) -> Result<gleam_core::ast::TypedModule, geam::frontend::FrontendError>
```

The previous Geam-owned parser/analyse prototype has been removed. Future
Geam-specific work should start from Gleam's typed AST boundary and move toward
profile validation, lowering, and runtime execution.
