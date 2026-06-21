# Testing

Geam uses Rust unit tests and `insta` snapshots for the first front-end
milestones.

## Commands

Run the normal test suite:

```sh
cargo test
```

Update snapshots only when intentionally changing parser output:

```sh
INSTA_UPDATE=always cargo test
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

Use the summary and HTML report to identify parser accept/reject gaps, lexer
error paths, and analyse/type inference paths that still need direct tests.
