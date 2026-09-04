# Testing

Geam uses Rust unit tests for compiler-boundary, lowering, and runtime
milestones.

The root Cargo workspace contains the `geam` facade and binary, `geam-core`,
`geam-stdlib`, `geam-json`, `geam-time`, `geam-cli`, and `geam-macros`. Each
extracted package owns tests for its production protocols. Root integration
targets own the public `geam::...` facade, cross-crate compatibility, and
standalone distribution behavior; they do not replace package-local owner
tests or built-in compatibility suites.

The Workspace workflow fixes the root feature profiles independently of the
default distribution:

```sh
cargo check --package geam --no-default-features --features embedding --all-targets --locked
cargo check --package geam --no-default-features --features provider --all-targets --locked
cargo check --package geam --no-default-features --features embedding,gleam-stdlib --all-targets --locked
```

Default workspace tests and installation still use the complete `full`
profile. Minimal checks prove only the selected facade and dependency graph;
they do not replace owner or acceptance tests.

For guidance on constructing owner tests, promoting diagnostic probes, and
closing coverage gaps, see [test-development.md](test-development.md).

`geam-macros` owns parser, diagnostic, expansion, compile-fail, and core-backed
execution tests for the provider authoring attributes. Its integration tests
use `geam-core` only as a dev-dependency and verify stateful scalar calls,
recursive native tuples, lazy Lists, directional custom values, generic
retention in simple and persistent-collection payloads, and external values
without adding a production runtime dependency to the
proc-macro crate. They fix generated schemas and stores, constructor and field
metadata, mixed custom/external/scalar List items, pass-through versus Vec
construction, source equality, inspection, escaped payload lifetime, and
structured linkage mismatch. A separately locked two-crate fixture proves that
the same static custom/external declaration protocol compiles, links, and runs
across crate boundaries. Consumer fixtures do not replace these owner tests.

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

The tracked `builtins/stdlib/tests/fixtures/project` project locks upstream
`gleam_stdlib` `v1.0.3` but does not track downloaded package source. The
`geam-stdlib` integration target downloads that exact source before executing
its package-local compatibility suite.

CI runs these tests with Gleam `v1.18.1`. Provider-free roots run through
`compile_typed_project -> plan_program -> ExecutionPlan::from_module_plan ->
run_main`; roots whose selected closure uses registered externals run through
`compile_typed_host_project -> plan_host_program ->
HostedExecution::try_from_module_plan -> run_main` with the explicit
`geam-stdlib` provider bundle. The tracked set covers `gleam/bit_array`,
`gleam/bool`, `gleam/bytes_tree`, `gleam/dict`, `gleam/dynamic`,
`gleam/dynamic/decode`, `gleam/float`, `gleam/function`, `gleam/int`, `gleam/io`,
`gleam/list`, `gleam/option`, `gleam/order`, `gleam/pair`, `gleam/result`,
`gleam/set`, `gleam/string`, `gleam/string_tree`, and `gleam/uri`. Each module
fixes its analyzed public surface and executes grouped source behavior. This
integration suite does not replace hermetic synthetic owner coverage for the
loader or providers.

The tracked `tests/fixtures/projects/gleam_http` project independently locks
upstream `gleam_http` `v4.3.0` and `gleam_stdlib` `v1.0.3`. It likewise keeps
downloaded package source out of Git and downloads the exact locked source
automatically before the integration target runs.

The HTTP package itself is Pure Gleam and registers no provider. Its selected
dependency closure reaches provider-backed stdlib modules, so the suite uses
the hosted resolved-project pipeline with the explicit stdlib provider bundle.
It fixes the public surface of all five package modules and executes every
public function.

The tracked `builtins/json/tests/fixtures/project` project independently locks
upstream `gleam_json` `v3.1.0` and `gleam_stdlib` `v1.0.3`. The `geam-json`
integration target downloads its exact locked source before executing the
package-local compatibility suite.

This target explicitly composes the stdlib and JSON provider bundles, fixes the
complete public `gleam/json` surface, and executes every public function.

The tracked `builtins/time/tests/fixtures/project` project independently locks
upstream `gleam_time` `v1.8.0` and `gleam_stdlib` `v1.0.3`. The `geam-time`
integration target downloads its exact locked source before executing the
package-local compatibility suite.

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

The independently locked managed embedding examples fix the user-facing
progression from the first generated function call through recursive ordinary
data, a Gleam package, caller-owned IO, and an external provider. Each example
owns a nested Gleam project, generated Rust bindings, a handwritten entry point,
and an integration test that executes the binary and fixes complete stdout and
stderr. Their READMEs explain one new boundary at a time; none depends on an
earlier example at build or run time.

Run the guided examples locally from the repository root:

```sh
cargo build --package geam --bin geam --locked
for example in \
  first_call \
  data \
  package \
  io \
  provider
do
  (
    cd "examples/embedding/$example"
    ../../../target/debug/geam embedding check
    (cd gleam && gleam format --check)
    cargo fmt --all --check
    CARGO_TARGET_DIR=../../../target/embedding cargo test --locked
    CARGO_TARGET_DIR=../../../target/embedding \
      cargo clippy --all-targets --locked -- -D warnings
  )
done
```

The independently locked
[`examples/embedding/application`](../../examples/embedding/application)
is the capstone managed Rust-first workflow. Its nested resolved Gleam project
uses imported source, stdlib IO, and the real text-pattern provider.
The Rust entry point keeps loading, binding, sealing, capabilities,
configuration, mutable state, Echo, and output handling visible. The
application's `inventory` module owns its typed call sequence and review
report. Its generated `src/geam_bindings.rs` is committed.
The inventory workflow consumes Vec rows and passes the retained List into
later total/first-valid calls before reading the rows for its report. Tests
beside that workflow fix mixed, all-rejected, and empty inputs, exact
Tuple/Result/Option values and reports, repeated calls, IO, and Echo. The
binary integration test fixes the complete stdout and empty stderr. The
domain Stock type stays in Gleam. Lazy-read costs, retained lifetime,
foreign-owner rejection, and recursive permutations remain core/CLI
owner-test contracts.

Run the same focused checks locally from the repository root. The first check
restores missing package sources from the committed Gleam lock without a
separate dependency-resolution step:

```sh
cargo build --package geam --bin geam --locked
(cd examples/embedding/application && ../../../target/debug/geam embedding check)
(cd examples/embedding/application/gleam && gleam format --check)
cargo tree --manifest-path examples/embedding/application/Cargo.toml \
  --locked --package geam --edges normal --depth 1
cargo fmt --manifest-path examples/embedding/application/Cargo.toml --all --check
cargo test --manifest-path examples/embedding/application/Cargo.toml --locked
cargo clippy --manifest-path examples/embedding/application/Cargo.toml \
  --all-targets --locked -- -D warnings
cargo run --quiet --manifest-path examples/embedding/application/Cargo.toml --locked
```

The Acceptance workflow's `Rust embedding` job owns these boundaries. It first
installs the current checkout with `cargo install --path . --locked` into a
temporary installation root, using the default features and release profile.
It checks, formats, tests, and lints every guided example; their integration
tests execute each binary and compare exact output. The following capstone
readiness and recovery checks use that installed binary, so CI also verifies
that feature separation preserves the default CLI installation. The job starts
without a separate Gleam download step, checks locked readiness, and verifies
that tracked application files remain unchanged. It then makes generated source
stale, requires `embedding check` to fail, and runs production sync to restore
the exact committed file before formatting, testing, linting, and running the
application with the exact inventory report and its captured Gleam IO. The same
job requires one Geam package identity, the exact core/macros/stdlib application
profile, the text-pattern provider, and no CLI/JSON/Time dependency.
Provider-example jobs remain separate because they own provider authoring and
standalone consumption rather than Rust-first application composition.

The same job checks Gleam formatting in `examples/embedding/manual` and runs the
small manual `rust_embedding` example, comparing its complete stdout with the
expected scalar results. The example's repeated-call and empty-Echo assertions
also execute; its Rust formatting and Clippy checks remain workspace-owned.

CLI owners separately cover fresh init, repeated sync, required built-in
features, existing dependency preservation, and explicit native-provider
validation. Check cases cover locked Hex/Git/local sources, cold and partial
caches, stale locks or generated files, invalid source, provider incompatibility,
and acquisition failures while comparing project bytes. Cargo's locked metadata
path may populate caches; neither it nor the check command compiles or executes
Rust build scripts, provider initialization, or the application. Runtime typed
value and ownership contracts remain in core tests.

The tracked `cli/tests/fixtures/standalone_cli` fixture verifies the complete CLI
assembly boundary. Its Gleam project combines a Pure Gleam path package,
version-locked stdlib, JSON, and Time dependencies, and two provider-backed path
packages. Upstream package source is downloaded outside Git, while the local
packages remain visible test-owned fixtures. The independent Rust providers
exercise state, callbacks, external storage, and a compound return through the
generated static profile. A standalone orchestration test adds both providers
through explicit local path selections, verifies their packaged metadata, and
carries those selections through the real root Cargo lock, generated runner
check, and runner execution. Fixture-only Cargo patches keep acquisition local
while preserving the production manifest, resolution, build, and execution
path.

CLI owner tests fix the exact preparation messages, conditional acquisition
and lock work, and native byte forwarding without newline buffering. Process
tests cover private metadata stdout, checked-process stdin isolation, large
output, failure diagnostics, and draining after output failure. The standalone
orchestration test also checks progress and repeated preparation. Binary and
distribution tests keep application stdout and IO/Echo ordering separate from
preparation stderr; native Cargo/Gleam wording is not a version-pinned assertion.

The [provider authoring examples](../../examples/provider) are consumer-facing macro
acceptance cases. `text_tools` maps one stateless provider to three Gleam
modules, `value_types` fixes every scalar mapping plus one-, multi-, and
nested-tuple mapping, lazy top-level Lists, directional custom values, and
standard source Result/Option values, and `tag_set` fixes generated external
semantics. `request_ids` combines mutable and read-only default state,
`feature_flags` owns configured initialization, and `run_metrics` retains
specialized manual external semantics. `call_tracing` verifies typed callback
return identity, same-component re-entry, exact state ordering, and fresh state
on repeated runs. `generic_box` verifies typed retention, cross-type
replacement, source semantics, and callback mapping without materialization.
The root `provider_examples` target follows each documented path add, prepare,
run, and repeated-run workflow against independently locked provider crates.
The complete Gleam entrypoints execute every public example function.
Repository-local Cargo patches select the current checkout until the authoring
crates are released.

The [`examples/provider/text_pattern`](../../examples/provider/text_pattern) example adds a
distribution-ready advanced macro provider. Its path test executes manual
external semantics, a custom error, source Result, and List output through the
managed root lock and generated runner. Provider-resolution tests separately
cover explicit registry, path, and Git requests, Cargo metadata, exact selected
dependencies, lock preservation, and provider metadata validation.

A separate `Published provider` acceptance job exercises the released
distribution rather than the checkout. It installs a known published Geam,
creates a clean project, pins the matching Hex package, and explicitly adds the
matching crates.io provider. Metadata must show exact registry dependencies,
the generated runner must include the provider component, and repeated
prepare/run must preserve the managed manifest, lock, and runner source. The
reference-example publication workflow runs the same path against every new
same-version combination before release finalization. The small Gleam entry
module is kept inline because these jobs own released command sequences rather
than reusable source behavior.

The same text-pattern test first runs the common Gleam entrypoint and the
Erlang-specific example using the package's native `re` implementation, before
adding a Rust provider. It then runs the common entrypoint twice on Geam and
checks Geam-specific replacement syntax, pattern equality, and exact inspection.
This test requires Erlang/OTP as well as Gleam; CI supplies OTP `29`. The native
Erlang source is included in the exported Hex package.

CI formats, tests, lints, and packages every independent example provider. The
nine macro examples select the current unreleased authoring surface through
repository-local patches and complete standalone execution. The independent
Provider SDK fixture remains the canonical low-level typed-host ABI acceptance
owner.

The [Acceptance workflow](../../.github/workflows/acceptance.yml) runs one matrix
job per documented example. Each job selects its exact `provider_examples`
test, runs the independent provider's tests, verifies its Cargo package, and
exports its Gleam package. A failed example does not cancel the other matrix
jobs. The parallel `Published provider` job has no repository checkout and
therefore cannot substitute path dependencies or checkout binaries for the
released artifacts it monitors. Formatting and Clippy remain in the Workspace
workflow.

Each example has a distinct cache key. Within a job, the root test binary and
independent provider use the checkout's `target/` directory so Cargo can reuse
matching build artifacts without changing either workspace's lockfile. The
generated runner still uses its temporary project's `build/geam/target/`;
those isolated runner artifacts are not shared or cached between jobs.

The normal suite executes the full generated runner with the fixture's locked
Gleam and Rust dependencies. CI exports the standalone fixture's three local
Gleam dependencies and all nine example Gleam packages. It also packages the
two standalone fixture providers and every example provider. No test-only
fixture package is published. The text-pattern provider and matching Hex package
are release-coupled public documentation artifacts and share every Geam release
version.

The root package keeps four explicit acceptance targets:

- `binary` starts the installed-shape `geam` process for command dispatch,
  process failures, pure execution, and IO/Echo ordering.
- `cross_crate_http` proves that the Pure Gleam `gleam_http` package works
  through the root facade and stdlib composition. HTTP is not a Geam built-in
  and has no provider crate.
- `provider_examples` executes the nine documented provider projects through
  the real binary and generated runners.
- `standalone_distribution` combines built-ins and two independent providers
  in one canonical managed-project flow.

Detailed project loading, provider selection validation, explicit provider
resolution, manifest, lock, and runner behavior remain in `geam-cli`; the root
targets only retain the process, facade, cross-crate, or distribution boundary
named above.

Source-level rejection fixtures live under categorized
`tests/fixtures/rejection/**/*.gleam` paths. They are reserved for public
boundary cases that are clearer as complete Gleam modules than as planner unit
tests.

## Commands

With the Rust toolchain, Gleam `v1.18.1`, and Erlang/OTP `29` installed, run the
full test suite:

```sh
cargo fetch --locked
cargo test --workspace --locked
```

Run `cargo fetch --locked` before the CLI tests, root acceptance targets, or
CLI coverage closure when starting with an empty Cargo cache or after changing
the workspace lockfile. These tests run nested Cargo commands in offline
fixtures. The workspace fetch supplies the complete locked dependency graph,
including target-specific crates that a native build need not download.
Provider fixtures separately fetch their own locked dependencies; their minimal
provider profile does not include every built-in used by a generated runner.
The relevant Acceptance and Coverage jobs explicitly fetch workspace
dependencies before testing, whether or not a cache was restored.

The workspace's explicit default members are the same seven packages, so
`cargo test --locked` remains equivalent for local use. CI spells out
`--workspace` so newly added internal packages cannot be omitted implicitly.

Run a package-owned compatibility suite directly:

```sh
cargo test --package geam-stdlib --test gleam_stdlib --locked
cargo test --package geam-json --test gleam_json --locked
cargo test --package geam-time --test gleam_time --locked
```

Run the root acceptance targets independently:

```sh
cargo test --package geam --test binary --locked
cargo test --package geam --test cross_crate_http --locked
cargo test --package geam --test provider_examples --locked
cargo test --package geam --test standalone_distribution --locked
```

To run one provider example with the same exact selection used by its CI job:

```sh
cargo test --package geam --test provider_examples --locked -- \
  --exact runs_the_documented_text_tools_provider_across_three_modules
```

The unfiltered `provider_examples` command still runs all nine examples locally.

Planner unit tests use the crate-internal `planner::dsl` expected-plan helpers
instead of snapshots, so supported lowering changes update the expected plan
directly next to the source being tested.

For review rules around planner profile/margin tests, helper shape, and coverage
policy, see [review-policy.md](review-policy.md).

Run formatting and lint checks:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
```

## Coverage

Geam uses `cargo-llvm-cov` for local coverage. It is LLVM-based, works well on
macOS, and keeps generated reports under `target/`.

Install the local tools:

```sh
rustup component add llvm-tools-preview
cargo install cargo-llvm-cov --locked
```

Coverage is measured through three production-consumer closures. Each closure
starts with an empty coverage profile, executes only the tests needed to reach
its package contracts, and then reports every production package separately.
This permits JSON tests to exercise stdlib contracts they consume without
allowing JSON coverage to compensate for an uncovered stdlib line or region.
Each closure explicitly runs `cargo llvm-cov clean --workspace` before
collection so cached instrumentation from another package cannot contribute to
its reports. This command clears artifacts; it does not execute additional
packages. Only a later collection command in the same closure uses `--no-clean`
to retain that closure's profiles.

The core and macro closure uses only those packages' owner tests. Both reports
must independently reach 100% without relying on built-in or CLI consumers.

Run the core and macro closure:

```sh
cargo llvm-cov clean --workspace
cargo llvm-cov --no-report --package geam-core --package geam-macros --locked
cargo llvm-cov report --package geam-core --summary-only --fail-under-lines 100 --fail-under-regions 100
cargo llvm-cov report --package geam-macros --summary-only --fail-under-lines 100 --fail-under-regions 100
```

Run the built-in closure with Gleam `v1.18.1` available:

```sh
cargo llvm-cov clean --workspace
cargo llvm-cov --no-report --package geam-stdlib --package geam-json --package geam-time --locked
cargo llvm-cov report --package geam-stdlib --summary-only --fail-under-lines 100 --fail-under-regions 100
cargo llvm-cov report --package geam-json --summary-only --fail-under-lines 100 --fail-under-regions 100
cargo llvm-cov report --package geam-time --summary-only --fail-under-lines 100 --fail-under-regions 100
```

Run the CLI and binary closure with Gleam `v1.18.1` available:

```sh
cargo llvm-cov clean --workspace
cargo llvm-cov --no-report --package geam-cli --locked
cargo llvm-cov --no-clean --package geam --test binary --locked --summary-only
cargo llvm-cov report --package geam-cli --summary-only --fail-under-lines 100 --fail-under-regions 100
cargo llvm-cov report --package geam --summary-only --fail-under-lines 100 --fail-under-regions 100
```

The independent Provider SDK workspace retains its own 100% gate shown above.
Geam keeps both line coverage and full-scope region coverage at 100% for every
reported production package. Region coverage is the stricter review signal
when a source line contains multiple expression regions.

When a coverage gap is hard to explain from the summary alone, inspect LLVM's
region and instantiation detail for the package after running its closure:

```sh
cargo llvm-cov report --package geam-core --text --show-instantiations --show-missing-lines
```

Replace `geam-core` with the package under investigation. Generate a
package-scoped HTML report from the same profile:

```sh
cargo llvm-cov report --package geam-core --html
```

The HTML report is written to:

```text
target/llvm-cov/html/index.html
```

Use the summary and HTML report to identify Gleam boundary wrapper, planner, and
runtime paths that still need direct tests.
