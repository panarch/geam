# Geam

[Gleam](https://gleam.run/) already runs on Erlang and JavaScript. Geam lets
you use Gleam with [Rust](https://www.rust-lang.org/) too.

You can run a Gleam project through Geam, call Gleam functions from Rust, or
give Gleam access to capabilities implemented in Rust. You keep writing Gleam
and using Gleam packages; Geam looks after the Rust side.

If you have embedded Lua in a native application, the basic idea is similar:
Rust is the host and Gleam supplies application logic. Geam also uses Gleam's
static types when it connects the two and generates Rust bindings.

## Try it

Geam currently uses Gleam's Erlang target to choose and analyse source, but it
does not execute BEAM code. Ordinary Gleam bodies and built-in integrations can
run directly. A bodyless Erlang external needs a matching Rust provider, while
a bodyless JavaScript-only external is not available to the standard Geam
workflow. See [compatibility](docs/reference/compatibility.md) for the exact
boundary.

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

Geam is a separate project, not an official third Gleam target or a replacement
for the Erlang and JavaScript targets. Use those targets when they already fit
how your application runs and deploys. Choose Geam when Rust needs to host the
application, provide native capabilities, or call Gleam through generated
bindings.

Gleam remains the source language. Geam uses Gleam's parser and type analysis to
prepare and run the selected modules, so you do not have to reproduce their
logic in Rust.

Providers also stay separate from Hex packages. A Gleam package keeps its source
and existing target implementations, while Geam users add a separate Rust
provider for the same API. The Hex package does not have to become a Rust
package.

## Growing, but experimental

Geam is experimental and has not reached a stable `1.0` API. The current
profile supports everyday Gleam data, functions, imports, custom types, pattern
matching, generics, official package integrations, native host providers, and
nested ordinary data passed between Gleam and Rust.

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
