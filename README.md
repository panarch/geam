# Geam

Gleam already lets typed programs run on Erlang and JavaScript. Geam helps that
same friendly language reach Rust-owned applications and execution environments
too.

You keep writing Gleam and using Gleam packages. Rust keeps control of the
process, native capabilities, and application integration. Geam connects the
two by analysing the selected Gleam modules, preparing their Rust execution
environment, and generating typed Rust boundaries when an application needs
them.

If you have embedded Lua in a native application, the shape is similar: Rust is
the host and Gleam supplies application logic. The difference is that Gleam's
static types guide Geam's planning and the Rust bindings it generates.

## Try it

You will need Gleam `v1.18.1`, Rust `1.96` or newer, and a 64-bit Rust target.
Ready? Install Geam with Cargo:

```sh
cargo install geam --locked
```

### Start with a Gleam project

Already have a Gleam application? Run it with Rust behind the scenes:

```sh
cd my_gleam_app
geam run
```

Geam prepares and maintains the project-local Rust runner. You continue working
in Gleam and do not have to write that runner yourself. The
[standalone guide](docs/standalone.md) continues from here.

### Start with a Rust application

Want to call Gleam functions from Rust? Add a conventional nested Gleam project
and its generated bindings:

```sh
cargo new my_rust_app
cd my_rust_app
geam embedding init
```

Initialization creates a starter Gleam function and its typed Rust declarations
without replacing your `src/main.rs`. The [Rust embedding
guide](docs/embedding.md) adds the small Rust call that prints `42`, then shows
how to add functions, packages, providers, and repeated calls.

## Choose your path

- **Run Gleam without writing a Rust runner.** Standalone projects let Gleam
  remain the application while Geam looks after its Rust execution environment.
- **Call Gleam from Rust.** Rust embedding gives an application generated typed
  handles for public Gleam functions while Rust keeps ownership of loading,
  state, IO, and call order.
- **Give Gleam access to Rust capabilities.** A host provider is an ordinary
  Rust crate that implements native functions and external values declared by a
  Gleam package. Start with the [host provider guide](docs/host-providers.md).

All three paths use the same runtime and static provider model. Pick the one
that matches the project you already own; you do not need to learn the other two
before getting started.

## Where Geam fits

Geam is a separate project, not an official third Gleam target or a replacement
for the Erlang and JavaScript targets. Use those targets when they already
provide the runtime and deployment model your application needs. Choose Geam
when the process must live in Rust, when Gleam needs Rust-native capabilities,
or when a Rust application wants a typed Gleam boundary.

Gleam remains the source language and compiler front-end. Geam uses Gleam's
parser and type analysis, then plans and executes the selected module graph
without introducing another language or asking you to reproduce Gleam logic in
Rust.

Providers also stay separate from Hex packages. A Gleam package can keep its
normal source and existing target implementations while Geam users select a
Rust provider for its external declarations. The Gleam-facing package can
therefore serve its existing targets and a Rust-hosted application without
turning the Hex package into a Rust package.

## Growing, but experimental

Geam is experimental and has not reached a stable `1.0` API. The current
profile supports the core Gleam value families, functions, imports, custom
types, pattern matching, generics, official package integrations, native host
providers, and a recursive ordinary-data boundary for Rust embedding.

See [compatibility](docs/reference/compatibility.md) for the verified Gleam and
package baselines, current limits, and platform requirements. See
[architecture](docs/reference/architecture.md) for how Gleam analysis, Geam
planning, Rust providers, and execution fit together.

## Documentation

- [Documentation overview](docs/index.md)
- [Standalone projects](docs/standalone.md)
- [Rust embedding](docs/embedding.md)
- [Host provider authoring](docs/host-providers.md)
- [Technical reference](docs/reference/README.md)
- [Geam development](docs/development/README.md)
- [Release notes](docs/releases/README.md)

The Rust API reference is published on
[docs.rs](https://docs.rs/geam). Repository documentation explains user
workflows, execution contracts, and project development rather than duplicating
generated Rust API pages.

## License

Geam is licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE).
