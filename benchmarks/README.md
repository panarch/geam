# Prepared Execution Benchmarks

This independent workspace runs the same Gleam workloads on a Geam release
executable, official Gleam Erlang output on OTP, and official Gleam JavaScript
output on Node.js. It measures workload execution. Preparation and application
startup finish outside the timed batches.

The repository provides the cases, preparation, execution and local analysis.
It does not contain published performance results. Artifact transfer, SSH,
measurement-host scheduling and result retrieval can wrap these foreground
commands separately.

## Prepare

From the repository root, install the Rust toolchain in
[`rust-toolchain.toml`](../rust-toolchain.toml), Gleam **1.18.1**, Erlang/OTP **29**,
and the exact Node.js version in [`.node-version`](.node-version). `git` and
`file` are required for preparation; native dependency inspection uses `otool`
and `xcrun` on macOS or `ldd` on Linux. Dependency acquisition may need network
access. The Gleam package versions are pinned in
[`project/manifest.toml`](project/manifest.toml).

```sh
cargo build --package geam --bin geam --locked
cargo build --manifest-path benchmarks/Cargo.toml --package geam-bench --release --locked

benchmarks/target/release/geam-bench prepare \
  --checkout . --suite benchmarks --geam target/debug/geam \
  --output benchmarks/bundles/current
```

`--checkout` selects the Geam runtime source. `--geam` selects the CLI used to
prepare it; rebuild that CLI from the intended revision. `--suite` selects the
shared benchmark workspace. Its provider, driver, workloads, FFI and controller
source hashes are retained alongside the runtime commit, dirty source hashes,
resolved application Cargo lock, build tools, flags and executable hashes.
Preparation copies its inputs into owned build space and leaves the source
project and its committed locks unchanged. Its owned output/cache directories
are explicitly listed as excluded outputs when nested inside the runtime
checkout, so generated logs are not mistaken for source changes. The optional
`--build-directory benchmarks/build-workspaces/current` reuses an exclusively
locked runner-owned build cache; it is not an execution dependency.

The default targets are `geam,erlang,javascript`. For a Geam-only bundle add
`--targets geam`; it needs neither Node nor Erlang. An Erlang or JavaScript-only
bundle does not need Rust or a Geam CLI during preparation, but still uses the
already built controller and Gleam. The controller itself is a Rust executable.

Geam uses ordinary `provider add --path` and `build --release`. Erlang uses
`gleam export erlang-shipment`, including its generated application and entry
modules. JavaScript uses the complete official compiled dependency tree and a
relative entry module. The runner seals `bundle.json` only after all payloads
and their hashes are complete.

## Run A Prepared Bundle

```sh
benchmarks/bundles/current/payload/controller run \
  --bundle benchmarks/bundles/current \
  --output benchmarks/runs/smoke
```

Always use the controller shipped in the bundle. Copy or archive the **whole
bundle directory**, preserving executable permissions, and invoke the same
command using its new path. Execution needs neither the source checkout nor
Rust, Cargo, Gleam or the build cache. Geam-only execution needs no backend
runtime. Selected Erlang and JavaScript targets require `erl` and `node` on
`PATH`, matching the runtime versions recorded during preparation.

Bundles are native to the build operating system and architecture. A transferred
Geam executable also needs the recorded system libraries and a compatible OS
version/deployment target. macOS and Linux are the supported runner platforms;
there is no cross-compilation or Windows runner contract. Building on one Mac
and executing on another requires matching these native requirements.

Select exact maintained case identities and a target subset:

```sh
benchmarks/bundles/current/payload/controller cases
benchmarks/bundles/current/payload/controller run \
  --bundle benchmarks/bundles/current --targets geam \
  --case bit_checksum/4000,parse_sum/4000 \
  --output benchmarks/runs/bitarray-smoke
```

`--case` accepts a comma-separated list or repeated arguments. Omit it for all
54 cases. Unknown or duplicate cases and targets are rejected. Each output
path must be new; commands never overwrite an existing run or report.

For an explicitly scheduled performance measurement on an otherwise idle host:

```sh
benchmarks/bundles/current/payload/controller run \
  --bundle benchmarks/bundles/current --mode baseline \
  --machine "CPU model; memory; OS version; power configuration" \
  --output benchmarks/runs/baseline
```

The runner executes one process at a time. Host-wide exclusivity, background
load, power, sleep and thermal conditions remain the operator's responsibility.
Do not compile, run tests or profile concurrently with a performance experiment.
`--timeout-seconds` bounds each measurement process (default 240). SIGINT and
SIGTERM stop the owned process group and retain an incomplete attempt; a retry
uses a new output directory.

## Compare Geam Revisions

Build each revision's CLI and prepare two bundles with the **same benchmark
suite and controller**, compiler versions and build settings. `--checkout` and
`--geam` may select different revisions; their exact runtime sources, CLI hashes
and resolved application dependencies remain visible in each manifest.

```sh
benchmarks/bundles/candidate/payload/controller compare \
  --baseline benchmarks/bundles/baseline \
  --candidate benchmarks/bundles/candidate \
  --case callback_control/1,bit_checksum/100 \
  --output benchmarks/runs/paired-smoke
```

Both bundles must contain Geam and pass shared-input compatibility checks.
Comparison uses four rounds: baseline/candidate on odd rounds and
candidate/baseline on even rounds, with case order reversed on even rounds.
`--mode baseline --machine "..."` selects the longer per-process budget without
changing that schedule. Every pair is retained, including slower candidates.
Identical-binary smoke comparisons verify the command, not a performance change.

## Analyze And Inspect Evidence

```sh
benchmarks/bundles/current/payload/controller analyze \
  --input benchmarks/runs/smoke --output benchmarks/reports/smoke
```

Analysis needs no source, compiler, Node or Erlang. It validates the completed
run's hashes, declared process matrix, receipts, raw sample protocol and
checksums, then reconstructs the same JSON and Markdown summaries. A transferred
run is sufficient input; its original bundle is not needed for analysis.

A bundle contains `bundle.json`, immutable `payload/` and preparation receipts.
The manifest distinguishes build-host information from runtime-source and
suite identities. A run contains:

- `configuration.json`: mode, budget, ordered case/target selection, complete
  bundle manifests and measurement-host information/environment overrides.
- `metadata/`: runtime and host probes with their command receipts and outputs.
- `processes/000000/`, etc.: `step.json`, `start.json`, `finish.json`, `stdout`
  and `stderr` for every attempted process, in execution order. Commands and
  explicit environment changes are recorded before spawning.
- `raw.jsonl`: every accepted batch, including variant, round, iterations,
  warmup evidence, elapsed time and independent checksum.
- `summary.json`, `comparisons.json`, `REPORT.md`: per-case statistics, ratios
  and the human-readable table. Ratios are numerator/denominator; above one
  means the numerator took longer. Paired output retains each round's ratio.
- `complete.json`: written last, only after the entire expected matrix and
  immutable bundle pass validation. It fingerprints the original evidence.

Failure, timeout, cancellation or invalid data yields a nonzero exit, preserves
available receipts/raw output and leaves no accepted completion record.
`failure.txt` records the failure when possible. Stderr retains the original
cause if saving that file also fails.
Incomplete attempts cannot be analyzed or silently resumed. Derived reports can
be regenerated from accepted raw evidence; they are not evidence inputs.

## Measurement Method

Every target uses the same [driver](project/src/geam_benchmarks.gleam),
[measurement loop](project/src/geam_benchmarks/measurement.gleam) and source
validation. The Rust controller independently checks expected results.

| Mode | Fresh rounds | Warmup/process | Batches/process | Target batch |
| --- | ---: | ---: | ---: | ---: |
| Smoke | 1 | 20 ms | 3 | 10 ms |
| Baseline | 3 | 2000 ms | 20 | 100 ms |
| Paired smoke | 4 | 20 ms | 3 | 10 ms |
| Paired baseline | 4 | 2000 ms | 20 | 100 ms |

Ordinary target order rotates each round. Calibration selects between one and
one million iterations. Very small controls can hit that cap before the target
duration; slow operations can exceed the target. Actual batch durations remain
in the report. Smoke timings are correctness diagnostics only.

Input construction, reference results, warmup, result validation and JSON output
are outside timed batches. Allocations, normal reclamation, the shared callback
and repetition loop, and result consumption are included. There are two clock
reads per batch. Geam's consumer uses `std::hint::black_box` without traversing
the returned generic value; Erlang stores the last value in its process
dictionary, and JavaScript stores it on `globalThis`. The final batch result
may be reclaimed outside its measured interval. These costs are not subtracted.

Normalize each batch by its iteration count; use the median of process medians
as the central value. Keep every process median, pooled batch-average IQR and
batch-duration range. The IQR is neither individual-operation latency nor a
confidence interval. No outliers, slower pairs or callback costs are removed,
and no aggregate runtime score is produced. `callback_control` shows the
optimized minimal path; it is not a fixed overhead to subtract from other cases.

## Maintained Workloads

| Family | Sizes | Measured work |
| --- | --- | --- |
| `callback_control` | 1 | Shared callback and result consumer |
| `count_ones`, `count_ones_assert`, `count_ones_equal` | 100, 1000, 10000 | List traversal through matching, assertion and equality |
| `odd_nums_between` | 100, 1000, 10000 | Construct and reverse odd integers |
| `slice_prefix`, `slice_suffix` | 100, 1000, 10000 | Copy a slice from 10001 elements |
| `arithmetic` | 100, 1000 | Integer arithmetic and tail recursion |
| `capturing_fold` | 100, 1000 | Fold through a captured function |
| `capture_chain` | 1, 9, 101, 201, 401, 801 | Build and invoke nested captures |
| `custom_match` | 100, 1000 | Construct and match custom values |
| `string_fields` | 100, 1000 | Split, trim and join repeated fields |
| `string_prefixes` | 0, 8, 1024, 4096, 16384 | Match successive string prefixes |
| `string_graphemes`, `string_graphemes_unicode` | 0, 8, 1024, 4096, 16384 | ASCII and mixed Unicode grapheme traversal |
| `bit_checksum`, `parse_sum` | 100, 1000, 4000 | Four-byte record checksum and decimal record parsing |

There are 54 cases across 17 families, including the established 4000-record
BitArray scaling points. Allocation instrumentation and diagnostic mutation
probes are separate from this timing suite.

The count, odd-number and slice algorithms originate in the
[official Gleam list benchmark](https://github.com/gleam-lang/gleam/blob/19bf207ebb7d953ea1391f041da48c214ee1440a/benchmark/list/test/list_test.gleam)
at commit `19bf207ebb7d953ea1391f041da48c214ee1440a`, under Apache-2.0.
Their bodies are preserved. The former `list.range` constructor uses an
`int.range` fold with the same order/values. Count input is inclusive: size 100
has 101 elements and 34 ones. Odd-number input excludes its upper bound. Slice
input is 0 through 10000, with exclusive suffix endpoint 10000, rather than the
upstream million-element range. These common sizes apply to every target.
Other workloads are local representative cases; the decimal parser does not
represent general CSV/JSON performance. All integers remain in JavaScript's
exact-integer range.

To add a maintained case, keep the shared input/algorithm/source checks together,
add its independently derived Rust oracle and catalogue entry in
[`runner/src/cases.rs`](runner/src/cases.rs), and run the mandatory workflow on
all three targets. The test's declared case/sample totals must be updated
explicitly when the catalogue grows. Never replace expected checksums with
values copied from a measured target.

## Development Checks

Build the checkout CLI first, then run:

```sh
cargo test --manifest-path benchmarks/Cargo.toml --workspace --locked
cargo fmt --manifest-path benchmarks/Cargo.toml --all --check
cargo clippy --manifest-path benchmarks/Cargo.toml --workspace --all-targets --locked -- -D warnings
RUSTDOCFLAGS='-D warnings' cargo doc --manifest-path benchmarks/Cargo.toml --workspace --no-deps --locked
(cd benchmarks/project && gleam format --check src)
```

`GEAM_BENCH_TEST_GEAM` can select an already built checkout CLI; otherwise the
workflow test uses `target/debug/geam`. That mandatory test prepares a real
bundle, moves it, deletes its source/build inputs, and executes all 54 cases on
three targets with build tools absent from `PATH`: 162 processes and 486 samples.
It also checks paired smoke (16 processes/48 samples), Geam-only execution,
offline reconstruction, cancellation, collisions and corrupt evidence.
Rust owner tests check filesystem failures, partial writes, durable completion,
failed evidence receipts and process cleanup. They use real files and children,
with narrow writer and process-control contracts for failure cases. Preparation
and result admission reject incomplete evidence before publishing a success
marker. These tests require no C fixture or shared-library injection.

The [testing guide](../docs/development/testing.md#benchmark-tooling) records
independent coverage commands and CI ownership. CI checks correctness and
contracts, without performance thresholds or publication of timing reports.
