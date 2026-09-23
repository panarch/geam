# Testing

Geam uses Rust unit tests for compiler-boundary, lowering, and runtime
milestones.

The root Cargo workspace contains the `geam` facade and binary, `geam-core`,
`geam-stdlib`, `geam-json`, `geam-time`, `geam-erlang`, `geam-builtin`, `geam-cli`, and `geam-macros`. Each
extracted package owns tests for its production protocols. Root integration
targets own the public `geam::...` facade, cross-crate compatibility, and
standalone distribution behavior; they do not replace package-local owner
tests or built-in compatibility suites.

The Workspace workflow fixes the root feature profiles independently of the
default distribution:

```sh
cargo check --package geam --no-default-features --features embedding --all-targets --locked
cargo check --package geam --no-default-features --features embedding,tokio --all-targets --locked
cargo check --package geam --no-default-features --features provider --all-targets --locked
cargo check --package geam --no-default-features --features standalone --all-targets --locked
cargo check --package geam --no-default-features --features embedding,gleam-stdlib --all-targets --locked
cargo check --package geam --no-default-features --features embedding,geam-builtin --all-targets --locked
cargo check --package geam --no-default-features --features embedding,gleam-erlang --all-targets --locked
cargo check --package geam --no-default-features --features embedding,gleam-erlang,geam-builtin,tokio --all-targets --locked
```

Default workspace tests and installation still use the complete `full`
profile. Minimal checks prove only the selected facade and dependency graph;
they do not replace owner or acceptance tests.

The Workspace workflow runs `geam-cli` owner tests in the separate `CLI tests`
job. The `Tests` job runs the remaining workspace packages, excluding the root
acceptance package and CLI, and retains feature-profile checks and package
assembly. Tests stay with their owning crates; the local workspace command and
independent coverage closures are unchanged.

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
structured linkage mismatch. Callable owner targets also cover private native
construction, nested callbacks and explicit work, generic custom captures,
phantom parameters, non-Clone payload retention, and once-only lifecycle effects.
A separately locked two-crate fixture proves that
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

The same mandatory fixture target also runs each successful and failing source
through the public embedding boundary. A synthetic Nil-returning entry
captures the original result through Echo, so arbitrary fixture return families
remain covered without adding a universal embedding return type. It compares
the exact result type and inspection, execution error, and Echo sequence with
the ordinary pipeline using the same source. These calls do not drive Future
work.

The observation entry is assembled by a shared test helper, not repeated in
tracked Gleam fixtures. In-memory cases append it before compilation. Resolved
project cases copy the fixture source, unchanged manifests and downloaded package
source into a temporary directory, add the entry only to that copy, and use the
normal project loader. Original fixture files and upstream source stay unchanged;
the temporary copy is removed after compilation.

Future lifecycle tests in `core/src/runtime/work.rs` and its child modules use
controlled polling, wakes, and drop observations to cover shared success and
failure, observer removal, last-owner release, re-entry, and scope shutdown.
The `core/tests/work_embedding.rs` target exercises the public typed boundary,
including existing work passed through functions and nested containers. Transfer
tests cover both pending work and shared completion with caller-borrowed state.
These tests establish the [work execution contract](review-policy.md#execution-and-explicit-work-rules)
independently of executor-specific examples.

The `geam-core` prepared integration target compiles emitted Rust tables and
loads them through the same evaluator as dynamic programs. Its maintained
artifacts under `core/tests/fixtures/prepared` cover arithmetic, the complete
value/function families, native conversions/callbacks, retained work, and
producer-authorized opaque custom sharing (including generic callable payloads
and missing grants).
Tests compare fresh preparation with these exact artifacts as well as checking
explicit runtime values, diagnostics and owner isolation.
Emission owner tests keep exact, indented Rust expressions beside their inputs,
including nested fields, arrays and escaped literals.

After changing the artifact format, emission, or these fixtures, regenerate
their Rust data through the public preparation API, then inspect the diff and
rerun the compiled consumer tests:

```sh
cargo run --package geam-core --example prepare_fixtures --locked
cargo test --package geam-core --features tokio --test prepared --locked
```

The generator does not include the old artifacts, so it can rebuild fixtures
whose previous representation no longer compiles. It does not update expected
runtime results; those remain explicit assertions in the integration target.

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

The tracked `builtins/erlang/tests/fixtures/project` pins `gleam_erlang v1.3.0`
and `gleam_stdlib v1.0.3`. Its independent surface inventory freezes all seven
public modules and 48 bodyless externals against the original package source.
Compatibility cases exercise Subjects, native Dynamic views, selectors and
suspended callbacks, process lifecycle, names, timers, and package resources.
Its deterministic host and clock keep timer and callback ordering reproducible.
The root standalone process case also runs the maintained Gleam service on
Erlang as an independent behavioral oracle.

The independent `tests/fixtures/provider_sdk` Cargo workspace verifies the
public path-provider boundary without adding its crates to Geam's development
dependencies. Its `runner/tests/public_usage.rs` keeps the complete Gleam
source, explicit component configuration, generated-like profile, provider
composition, native function construction/return/invocation, hosted pipeline,
expected value, and state assertions visible as
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
data, a Gleam package, caller-owned IO, an external provider, and a caller-owned
async executor. Each example owns a nested Gleam project, generated Rust
bindings, a handwritten entry point, and an integration test that executes the
binary and fixes complete stdout and stderr. Their READMEs explain one new
boundary at a time; none depends on an earlier example at build or run time.

Run the guided examples locally from the repository root:

```sh
cargo build --package geam --bin geam --locked
for example in \
  first_call \
  data \
  package \
  io \
  provider \
  async_host \
  session \
  processes \
  prepared \
  callables
do
  (
    export CARGO_TARGET_DIR="$PWD/target"
    cd "examples/embedding/$example"
    case "$example" in
      prepared|callables) ../../../target/debug/geam embedding sync ;;
    esac
    ../../../target/debug/geam embedding check
    (cd gleam && gleam format --check)
    cargo fmt --all --check
    cargo test --locked
    cargo clippy --all-targets --locked -- -D warnings
  )
done
```

The `async_host` example owns the generated explicit-Future user workflow.
It loads an independently locked macro-authored file provider, creates work
through ordinary Gleam source, drives it using the application's Tokio
executor, and observes its completion again. Its binary test fixes stdout and
stderr alongside an ordinary value-returning call in the same execution.

The `callables` example shares an app-local declaration module with preparation
and binds real Rust bodies from ordinary application modules. It constructs a
capturing function, passes it through a private Gleam custom value and a generic
source wrapper, and compares exact dynamic/prepared output. The
`prepared_embedding` acceptance target copies that consumer, checks declaration
drift and repeated generation without lock changes, then removes the original
application and runs its relocated prepared binary with an empty PATH.

The `session` example returns an opaque source value with a private closure,
retains it in Rust, and passes it back through generated bindings. It requires
neither a provider nor a source Future. Exact owner/type/lifetime rejection,
hidden work and non-Clone/non-Sync payload retention remain core and CLI owner
test obligations.

The `processes` example uses actual generated Pid and Subject handles to retain
a running Gleam service between Rust calls, issue multiple requests, and await
its termination. Its public binary test fixes the complete output. Core and
built-in owners separately prove cancellation, routing, lifetime and scheduling
behavior. The `Host execution` CI matrix runs the package and example on Linux,
macOS, and Windows as well as the guided-example checks below.

The `execution` example uses the manual hosted API to run pure Gleam, observe
its first Echo, cancel the running entry, and successfully call another entry
in the same module. Its integration test fixes the complete output; core owner
tests establish budget and cancellation behavior with controlled scheduling.
The same Rust embedding CI job checks it:

```sh
(cd examples/embedding/execution/gleam && gleam format --check)
cargo fmt --manifest-path examples/embedding/execution/Cargo.toml --check
CARGO_TARGET_DIR=target/embedding cargo test --manifest-path examples/embedding/execution/Cargo.toml --locked
CARGO_TARGET_DIR=target/embedding cargo clippy --manifest-path examples/embedding/execution/Cargo.toml --all-targets --locked -- -D warnings
```

The `prepared` example uses the first-call function with generated `load()` and
compiler-visible execution data. Managed embedding consumers keep typed bindings in
Git and ignore their generated `src/geam_bindings/program.rs`. Run sync before
check, formatting or compilation on a fresh checkout. The release-preparation
workflow regenerates and checks the prepared example after changing dependency
versions and locks, committing only the bindings. The exact reference artifacts
under `core/tests/fixtures/prepared` remain tracked test expectations.

The Prepared distribution fixtures run Cargo offline. Fetch both the workspace
dependencies and the callable consumer's independently locked dependencies first:

```bash
cargo fetch --locked
cargo fetch --manifest-path examples/embedding/callables/Cargo.toml --locked
cargo fetch --manifest-path examples/provider/process_service/embedding/Cargo.toml --locked
cargo fetch --manifest-path tests/fixtures/otp_service/embedding/Cargo.toml --locked
cargo test --package geam --test prepared_embedding --test standalone_build --locked
```

The root `prepared_embedding` target runs init, both and prepared sync/check,
binding drift recovery, ordinary Cargo builds and explicit semantic assertions.
It also verifies that `cargo fmt` and repeated sync preserve the generated bytes.
The callable and service consumers start without prepared program data, even
when a developer has generated it locally. The callable consumer starts with
CRLF Rust files on every platform and also converts a freshly generated program
to CRLF before syncing again. Sync recognizes generated ownership markers with
LF or CRLF line endings and writes canonical LF output; check still requires
exact generated bytes.
Prepared statics retain generator-owned indentation under `rustfmt::skip`, while
the complete normalized preparation inputs produce a SHA-256 fingerprint comment
after the code. Check still regenerates and compares the complete output;
dependency-only changes remain detectable even when executable data is unchanged.
It moves the executable, removes the original Gleam project, and verifies both
normal output and embedded source diagnostics with an empty PATH. It then
packages a Git consumer whose program is ignored but explicitly included by
its Cargo manifest, inspects Cargo's extracted verification tree and builds
and runs that tree from another directory after removing the original consumer.
Failing Geam/Gleam command sentinels ensure packaging and rebuilding do not
silently invoke a generation tool. The Prepared distribution CI matrix runs this
boundary on Linux, macOS and Windows; native feature/registration mismatch and
rich hosted generation remain CLI owner tests.

The same matrix runs `standalone_build`: real debug/release builds, selected
modules, source-backed command transitions, and relocated execution with an
empty PATH. It composes the maintained provider and Future fixtures with
process execution, configures providers and package resources at startup, and
verifies untouched application arguments, cancellation, shutdown and source
diagnostics after removing the source/build tree. CLI owner tests separately
fix Cargo message admission, output ownership, locking and preparation failure.
The root standalone support module owns configuration/path, IO and driver-join
tests; its tests run in the CLI/binary coverage closure. Join tests cover owned
state, panic payloads and cancellation without detaching the driver. The
`future_builtins` target also checks process and timer progress with the driver
and a CPU-bound Gleam process sharing one Tokio worker.

Core owners and `geam-macros`'s `async_provider` target exercise deterministic
Pending, shared completion, bounded state access, rich callbacks, cancellation,
and cross-worker transfer. The root `future_builtins` target composes the same
path with pinned official built-in source. These obligations are not delegated
to the filesystem example.

The embedding example also checks missing-file and invalid-Unicode results
through the generated bindings and the actual file provider. CI separately
checks the independent `examples/provider/async_files/provider` crate's build,
formatting, and Clippy.
The `geam-builtin` owner tests use the ordinary package source from
`builtins/geam/gleam`. Package tests also exercise the official Gleam language
server and native Erlang diagnostics. CI builds a Hex tarball locally without
publishing it; the Rust archive excludes the nested Gleam package and its build
products.

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

The Acceptance workflow's `Checkout CLI` job installs the current checkout with
`cargo install --path . --locked` into a temporary installation root, using the
default features and release profile. It uploads that executable as a workflow
artifact for the Linux embedding jobs. Each consumer downloads the same binary
and restores its executable permission; it does not install another CLI.

The `Embedding examples` matrix runs the ten guided examples in three groups.
Each group checks, formats, tests, and lints its examples sequentially. Their
integration tests execute each binary and compare exact output. Each example
has a separate log section; a failure stops that example's remaining commands
but does not skip the other examples or cancel sibling jobs. The group fails if
any of its examples fail.

Each group has its own cache key for the root workspace's Rust dependencies.
Cargo metadata and all build commands receive the job-wide `CARGO_TARGET_DIR`,
so preparation, tests, and Clippy use the cached target directory. Preparation
keeps its separate `geam-embedding` subdirectory there.

The `Rust embedding` job retains the application, manual embedding, execution
control, and async provider checks. Its capstone readiness and recovery checks
use the same installed binary, so CI also verifies that feature separation
preserves the default CLI installation. It starts without a separate Gleam
download step, checks locked readiness, and verifies that tracked application
files remain unchanged. It then makes generated source stale, requires
`embedding check` to fail, and runs production sync to restore
the exact committed file before formatting, testing, linting, and running the
application with the exact inventory report and its captured Gleam IO. The same
job requires one Geam package identity, the exact core/macros/stdlib/builtin application
profile, the text-pattern provider, and no CLI/JSON/Time/Erlang dependency.
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
path may populate caches. Dynamic-only checking does not compile Rust or run
build scripts. Prepared and both checking compile and run a disposable helper
using the consumer dependencies and compare the complete generated output set.
Dependency build scripts and provider registration can run; application main,
application build scripts and provider run-state initialization do not. All
choices preserve managed files on check. Runtime typed value and ownership
contracts remain in core tests.

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
on repeated runs. `callables` verifies Rust-created capturing functions,
generic constants and wrappers, alias identity, and custom-held reply callbacks.
`generic_box` verifies typed retention, cross-type
replacement, source semantics, and callback mapping without materialization.
`native_records` verifies declared symbols and records through actual stdlib
Dynamic decoding, bidirectional equality, dictionary key hashing, inspection,
and a typed callback with retained captures. Core owners separately prove
nominal/generic restoration, sealed conversion permissions, recursive values,
lazy native traversal, payload access, and release; the example does not replace
those tests.
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
thirteen provider examples select the current unreleased authoring surface through
repository-local patches and complete standalone execution. The independent
Provider SDK fixture remains the canonical low-level typed-host ABI acceptance
owner.

The [Acceptance workflow](../../.github/workflows/acceptance.yml) runs the twelve
provider examples other than `async_files` in four matrix groups. Groups
balance observed execution times rather than following
the guide's reading order. For each example, the job selects its exact
`provider_examples` test, runs the independent provider's tests, verifies its
Cargo package, and exports its Gleam package. Log sections and failure
annotations identify individual examples. A failure stops that example's
remaining commands but does not skip later examples or cancel sibling jobs;
the group fails if any example fails.

The parallel `Published provider` job has no repository checkout and
therefore cannot substitute path dependencies or checkout binaries for the
released artifacts it monitors. Formatting and Clippy remain in the Workspace
workflow. The Rust embedding job checks the independent `async_files` provider
and its standalone case, sharing its existing embedding build cache. It builds
the Gleam package with the checkout's local `geam` dependency; a Hex export of
that example requires a published `geam` dependency and is not a local gate.

Each group has a distinct cache key for the root workspace's Rust dependencies.
Within a job, the root test binary and independent providers use the checkout's
`target/` directory so Cargo can reuse matching build artifacts without changing
any workspace's lockfile. The generated runner still uses its temporary
project's `build/geam/target/`;
those isolated runner artifacts are not shared or cached between jobs.

The normal suite executes the full generated runner with the fixture's locked
Gleam and Rust dependencies. CI exports the standalone fixture's three local
Gleam dependencies and the same twelve example Gleam packages. It also packages the
two standalone fixture providers and every example provider. No test-only
fixture package is published. The text-pattern provider and matching Hex package
are release-coupled public documentation artifacts and share every Geam release
version.

The root package keeps seven explicit acceptance targets:

- `binary` starts the installed-shape `geam` process for command dispatch,
  process failures, pure execution, and IO/Echo ordering.
- `cross_crate_http` proves that the Pure Gleam `gleam_http` package works
  through the root facade and stdlib composition. HTTP is not a Geam built-in
  and has no provider crate.
- `provider_examples` executes the thirteen documented provider projects and the independent OTP fixture through
  the real binary and generated runners.
- `future_builtins` composes a macro-authored asynchronous provider with stdlib,
  JSON, and Time in a caller-driven Rust embedding scope.
- `prepared_embedding` verifies generated preparation, source-free execution,
  diagnostics and extracted Cargo package consumption.
- `standalone_build` verifies built executable assembly and source-free deployment.
- `standalone_distribution` combines built-ins and two independent providers
  in one canonical managed-project flow. Its Future cases verify exact outer
  entry completion and a generated Tokio host using controlled timers, loopback
  I/O, state initialization, repeated calls, and bounded shutdown. Process cases
  run the maintained service on Erlang and Geam, retain the generated profile
  across source changes, and fix ordinary and outer-Future domain termination.

Detailed project loading, provider selection validation, explicit provider
resolution, manifest, lock, and runner behavior remain in `geam-cli`; the root
targets only retain the process, facade, cross-crate, or distribution boundary
named above.

Source-level rejection fixtures live under categorized
`tests/fixtures/rejection/**/*.gleam` paths. They are reserved for public
boundary cases that are clearer as complete Gleam modules than as planner unit
tests.


## Execution Service Consumers

The [process service example](../../examples/provider/process_service) owns an
ordinary typed request/reply API over the shared Erlang process service. Its
independent provider tests use the original application, a manual host clock,
and exact results for missing names, timeout, termination and malformed native
replies. The [OTP fixture](../../tests/fixtures/otp_service) uses pinned original
OTP source and an independent profile-dependent provider. Its maintained source
hashes, native inventory and contract map separate the exercised integration
from a complete provider implementation of `gleam_otp`.

OTP owner unit tests live beside their implementations in `provider/src/`.
The separate `provider/tests/original_otp.rs` integration target composes the
provider's public component with the original OTP sources. Private unit support
and original-source lifecycle observers belong to their respective test targets;
the provider exposes no test-only API. The ordinary provider test command runs
both targets, including the cancellation and retained-callback scenarios.

Core owns static service composition and callable lifetime. `geam-erlang` owns
process identity, shared values, mailbox selection, clocks, monitor/link effects
and lifecycle cleanup. CLI owns service dependency admission and generated
profile assembly. Consumer tests exercise these public boundaries together;
their results do not replace any package's owner tests.

```sh
cargo test --manifest-path examples/provider/process_service/provider/Cargo.toml --locked
cargo test --manifest-path tests/fixtures/otp_service/provider/Cargo.toml --locked
cargo fmt --manifest-path tests/fixtures/otp_service/provider/Cargo.toml --check
cargo clippy --manifest-path tests/fixtures/otp_service/provider/Cargo.toml --all-targets --locked -- -D warnings
cargo test --package geam --test provider_examples --locked -- --exact runs_the_process_service_provider_on_the_builtin_mailbox
cargo test --package geam --test provider_examples --locked -- --exact otp_service::runs_profile_dependent_otp_callbacks_in_the_generated_standalone_host
cargo test --package geam --test prepared_embedding --locked -- process_consumers
```

The `process_consumers` tests generate both independently locked embeddings
before checking repeated generation, formatting and warnings-denied Clippy,
dynamic/prepared output, and standalone assembly. Generation must preserve the
tracked bindings and Cargo lock. They relocate each compiled executable, remove
the original sources/build tree, and run with an empty PATH. Prepared
distribution runs these checks, including embedding formatting, on Linux, macOS
and Windows. Workspace formats and lints the providers, while Acceptance runs
their original-source workflows.
These new consumers use their own Cargo locks and ordinary public Geam APIs.

Each provider also has an independent coverage closure. Run them sequentially,
with a fresh profile for each workspace. The denominator is the provider package
itself; its Geam dependencies retain their separate owner gates.

```sh
cargo llvm-cov clean --manifest-path examples/provider/process_service/provider/Cargo.toml --workspace
cargo llvm-cov --manifest-path examples/provider/process_service/provider/Cargo.toml --locked --summary-only --fail-under-lines 100 --fail-under-regions 100
cargo llvm-cov clean --manifest-path tests/fixtures/otp_service/provider/Cargo.toml --workspace
cargo llvm-cov --manifest-path tests/fixtures/otp_service/provider/Cargo.toml --workspace --locked --summary-only --fail-under-lines 100 --fail-under-regions 100
```

## Benchmark Tooling

The independently locked [`benchmarks/`](https://github.com/panarch/geam/tree/main/benchmarks) workspace
owns `geam-bench` (preparation, artifact admission, process execution and result
analysis) and `geam-benchmark-support` (the ordinary clock/environment/consumer
provider). These are tooling packages; benchmark results and private remote
machine administration are outside the repository's verification contract.

Build the checkout CLI and install Gleam `1.18.1`, Erlang/OTP `29` and the exact
Node.js version in `benchmarks/.node-version` before running its mandatory tests:

```sh
cargo build --package geam --bin geam --locked
cargo fetch --manifest-path benchmarks/Cargo.toml --locked
cargo test --manifest-path benchmarks/Cargo.toml --workspace --locked
cargo fmt --manifest-path benchmarks/Cargo.toml --all --check
cargo clippy --manifest-path benchmarks/Cargo.toml --workspace --all-targets --locked -- -D warnings
RUSTDOCFLAGS='-D warnings' cargo doc --manifest-path benchmarks/Cargo.toml --workspace --no-deps --locked
(cd benchmarks/project && gleam format --check src)
```

`GEAM_BENCH_TEST_GEAM` selects an existing checkout CLI instead of the default
`target/debug/geam`. The public workflow test prepares the actual shared suite,
relocates the complete executable bundle, deletes original source/build paths,
and removes compilers from its execution PATH. It runs all 54 cases on three
targets (162 processes / 486 smoke samples), two cases in four paired rounds
(16 processes / 48 samples), Geam-only execution and runtime-free analysis.
It also verifies interruption, collisions and changed evidence. The timings
are diagnostic; there are no performance thresholds or published result tables.
Owner tests separately fix protocols, case oracles, schedules, normalization,
receipts, artifact admission and typed provider behavior.
Failure tests use Rust writer/durability and process-control boundaries, actual
filesystem errors and acquired OS results. They verify partial writes, failed
synchronization, primary-error preservation and termination/reaping ownership.
Preparation and result admission check source changes and incomplete evidence.
The real workflow continues to execute compiled workloads; owner-level failure
inputs do not replace a workload evaluator. No C fixture or loader injection is
required.

Workspace owns Rust/Gleam formatting, warnings-denied Clippy and rustdoc.
Acceptance's mandatory Linux `Benchmark tooling` job reuses the `Checkout CLI`
artifact. Coverage has a separate benchmark-only closure; root-package or other
workspace coverage does not compensate for either benchmark package:

```sh
cargo llvm-cov clean --manifest-path benchmarks/Cargo.toml --workspace
cargo llvm-cov --manifest-path benchmarks/Cargo.toml --workspace --no-report --locked
cargo llvm-cov report --manifest-path benchmarks/Cargo.toml --package geam-bench --summary-only --fail-under-lines 100 --fail-under-regions 100
cargo llvm-cov report --manifest-path benchmarks/Cargo.toml --package geam-benchmark-support --summary-only --fail-under-lines 100 --fail-under-regions 100
```

Both line and full-scope region gates remain 100%. The detailed LLVM report
commands below also accept `--manifest-path benchmarks/Cargo.toml` and either
benchmark package name. Runtime/performance baselines, profiling and private
remote-host validation are separate explicitly scheduled work.

## Commands

With the Rust toolchain, Gleam `v1.18.1`, and Erlang/OTP `29` installed, run the
full test suite:

```sh
cargo fetch --locked
cargo fetch --manifest-path examples/embedding/callables/Cargo.toml --locked
cargo fetch --manifest-path examples/provider/process_service/embedding/Cargo.toml --locked
cargo fetch --manifest-path tests/fixtures/otp_service/embedding/Cargo.toml --locked
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
The `prepared_embedding` target also needs the separate callable consumer fetch
shown above; fetching the root workspace does not populate its independent lock.

When packaging a provider against this checkout, pass the `geam` path patch
through Cargo configuration (`--config` or `.cargo/config.toml`). Cargo removes
manifest-level patches when generating the package, so a patch in `Cargo.toml`
alone makes packaging depend on registry availability instead of the checkout.

The workspace's explicit default members include every production package, so
`cargo test --locked` remains equivalent for local use. CI spells out
`--workspace` so newly added internal packages cannot be omitted implicitly.

Run a package-owned compatibility suite directly:

```sh
cargo test --package geam-stdlib --test gleam_stdlib --locked
cargo test --package geam-json --test gleam_json --locked
cargo test --package geam-time --test gleam_time --locked
cargo test --package geam-builtin --locked
cargo test --package geam-erlang --locked
```

Run the root acceptance targets independently:

```sh
cargo test --package geam --test binary --locked
cargo test --package geam --test cross_crate_http --locked
cargo test --package geam --test provider_examples --locked
cargo test --package geam --test future_builtins --locked
cargo test --package geam --test prepared_embedding --locked
cargo test --package geam --test standalone_distribution --locked
cargo test --package geam --test standalone_build --locked
```

To run one provider example with the same exact selection used by its CI job:

```sh
cargo test --package geam --test provider_examples --locked -- \
  --exact runs_the_documented_text_tools_provider_across_three_modules
```

The unfiltered `provider_examples` command runs all thirteen documented examples and the OTP fixture locally.

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
It includes the optional Tokio adapter. The separate Acceptance `Host execution`
matrix also exercises that adapter and the public resumable embedding boundary
on Linux, macOS, and Windows; those platform checks do not replace owner coverage.

Run the core and macro closure:

```sh
cargo llvm-cov clean --workspace
cargo llvm-cov --no-report --package geam-core --package geam-macros --features geam-core/tokio --locked
cargo llvm-cov report --package geam-core --summary-only --fail-under-lines 100 --fail-under-regions 100
cargo llvm-cov report --package geam-macros --summary-only --fail-under-lines 100 --fail-under-regions 100
```

Run the built-in closure with Gleam `v1.18.1` available:

```sh
cargo llvm-cov clean --workspace
cargo llvm-cov --no-report --package geam-stdlib --package geam-json --package geam-time --package geam-builtin --package geam-erlang --locked
cargo llvm-cov report --package geam-stdlib --summary-only --fail-under-lines 100 --fail-under-regions 100
cargo llvm-cov report --package geam-json --summary-only --fail-under-lines 100 --fail-under-regions 100
cargo llvm-cov report --package geam-time --summary-only --fail-under-lines 100 --fail-under-regions 100
cargo llvm-cov report --package geam-builtin --summary-only --fail-under-lines 100 --fail-under-regions 100
cargo llvm-cov report --package geam-erlang --summary-only --fail-under-lines 100 --fail-under-regions 100
```

Run the CLI and binary closure with Gleam `v1.18.1` available:

```sh
cargo llvm-cov clean --workspace
cargo llvm-cov --no-report --package geam-cli --locked
cargo llvm-cov --no-clean --package geam --lib --test binary --locked --summary-only
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
