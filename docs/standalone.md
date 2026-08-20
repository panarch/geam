# Standalone Execution

The `geam` binary prepares and runs already resolved Gleam projects through a
project-local Rust runner. Gleam remains responsible for Gleam dependencies;
Cargo resolves Geam and approved Rust provider crates.

## Quick Start

Run commands from the project directory or one of its descendants. Geam walks
upward to the first `gleam.toml`.

```sh
gleam add gleam_json
geam prepare
geam run
```

The default entry module is the root package name. Select another supplied
module explicitly when needed:

```sh
geam prepare --module worker
geam run --module worker
```

`prepare` resolves the selected source closure, reconciles providers, generates
the static runner, and checks planning and hosted sealing. It does not initialize
provider state or execute `main`. `run` performs the same reconciliation, then
initializes state and executes directly without a separate check run. A main
return value is not printed; Gleam IO writes to its selected stream and Echo
writes to stderr in source order.

## Managed Files

Geam creates these files in the Gleam project root:

```text
Cargo.toml
Cargo.lock
build/geam/runner.rs
build/geam/target/
```

The generated `Cargo.toml` begins with an exact Geam managed marker and has
`publish = false`. Provider selections are ordinary exact Cargo dependencies,
and `Cargo.lock` is their only lock. Runner source and Cargo artifacts belong
under `build/geam/`; add that directory to source-control ignores.

Geam regenerates the whole managed manifest canonically and leaves unchanged
bytes alone. It removes selections for Gleam packages no longer present in the
resolved manifest. It refuses to modify any existing `Cargo.toml` without its
managed marker. Rust applications and libraries that own their Cargo manifest
use the manual embedding API; automatic publishable embedding is tracked
separately by [#115](https://github.com/panarch/geam/issues/115).

## Provider Selection

Built-in stdlib, JSON, and Time support occupies the first entries in the same
static component graph as approved dependencies, but is never searched on
crates.io, added to the managed manifest, approved, or configured. For another
resolved package with mandatory Erlang externals, Geam derives a crates.io
search name from the Gleam package. A requirement for `company_image` searches
for `geam-company-image` and considers that exact crate plus kebab-case
alternatives under `geam-company-image-*`. Before presenting any result, Geam
verifies the sparse-index version, archive checksum, packaged provider metadata,
exact `company_image` target, and declared Gleam version range. Explicit
registry, path, and Git selections bypass discovery naming because the user
names and approves the crate directly.

Metadata compatibility is not an endorsement. Geam presents verified
candidates and asks before adding native code. It does not select a new provider
when stdin is noninteractive, and it asks again before replacing an approved
provider that no longer supports the resolved Gleam version. Existing compatible
exact selections remain pinned until changed explicitly.

Explicit selection is itself approval:

```sh
geam provider add geam-company-image@0.3.1
geam provider add --path ../geam-company-image
geam provider add --path ../provider-workspace --package geam-company-image
geam provider add --git https://example.com/provider.git --rev COMMIT
geam provider remove company_image
```

Registry versions are recorded exactly. Git revisions and path packages follow
Cargo source and lock semantics. One Gleam package has at most one provider, and
provider metadata must declare that exact package and a compatible Hex version
range. Provider crates export their component as `Component` at the crate root.

## Configuration

Pass provider configuration only when running:

```sh
geam run \
  --provider-config company_image=config/company_image.toml \
  --provider-config search=config/search.toml
```

Paths are resolved from the invocation directory. Each file is a top-level TOML
table containing strings, signed 64-bit integers, floats, booleans, arrays, and
recursive tables. Unknown packages, duplicate entries, TOML datetimes, and
invalid files are command errors. Omitting a file supplies an empty
configuration; a component that requires values reports its own initialization
error.

Configuration, credentials, state, and runtime values are never stored in Cargo
metadata, generated Rust source, process-global state, or execution plans.

## Failure Boundaries

- Missing `manifest.toml` or downloaded package source triggers one
  `gleam deps download` retry. Invalid project data and compiler errors do not.
- Registry and Cargo failures preserve the failed operation, status, and stderr.
- Configured provider initialization and runner capability construction finish
  before planning and execution.
- Missing registrations and schema mismatches remain hosted linkage errors.
- IO already emitted before a later source panic or host failure remains visible.
- OS output failures are CLI output errors, not Gleam execution errors.

Provider discovery never extracts an archive or executes code. Cargo first sees
the selected crate only after approval, when it locks and builds the generated
static runner.
