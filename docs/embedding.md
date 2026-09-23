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

## Prepare a program before building

By default, `project().compile()` reads the Gleam project when the Rust
application initializes. To include the program in the executable instead, add
this setting to `Cargo.toml`:

```toml
[package.metadata.geam.embedding]
generate = "prepared"
```

Then prepare the program:

```sh
geam embedding sync
```

Sync generates the typed bindings and the complete execution plan as Rust data.
For the same `double` function above, `src/main.rs` becomes:

```rust
mod geam_bindings;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (module, functions) = geam_bindings::load()?;
    let mut echo = Vec::new();

    let value = module.call(&functions.double, (21.into(),), &mut echo)?;
    println!("{value}");
    Ok(())
}
```

```sh
geam embedding check
cargo run --release
```

The application still prints `42`. `load()` creates a fresh module and its
typed handles from the included program, without reading Gleam files. Calls
use the same runtime as dynamic embedding.

The setting controls which interfaces sync generates, not a global execution
mode:

| `generate` | Generated loading interfaces |
| --- | --- |
| `"dynamic"` (default) | `project()` and `bind()` load current source |
| `"prepared"` | `load()` loads the prepared program |
| `"both"` | Both interfaces; choose at each initialization site |

All three share the same function and value definitions. With providers,
`load()` returns the hosted module and handles; keep the same caller-owned
configuration, state, Echo and execution host used for dynamic calls.

Prepared sync and check can compile and run a Rust preparation helper using
the application's dependencies. They do not build the application entry point
or initialize its run state. Ordinary Cargo builds compile the existing
generated files without running Geam or regenerating the plan. Run sync on a
fresh checkout and after source or dependency changes, including function-body
changes.

The [prepared example](../examples/embedding/prepared) keeps the complete
project and test together.

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
| Opaque session | Keep Gleam-owned private data between calls | [`session`](../examples/embedding/session) |
| Execution control | Cancel a running Gleam call and continue using its module | [`execution`](../examples/embedding/execution) |
| Process service | Retain Pid and Subject handles and call a running Gleam service | [`processes`](../examples/embedding/processes) |
| Prepared program | Include the Gleam program in the Rust executable | [`prepared`](../examples/embedding/prepared) |

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
only when its contents change. Prepared generation also rebuilds the execution
data. Run the Rust application separately with the usual Cargo commands.

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
modules may use records, custom types, and provider-backed values. Concrete
custom and external types cross this boundary as opaque handles: Rust can
retain them and pass them back without reconstructing their fields.

Commit the Cargo and Gleam manifests and lockfiles, handwritten Gleam and Rust
source, and generated `src/geam_bindings.rs`. Ignore Cargo's `target/` and
Gleam's `gleam/build/` cache. The typed bindings are reviewed and committed;
no build script regenerates them implicitly.

With `generate = "prepared"` or `"both"`, keep the generated execution data out
of Git by adding `/src/geam_bindings/program.rs` to the application's
`.gitignore`. Run `geam embedding sync` before checking or building a fresh
checkout. Subsequent `embedding check` calls verify both the bindings and the
generated program without changing them.

Cargo packages must still include `src/geam_bindings/program.rs` alongside the
bindings. Generate it before packaging and explicitly include it in the package,
since Cargo's default file selection respects Git ignores. For example, an
application that ships only its prepared Rust source can use:

```toml
[package]
include = ["src/**", "Cargo.toml", "Cargo.lock"]
```

Keep any other required package files in that list. Consumers of the resulting
package build it with ordinary Cargo commands without Geam or Gleam installed.

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
stdlib, JSON, Time, and Erlang integrations are added explicitly; unused Gleam
dependencies do not add Rust components.

For a concurrent Gleam service, see [Gleam processes](processes.md). Generated
Pid and Subject handles remain usable across calls in the same execution scope.

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

## Drive hosted calls

Hosted bindings use the Rust application's executor. This lets Gleam execution
yield during long computations and resume after a native callback waits.
The application keeps its provider state and Echo sink throughout the scope.

For Tokio, enable Geam's `tokio` feature and connect the adapter to your existing
runtime:

```rust
use geam::execution::TokioHost;

let host = TokioHost::new(tokio::runtime::Handle::current());
module.with_execution(&host, &mut state, &mut echo, async |scope| {
    let first = scope.call(&functions.double, (21.into(),)).await?;
    println!("{first}");
    Ok::<_, Box<dyn std::error::Error>>(())
}).await??;
```

The enclosing Rust Future services Gleam calls while the body runs. Returning
from the body ends the scope and waits for its workers to release their inputs.
Dropping a pending `scope.call` cancels that call; dropping the enclosing Future
requests scope shutdown without blocking the dropping thread.

`TokioHost` uses the runtime you provide; it does not create another runtime.
Other executors can implement `geam::execution::ExecutionHost`, including its
task cancellation and clock contracts. Pure bindings that use `ModuleBuilder`
also retain the direct `module.call` API shown in the first example.

## Pass and create function values

A generated binding can accept or return a concrete Gleam function type. Keep
the returned handle within its execution scope and invoke it explicitly:

```rust
let alias = scope.call(&functions.keep, (&callback,)).await?;
let next = scope.invoke(&alias, (BigInt::from(7),)).await?;
```

A function value has its own captures and identity. Its aliases use the same
execution and provider state. Each call has independent arguments and pending
work; passing a function does not invoke it or observe a returned Future.

The [callables example](../examples/embedding/callables) also creates a capturing
Rust function inside the application. It shares `src/declarations.rs` with the
preparation helper, registers real bodies from ordinary application modules,
selects a factory before sealing, and constructs it inside the live scope.
Dynamic and prepared loading expose the same typed calls. The example keeps a
callback in private Gleam data and then invokes a source wrapper around it.
See the [function-value reference](reference/embedding-boundary.md#function-values-and-native-construction)
for declaration configuration and exact ownership rules.

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

Run `geam embedding sync` after adding the Gleam package and Rust provider.
Sync enables the features needed by the selected packages and providers.
Ordinary functions and Future functions share one loaded module and provider
state.

The generated project and bindings use the same loading sequence:

```rust
let program = geam_bindings::project().compile()?;
let builder = HostedModuleBuilder::new(program)?;
let (bindings, functions) = geam_bindings::bind(builder)?;
let mut module = bindings.seal()?;
```

After initializing the generated `RunStateInputs`, use the same host-driven
scope for calls and Future observations:

```rust
module.with_execution(&host, &mut state, &mut echo, async |scope| {
    let doubled = scope.call(&functions.double, (21.into(),)).await?;
    let work = scope.call(&functions.greeting, (path.into(),)).await?;
    let result = scope.observe(&work).await?;
    result.read(|value| println!("{value:?}"));
    Ok::<_, Box<dyn std::error::Error>>(())
})
.await??;
```

The application drives this enclosing Rust Future with its own executor.
`scope.call(...).await` evaluates the Gleam function and returns its value. For a function
returning `Future`, that value is work to observe, not its eventual result.
`scope.observe` drives the work and returns shared access to its result.
Observing the same work again reuses its completion rather than running its
native effects again.

The scope borrows the module, provider state, and Echo sink. Dropping one
observation leaves separately retained work available for another observation
in the same scope. Ending the scope cancels pending work; plain results already
obtained with `observe` remain available. See the
[embedding reference](reference/embedding-boundary.md#explicit-future-values)
for nested Future values and completion errors.

The [async-host example](../examples/embedding/async_host) contains the complete
Gleam package, independent async file provider, generated bindings, state
initialization, and caller-owned executor. The same provider also works through
the [standalone async example](../examples/provider/async_files).

State, retained values, native Futures, and the Echo sink must be `Send`; borrowed
host resources need only live for the execution scope. Geam does not require
`Sync` for exclusively accessed state or create an executor for embedding.

## Verify a checkout

For a prepared or both-mode checkout, run `geam embedding sync` first to create
the ignored program data. Then use `check` in review or CI; dynamic-only
checkouts can start with `check` directly:

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
connection. In dynamic mode it does not compile Rust. In prepared or both mode
it regenerates the expected plan in a disposable helper and compares it with
the generated data, including source bodies and dependencies. Dependency build
scripts and native registration can run during this preparation; application
entry points and run-state initialization do not. `cargo check` and `cargo test`
remain responsible for compiling and testing the handwritten Rust application.

## Pass data between Gleam and Rust

Generated bindings currently support this recursive data grammar:

```text
Scalar | Tuple(Data...) | Result(Data, Data) | Option(Data) | List(Data) | Future(Data) | fn(Data...) -> Data | Named
```

This includes nested Lists and combinations of Tuple, Result, and Option.
Bindings also recognize the nominal `geam/future.Future` type in
these positions.
Concrete custom and external types, including generic specializations such as
`Session(Int)`, map to opaque handles. Rust can keep a value and pass it back
to Gleam in the same execution scope:

```rust
let session = scope.call(&functions.start, (40.into(),)).await?;
let next = scope.call(&functions.next, (&session,)).await?;
let total = scope.call(&functions.total, (next,)).await?;
```

The [Session example](../examples/embedding/session) includes the corresponding
Gleam type, functions, and generated Rust bindings. Its private fields stay in
Gleam. Expose a Gleam accessor when Rust needs to inspect them. Function values
use scoped typed callable handles, including inside containers; public roots
with unbound generic parameters remain outside generated signatures.

Lists returned from Gleam are retained, immutable handles. Rust can inspect
them lazily or pass them back to the same loaded module without reconstructing
their items. The [structured-data
example](../examples/embedding/data)
shows both operations without adding providers. See the [embedding
boundary](reference/embedding-boundary.md) for the complete type map, ownership
rules, list transfer behavior, provider state, and lower-level manual binding
API.

Within a host-driven execution scope, `List<T>` declarations produce `SharedList`
values with borrowed item access. Direct module calls return `List<T>` values
with owned item access. Both retain their source storage. Nested Future values
keep their execution scope; putting work inside a List does not erase its owner.

## Deploy the application

Dynamic initialization reads `gleam/` and its resolved package sources from the
Cargo manifest directory. Deploy that source graph in the layout expected by
the binary when using `project().compile()`.

Prepared initialization uses the program compiled into the executable. The
binary can run without the original Gleam project, Geam CLI or Gleam CLI.
Include both generated Rust files when distributing a Cargo source package;
building that package does not require the original Gleam project either.

Application resources are separate from code. A program that uses files or
`gleam/erlang/application.priv_directory` still needs those resources and an
explicit resource catalog from its Rust host. Preparing a program does not
bundle resource files or build-machine paths.

After upgrading Geam, run `geam embedding sync` before building. Prepared data
is tied to its Geam artifact format; an incompatible format or linked provider
contract is rejected rather than falling back to source compilation.

For the planner, runtime, and host ownership model, continue with
[architecture](reference/architecture.md) and
[runtime semantics](reference/runtime-semantics.md).
