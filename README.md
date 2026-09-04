# Geam

[![crates.io](https://img.shields.io/crates/v/geam.svg)](https://crates.io/crates/geam)
[![LICENSE](https://img.shields.io/crates/l/geam.svg)](LICENSE)
[![CI](https://img.shields.io/github/actions/workflow/status/panarch/geam/workspace.yml?branch=main&label=CI)](https://github.com/panarch/geam/actions/workflows/workspace.yml)
[![docs.rs](https://docs.rs/geam/badge.svg)](https://docs.rs/geam)

Geam is a [Rust](https://www.rust-lang.org/) runtime and embedding layer for
[Gleam](https://gleam.run/).

Run a Gleam project, call Gleam functions from Rust, or connect a Gleam package
to native Rust code. You keep writing Gleam and using Gleam packages; Geam
looks after the Rust side.

The basic arrangement is similar to embedding Lua in a native application: Rust
owns the process, while Gleam supplies statically typed application logic. Gleam
source stays Gleam; Geam runs the type-checked program and generates the Rust
runner or bindings that connect it to the host.

## Try it

Geam runs the Erlang-compatible path of a Gleam project in its Rust runtime.
Packages that use bodyless Erlang externals need matching Rust providers;
JavaScript-only externals are unavailable in this workflow. See
[compatibility](docs/reference/compatibility.md) for the exact rules.

Geam requires Gleam `v1.18.1`, Rust `1.96` or newer, and a 64-bit Rust target.
Install Geam with Cargo:

```sh
cargo install geam --locked
```

### Run a Gleam project

Run an existing Gleam application on Geam's Rust runtime:

```sh
cd my_gleam_app
geam run
```

Geam prepares and maintains the project-local Rust runner while you continue
working in Gleam. The
[standalone guide](docs/standalone.md) continues from here.

### Call Gleam from Rust

To call Gleam functions from Rust, create a Rust application and initialize its
nested Gleam project and generated bindings:

```sh
cargo new my_rust_app
cd my_rust_app
geam embedding init
```

`geam embedding init` adds a nested Gleam project and generated Rust bindings
to the Cargo package. The starter module it creates contains this function:

```gleam
// gleam/src/my_rust_app.gleam
pub fn double(value: Int) -> Int {
  value * 2
}
```

To call `double` from Rust, use this as `src/main.rs`:

```rust
mod geam_bindings;

use geam::embedding::ModuleBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = geam_bindings::project().compile()?;
    let builder = ModuleBuilder::from_program(program)?;
    let (bindings, functions) = geam_bindings::bind(builder)?;
    let module = bindings.seal();
    let mut echo = Vec::new();

    let value = module.call(&functions.double, (21.into(),), &mut echo)?;
    println!("{value}");
    Ok(())
}
```

`cargo run` now prints `42`. The [Rust embedding guide](docs/embedding.md)
explains each part of this lifecycle. The executable [embedding
examples](examples/embedding)
then add structured data, Gleam packages, IO routed through Rust, an external
provider, and repeated calls one step at a time.

### Add Rust to a Gleam package

Ordinary Gleam packages run without this extra step. When a function is
implemented outside Gleam, the Gleam package still owns the public API and a
companion Rust crate can provide its Geam implementation. Geam calls that crate
a host provider.

The Gleam package declares the function:

```gleam
// src/example_text_tools/casing.gleam
@external(erlang, "geam_example_text_tools_casing", "upper")
pub fn upper(value: String) -> String
```

`@external` marks the function as implemented outside Gleam. Geam follows the
Erlang-compatible source path, but links this declaration to Rust instead of
calling the named Erlang function.

The provider implements the same package, module, function, and signature in
Rust:

```rust
#[geam::provider(
    package = "example_text_tools",
    modules = [casing],
)]
pub struct Component;

#[geam::module(path = "example_text_tools/casing")]
mod casing {
    use geam::provider::EcoString;

    #[geam::function]
    fn upper(value: EcoString) -> EcoString {
        value.to_uppercase()
    }
}
```

The application keeps using the Gleam module:

```gleam
import example_text_tools/casing

pub fn main() {
  assert casing.upper("Geam") == "GEAM"
}
```

Geam links that call to the selected Rust implementation. A standalone project
selects the crate with `geam provider add`; a Rust embedding application adds
it as a direct Cargo dependency. The provider's Cargo metadata declares the
Gleam package and versions it implements, so Geam can validate the selection
before running application code.

The [Add Rust to a Gleam package](docs/host-providers.md) guide follows one
function from its Gleam declaration to Rust and then through `geam run`. It
also shows how standalone and embedding applications select a provider.

## Where Geam fits

Geam is an independent project that brings Gleam code into Rust-hosted
applications. It complements Gleam's Erlang and JavaScript targets, which remain
the natural choice when they fit how your application runs and deploys. Choose
Geam when Rust needs to host the application, provide native capabilities, or
call Gleam through generated bindings.

Geam uses the parser and type analysis from the supported Gleam release without
modifying their implementation, then builds executable plans for its Rust
runtime. Generated Rust provides host integration: a managed standalone runner
or typed embedding bindings.

A Gleam package keeps its Hex identity, source, and existing target
implementations when it gains a Geam provider. The companion Rust crate adds a
Geam implementation; it does not replace or translate the Gleam package.

## Growing, but experimental

Geam is actively evolving toward a stable `1.0` API, so public APIs may still
change. Geam currently supports everyday Gleam data, functions, imports, custom
types, pattern matching, generics, verified package integrations, native host
providers, and nested ordinary data passed between Gleam and Rust.

See [compatibility](docs/reference/compatibility.md) for the verified Gleam and
package baselines, current limits, and platform requirements. See
[architecture](docs/reference/architecture.md) for how Gleam analysis, Geam
planning, Rust providers, and execution fit together.

## Documentation

- [Documentation overview](docs/index.md)
- [Standalone projects](docs/standalone.md)
- [Rust embedding](docs/embedding.md)
- [Executable examples](examples)
- [Add Rust to a Gleam package](docs/host-providers.md)
- [Technical reference](docs/reference/README.md)
- [Geam development](docs/development/README.md)
- [Release notes](docs/releases/README.md)

The Rust API reference is published on [docs.rs](https://docs.rs/geam).
Repository documentation covers user workflows, execution contracts, and
project development, while docs.rs documents individual Rust API items.

## License

Geam is licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE).
