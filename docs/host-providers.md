# Host Provider Components

An ordinary Rust crate can provide implementations for source-declared Gleam
externals. The crate does not need dynamic loading or a Geam-specific package
format: a runner adds it as a normal Cargo path, Git, or registry dependency and
composes it into a concrete hosted profile at compile time.

This boundary is intentionally static. The standalone CLI can discover and
approve provider dependencies, parse explicit configuration, and generate a
concrete runner, but the resulting Rust program still composes every component
at compile time. It does not choose or type-erase implementations at runtime.

## Scalar Provider Authoring

A scalar provider declares the same function once in Gleam and once in Rust.
The Gleam package owns the source-visible function shape:

```gleam
@external(erlang, "geam_counter", "next")
pub fn next(label: String) -> String
```

The Rust provider owns configuration, state, and behavior:

```rust
use ecow::EcoString;
use geam::provider::{Configuration, InitializationError};

pub struct RunState {
    next: i64,
}

fn initialize(
    configuration: &Configuration,
) -> Result<RunState, InitializationError> {
    let next = configuration
        .get("start")
        .and_then(|value| value.as_integer())
        .ok_or_else(|| {
            InitializationError::new(
                "configuration key `start` must be an Integer",
            )
        })?;

    Ok(RunState { next })
}

#[geam::provider(
    id = "geam-counter",
    package = "counter",
    state = RunState,
    initialize = initialize,
    modules = [counter],
)]
pub struct Component;

#[geam::module(path = "counter")]
mod counter {
    use super::{EcoString, RunState};

    #[geam::function]
    fn next(
        #[geam::state] state: &mut RunState,
        label: EcoString,
    ) -> EcoString {
        let next = state.next;
        state.next += 1;
        format!("{label}:{next}").into()
    }
}
```

`#[geam::state]` is an injected Rust parameter and is not part of the Gleam
function. The remaining Rust arguments and return type generate the typed host
schema `fn(String) -> String` through Geam's existing host type contracts. The
macro does not parse Gleam source or maintain a second Rust-to-Gleam type table.
The Erlang annotation strings only tell Gleam that an external implementation
exists; Geam links providers by the source package, module, function, and exact
scheme.

Validation therefore has two distinct phases. Rust compilation validates the
macro target, state injection, supported scalar host types, and generated
component implementation. `geam prepare` then compiles the complete Gleam
project and links the generated schema against the source declaration. A Rust
and Gleam signature mismatch remains an exact hosted-linkage error before any
provider state is initialized or source code executes.

This first authoring slice supports stateful scalar functions. External values,
custom types, compound values, callbacks, and source `Result` construction
still use the low-level typed host contracts described below. The completed
counter is an interface-review checkpoint, not a claim that the authoring API
for those later capabilities is final.

## Generated Component Boundary

Each provider crate exports one marker that implements
`HostProviderComponent`. The component owns its store and run-state types.
Provider crates that consume configuration implement the separate
`HostProviderComponentInitialization` contract. Scalar authoring macros
generate these implementations; the explicit form remains the underlying SDK
boundary for capabilities outside the first macro slice.

```rust
pub struct Component;

impl HostProviderComponent for Component {
    const ID: &'static str = "example";
    type Stores = Stores;
    type RunState = RunState;
}

impl HostProviderComponentInitialization for Component {
    fn initialize(
        configuration: &HostProviderConfiguration,
    ) -> Result<RunState, HostProviderInitializationError> {
        // Read owned String, i64, f64, bool, array, or table values here.
        todo!()
    }
}
```

Configuration has no environment-variable lookup, global state, parser, or
hidden defaults. A runner constructs `HostProviderConfiguration` explicitly.
Initialization failure names the component and remains an assembly error before
planning or execution; it is not an `ExecutionError` or host callback failure.
Geam's built-in stdlib, JSON, and Time components do not implement configured
initialization: the runner separately constructs their IO, entropy, stateless
JSON, and clock capabilities.

The same component implements `HostProviderComponentRegistration<Profile>` for
every concrete profile that projects it. Registration returns source-backed
`HostProviderModule`s and uses the normal typed host APIs.

```rust
impl<Profile> HostProviderComponentRegistration<Profile> for Component
where
    Profile: HostComponentProfile<Self>,
{
    fn providers() -> Result<Vec<HostProviderModule<Profile>>, HostRegistrationError> {
        HostProviderModule::new("example_package", "example/module")
            .and_then(|provider| {
                provider.with_scoped_function::<Provider, _, _, _>(
                    "run",
                    run::<Profile>,
                )
            })
            .map(|provider| vec![provider])
    }
}
```

The provider callback marker implements `HostProvider<Profile>` generically and
projects only this component's state through `HostComponentProfile<Component>`.
That keeps callback state concrete without making the aggregate runner profile
part of the provider crate.

## Provider Authoring Example

[`geam-example-text-pattern`](../examples/text_pattern/provider) is a compact
provider intended to be read as normal crate source. It maps the ordinary
`example_text_pattern` Gleam package to Rust `regex` without adding package-side
Geam metadata. The component demonstrates:

- a constructorless `Pattern` external backed by immutable provider storage;
- source equality, hashing, and canonical inspection for opaque values;
- `Result(Pattern, CompileError)` construction through sealed intermediate
  capabilities;
- scalar arguments and returns plus `List(String)` output; and
- the explicit stateless component-initialization boundary.

Its Cargo manifest uses the canonical discovery name and schema-1 metadata:

```toml
[package]
name = "geam-example-text-pattern"

[package.metadata.geam.provider]
schema = 1
gleam-package = "example_text_pattern"
gleam-version = ">= 1.0.0 and < 2.0.0"
```

The [complete example](../examples/text_pattern/README.md) executes this crate
through explicit path selection and packages it with ordinary Cargo tooling.
Its [provider README](../examples/text_pattern/provider/README.md) also records
the concrete higher-level authoring target that will replace the current
low-level declarations before publication.

## Runner Profile

A runner combines selected components with ordinary struct fields. An embedding
application can write this regular Rust directly, while the standalone CLI
emits the same shape for a managed project.

```rust
struct Profile;

#[derive(Default)]
struct Stores {
    example: <Component as HostProviderComponent>::Stores,
}

struct RunState {
    example: <Component as HostProviderComponent>::RunState,
}

impl HostProfile for Profile {
    type ExternalStores = Stores;
    type RunState = RunState;
}

impl HostComponentProfile<Component> for Profile {
    fn component_stores(
        stores: &Stores,
    ) -> &<Component as HostProviderComponent>::Stores {
        &stores.example
    }

    fn component_state(
        state: &mut RunState,
    ) -> &mut <Component as HostProviderComponent>::RunState {
        &mut state.example
    }
}
```

For multiple components, the aggregate structs add one concrete field and one
projection implementation per component. No trait object, type-erased map, or
runtime registry is involved. Geam built-ins and approved Cargo dependencies
use this same field, projection, and registration path; discovery and state
construction are the parts that differ.

The runner then performs the complete hosted pipeline explicitly:

```text
Component::providers
-> HostProviderSet
-> compile_typed_host_program or compile_typed_host_project
-> configured Component::initialize and runner capability construction
-> plan_host_program
-> HostedExecution::try_from_module_plan
-> HostedExecution::run_main with aggregate RunState
```

The readable, executable version of this assembly is
[`tests/fixtures/provider_sdk/runner/tests/public_usage.rs`](../tests/fixtures/provider_sdk/runner/tests/public_usage.rs).
It keeps the complete Gleam declarations, provider composition, expected value,
and state assertions visible in one file.

## External Storage

External payload ownership stays in the provider crate. A local adapter
implements `HostExternalStorage<Profile, Schema>` generically for profiles that
project the component, and the provider marker selects it with
`HostExternalBinding<Profile, Schema>`.

```text
provider marker
-> external schema
-> provider-owned storage adapter
-> component Stores field
-> aggregate profile projection
```

This avoids requiring the final runner profile to implement a foreign storage
trait for a foreign schema. The adapter supplies Gleam equality, source hashing,
and canonical inspection. Public external values retain opaque payload leases;
they never expose the Rust payload or borrow the runner state.

## Compound Construction

An exact return type can still be built with the return-specific `HostCall`
methods. Intermediate lists, tuples, ordinary custom values, and externals must
be declared when the callback is registered:

```rust
type Constructions = HostTypeList<HostListType<EcoString>, HostTypeListEnd>;

provider.with_scoped_function_and_constructions::<
    Provider,
    Arguments,
    Return,
    Constructions,
    _,
>("summarize", summarize::<Profile>)?;
```

The callback receives `HostConstructions<'call, Constructions>` immediately
after `HostCall`. `constructions.at::<HostTypeIndex0>()` produces a token for
the exact registered list type, which can be passed to
`HostCall::construct_list`. The token cannot be forged, selected at the wrong
type or index, or retained beyond the active call. Registration metadata and
the callback capability come from the same type list, so runtime does not need
signature or permission checks. Generic construction types may refer only to
type parameters already bound by the function signature.

## Standalone CLI Boundary

The standalone CLI emits one aggregate `Stores`, `RunState`, `Profile`,
projection, and registration graph for Geam built-ins and approved Cargo
dependencies. A provider crate advertises one component through
`[package.metadata.geam.provider]`; the CLI verifies that metadata before
recording the crate as an ordinary exact Cargo dependency. Generated code uses
only the crate-root `Component` and the public component contracts. Configured
dependency initialization and runner-owned capability construction remain
separate strategies within that graph.

To make a published provider discoverable, derive its Cargo package name from
the target Gleam package: add the `geam-` prefix and replace underscores with
hyphens. A provider whose metadata targets `company_image` therefore publishes
as `geam-company-image`; alternatives may append a kebab-case suffix. The name
only places the crate in the discovery namespace. Packaged metadata remains the
authority for the exact `company_image` identity, and explicitly selected
registry, path, or Git crates may use other names.

Discovery, native-code approval, managed Cargo files, and runtime configuration
belong to the CLI rather than this SDK. Provider callbacks, stores, and state
remain governed by the same static ABI whether a runner is generated or written
by an embedding application. See [standalone execution](standalone.md) for the
CLI workflow and trust boundary.
