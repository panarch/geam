# Testing

Geam uses Rust unit tests for compiler-boundary, lowering, and runtime
milestones.

The current compiler-boundary and runtime milestones depend on a local Gleam
checkout at the baseline recorded in the README. `cargo test` compiles the
`gleam-core` path dependency as part of the normal suite.

Source-level execution tests live under `tests/fixtures/execution/*.gleam`.
Each fixture must end with a `// geam:expect ...` line, for example:

```gleam
pub fn main() {
  1 + 2
}

// geam:expect Int(3)
```

The integration runner reads those fixtures through the public Geam API:
`compile_typed_module -> plan_module -> run_main`.

## Commands

Run the normal test suite:

```sh
cargo test
```

Planner unit tests use `planner::dsl` expected plans instead of snapshots, so
supported lowering changes should update the expected plan directly next to the
source being tested.

Planner test names should make the source of the case clear:

- `plan_*`: supported lowering from Gleam source into a Geam `ModulePlan`.
- `reject_profile_*`: valid Gleam source that Geam's current execution profile
  intentionally rejects.
- `reject_margin_*`: synthetic typed AST margin cases that are difficult or
  impossible to express as ordinary source fixtures, but still need explicit
  planner behavior.

Prefer source-backed tests for ordinary supported and unsupported language
features. Use synthetic typed AST construction only when the test is covering
Gleam typed-AST margin, defensive planner behavior, or an internal compiler
shape that cannot be produced cleanly from a small source example.

Runtime unit tests use inline Gleam source strings for valid execution behavior
and reserve direct internal tests for runtime-only units such as local frames.
Fixture files are kept for integration tests that document the public execution
pipeline.

Run formatting and lint checks:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

## Line Coverage

Geam uses `cargo-llvm-cov` for local line coverage. It is LLVM-based, works well
on macOS, and keeps generated reports under `target/`.

Install the local tools:

```sh
rustup component add llvm-tools-preview
cargo install cargo-llvm-cov --locked
```

Print a line coverage summary:

```sh
cargo llvm-cov --summary-only
```

Generate an HTML report:

```sh
cargo llvm-cov --html
```

The HTML report is written to:

```text
target/llvm-cov/html/index.html
```

Use the summary and HTML report to identify Gleam boundary wrapper, planner, and
runtime paths that still need direct tests. Coverage gaps should be treated as
work unless they are clearly test-only guard paths used to assert fixture or
typed-AST shape. If those guard paths become noisy, move them into small tested
helpers rather than scattering inline `panic!` guards through planner tests.
