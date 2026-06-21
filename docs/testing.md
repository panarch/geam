# Testing

Geam uses Rust unit tests for compiler-boundary, lowering, and runtime
milestones.

The current compiler-boundary milestone depends on a local Gleam checkout at the
baseline recorded in the README. `cargo test` compiles the `gleam-core` path
dependency as part of the normal suite.

## Commands

Run the normal test suite:

```sh
cargo test
```

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

Use the summary and HTML report to identify Gleam boundary wrapper paths and
later lowering/profile/runtime paths that still need direct tests.
