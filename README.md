# Geam

Geam is a [Rust](https://www.rust-lang.org/) runtime and embedding layer for
[Gleam](https://gleam.run/).

Run a Gleam project, call Gleam functions from Rust, or add Rust capabilities
to a Gleam package. You keep writing Gleam and using Gleam packages; Geam looks
after the Rust side.

The basic arrangement is similar to embedding Lua in a native application: Rust
owns the process, while Gleam supplies statically typed application logic. Gleam
source stays Gleam; Geam runs the type-checked program and generates the Rust
runner or bindings that connect it to the host.

## Try it

Geam selects Gleam's Erlang-compatible source path and runs ordinary Gleam
bodies and built-in integrations directly in its Rust runtime. It does not
execute BEAM code. Bodyless Erlang externals connect through matching Rust
providers, while bodyless JavaScript-only externals sit outside the standard
Geam workflow. See [compatibility](docs/reference/compatibility.md) for the
exact boundary.

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

Want to call Gleam functions from Rust? Create a nested Gleam project and its
Rust bindings:

```sh
cargo new my_rust_app
cd my_rust_app
geam embedding init
```

Initialization creates a starter Gleam function and generated Rust bindings
without replacing your `src/main.rs`. The [Rust embedding
guide](docs/embedding.md) adds the small Rust call that prints `42`, then shows
how to add functions, packages, providers, and repeated calls.

### Start with a Gleam package that needs Rust

Building a Gleam package that needs native Rust code? Keep its public API in
Gleam and implement the target-specific functions in a separate Rust crate. The
[host provider guide](docs/host-providers.md) starts with one function and shows
how Geam connects the two packages.

## Where Geam fits

Geam is an independent project that brings Gleam code into Rust-hosted
applications. It complements Gleam's Erlang and JavaScript targets, which remain
the natural choice when they fit how your application runs and deploys. Choose
Geam when Rust needs to host the application, provide native capabilities, or
call Gleam through generated bindings.

Geam uses Gleam's parser and type analysis to build executable plans for its
Rust runtime. Generated Rust provides host integration: a managed standalone
runner or typed embedding bindings.

A Gleam package keeps its Hex identity, source, and existing target
implementations. A companion Rust crate implements its target-specific
functions for Geam, and each package follows its own release schedule.

## Growing, but experimental

Geam is actively evolving toward a stable `1.0` API, so public APIs may still
change. The current profile supports everyday Gleam data, functions, imports,
custom types, pattern matching, generics, official package integrations, native
host providers, and nested ordinary data passed between Gleam and Rust.

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

The Rust API reference is published on [docs.rs](https://docs.rs/geam).
Repository documentation covers user workflows, execution contracts, and
project development, while docs.rs documents individual Rust API items.

## License

Geam is licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE).
