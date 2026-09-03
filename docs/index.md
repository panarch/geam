# Geam

Geam is a [Rust](https://www.rust-lang.org/) runtime and embedding layer for
[Gleam](https://gleam.run/).

Run a Gleam project, call Gleam functions from a Rust application, or connect a
Gleam package to native Rust code.

Gleam source stays Gleam. Geam runs the type-checked program and generates the
Rust runner or bindings that connect it to the host.

## What do you want to build?

| I want to... | Use | Start here |
| --- | --- | --- |
| Run a Gleam application without writing a Rust runner | Standalone | [Run a Gleam project](standalone.md) |
| Call selected Gleam functions from a Rust application | Rust embedding | [Embed Gleam in Rust](embedding.md) |
| Add a Rust implementation to a Gleam package | Host provider | [Add Rust to a Gleam package](host-providers.md) |

Each workflow is a complete starting point for the same runtime. Start with the
project you already have.

Geam is actively evolving toward a stable `1.0` API, so public APIs may still
change. See [compatibility](reference/compatibility.md) for verified versions,
supported integrations, and current deployment limits.

## Before you start

The supported toolchain baseline is:

- Rust `1.96` or newer on a 64-bit target;
- Gleam `v1.18.1`; and
- Cargo for installation, provider resolution, and Rust application builds.

Install Geam with:

```sh
cargo install geam --locked
```

Geam runs the Erlang-compatible path of a Gleam project in its Rust runtime.
Packages that use bodyless Erlang externals need matching Rust providers;
JavaScript-only externals are unavailable in this workflow. See
[compatibility](reference/compatibility.md) for the exact rules.

## See your first result

From an existing Gleam application:

```sh
cd my_gleam_app
geam run
```

Geam prepares the Rust runner and then executes the application's `main`.

From a new Rust application:

```sh
cargo new my_rust_app
cd my_rust_app
geam embedding init
```

Initialization adds the nested Gleam project and generated Rust bindings beside
your handwritten `src/main.rs`. The [embedding guide](embedding.md) supplies
and explains the complete first call that prints `42`, then links independently
runnable examples for structured data, packages, IO, and providers. From there,
edit the Gleam module, run `geam embedding sync`, and keep using Cargo as usual.

## What Geam takes care of

What Geam manages depends on where you start:

- In a **standalone project**, Gleam remains the application. Geam manages the
  project-local Rust runner and approved providers.
- In a **Rust embedding project**, Rust remains the application. Geam manages
  the nested Gleam project and generated bindings.
- A **host provider** is the companion Rust crate that implements a Gleam
  package's native functions for Geam. Either kind of project can use one.

Most Gleam packages need no provider. When a package includes native functions,
users still add the Hex package and import its Gleam modules as usual. The
separate provider crate is compiled into the Rust host only to implement those
functions for Geam.

## When Geam fits

Geam is an independent project that brings Gleam code into Rust-hosted
applications. It complements Gleam's Erlang and JavaScript targets, which remain
the natural choice when they fit how your application runs and deploys. Choose
Geam when Rust needs to host the application, provide native capabilities, or
call Gleam through generated bindings.

## How the pieces fit

Geam starts after Gleam has parsed and analysed the selected source graph:

```text
resolved Gleam sources
-> Gleam typed program
-> Geam module and execution plans
-> Rust-owned runtime
```

Geam generates the standalone runner or typed embedding bindings around this
pipeline. Planning checks that the selected Gleam code and Rust providers fit
the runtime before the application starts.

Read [architecture](reference/architecture.md) for the complete pipeline and
ownership model. Read [compatibility](reference/compatibility.md) before relying
on a particular Gleam package, runtime effect, target, or deployment shape.

## Keep exploring

- [Technical reference](reference/README.md) documents public execution,
  embedding, provider, and compatibility contracts.
- [Geam development](development/README.md) covers tests, review rules,
  upstream synchronization, and releases for contributors working on Geam
  itself.
- [Release notes](releases/README.md) describe user-visible changes by version.
