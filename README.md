# Geam

Geam is a Rust-embedded runtime and toolchain layer for a small Gleam-compatible
profile. It aims to reuse the Gleam authoring experience while accepting only
syntax and semantics that can be lowered into Geam's own runtime representation.

## Upstream Gleam Reference

Geam's initial front-end design references the latest official Gleam release at
the time the project baseline was recorded:

- Repository: https://github.com/gleam-lang/gleam
- Release: `v1.17.0`
- Commit: `afc1b7d956b433e638d52dbd06470f53a0b26f6a`
- Release date: 2026-06-02

This baseline is intentionally release-based rather than `main`-based so AST,
parser, and test comparisons are made against a published Gleam toolchain.
Future upstream sync work should record both the old and new release tag and
exact commit hash.

If Geam copies or adapts Gleam source files, preserve the applicable upstream
license notices and document the intentional differences.

See [docs/upstream-gleam.md](docs/upstream-gleam.md) for the tracked upstream
reference, adapted areas, excluded compiler internals, and sync policy.

See [docs/testing.md](docs/testing.md) for test, snapshot, and line coverage
commands.

## Front-End Direction

Geam should not parse full Gleam and then reject unsupported programs as a later
runtime concern. The front-end should define a Geam subset AST and parser shaped
close to Gleam's AST/parser structure, then reject unsupported syntax at the
syntax/profile boundary.

The initial implementation target is:

```text
source text
-> Geam lexer/token
-> Geam subset parser
-> Geam source AST
```

Core IR and execution should be designed after the source AST and parser boundary
is stable enough to test against selected Gleam parser fixtures.
