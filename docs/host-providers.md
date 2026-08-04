# Host Provider Components

An ordinary Rust crate can provide implementations for source-declared Gleam
externals. The crate does not need dynamic loading or a Geam-specific package
format: a runner adds it as a normal Cargo path, Git, or registry dependency and
composes it into a concrete hosted profile at compile time.

This boundary is intentionally static. Geam does not discover provider crates,
parse provider configuration, generate a runner, or choose implementations at
runtime. Those are future CLI responsibilities built on the contracts described
here.

## Provider Crate

Each provider crate exports one marker that implements
`HostProviderComponent`. The component owns its store and run-state types and
initializes caller-owned state from an explicit, read-only configuration.

```rust
pub struct Component;

impl HostProviderComponent for Component {
    const ID: &'static str = "example";
    type Stores = Stores;
    type RunState = RunState;

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

## Runner Profile

A runner combines selected components with ordinary struct fields. The shape is
the target for future generated code, but it is regular Rust today.

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
runtime registry is involved.

The runner then performs the complete hosted pipeline explicitly:

```text
component configuration
-> Component::initialize
-> Component::providers
-> HostProviderSet
-> compile_typed_host_program or compile_typed_host_project
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
signature or permission checks.

## Future CLI Boundary

A future CLI may resolve provider dependencies, parse configuration, and emit
the aggregate `Stores`, `RunState`, `Profile`, projections, and initialization
code shown above. It does not need a new provider ABI to do so. This SDK does
not itself define discovery metadata, build caching, source generation, or a
publication workflow.
