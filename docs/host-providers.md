# Add Rust to a Gleam package

Most Gleam packages need no provider. Geam can run ordinary Gleam source and
its built-in package integrations directly.

A **host provider** is a companion Rust crate that implements native functions
from a Gleam package for Geam. Use one when part of the package API must run
inside the Rust host.

## One API, two packages

The Gleam package and provider have separate jobs:

| Artifact | Contains | Used for |
| --- | --- | --- |
| Gleam package, usually on Hex | Public Gleam modules, types, and external declarations | What applications add, import, and call |
| Rust provider on crates.io, Git, or a local path | Native implementations for those declarations | What Cargo compiles into the Geam host |

The provider does not replace or translate the Gleam package. Users keep
writing the same Gleam calls:

```text
Gleam application
  adds and imports example_text_tools as a Gleam dependency
  calls example_text_tools/casing.upper("Geam")
                              |
                              v
Geam links the declaration to geam-example-text-tools from Cargo
```

A package can keep separate Erlang or JavaScript implementations for the same
API. The provider adds the implementation used when Geam runs the package. The
same provider can be used by a standalone Gleam project or a Rust embedding
application.

## Select a provider explicitly

A provider may use any Cargo package name. Its documentation should tell
applications which crate to select and identify a particular version only when
one is required.

In a standalone Gleam project that contains the matching Gleam package, choose
one provider source and select it with the corresponding Geam command:

```sh
# Published reference provider on crates.io
geam provider add geam-example-text-pattern

# Local path
geam provider add --path ../provider

# Git revision
geam provider add --git https://example.com/provider.git --rev COMMIT
```

Append `@VERSION` to a crates.io name when selecting a particular release.

In a Rust embedding application, add the provider directly to the application's
`Cargo.toml`, using `cargo add` or an equivalent manifest edit, then run
`geam embedding sync`.

In both workflows, packaged metadata declares which Gleam package and versions
the crate implements. Geam uses that metadata to validate the application's
selection before planning or running application code.

## Follow one function from Gleam to Rust

The Gleam package declares a function without a Gleam body:

```gleam
// src/example_text_tools/casing.gleam
@external(erlang, "geam_example_text_tools_casing", "upper")
pub fn upper(value: String) -> String
```

The `erlang` target is intentional. Standard Geam workflows analyse the
Erlang-compatible source path and treat a bodyless Erlang external as a Rust
provider requirement. Geam does not call the named Erlang module or function;
it links the declaration to Rust by package, module, function, and exact type
signature. A bodyless JavaScript-only external is not available in this
workflow.

Create an ordinary Rust library crate beside the Gleam package:

```sh
cargo new --lib geam-example-text-tools
cd geam-example-text-tools
cargo add geam --no-default-features --features provider
```

Then declare the package and implement the matching module and function:

```rust
use geam::provider::StringValue;

#[geam::provider(
    package = "example_text_tools",
    modules = [casing],
)]
pub struct Component;

#[geam::module(path = "example_text_tools/casing")]
mod casing {
    use super::StringValue;

    #[geam::function]
    fn upper(value: StringValue) -> StringValue {
        value.to_uppercase().into()
    }
}
```

The macros generate provider registration and typed wiring. Rust checks the
declarations and supported Rust types at compile time. During preparation,
Geam compares that generated description with the typed Gleam declaration
before provider state is initialized or application code runs.

Geam re-exports its author-facing value types from `geam::provider`, so this
single dependency supplies types such as `StringValue`, `BigInt`, and `List`.

## Await Rust in a Gleam call

Use `#[geam::function(await)]` when an async Rust implementation should return
its completed result to Gleam. The Gleam call waits for that result while the
executor can run other work. For example, a provider can await a Gleam callback:

```rust
#[geam::function(await)]
async fn around<Item>(
    #[geam::call] call: &mut Call<RunState>,
    callback: Callback<fn() -> Value<Item>>,
) -> HostResult<Value<Item>> {
    call.with_state(|state| state.entries.push("before".into())).await?;
    let returned = call.invoke(&callback, ()).await?;
    call.with_state(|state| state.entries.push("after".into())).await?;
    Ok(returned)
}
```

The Gleam declaration still returns the callback's value:

```gleam
@external(erlang, "geam_example_call_tracing", "around")
pub fn around(callback: fn() -> item) -> item
```

The native function resumes when the callback completes, including when that
callback waits in another provider. State access uses bounded closures so the
callback can enter the same provider again. The [call-tracing example](../examples/provider/call_tracing)
does this with a delayed record operation on Tokio and preserves the
`before`, `inside`, `after` order.

The `await` marker returns the completed result through the ordinary Gleam
call. Without this marker, a Rust `async fn` returns an explicit source Future
instead, as shown next.

## Return a Rust-created function

A provider can return a function with immutable captures. Declare its private
body with `#[geam::callable(factory = Add)]`, mark captures with
`#[geam::capture]`, and give the creating function a `#[geam::factory]`
`Factory<Add>` parameter. `call.create(&factory, (offset,))` returns a typed
`Callback<fn(BigInt) -> BigInt>` that Gleam can store and call normally.
The private body does not need a Gleam external declaration.

The [callables example](../examples/provider/callables) includes the complete
Rust/Gleam pair, a generic constant factory, a wrapper whose argument and result
types differ, and a callback stored in a custom `Reply(item)`. Each function
instance keeps its captures and original execution. Aliases retain identity;
repeated construction creates distinct functions. See the
[reference](reference/provider-boundary.md#rust-created-function-values) for
signature, construction, and lifetime rules.

## Return async Rust work

An async provider function returns explicit work to Gleam. Its source
declaration uses the ordinary `geam` package:

```gleam
import geam/future.{type Future}

@external(erlang, "example_async_files", "read")
pub fn read(path: String) -> Future(Result(String, String))
```

The same function macro accepts an ordinary Rust `async fn`. Here the
provider uses the `async-fs` crate to read a file:

```rust
#[geam::module(path = "example_async_files")]
mod files {
    use geam::provider::StringValue;

    #[geam::function]
    async fn read(path: StringValue) -> Result<StringValue, StringValue> {
        async_fs::read_to_string(path.as_str())
            .await
            .map(StringValue::from)
            .map_err(|error| StringValue::from(error.to_string()))
    }
}
```

The macro maps the returned Rust Future to `Future(Result(String, String))`;
it does not wait for the file read while returning an ordinary Gleam Result.
No additional async metadata flag is needed. Gleam composes the work with
`future.map`, `future.then`, or `future.all`. `geam run` drives the Future returned
by `main`; a Rust embedding application observes work with its own executor.

Follow [Add the package](future.md#add-the-package) to include `geam`
in the Gleam package. The Future guide also explains the composition functions
and shared results.

The [async files provider](../examples/provider/async_files) includes a runnable
standalone project. Its [embedding application](../examples/embedding/async_host)
uses the same provider from Rust.

Provider state, retained payloads, and native Futures must be `Send`. They do not
need to be `Sync`: an async `Call` gives bounded access to the original mutable
state. A provider using Tokio can use the standalone runner's I/O and time
drivers; an embedding application supplies the runtime its providers require.

For Rust-owned values that need to work with Gleam Dynamic decoders, see
[native representations](reference/provider-boundary.md#native-representations).
The [native records example](../examples/provider/native_records) shows a record
decoded from Gleam and passed to a typed callback.

## Declare which Gleam versions it supports

Providers that share an execution-domain service, or whose component depends
on the generated profile, use the schema 2 composition contract described in
[execution services](reference/execution-services.md). Ordinary providers can
continue using schema 1 below.

Cargo metadata connects the crate to its Gleam package and states the package
versions implemented by this Rust code:

```toml
[package]
name = "geam-example-text-tools"
version = "0.1.0"

[package.metadata.geam.provider]
schema = 1
gleam-package = "example_text_tools"
gleam-version = ">= 1.0.0 and < 2.0.0"
```

`gleam-version` refers to the target Hex package, not the Gleam compiler. The
provider and Gleam package versions do not need to match.

Geam uses this metadata to check the exact package identity and supported range.
The Cargo package name itself does not establish that relationship.

## Run the complete pair

Keep a small Gleam application beside the provider while authoring it:

```text
example/
  project/   Gleam application and local Gleam package
  provider/  Rust provider crate
```

The application imports the Gleam package and calls its API normally:

```gleam
import example_text_tools/casing

pub fn main() {
  assert casing.upper("Geam") == "GEAM"
}
```

From the Gleam project, select the local companion crate and run the same path
that a standalone user relies on:

```sh
cd project
geam provider add --path ../provider
geam prepare
geam run
```

`provider add` verifies the crate metadata and records the local selection.
`prepare` checks the Rust implementation against the Gleam declarations and
builds the generated runner. `run` executes the Gleam application with that
implementation.

The repository's [text tools example](../examples/provider/text_tools) is this
complete flow with three Gleam modules. Its entrypoint asserts results such as
`upper("Geam") == "GEAM"`; a successful run is silent because every check
passes.

Keep unit tests for Rust-only logic and use this end-to-end run to verify the
Gleam declaration, provider metadata, generated component, and application call
together.

## Share producer-owned custom values

A source module can delegate its custom representation to other selected Rust
providers. The producer sets `HostCustomSchema::SHARED = true` in the schema used
by its SDK and registers that exact schema with
`HostProviderModule::with_shared_custom_type::<Schema>()` on the defining
package and module. Preparation without bodies uses the corresponding
`HostProviderModuleDeclaration` method; `into_declarations()` preserves grants.

Each native use of a shared schema requires the selected producer's matching
grant. Planning checks the source definition, complete constructor/field schema,
nominal type arguments and original public/internal/private scope. Prepared
loading checks the same requirement against the actual provider registrations,
including producers with no native functions. A missing or replaced grant fails
before execution. Changing this contract requires regenerating prepared data.

Sharing delegates constructor and field access to native code. It does not
change Gleam's opaque rules and is not a value-only permission. Producer SDKs
should keep ordinary value wrappers' storage private and expose the operations
they own. A consumer then retains or passes the wrapper and calls those
operations without reproducing schemas, storage or decoding. Existing schemas
with the default `SHARED = false` retain their previous visibility rules.
No runtime permission lookup, value copy, or new storage is introduced.

## Grow the provider with the package

The smallest provider is a collection of ordinary Rust functions. Add other
features only when the Gleam API calls for them:

- Use Rust scalars, tuples, `Result`, `Option`, and Geam's lazy `List` boundary
  for ordinary source values.
- Map a Gleam custom type when Rust constructs or receives its constructors.
- Use an external value when an opaque payload must remain owned by Rust.
- Add component state for process-local mutable or read-only capabilities.
- Add explicit configuration when constructing that state needs caller input.
- Accept a typed callback when provider code must call a Gleam function.
- Retain a generic source value only when an external value must own it across
  calls.

The [provider examples](../examples/provider) form an executable path through
those choices, beginning with scalar functions and ending with a separately
published Hex package and provider crate.

## Use the provider from an application

Provider crates are native code. A standalone project records an explicit
`geam provider add` selection in its managed Cargo files. A Rust embedding
application owns the provider as an ordinary direct Cargo dependency. Geam
verifies metadata and typed linkage in both workflows, but neither check is a
security endorsement.

Standalone applications pass provider configuration as TOML at run time.
Embedding applications construct the corresponding Rust configuration and
state values through generated bindings. External values and mutable state stay
inside the running application in both workflows.

See [standalone provider selection](standalone.md#use-a-gleam-package-with-a-rust-provider)
and [embedding package synchronization](embedding.md#use-gleam-packages-and-rust-providers)
for the consuming side of each workflow.

## Publish the pair

Publish the Gleam package with Gleam's Hex tooling and the provider with Cargo.
The Hex release contains the API that applications import. The crates.io
release contains the native code that Rust hosts compile. Test the packaged
provider with `cargo publish --locked --dry-run`, verify the public
Gleam-to-provider path, and widen `gleam-version` only when that package range
has been checked. Document the provider crate in both the package and provider
guides, and identify a particular release only when applications need one.

The final [text pattern example](../examples/provider/text_pattern) shows a Hex
package, a crates.io provider, and a separate Erlang implementation of the same
Gleam API.

## Exact reference

Continue with the [host provider boundary](reference/provider-boundary.md) for
the complete type mappings, custom and external values, state, configuration,
callbacks, retained storage, generated component contract, and runner profile.
The [runtime semantics](reference/runtime-semantics.md) document defines
ownership, equality, hashing, inspection, and failure behavior after linkage.
