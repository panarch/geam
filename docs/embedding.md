# Embed Gleam in Rust

Have a Rust application and some logic that belongs in Gleam? Keep each side in
the language that suits it. The Rust application owns the process and native
capabilities, the nested Gleam project owns the typed application logic, and
Geam provides the runtime and generated bridge between them.

Geam creates a conventional nested Gleam project, generates typed Rust bindings
for its public boundary module, and lets the Rust application control loading,
state, IO, and call order.

The result is not a second executable hidden beside the Rust application or a
set of untyped foreign calls. The Gleam module becomes one explicit typed
component that the application can initialize once and call repeatedly.

By the end of the first section, `cargo run` will call Gleam from Rust and print
`42`.

## Before you start

This guide assumes Rust `1.96` or newer and Gleam `v1.18.1` are installed.
Install the Geam command with:

```sh
cargo install geam --locked
```

Start from an ordinary Cargo package, not a virtual workspace root. Geam uses
the nearest package's name to derive one conventional Gleam package and public
module name.

## Make your first call

Create a Rust application and initialize its Gleam project:

```sh
cargo new inventory-app
cd inventory-app
geam embedding init
```

Initialization creates `gleam/`, prepares both dependency graphs, adds Geam's
embedding feature profile when needed, and generates `src/geam_bindings.rs`.
It leaves handwritten Rust files untouched.

The generated starter module is named after the Cargo package, with hyphens
replaced by underscores:

```gleam
// gleam/src/inventory_app.gleam
pub fn double(value: Int) -> Int {
  value * 2
}
```

Use the generated bindings from `src/main.rs`:

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

Run the Rust application normally:

```sh
cargo run
```

It prints `42`. Keep the sealed module and its generated function handles when
the application needs repeated calls; initialization is not part of every
function invocation.

## Keep Gleam and Rust in sync

Add or change public functions in the boundary module. For example:

```gleam
pub fn increment(value: Int) -> Int {
  value + 1
}
```

Regenerate the typed declarations after changing the public Gleam surface,
imports, or dependencies:

```sh
geam embedding sync
```

The generated `Functions` aggregate now exposes `increment`:

```rust
let value = module.call(&functions.increment, (41.into(),), &mut echo)?;
```

The normal loop is:

```text
edit Gleam source
-> geam embedding sync
-> update handwritten Rust calls when the public boundary changed
-> cargo run or cargo test
```

Sync prepares locked Gleam and Cargo dependencies, compiles the selected source
closure, validates every public boundary function, and replaces generated Rust
atomically only when its bytes change. It does not run the Rust application,
provider initialization, or Cargo build scripts.

Do not edit `src/geam_bindings.rs` manually. Geam refuses to silently replace a
handwritten file at that path.

## Know where the files live

Embedding uses one fixed project convention:

```text
inventory-app/
  Cargo.toml
  Cargo.lock
  src/
    main.rs
    geam_bindings.rs
  gleam/
    gleam.toml
    manifest.toml
    src/
      inventory_app.gleam
```

The `gleam/` directory is an ordinary Gleam project. Use Gleam's formatter,
tests, and package commands there as usual; return to the Cargo package root for
`geam embedding sync` and Rust builds.

Internal Gleam modules can live below `gleam/src/inventory_app/`. Only public
functions from the same-name root module become Rust bindings. Imported modules
can keep domain-specific custom types and provider-backed values behind that
ordinary-data boundary.

Commit the Cargo and Gleam manifests and lockfiles, handwritten Gleam and Rust
source, and generated `src/geam_bindings.rs`. Ignore Cargo's `target/` and
Gleam's `gleam/build/` cache. Generated Rust is reviewed and committed; no build
script regenerates it implicitly.

Embedding commands intentionally do not offer alternate project, module, or
output paths. A fixed layout makes checkouts, generated code, CI, and examples
agree on the same connection.

## Bring in Gleam packages and Rust providers

Add Gleam dependencies from the nested project:

```sh
cd gleam
gleam add gleam_stdlib
cd ..
geam embedding sync
```

Sync enables only the built-in Geam features required by the selected source
closure. Official stdlib, JSON, and Time integrations are composed explicitly;
unused Gleam dependencies do not add components.

When an imported package requires another native provider, sync verifies
registry candidates and asks before adding an exact Cargo dependency. Existing
compatible registry, path, or Git declarations are reused. A noninteractive
sync cannot approve new native code, so perform and commit provider selection
before relying on CI.

Generated hosted bindings expose the capabilities and provider configuration
that Rust must supply. They do not hide mutable state or invent configuration.
The canonical [Rust embedding
application](https://github.com/panarch/geam/tree/main/examples/rust_embedding_application)
shows stdlib IO, a published-style external provider, retained Lists, and
repeated calls in one application.

## Verify a prepared checkout

Use `check` after cloning, in review, or in CI:

```sh
geam embedding check
cargo test --locked
```

Check validates existing Cargo and Gleam declarations, both locks, provider
composition, and the expected generated bindings without rewriting project
files. It may fetch locked Cargo packages or restore missing locked Gleam
package sources. It never selects a new version, follows a moving Git branch in
place of its locked commit, approves a provider, or regenerates stale bindings.

Use `init` for an uninitialized package and `sync` after intentional source or
dependency changes. `embedding check` verifies the connection; it does not
replace `cargo check` or `cargo test`.

## Keep the Rust boundary small

Generated bindings support recursive ordinary data:

```text
Scalar | Tuple(Data...) | Result(Data, Data) | Option(Data) | List(Data)
```

This includes nested Lists and combinations of Tuple, Result, and Option.
Records, arbitrary custom types, external values, callbacks, and generic public
signatures stay inside Gleam modules. Expose a small boundary function that
projects domain values into ordinary data instead of duplicating the domain
model in Rust.

Lists returned from Gleam are retained, immutable handles. Rust can inspect
them lazily or pass them back to the same loaded module without reconstructing
their items. See the [embedding boundary](reference/embedding-boundary.md) for
the complete type map, ownership rules, list transfer behavior, provider state,
and lower-level manual binding API.

## Ship the Gleam sources with your application

The generated project selection reads `gleam/` and its resolved package sources
from the Cargo manifest directory when the application initializes. The current
workflow does not bundle that source graph into the executable. Copying only the
compiled binary is therefore not a self-contained deployment.

Keep the nested Gleam project beside the application in the layout expected by
the binary. Source bundling is a separate deployment capability rather than a
hidden behavior of sync or Cargo builds.

For the planner, runtime, and host ownership model, continue with
[architecture](reference/architecture.md) and
[runtime semantics](reference/runtime-semantics.md).
