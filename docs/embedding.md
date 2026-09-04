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

The generated Gleam project uses `target = "erlang"` because Geam runs the
Erlang-compatible source path in its Rust runtime. A bodyless Erlang external
needs a matching Rust provider; JavaScript-only externals are unavailable in
the standard embedding workflow.

## Make your first call

Create a Rust application and initialize its Gleam project:

```sh
cargo new inventory-app
cd inventory-app
geam embedding init
```

Initialization creates the nested `gleam/` project, generates
`src/geam_bindings.rs`, and enables Geam embedding in `Cargo.toml`. You write
the application calls in `src/main.rs`.

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
reuse the module and its generated function handles.

The setup has four distinct responsibilities:

- `project().compile()` reads the nested Gleam project and produces its checked
  Geam program.
- `ModuleBuilder` starts the callable module for that program.
- `bind` returns the generated function handles, and `seal` finishes
  registration before calls begin.
- `module.call` invokes one typed handle. The final mutable argument collects
  any Gleam `echo` output for the Rust caller.

The complete [first-call
example](../examples/embedding/first_call)
keeps the runnable Gleam source, generated bindings, handwritten Rust, and test
together.

## Learn with runnable examples

The repository examples add one practical feature at a time. Each is a complete
application that can be run and tested on its own:

| Stage | Adds | Example |
| --- | --- | --- |
| First call | Call one scalar Gleam function from Rust | [`first_call`](../examples/embedding/first_call) |
| Structured data | Pass nested Lists, Tuples, and Results, then reuse a returned List | [`data`](../examples/embedding/data) |
| Gleam package | Call a function from `gleam_stdlib` | [`package`](../examples/embedding/package) |
| Gleam IO | Route Gleam IO through Rust and capture Echo separately | [`io`](../examples/embedding/io) |
| External provider | Call Gleam code backed by a configured Rust provider | [`provider`](../examples/embedding/provider) |
| Application | Combine packages, IO, a provider, structured data, and repeated calls | [`application`](../examples/embedding/application) |

Follow the stages in order when learning the API, or open the smallest example
that contains the feature your application needs.

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
only when its contents change. Run the Rust application and Cargo build scripts
separately with the usual Cargo commands.

Keep `src/geam_bindings.rs` as generated, tool-owned code and make Rust changes
in neighboring handwritten modules. If a handwritten file already occupies the
generated path, sync stops instead of replacing it.

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
functions from the same-name root module become Rust bindings. Their arguments
and returns must use the generated binding types described below. Imported
modules may use records, custom types, and provider-backed values, but those
values cannot cross the generated binding boundary directly.

Commit the Cargo and Gleam manifests and lockfiles, handwritten Gleam and Rust
source, and generated `src/geam_bindings.rs`. Ignore Cargo's `target/` and
Gleam's `gleam/build/` cache. Generated Rust is reviewed and committed; no build
script regenerates it implicitly.

Embedding commands use one fixed project, module, and output layout. This makes
checkouts, generated code, CI, and examples agree on the same connection.

## Use Gleam packages and Rust providers

Add Gleam dependencies from the nested project:

```sh
cd gleam
gleam add gleam_stdlib
cd ..
geam embedding sync
```

Sync enables only the built-in Geam support used by imported Gleam code. Geam's
stdlib, JSON, and Time integrations are added explicitly; unused Gleam
dependencies do not add Rust components.

Most packages need nothing else. If an imported package has native functions
implemented for Geam, its Hex package remains the Gleam dependency and a
companion provider crate supplies the Rust implementation compiled into the
host application.

Automatic crates.io discovery uses the Gleam package name. For
`company_image`, sync considers `geam-company-image` and
`geam-company-image-<suffix>`, where the optional suffix is lowercase
kebab-case. It lists the exact name first, but neither form is treated as
official, trusted, or automatically selected. A differently named crate is not
found from metadata alone. The
[provider names for automatic discovery](host-providers.md#provider-names-for-automatic-discovery)
section defines the complete rule.

Sync verifies each candidate's metadata and package-version range before showing
it. With several verified candidates, an interactive sync asks which provider
to use and then asks for approval:

```text
Gleam package company_image 1.4.0 requires native provider code.
Metadata compatibility is not an endorsement.
  1. geam-company-image 0.3.1 (Gleam >= 1.0.0 and < 2.0.0)
  2. geam-company-image-aws 0.2.0 (Gleam >= 1.4.0 and < 2.0.0)
Select a provider [1-2], or 0 to cancel: 2
Approve geam-company-image-aws 0.2.0? [y/N] y
```

With one verified candidate, sync skips the numbered selection and asks for
approval directly. After approval, it records that exact Cargo dependency and
regenerates the Rust bindings. Existing compatible registry, path, or Git
declarations are reused and may use another crate name. A noninteractive sync
cannot approve new native code, so perform and commit provider selection before
relying on CI.

When imported code needs IO, time, or provider state, the generated Rust API
asks the application for those inputs.
The staged examples show a [Gleam
package](../examples/embedding/package),
[IO routed through Rust](../examples/embedding/io), and
an [external
provider](../examples/embedding/provider)
separately. The [application example](../examples/embedding/application) then
combines stdlib IO, an external provider, structured data, and repeated calls.

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

## Pass data between Gleam and Rust

Generated bindings currently support this recursive data grammar:

```text
Scalar | Tuple(Data...) | Result(Data, Data) | Option(Data) | List(Data)
```

This includes nested Lists and combinations of Tuple, Result, and Option.
Records, arbitrary custom types, external values, callbacks, and generic types
cannot currently be used in generated Rust function signatures. Gleam code may
use them internally. Through generated bindings, Rust can call such code only
through a public function in the same-name root module whose arguments and
return value use supported types.

Lists returned from Gleam are retained, immutable handles. Rust can inspect
them lazily or pass them back to the same loaded module without reconstructing
their items. The [structured-data
example](../examples/embedding/data)
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
