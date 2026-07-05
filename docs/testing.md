# Testing

Geam uses Rust unit tests for compiler-boundary, lowering, and runtime
milestones.

The current compiler-boundary and runtime milestones depend on `gleam-core`
pinned to the upstream baseline recorded in the README. `cargo test` compiles
that Git dependency as part of the normal suite.

Source-level execution tests live under categorized
`tests/fixtures/execution/**/*.gleam` paths. Each fixture must end with a
`// geam:expect ...` line, for example:

```gleam
pub fn main() {
  1 + 2
}

// geam:expect Int(3)
```

The integration runner reads those fixtures through the public Geam API:
`compile_typed_module -> plan_module -> run_main`.

Source-level rejection fixtures live under categorized
`tests/fixtures/rejection/**/*.gleam` paths. They are reserved for public
boundary cases that are clearer as complete Gleam modules than as planner unit
tests.

## Commands

Run the normal test suite:

```sh
cargo test
```

Planner unit tests use the crate-internal `planner::dsl` expected-plan helpers
instead of snapshots, so supported lowering changes update the expected plan
directly next to the source being tested.

For review rules around planner profile/margin tests, helper shape, and coverage
policy, see [review-policy.md](review-policy.md).

Run formatting and lint checks:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

## Coverage

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

Line coverage is the current enforced gate, and Geam keeps it at 100%. Region
coverage is not yet an enforced gate, but it is a useful direction for tightening
tests as the supported profile grows. Prefer tests that cover meaningful region
gaps when the missing region maps to a real planner, plan, or runtime branch.

When a coverage gap is hard to explain from the summary alone, split the target
and inspect LLVM's region and instantiation detail before adding fixtures:

```sh
cargo llvm-cov --lib --text --show-instantiations --show-missing-lines
```

Use that report to decide where the test belongs. Public execution behavior
belongs in fixture-based integration tests; planner or runtime implementation
branches belong in the owning unit test next to that module.

Generate an HTML report:

```sh
cargo llvm-cov --html
```

The HTML report is written to:

```text
target/llvm-cov/html/index.html
```

Use the summary and HTML report to identify Gleam boundary wrapper, planner, and
runtime paths that still need direct tests.
