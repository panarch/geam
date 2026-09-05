# Run a Gleam project

An existing Gleam application remains an ordinary Gleam project. Run it with
`geam run`, and Geam prepares and maintains the project-local Rust runner while
the application source stays in Gleam.

## Before you start

This guide assumes Rust `1.96` or newer and Gleam `v1.18.1` are installed.
Install the Geam command with:

```sh
cargo install geam --locked
```

Run Geam from a directory containing `gleam.toml` or one of its descendants.
Geam walks upward to the nearest Gleam project root.

Geam runs the Erlang-compatible path through the selected modules in its Rust
runtime, regardless of the project's default build target.

In the standard standalone workflow:

- ordinary Gleam bodies and Gleam fallback bodies can run directly;
- built-in Geam integrations supply their supported native operations;
- a bodyless `@external(erlang, ...)` needs a matching Rust provider; and
- a bodyless JavaScript-only external cannot be called from the selected code.

If the last case is reached, source analysis fails before provider validation.

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

That is the complete first workflow. On the first run, Geam restores locked
Gleam dependencies when needed, prepares the Rust runner, and executes the
package module's `main`. If imported code needs an external Rust provider that
has not been selected, Geam stops and tells you to add one explicitly.

When you want to prepare and check the runner without executing application
code, use:

```sh
geam prepare
```

`prepare` performs the same setup checks but stops before running application
code. `run` continues by starting the application. Normal Gleam IO keeps its
selected output stream, while Gleam's `echo` output is written to stderr. A
value returned by `main` is not printed automatically.

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

Both commands use the root package module by default. Choose another module
when it contains the `main` you want to run:

```sh
geam prepare --module worker
geam run --module worker
```

## Use Gleam packages

Add Gleam dependencies exactly as you normally would:

```sh
gleam add gleam_json
```

Adding a dependency does not by itself add work to the Geam runner. Only
packages imported by the selected module or its imports are included. This
keeps unused dependencies and their native requirements out of the executable.

Geam includes explicit support for the verified `gleam_stdlib`, `gleam_json`,
and `gleam_time` integrations. These built-in components do not require an
external provider selection. `gleam_http` uses its unchanged Gleam source and
the stdlib support required by its imported code.

See [compatibility](reference/compatibility.md) for the exact package baselines
and supported effects.

## Use a Gleam package with a Rust provider

Most Gleam packages run without an additional Rust crate. Some packages expose
functions whose implementation is supplied separately for each execution
target. When Geam runs one of those functions, a companion Rust crate supplies
the implementation. Geam calls that crate a provider.

The two dependencies have different jobs: the Hex package contains the Gleam
API and source that the application imports, while the provider crate contains
the native implementation compiled into the Rust runner. A package may keep
its existing Erlang or JavaScript implementation; adding a provider does not
replace it or change how Gleam code imports the package.

Select the provider named by the package or provider documentation before
preparing the project. Choose the command for its source:

```sh
# Published reference package and provider
gleam add example_text_pattern
geam provider add geam-example-text-pattern

# Local path
geam provider add --path ../provider

# Git revision
geam provider add --git https://example.com/provider.git --rev COMMIT
```

Append `@VERSION` to a crates.io name when selecting a particular release.

Inspect and remove stored external selections without compiling or contacting
a registry:

```sh
geam provider list
geam provider remove example_text_pattern
```

A provider crate may use any Cargo package name. Geam verifies its packaged
metadata against the resolved Gleam package and package version, then records
an exact registry version, path, or Git revision in the managed Cargo project.
One Gleam package has at most one selected external provider. Built-in
components are not stored selections and do not appear in `provider list`.

`prepare` and `run` validate the current selections. If no provider is selected,
or the selected version is incompatible, choose one explicitly and rerun the
command.

To author the companion crate, continue with
[Add Rust to a Gleam package](host-providers.md).

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

`Cargo.toml` carries an exact Geam-managed marker and the selected provider
dependencies. `Cargo.lock` fixes the Rust dependency graph. Commit both files
so review and CI retain the native-code choices. Ignore `build/geam/`;
its runner source and Cargo target are reproducible build artifacts.

Geam rewrites the managed manifest canonically and removes selections whose
Gleam packages are no longer in the resolved project. Geam manages only Cargo
manifests carrying its exact marker, leaving existing user-owned manifests
unchanged. For a Rust application that already owns `Cargo.toml`, use the
[Rust embedding workflow](embedding.md).

Gleam still owns `gleam.toml`, `manifest.toml`, package sources, and its package
cache. Geam may run `gleam deps download` once when a locked manifest or package
source is missing, but it does not choose a new Gleam dependency version.

## See what Geam is doing

Geam writes plain `geam: ...` progress lines to stderr at real preparation
boundaries. Gleam and Cargo output passes through unchanged on the streams those
tools select, including their native progress displays.

The first command can spend most of its time downloading or compiling Rust
dependencies. Later commands still report phases they actually invoke, while
Cargo decides whether a rebuild is needed.

`prepare` ends with `geam: Prepared MODULE` only after the runner check
succeeds. `run` prints `geam: Starting standalone runner for MODULE` before
handing stdin, stdout, and stderr to the application. Application output remains
the final output.

## When something fails

- If `manifest.toml` or a locked package source is missing, let Geam perform its
  single `gleam deps download` recovery and inspect any native Gleam error.
- If a provider is required, add it explicitly and commit the resulting managed
  Cargo manifest and lock before relying on CI.
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
