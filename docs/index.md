# Geam

Geam runs [Gleam](https://gleam.run/) programs with
[Rust](https://www.rust-lang.org/).

Run a Gleam project, call Gleam functions from a Rust application, or add Rust
capabilities to a Gleam package.

## What do you want to build?

| I want to... | Use | Start here |
| --- | --- | --- |
| Run a Gleam application without writing a Rust runner | Standalone | [Run a Gleam project](standalone.md) |
| Call selected Gleam functions from a Rust application | Rust embedding | [Embed Gleam in Rust](embedding.md) |
| Give a Gleam package capabilities implemented in Rust | Host provider | [Author a host provider](host-providers.md) |

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

Geam selects Gleam's Erlang-compatible source path and runs ordinary Gleam
bodies and built-in integrations directly in its Rust runtime. It does not
execute BEAM code. Bodyless Erlang externals connect through matching Rust
providers, while bodyless JavaScript-only externals sit outside the standard
workflow. See [compatibility](reference/compatibility.md) for the exact rules.

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

Initialization creates the nested Gleam project and generated Rust bindings
without replacing your application code. The [embedding guide](embedding.md)
adds the small Rust call that prints `42`. From there, edit the Gleam module,
run `geam embedding sync`, and keep using Cargo as usual.

## What Geam takes care of

What Geam manages depends on where you start:

- In a **standalone project**, Gleam remains the application. Geam manages the
  project-local Rust runner and approved providers.
- In a **Rust embedding project**, Rust remains the application. Geam manages
  the nested Gleam project and generated bindings.
- A **host provider** supplies Rust capabilities that either kind of project can
  use from Gleam.

A provider remains separate from the Gleam package. The package can keep its
existing source and target implementations while a Rust crate implements the
functions that need native code. Users keep importing and calling the Gleam
modules they already know.

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

Unsupported execution semantics are rejected during planning rather than
becoming partially defined runtime behavior. The runtime can be provider-free
or statically composed with Rust host providers.

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
