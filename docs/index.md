# Geam Documentation

Gleam already makes it pleasant to build typed programs for Erlang and
JavaScript. Geam carries that experience into Rust-hosted applications, so
Gleam code can take part where Rust owns the process, native capabilities, or
integration boundary.

You still write Gleam, use Gleam packages, and rely on Gleam's parser and type
analysis. Geam looks after the Rust execution environment and, when Rust is the
application host, generates the typed bridge used to call your Gleam functions.

## What do you want to build?

| I want to... | Use | Start here |
| --- | --- | --- |
| Run a Gleam application without writing a Rust runner | Standalone | [Run a Gleam project](standalone.md) |
| Call selected Gleam functions from a Rust application | Rust embedding | [Embed Gleam in Rust](embedding.md) |
| Implement Gleam externals with native Rust code | Host provider | [Author a host provider](host-providers.md) |

These are three entrances to the same runtime, not three steps you must learn in
order. Start with the project you already have.

## See your first result

From an existing Gleam application:

```sh
cd my_gleam_app
geam run
```

Geam prepares the Rust runner and then executes the package module's `main`.

From a new Rust application:

```sh
cargo new my_rust_app
cd my_rust_app
geam embedding init
```

Initialization creates the nested Gleam project and typed Rust declarations
without replacing your application code. The [embedding guide](embedding.md)
adds the small Rust call that prints `42`. From there, edit the Gleam module,
run `geam embedding sync`, and keep using Cargo as usual.

## What Geam takes care of

The owner of the application determines the workflow:

- In a **standalone project**, Gleam owns the application and its package graph.
  Geam manages the project-local Rust runner and approved native providers.
- In a **Rust embedding project**, Cargo owns the application. Geam manages the
  conventional nested Gleam connection and generated typed bindings.
- A **provider author** supplies a native Rust capability that either workflow
  can compose statically for a Gleam package.

A provider remains separate from the Gleam package. The package can keep its
existing source and target implementations while a Rust crate implements its
external declarations for Geam. Users keep importing and calling the Gleam
modules they already know.

## Before you start

The supported toolchain baseline is:

- Rust `1.96` or newer on a 64-bit target;
- Gleam `v1.18.1`; and
- Cargo for installation, provider resolution, and Rust application builds.

Ready? Install Geam with:

```sh
cargo install geam --locked
```

Geam is a separate project rather than an official third Gleam target. Choose
Erlang or JavaScript when either already provides the process and deployment
model you need. Choose Geam when Rust needs to own the process or when Gleam and
Rust need an explicit typed boundary.

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
