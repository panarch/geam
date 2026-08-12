# Testing

Geam uses Rust unit tests for compiler-boundary, lowering, and runtime
milestones.

For guidance on constructing owner tests, promoting diagnostic probes, and
closing coverage gaps, see [test-development.md](test-development.md).

The current compiler-boundary and runtime milestones depend on the exact
`geam-gleam-core` package recorded in the upstream guide. `cargo test` resolves
that published package and its compiler components from the locked crates.io
dependency graph as part of the normal suite.

Source-level execution tests live under categorized
`tests/fixtures/execution/**/*.gleam` paths. Each fixture must end with an
`// @geam:expect ...` directive, for example:

```gleam
pub fn main() {
  1 + 2
}

// @geam:expect Int(3)
```

The `@geam:` namespace distinguishes fixture-runner directives from ordinary
source comments and expected output. Its directives are `expect`,
`expect-error`, `echo`, `explain`, and `reject`. Each `echo` block contains one
exact `EchoOutput::to_string()` result:

```gleam
pub fn main() {
  echo 1 as "selected"
}

// @geam:echo
// tests/fixtures/execution/example.gleam:2 selected
// 1
// @geam:expect Int(1)
```

The integration runner reads those fixtures through the public Geam API:
`compile_typed_module -> plan_module_with_source ->
ExecutionPlan::from_module_plan -> run_main`.

Multi-module execution cases live under
`tests/fixtures/execution/modules/<case>/`. The runner derives canonical module
names from paths relative to the case directory (`main.gleam` becomes `main`,
and `support/math.gleam` becomes `support/math`) and uses the public
`compile_typed_program -> plan_program` pipeline. It does not perform package or
filesystem module resolution beyond loading the fixture case.

Resolved-project loader behavior is covered by synthetic temporary projects in
the frontend owner tests. These tests construct Hex, Git, and Local package
layouts without network access or an installed Gleam CLI, keeping loader owner
coverage independent of external package acquisition.

The tracked `tests/fixtures/projects/gleam_stdlib` project locks official
`gleam_stdlib` `v1.0.3` but does not track downloaded package source. Full test
and coverage runs automatically download that exact source before executing the
integration target.

CI runs these tests with Gleam `v1.18.1`. Provider-free roots run through
`compile_typed_project -> plan_program -> ExecutionPlan::from_module_plan ->
run_main`; roots whose selected closure uses registered externals run through
`compile_typed_host_project -> plan_host_program ->
HostedExecution::try_from_module_plan -> run_main` with the explicit
`geam::gleam_stdlib` provider bundle. The tracked set covers `gleam/bit_array`,
`gleam/bool`, `gleam/bytes_tree`, `gleam/dict`, `gleam/dynamic`,
`gleam/dynamic/decode`, `gleam/float`, `gleam/function`, `gleam/int`, `gleam/io`,
`gleam/list`, `gleam/option`, `gleam/order`, `gleam/pair`, `gleam/result`,
`gleam/set`, `gleam/string`, `gleam/string_tree`, and `gleam/uri`. Each module
fixes its analyzed public surface and executes grouped source behavior. This
integration suite does not replace hermetic synthetic owner coverage for the
loader or providers.

The tracked `tests/fixtures/projects/gleam_http` project independently locks
official `gleam_http` `v4.3.0` and `gleam_stdlib` `v1.0.3`. It likewise keeps
downloaded package source out of Git and downloads the exact locked source
automatically before the integration target runs.

The HTTP package itself is Pure Gleam and registers no provider. Its selected
dependency closure reaches provider-backed stdlib modules, so the suite uses
the hosted resolved-project pipeline with the explicit stdlib provider bundle.
It fixes the public surface of all five package modules and executes every
public function.

The tracked `tests/fixtures/projects/gleam_json` project independently locks
official `gleam_json` `v3.1.0` and `gleam_stdlib` `v1.0.3`. Full test and
coverage runs automatically download its exact locked source before executing
the integration target.

This target explicitly composes the stdlib and JSON provider bundles, fixes the
complete public `gleam/json` surface, and executes every public function.

The tracked `tests/fixtures/projects/gleam_time` project independently locks
official `gleam_time` `v1.8.0` and `gleam_stdlib` `v1.0.3`. Full test and
coverage runs automatically download its exact locked source before executing
the integration target.

This target explicitly composes the stdlib and Time provider bundles. It fixes
the complete public surfaces of `gleam/time/duration`, `gleam/time/calendar`,
and `gleam/time/timestamp`, executes every public function, and supplies a
deterministic caller-owned clock for system effects.

The independent `tests/fixtures/provider_sdk` Cargo workspace verifies the
public path-provider boundary without adding its crates to Geam's development
dependencies. Its `runner/tests/public_usage.rs` keeps the complete Gleam
source, explicit component configuration, generated-like profile, provider
composition, hosted pipeline, expected value, and state assertions visible as
one executable example.

```sh
cargo test --manifest-path tests/fixtures/provider_sdk/Cargo.toml --workspace --locked
cargo clippy --manifest-path tests/fixtures/provider_sdk/Cargo.toml --workspace --all-targets --locked -- -D warnings
cargo llvm-cov --manifest-path tests/fixtures/provider_sdk/Cargo.toml --workspace --locked --summary-only --fail-under-lines 100 --fail-under-regions 100
```

This workspace is independently locked and needs neither a Gleam CLI nor
downloaded Gleam package source. Its Rust dependencies use Cargo's ordinary
locked acquisition path. CI runs it as a separate provider SDK boundary.

The tracked `tests/fixtures/standalone_cli` fixture verifies the complete CLI
assembly boundary. Its Gleam project combines a Pure Gleam path package,
version-locked stdlib, JSON, and Time dependencies, and two provider-backed path
packages. Official package source is downloaded outside Git, while the local
packages remain visible test-owned fixtures. The independent Rust providers
exercise state, callbacks, external storage, and a compound return through the
generated static profile. Registry owner tests feed real `cargo package`
archives through a fake bounded registry, checksum and metadata verification,
approval, and registry-shaped manifest generation. Fixture-only Cargo patches
then keep provider acquisition and runner builds local.

The normal suite executes the full generated runner with the fixture's locked
Gleam and Rust dependencies. CI also runs `gleam export hex-tarball` for each
local Gleam dependency and `cargo package` for both provider crates. No fixture
package is published.

Source-level rejection fixtures live under categorized
`tests/fixtures/rejection/**/*.gleam` paths. They are reserved for public
boundary cases that are clearer as complete Gleam modules than as planner unit
tests.

## Commands

With the Rust toolchain and Gleam `v1.18.1` installed, run the full test suite:

```sh
cargo test --locked
```

Planner unit tests use the crate-internal `planner::dsl` expected-plan helpers
instead of snapshots, so supported lowering changes update the expected plan
directly next to the source being tested.

For review rules around planner profile/margin tests, helper shape, and coverage
policy, see [review-policy.md](review-policy.md).

Run formatting and lint checks:

```sh
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
```

## Coverage

Geam uses `cargo-llvm-cov` for local coverage. It is LLVM-based, works well on
macOS, and keeps generated reports under `target/`.

Install the local tools:

```sh
rustup component add llvm-tools-preview
cargo install cargo-llvm-cov --locked
```

Run the enforced coverage gate:

```sh
cargo llvm-cov --locked --summary-only --fail-under-lines 100 --fail-under-regions 100
```

Geam keeps both line coverage and full-scope region coverage at 100%. Region
coverage is the stricter review signal when a source line contains multiple
expression regions.

When a coverage gap is hard to explain from the summary alone, split the target
and inspect LLVM's region and instantiation detail before adding fixtures:

```sh
cargo llvm-cov --text --show-instantiations --show-missing-lines
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
runtime paths that still need direct tests.
