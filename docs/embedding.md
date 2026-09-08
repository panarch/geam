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
| Async Rust host | Return explicit work and drive it on the application's executor | [`async_host`](../examples/embedding/async_host) |

Follow the stages in order when learning the API, or open the smallest example
that contains the feature your application needs. The application example
combines the preceding examples; the async-host example then adds explicit
Future values.

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

Add the provider crate named by the package or provider documentation as a
direct dependency of the Rust application, then synchronize the generated
bindings:

```sh
cd gleam
gleam add example_text_pattern
cd ..
cargo add geam-example-text-pattern
geam embedding sync
```

Use Cargo's `crate@version` form when the documentation requires a particular
provider release.

For a local path or Git revision, choose the corresponding Cargo form instead:

```sh
# Local path
cargo add --path ../provider

# Git revision
cargo add --git https://example.com/provider.git --rev COMMIT
```

During sync, Geam reads provider metadata from direct Cargo dependencies,
verifies the target Gleam package and supported package-version range, and
regenerates the Rust bindings. If a required provider is missing or
incompatible, update the Cargo dependency and run sync again. Commit the Cargo
manifest and lockfile with the generated bindings before relying on CI.

When imported code needs IO, time, or provider state, the generated Rust API
asks the application for those inputs.
The staged examples show a [Gleam
package](../examples/embedding/package),
[IO routed through Rust](../examples/embedding/io), and
an [external
provider](../examples/embedding/provider)
separately. The [application example](../examples/embedding/application) then
combines stdlib IO, an external provider, structured data, and repeated calls.

## Drive explicit Future values

A Rust provider can expose an `async fn` as a Gleam function returning
`Future(a)`. Gleam creates and composes that work; the Rust application decides
when to drive it. The [Future guide](future.md) covers package setup and Gleam
composition. Ordinary functions still return ordinary values:

```gleam
import example_async_files as files
import geam/future.{type Future}

pub fn double(value: Int) -> Int {
  value * 2
}

pub fn greeting(path: String) -> Future(Result(String, String)) {
  use result <- future.map(files.read(path))
  case result {
    Ok(text) -> Ok("Hello " <> text)
    Error(error) -> Error(error)
  }
}
```

Select transferable storage once in the Rust application's Cargo manifest,
then run `geam embedding sync`. Sync enables the required `geam-runtime-api`
feature on the application's Geam dependency:

```toml
[package.metadata.geam.embedding]
storage = "transferable"
```

This selects the storage used by the generated module, including its provider
values and state. It does not change an ordinary function into an async
function. The default local storage remains available for applications with
local-only Rust values.

The generated project and bindings use the same loading sequence:

```rust
let program = geam_bindings::project().compile()?;
let builder = WorkModuleBuilder::new(program)?;
let (bindings, functions) = geam_bindings::bind(builder)?;
let mut module = bindings.seal()?;
```

After initializing the generated `RunStateInputs`, attach the module to an
execution scope owned by the Rust application:

```rust
with_execution_scope(async |guard| {
    let mut scope = module.attach(guard, &mut state, &mut echo);
    let doubled = scope.call(&functions.double, (21.into(),))?;
    let work = scope.call(&functions.greeting, (path.into(),))?;
    let result = scope.observe(&work).await?;
    result.read(|value| println!("{value:?}"));
    Ok::<_, Box<dyn std::error::Error>>(())
})
.await?;
```

The application drives this enclosing Rust Future with its own executor.
`scope.call` evaluates the Gleam function and returns its value. For a function
returning `Future`, that value is work to observe, not its eventual result.
`scope.observe` drives the work and returns shared access to its result.
Observing the same work again reuses its completion rather than running its
native effects again.

The scope borrows the module, provider state, and Echo sink. Dropping one
observation leaves separately retained work available for another observation
in the same scope. Ending the scope cancels pending work; plain results already
obtained with `observe` remain available. See the
[embedding reference](reference/embedding-boundary.md#explicit-future-execution)
for nested Future values and completion errors.

The [async-host example](../examples/embedding/async_host) contains the complete
Gleam package, independent async file provider, generated bindings, state
initialization, and caller-owned executor. This workflow uses Rust embedding;
`geam run` does not drive source Future values.

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
place of its locked commit, changes a provider dependency, or regenerates stale
bindings.

Use `init` for an uninitialized package and `sync` after intentional source or
dependency changes. `embedding check` verifies the generated Gleam-Rust
connection, while `cargo check` and `cargo test` remain responsible for Rust
compilation and tests.

## Pass data between Gleam and Rust

Generated bindings currently support this recursive data grammar:

```text
Scalar | Tuple(Data...) | Result(Data, Data) | Option(Data) | List(Data) | Future(Data)
```

This includes nested Lists and combinations of Tuple, Result, and Option.
Transferable bindings also recognize the nominal `geam/future.Future` type in
these positions.
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

Both storage choices use `List<T>` in declarations. Transferable calls expose
shared, lazily inspected list values, so the same source type can move with
the application's Future between executor workers. Nested Future values keep
their execution scope; putting work inside a List does not erase its owner.

## Ship the Gleam sources with your application

The generated project selection reads `gleam/` and its resolved package sources
from the Cargo manifest directory when the application initializes. A deployment
therefore includes both the compiled binary and that source graph, kept in the
layout expected by the binary. Executable source bundling is a separate
deployment capability.

For the planner, runtime, and host ownership model, continue with
[architecture](reference/architecture.md) and
[runtime semantics](reference/runtime-semantics.md).
