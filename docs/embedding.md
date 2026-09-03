# Embed Gleam in Rust

This guide connects a Rust application to Gleam. The first workflow ends with
`cargo run` calling a Gleam function and printing `42`.

Geam keeps the Gleam source in a nested project and generates typed Rust
bindings for the Gleam functions Rust can call. The Gleam code runs inside the
Rust application rather than as a second executable. The Rust application stays
in control of when and how those functions are called.

## Before you start

This guide assumes Rust `1.96` or newer and Gleam `v1.18.1` are installed.
Install the Geam command with:

```sh
cargo install geam --locked
```

Start from an ordinary Cargo package, not a virtual workspace root. Geam uses
the Cargo package name for the nested Gleam package and its public module.

The generated Gleam project uses `target = "erlang"` because Geam analyses the
Erlang-compatible source path. It does not execute BEAM code. A bodyless Erlang
external needs a matching Rust provider, while a bodyless JavaScript-only
external cannot be called through the standard embedding workflow.

## Make your first call

Create a Rust application and initialize its Gleam project:

```sh
cargo new inventory-app
cd inventory-app
geam embedding init
```

Initialization creates the nested `gleam/` project and generates
`src/geam_bindings.rs`. It makes sure `Cargo.toml` enables Geam embedding and
leaves handwritten Rust files untouched.

The generated starter module is named after the Cargo package, with hyphens
replaced by underscores:

```gleam
// gleam/src/inventory_app.gleam
pub fn double(value: Int) -> Int {
  value * 2
}
```

Replace Cargo's starter `src/main.rs` with this application code:

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

It prints `42`. For repeated calls, initialize and seal the module once, then
keep the module and its generated function handles. You do not repeat that
setup for every function call.

The setup has four distinct responsibilities:

- `project().compile()` reads the nested Gleam project and produces its checked
  Geam program.
- `ModuleBuilder` selects a provider-free execution owner for that program.
- `bind` registers the generated function handles before `seal` closes the
  module to further registration.
- `module.call` invokes one typed handle. The final mutable argument collects
  any Gleam `echo` output for the Rust caller.

The complete [first-call
example](https://github.com/panarch/geam/tree/main/examples/embedding/first_call)
keeps the Gleam source, generated bindings, handwritten Rust, lockfiles, and
exact-output test together.

## Learn one boundary at a time

The repository examples form a progression, but each one is an independently
locked application that can be run and tested on its own:

| Stage | Adds | Example |
| --- | --- | --- |
| First call | Plain loading, binding, sealing, and one scalar call | [`first_call`](https://github.com/panarch/geam/tree/main/examples/embedding/first_call) |
| Structured data | Recursive List, Tuple, Result, and retained List reuse | [`data`](https://github.com/panarch/geam/tree/main/examples/embedding/data) |
| Gleam package | A locked package, hosted bindings, and explicit stdlib state | [`package`](https://github.com/panarch/geam/tree/main/examples/embedding/package) |
| Caller-owned IO | Explicit stdlib state, IO routing, and Echo separation | [`io`](https://github.com/panarch/geam/tree/main/examples/embedding/io) |
| External provider | Provider selection, configuration, and opaque Gleam values | [`provider`](https://github.com/panarch/geam/tree/main/examples/embedding/provider) |
| Complete application | Packages, IO, a provider, retained data, and repeated calls together | [`application`](https://github.com/panarch/geam/tree/main/examples/embedding/application) |

Follow the stages in order when learning the API, or open the smallest example
that contains the boundary your application needs.

## Keep Gleam and Rust in sync

Add or change public functions in the generated Gleam module. For example:

```gleam
pub fn increment(value: Int) -> Int {
  value + 1
}
```

Regenerate the Rust bindings after changing public Gleam functions, imports, or
dependencies:

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
-> update handwritten Rust calls when the public Gleam API changed
-> cargo run or cargo test
```

Sync restores locked Gleam and Cargo dependencies, checks that every public
function exposed to Rust uses supported types, and updates the generated Rust
only when its contents change. It does not run the Rust application, initialize
providers, or run Cargo build scripts.

Keep `src/geam_bindings.rs` as generated, tool-owned code and make Rust changes
in neighboring handwritten modules. Geam protects handwritten files at the
generated path from replacement.

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

Embedding commands use one fixed project, module, and output layout. This makes
checkouts, generated code, CI, and examples agree on the same connection.

## Bring in Gleam packages and Rust providers

Add Gleam dependencies from the nested project:

```sh
cd gleam
gleam add gleam_stdlib
cd ..
geam embedding sync
```

Sync enables only the built-in Geam support used by imported Gleam code.
Official stdlib, JSON, and Time integrations are added explicitly; unused Gleam
dependencies do not add Rust components.

When an imported package requires another native provider, sync verifies
registry candidates and asks before adding an exact Cargo dependency. Existing
compatible registry, path, or Git declarations are reused. A noninteractive
sync cannot approve new native code, so perform and commit provider selection
before relying on CI.

Generated hosted bindings make every required capability, provider
configuration, and mutable state dependency explicit in the Rust API.
The staged examples show a [Gleam
package](https://github.com/panarch/geam/tree/main/examples/embedding/package),
[caller-owned
IO](https://github.com/panarch/geam/tree/main/examples/embedding/io), and
an [external
provider](https://github.com/panarch/geam/tree/main/examples/embedding/provider)
separately. The complete [Rust embedding
application](https://github.com/panarch/geam/tree/main/examples/embedding/application)
then combines stdlib IO, a published-style external provider, retained Lists,
and repeated calls in one application.

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
dependency changes. `embedding check` verifies the generated Gleam-Rust
connection, while `cargo check` and `cargo test` remain responsible for Rust
compilation and tests.

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
their items. The [structured-data
example](https://github.com/panarch/geam/tree/main/examples/embedding/data)
shows both operations without adding providers. See the [embedding
boundary](reference/embedding-boundary.md) for the complete type map, ownership
rules, list transfer behavior, provider state, and lower-level manual binding
API.

## Ship the Gleam sources with your application

The generated project selection reads `gleam/` and its resolved package sources
from the Cargo manifest directory when the application initializes. A deployment
therefore includes both the compiled binary and that source graph, kept in the
layout expected by the binary. Executable source bundling is a separate
deployment capability.

For the planner, runtime, and host ownership model, continue with
[architecture](reference/architecture.md) and
[runtime semantics](reference/runtime-semantics.md).
