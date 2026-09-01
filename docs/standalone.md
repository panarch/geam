# Run a Gleam project

Already have a Gleam application? Keep writing and managing it as a Gleam
project. Geam supplies the Rust host behind it, so Rust remains an
implementation detail rather than another application you must maintain.

Geam reads the resolved Gleam project, prepares a project-local Rust runner,
and executes the selected Gleam module. You choose any required native
providers; Geam connects them and looks after the runner.

## Before you start

This guide assumes Rust `1.96` or newer and Gleam `v1.18.1` are installed.
Install the Geam command with:

```sh
cargo install geam --locked
```

Run Geam from a directory containing `gleam.toml` or one of its descendants.
Geam walks upward to the nearest Gleam project root.

## Run your first project

Create a Gleam application or move into an existing one:

```sh
gleam new hello_geam
cd hello_geam
```

Run the application:

```sh
geam run
```

That is the complete first workflow. On the first run, Geam resolves the
selected source closure, acquires locked Gleam dependencies when needed,
selects the required built-in and approved external providers, prepares the
Rust runner, and then executes the package module's `main`.

When you want to prepare and check the runner without executing application
code, use:

```sh
geam prepare
```

`prepare` performs the same reconciliation and verifies that the complete
program can be planned and sealed. `run` continues by initializing provider
state and starting the application. Gleam IO keeps its selected output stream;
language Echo is written to stderr. A value returned by `main` is not printed
automatically.

## Keep working in Gleam

Edit the Gleam project as usual, then run:

```sh
geam run
```

Use `prepare` when you want to verify the complete runner without executing
application code, such as before review or while diagnosing provider setup:

```sh
geam prepare
```

Both commands select the root package module by default. Choose another module
from the resolved project explicitly when it owns the entry point:

```sh
geam prepare --module worker
geam run --module worker
```

## Use Gleam packages

Add Gleam dependencies exactly as you normally would:

```sh
gleam add gleam_json
```

Adding a dependency does not by itself add work to the Geam runner. The package
must be imported by the selected module or its source closure. This keeps
unused dependencies and their native requirements out of the executable.

Geam includes explicit support for the verified `gleam_stdlib`, `gleam_json`,
and `gleam_time` integrations. These built-in components do not require
registry discovery or native-code approval. `gleam_http` uses its unchanged
Gleam source and the stdlib support selected by its imported closure.

See [compatibility](reference/compatibility.md) for the exact package baselines
and supported effects.

## Bring Rust capabilities to a Gleam package

A Gleam package may already pair its source API with an Erlang or JavaScript
external implementation. A Geam provider lets the same package gain a Rust
implementation without moving that API out of Gleam.

When an imported source closure contains a required external that is not built
in, Geam searches crates.io for metadata-compatible provider crates and asks
before adding native code:

```text
Gleam package company_image 1.4.0 requires native provider code.
Metadata compatibility is not an endorsement.
  1. geam-company-image 0.3.1 (Gleam >= 1.0.0 and < 2.0.0)
Approve geam-company-image 0.3.1? [y/N]
```

Approval records an exact Cargo dependency. Geam never treats matching metadata
as consent, and noninteractive commands do not approve a new provider.

Before presenting a registry candidate, Geam checks its sparse-index version,
archive checksum, packaged provider metadata, exact target Gleam package, and
declared package-version range. Discovery does not extract the crate or execute
provider code. Cargo receives the selected crate only after approval.

You can select a provider explicitly before preparation:

```sh
geam provider add geam-company-image@0.3.1
geam provider add --path ../geam-company-image
geam provider add --git https://example.com/provider.git --rev COMMIT
```

Inspect and remove stored external selections without compiling or contacting
a registry:

```sh
geam provider list
geam provider remove company_image
```

An explicit selection is still verified against its packaged provider metadata
and the resolved Gleam package. One Gleam package has at most one selected
external provider. Built-in components are not stored selections and do not
appear in `provider list`.

To implement a provider, start with [host provider authoring](host-providers.md).

## Pass provider configuration at run time

Pass runtime configuration by Gleam package name when starting the application:

```sh
geam run \
  --provider-config company_image=config/company_image.toml \
  --provider-config search=config/search.toml
```

Each file is a top-level TOML table containing strings, signed 64-bit integers,
floats, booleans, arrays, and recursive tables. Paths are resolved from the
directory where the command is invoked. Configuration is supplied only during
state initialization; Geam does not copy credentials or state into Cargo
metadata, generated source, or execution plans.

## Files Geam looks after

Most of the project stays exactly where Gleam put it. The first Geam preparation
adds this Rust-owned state to the Gleam project root:

```text
Cargo.toml
Cargo.lock
build/geam/runner.rs
build/geam/target/
```

`Cargo.toml` carries an exact Geam-managed marker and the approved provider
selections. `Cargo.lock` fixes the Rust dependency graph. Commit both files so
review and CI retain the approved native-code choices. Ignore `build/geam/`;
its runner source and Cargo target are reproducible build artifacts.

Geam rewrites the managed manifest canonically and removes selections whose
Gleam packages are no longer in the resolved project. It refuses to adopt or
overwrite a user-owned Cargo manifest without its exact managed marker. A Rust
application that already owns `Cargo.toml` must use the separate
[Rust embedding workflow](embedding.md).

Gleam still owns `gleam.toml`, `manifest.toml`, package sources, and its package
cache. Geam may run `gleam deps download` once when a locked manifest or package
source is missing, but it does not choose a new Gleam dependency version.

## See what Geam is doing

Geam writes plain `geam: ...` progress lines to stderr at real preparation
boundaries. Gleam and Cargo output remains on the streams those tools select;
Geam does not add prefixes, emulate a terminal, or replace their progress
display.

The first command can spend most of its time downloading or compiling Rust
dependencies. Later commands still report phases they actually invoke, while
Cargo decides whether a rebuild is needed.

`prepare` ends with `geam: Prepared MODULE` only after the runner check
succeeds. `run` prints `geam: Starting standalone runner for MODULE` before
handing stdin, stdout, and stderr to the application. It does not print a
completion footer after application output.

## When something fails

- If `manifest.toml` or a locked package source is missing, let Geam perform its
  single `gleam deps download` recovery and inspect any native Gleam error.
- If a provider is required in noninteractive CI, approve and commit its managed
  Cargo selection from an interactive checkout first.
- If Geam refuses an existing `Cargo.toml`, do not add its marker manually. Use
  Rust embedding or move the user-owned Cargo project outside the Gleam root.
- If a provider version is incompatible, choose a compatible exact version or
  update the provider; Geam does not silently replace it.
- Registry, Cargo, configuration, planning, and runtime errors preserve the
  failed operation or source identity. Fix the reported boundary and rerun the
  same command.

For exact provider linkage and execution contracts, continue with the
[provider reference](reference/provider-boundary.md) and
[runtime semantics](reference/runtime-semantics.md).
