# Upstream Gleam Compiler Boundary

Geam does not own a separate source language front-end. It relies on Gleam's
published compiler front-end, then starts Geam-specific work at the typed AST
boundary. This document records the upstream baseline, what is used directly,
and where Geam's runtime-specific boundary begins.

## Baseline

Geam's compiler boundary reference is:

```text
Repository: https://github.com/gleam-lang/gleam
Release:    v1.18.1
Commit:     4a83802ca33a8a96227a1b332768725f232f9779
Published:  2026-08-01
Cargo:      geam-gleam-core 1.18.1-geam.2
```

The baseline is release-based rather than `main`-based so typed AST and
compiler-boundary behavior are compared against a published Gleam toolchain.
The release, commit, and publication date above refer to the
[upstream Gleam release](https://github.com/gleam-lang/gleam/releases/tag/v1.18.1)
(UTC), not the compiler crate's publication date.

The `geam-gleam-core` package and its compiler-component dependencies are
published from the release-tracking `panarch/gleam` mirror. Its
[packaging release](https://github.com/panarch/gleam/releases/tag/geam-v1.18.1-geam.2)
records the same upstream commit. Geam pins that package exactly; the `geam.2`
suffix is a mirror packaging revision, not a different Gleam release or Geam's
own lockstep version.

The official standard library has an independent baseline:

```text
Repository: https://github.com/gleam-lang/stdlib
Hex package: gleam_stdlib
Release:     v1.0.3
```

Geam does not bundle or patch that source. A dedicated integration test uses
Gleam CLI `v1.18.1` to download the locked package, then executes selected
official modules through Geam's resolved-project pipelines. Provider-free
modules use the plain pipeline, while modules whose selected closure contains
externals use the hosted pipeline with the explicit Rust provider bundle. Each
tracked module locks its public names, argument labels, and signatures while
allowing private implementation and declaration-order changes.

The first independently versioned Pure Gleam package baseline is:

```text
Repository:  https://github.com/gleam-lang/http
Hex package: gleam_http
Release:     v4.3.0
```

Geam does not bundle or patch this source either. Its compatibility suite locks
`gleam_http` and `gleam_stdlib` independently, then uses the hosted resolved
project pipeline because the selected stdlib closure requires the existing
explicit provider bundle.

The first independently versioned provider-backed package baseline is:

```text
Repository:  https://github.com/gleam-lang/json
Hex package: gleam_json
Release:     v3.1.0
```

Its compatibility suite locks `gleam_json` and `gleam_stdlib` independently.
The resolved project explicitly composes the separate stdlib and JSON provider
bundles; project loading does not infer either bundle.

The caller-clock package baseline is:

```text
Repository:  https://github.com/gleam-lang/time
Hex package: gleam_time
Release:     v1.8.0
```

Its compatibility suite locks `gleam_time` and `gleam_stdlib` independently.
The resolved project explicitly composes the stdlib and Time provider bundles;
the caller supplies both stdlib run state and the wall-clock source.

## Used Gleam Areas

The first compiler-boundary milestone imports the `gleam_core` Rust library
from the exact `geam-gleam-core` package recorded above. The primary compiler
areas used by Geam are:

```text
compiler-core/src/parse.rs
compiler-core/src/parse/error.rs
compiler-core/src/analyse.rs
compiler-core/src/type_.rs
compiler-core/src/ast.rs
compiler-core/src/ast/untyped.rs
compiler-core/src/ast/typed.rs
```

The boundary wrapper also uses Gleam support APIs required to run analyse in the
same shape as Gleam itself:

```text
compiler-core/src/build.rs
compiler-core/src/config.rs
compiler-core/src/line_numbers.rs
compiler-core/src/uid.rs
compiler-core/src/warning.rs
```

The Geam wrapper parses all supplied source text, derives a deterministic
dependency-first order, inserts Gleam's prelude and previously analysed module
interfaces, and runs `ModuleAnalyzerConstructor::infer_module` with one shared
ID generator. The result is a `TypedProgram` containing the complete typed
module graph. `compile_typed_module` is the one-module convenience view of this
same implementation.

Runtime behavior decisions that start after this compiler boundary are recorded
in [runtime-semantics.md](runtime-semantics.md).

## Geam Boundary

Geam intentionally does not define a source AST or source-language compiler. Its
current boundary is:

```text
resolved project or module sources
-> Gleam TypedProgram
-> Geam ModulePlan
-> Geam ExecutionPlan
-> Geam runtime Value
```

Geam-specific profile validation belongs in planning from Gleam's typed AST
into `ModulePlan`. Every supplied module body is validated, including
dependency definitions that are unreachable from the root entry. The following
consuming lowering into `ExecutionPlan` is total and does not add another
validation boundary. Unsupported execution semantics are therefore rejected
before executable lowering and evaluation.

The earlier Geam-owned parser and analyse prototype has been removed. The active
direction is to rely on Gleam's typed AST and build Geam profile validation,
lowering, and execution after that boundary.

This document intentionally avoids mirroring Geam's current internal file tree.
Use the repository itself as the source of truth for exact module and test file
locations.

The current public compiler-boundary API is:

```rust
pub fn compile_typed_module(
    module_name: impl Into<ecow::EcoString>,
    path: impl Into<camino::Utf8PathBuf>,
    src: &str,
) -> Result<gleam_core::ast::TypedModule, geam::frontend::FrontendError>

pub fn compile_typed_program(
    root_module: impl Into<ecow::EcoString>,
    modules: impl IntoIterator<Item = geam::ModuleSource>,
) -> Result<geam::TypedProgram, geam::frontend::FrontendError>

pub fn compile_typed_package_program(
    root_package: impl Into<ecow::EcoString>,
    root_module: impl Into<ecow::EcoString>,
    packages: impl IntoIterator<Item = geam::PackageSource>,
) -> Result<geam::TypedProgram, geam::frontend::FrontendError>

pub fn compile_typed_project(
    project_root: impl Into<camino::Utf8PathBuf>,
    root_module: impl Into<ecow::EcoString>,
) -> Result<geam::TypedProgram, geam::ProjectError>

pub fn compile_typed_host_program<Profile: geam::HostProfile>(
    root_package: impl Into<ecow::EcoString>,
    root_module: impl Into<ecow::EcoString>,
    packages: impl IntoIterator<Item = geam::PackageSource>,
    hosts: geam::HostProviderSet<Profile>,
) -> Result<geam::HostedTypedProgram<Profile>, geam::FrontendError>

pub fn compile_typed_host_project<Profile: geam::HostProfile>(
    project_root: impl Into<camino::Utf8PathBuf>,
    root_module: impl Into<ecow::EcoString>,
    hosts: geam::HostProviderSet<Profile>,
) -> Result<geam::HostedTypedProgram<Profile>, geam::ProjectError>
```

`compile_typed_project` is a read-only loader for a Gleam project whose
dependencies have already been resolved. It reads `gleam.toml`,
`manifest.toml`, root sources, and resolved Hex/Git/Local package sources.
It never runs Gleam CLI, downloads dependencies, or modifies project files.
The loader follows production dependencies and selects the
`Target::Erlang` import closure rooted at the requested module. Every selected
module body is then analysed and planned in full.

`compile_typed_host_project` uses that same manifest, source-catalog,
import-closure, and parse owner, then combines the parsed program with an
explicit host provider set. Missing providers and declaration linkage remain
hosted planning errors; the project loader does not infer, download, or inject
providers.

`compile_typed_host_program` adds package-qualified source-less module
interfaces to the same in-memory analysis graph. `HostProviderModule` instead
binds implementations to existing source external declarations without
replacing their analyzer-owned interface. Neither path generates fake Gleam
bodies. Host provenance and Rust implementations remain Geam-owned data and
are not interpreted as pure Gleam definitions.

Host registrations provide an exact typed schema to the frontend. Direct
closures support the documented scalar families and zero through seven
arguments; scoped registrations describe generic, compound, custom, and
function values without exposing materialized runtime `Value`s. Source-backed
constructorless external types can bind profile-owned Rust payloads, including
exact typed Gleam values and existential values carrying their specialized
Gleam shapes. Existential decode remains a typed host-call operation and shape
mismatch is provider-level `Option` semantics. These are Geam host-ABI
constraints, not additions to Gleam's analyzer or typed AST. See [runtime
semantics](runtime-semantics.md) for the value, state, specialization,
re-entry, and failure contracts.

External storage providers also define source equality, runtime hashing, and
canonical inspection. Equal payloads must hash equally, collisions are checked
with source equality, and retained Gleam values are available only through the
narrow operation-specific contexts. Hashes are runtime indexes rather than a
stable package or serialization contract.

Private transient-style external APIs can be represented by returning new
persistent payload versions that share immutable retained entries. This does
not add a general mutable external-value model or cyclic runtime graph support.

### Gleam Stdlib v1.0.3 Compatibility

Checked modules have exact public-surface coverage and execute unchanged
official source through the upstream integration suite. An unchecked module is
not necessarily rejected; it has not yet been verified end to end.
Geam currently verifies all 19 public modules in this baseline.

#### No Module-Specific Provider

- [x] `gleam/bool`
- [x] `gleam/bytes_tree`
- [x] `gleam/function`
- [x] `gleam/list`
- [x] `gleam/option`
- [x] `gleam/order`
- [x] `gleam/pair`
- [x] `gleam/result`
- [x] `gleam/set`

#### Explicit Rust Provider

- [x] `gleam/bit_array`
- [x] `gleam/dict`
- [x] `gleam/dynamic`
- [x] `gleam/dynamic/decode`
- [x] `gleam/float`
- [x] `gleam/int`
- [x] `gleam/io`
- [x] `gleam/string`
- [x] `gleam/string_tree`
- [x] `gleam/uri`

`geam::gleam_stdlib::host_providers` supplies the explicit provider bundle for
provider-backed modules. Callers compose that bundle into a `HostProviderSet`;
project loading selects only providers in the resolved source closure and does
not infer or inject the bundle. Functions with valid Gleam fallback bodies,
including the annotated `gleam/bytes_tree` operations and `gleam/uri.parse`,
continue to compile and execute from the unchanged package source. The URI
provider binds only its five bodyless string and percent-codec externals.

The provider-backed modules retain their source-facing value distinctions.
Dictionary lookup uses source hashing followed by source equality within a
collision bucket. Dynamic values retain their exact specialized shape for
typed Decode operations. StringTree uses persistent acyclic structure, and
BitArray values preserve logical bit ranges over shared immutable backing.
Random operations use caller-owned `GleamStdlibRunState`; there is no hidden
seed or global random state. Official IO operations emit owned stdout and
stderr events through the `IoSink` projected by `GleamStdlibHostProfile`. The
default run state collects those events, and project loading does not select a
terminal destination.

### Gleam HTTP v4.3.0 Compatibility

The HTTP compatibility suite tracks all five unchanged package modules:

- [x] `gleam/http`
- [x] `gleam/http/cookie`
- [x] `gleam/http/request`
- [x] `gleam/http/response`
- [x] `gleam/http/service`

The package declares no external functions or external types, so Geam adds no
HTTP provider. Its 44 public functions execute through official source, while
the suite separately fixes 9 public custom types and 3 public type aliases.
The deprecated service aliases and functions remain part of the `v4.3.0`
compatibility surface.

This baseline covers HTTP values and transformations, method and scheme
handling, cookies, URI-backed requests, responses, content disposition, and
streaming multipart parsing. It does not provide a client, server, socket,
transport, or network capability. Provider selection remains explicit and the
project loader does not inject the stdlib bundle merely because an HTTP module
is imported.

### Gleam JSON v3.1.0 Compatibility

The JSON compatibility suite tracks unchanged `gleam/json`, including all 14
public functions, the constructorless `Json` type, and all four `DecodeError`
constructors. The explicit package provider binds the external `Json` storage
and the ten Erlang callbacks. JavaScript-only `decode_string` is not registered;
the selected Erlang implementation uses `decode_to_dynamic` instead.

Encoded Json values share persistent StringTree structure. Array and object
construction retain child trees, and `to_string_tree` reuses the same root;
flattening occurs only for string conversion or sealed inspection. Parsing
consumes bytes iteratively and constructs exact Dynamic scalar, List, and Dict
values directly rather than a generic JSON AST. Nested values are assembled
child-first to keep the external graph acyclic. JSON objects become
Dynamic-keyed dictionaries whose keys are Dynamic String values, matching the
official Decode ABI; duplicate encoded object fields remain ordered, while
decoded dictionaries preserve the first key occurrence.

### Gleam Time v1.8.0 Compatibility

The Time compatibility suite tracks all three unchanged package modules:

- [x] `gleam/time/duration`
- [x] `gleam/time/calendar`
- [x] `gleam/time/timestamp`

The explicit package provider binds only
`calendar.local_time_offset_seconds` and `timestamp.get_system_time`. All 34
public functions, 6 public types, and the `utc_offset` and `unix_epoch`
constants are exercised through official source. Duration arithmetic,
calendar conversion, RFC3339 parsing, and formatting remain Pure Gleam.

`GleamTimeRunState` owns the stdlib state and a caller-selected `TimeSource`.
The source reports a non-monotonic wall clock and the current system UTC offset;
it does not expose timezone history, monotonic time, timers, or sleep. Provider
failures remain explicit host failures rather than silent UTC or clock
fallbacks.

The current public execution APIs are:

```rust
pub fn plan_module(
    module: gleam_core::ast::TypedModule,
) -> Result<geam::ModulePlan, geam::PlanError>

pub fn plan_module_with_source(
    module: gleam_core::ast::TypedModule,
    source_context: geam::SourceContext,
) -> Result<geam::ModulePlan, geam::PlanError>

pub fn plan_program(
    program: geam::TypedProgram,
) -> Result<geam::ModulePlan, geam::PlanError>

pub fn plan_host_program<Profile: geam::HostProfile>(
    program: geam::HostedTypedProgram<Profile>,
) -> Result<geam::HostedModulePlan<Profile>, geam::PlanError>

pub fn run_main(
    plan: &geam::ExecutionPlan,
    echo: &mut dyn geam::EchoSink,
) -> Result<geam::Value, geam::ExecutionError>

impl geam::ExecutionPlan {
    pub fn explain(&self) -> geam::ExecutionPlanExplanation<'_>
}

impl<Profile: geam::HostProfile> geam::HostedExecution<Profile> {
    pub fn try_from_module_plan(
        plan: geam::HostedModulePlan<Profile>,
    ) -> Result<Self, geam::HostSpecializationError>

    pub fn run_main(
        &self,
        state: &mut Profile::RunState,
        echo: &mut dyn geam::EchoSink,
    ) -> Result<geam::Value, geam::ExecutionError>

    pub fn explain(&self) -> geam::ExecutionPlanExplanation<'_>
}
```

`ExecutionPlan::from_module_plan(module_plan)` consumes the inspectable
`ModulePlan` and produces the runtime-only plan accepted by `run_main`. Its raw
execution nodes remain opaque, while `ExecutionPlan::explain()` provides a
human-readable view of lowered functions, constant programs, typed values,
instructions, block parameters, and control-flow edges. Its text is not a
machine-stable serialization format.

The hosted pipeline is intentionally a separate type-level boundary. A
`HostedModulePlan` exposes source templates and host schemas while carrying
registered callbacks in a private sidecar. `HostedExecution` retains only the
implementations selected by lowering, pairs them with first-use host targets,
and cannot be passed to the plain `run_main` function.
`HostedExecution::try_from_module_plan` seals entry-reachable generic
specializations into concrete runtime storage. Unused providers do not block
execution. Source external providers are selected before body planning: an
exact provider wins, a provider-less declaration with a Gleam body uses that
fallback, and a bodyless declaration without a provider is rejected. Provider
state remains owned by the caller and is borrowed only for
`HostedExecution::run_main`.

The caller supplies the `EchoSink` used by `run_main`. Each emitted
`EchoOutput` owns its materialized value, optional message, and compact source
location; Geam does not select an output destination for the host.
The official `gleam/io` provider uses a separate caller-owned `IoSink` for
stdout and stderr text events. Echo and stdlib IO do not share a hidden queue.

## Intentionally Out Of Scope

These areas are intentionally excluded from the first compiler-boundary
milestone:

- LSP node lookup helpers.
- Erlang and JavaScript target metadata.
- Code generation metadata.
- Package resolution, dependency download, package cache mutation, and artifact
  writing.
- Source-less external type declarations, async providers, retained or
  Rust-created callbacks, and CLI behavior.

## Current Source Boundary

Source acceptance follows Gleam `v1.18.1` parser and analyse rules. Geam's
smaller execution profile is enforced by typed-AST planning, not by forking
Gleam's parser or type inferencer.

## Sync Policy

This document is maintained manually when the upstream baseline or mirror
packaging revision changes. Geam release-version automation must not update
compiler or official package baselines.

The [Workspace workflow](../.github/workflows/workspace.yml) checks the `Cargo:`
line against the compiler version resolved by `cargo metadata --locked`. This
only detects pin drift: release identity, publication date, and compatibility
claims still require manual verification against the upstream and mirror
releases. The check does not infer or rewrite that narrative.

When updating Geam to a newer Gleam baseline, record:

- Old and new Gleam release tags.
- Old and new commit hashes.
- Old and new `geam-gleam-core` package versions and mirror releases.
- Old and new `gleam_stdlib` integration releases when that baseline changes.
- Old and new `gleam_http` integration releases when that baseline changes.
- Old and new `gleam_json` integration releases when that baseline changes.
- Old and new `gleam_time` integration releases when that baseline changes.
- Compiler-boundary files compared.
- Geam compiler-boundary wrapper or lowering changes made.
- Profile/lowering fixture changes.
- License or provenance changes, if any.

The README keeps the current upstream release visible. This document records
the exact upstream commit and packaged compiler version, and explains the
practical differences between Geam and the referenced Gleam version.
