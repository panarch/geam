# Host Provider Components

An ordinary Rust crate can provide implementations for source-declared Gleam
externals. The crate does not need dynamic loading or a Geam-specific package
format: a runner adds it as a normal Cargo path, Git, or registry dependency and
composes it into a concrete hosted profile at compile time.

This boundary is intentionally static. The standalone CLI can discover and
approve provider dependencies, parse explicit configuration, and generate a
concrete runner, but the resulting Rust program still composes every component
at compile time. It does not choose or type-erase implementations at runtime.

Start with the [provider authoring examples](../examples/README.md). They present
multi-module registration, scalar, tuple, List, custom, Result, and Option value
mappings, stateless, default-state, configured-state, default external, and
manual external choices plus generic retention as complete Gleam/Rust pairs
before this document describes the generated and low-level contracts.

## Value Type Provider Authoring

The [value-types example](../examples/value_types/README.md) is the canonical map
from Gleam source values to macro-authored Rust signatures. Its scalar module
maps `String`, `Int`, `Float`, `BitArray`, `UtfCodepoint`, `Bool`, and `Nil` to
`EcoString`, `BigInt`, `f64`, `BitArrayValue`, `char`, `bool`, and `()`. Its
tuple module recursively composes those leaves with native Rust tuples, and its
List module provides lazy indexed views plus explicit new-list construction. A
tuple remains one Gleam source argument even when it contains several elements:

```rust
#[geam::function]
fn swap(value: (EcoString, BigInt)) -> (BigInt, EcoString) {
    let (label, count) = value;
    (count, label)
}

#[geam::function]
fn reassociate(
    value: (EcoString, (BigInt, bool)),
) -> ((EcoString, BigInt), bool) {
    let (label, (count, enabled)) = value;
    ((label, count), enabled)
}
```

Rust `(T,)` corresponds to Gleam `#(T)`, while Rust `()` keeps its existing
Gleam `Nil` meaning. Tuple elements can recursively use the scalar and external
payload forms supported by the macro; external arguments remain immutable
payload views and external returns remain owned payloads.

A top-level Gleam `List(T)` argument maps to opaque `geam::List<T>`. Retaining
that view and asking for its length are O(1); `get` decodes only the requested
item. Returning a received `geam::List<T>` passes through the original runtime
List, while returning `Vec<T>` constructs one new source List:

```rust
#[geam::function]
fn first_or(
    values: geam::List<EcoString>,
    fallback: EcoString,
) -> EcoString {
    values.get(0).unwrap_or(fallback)
}

#[geam::function]
fn identity(values: geam::List<BigInt>) -> geam::List<BigInt> {
    values
}

#[geam::function]
fn reverse(values: geam::List<EcoString>) -> Vec<EcoString> {
    (0..values.len())
        .rev()
        .map(|index| values.get(index).expect("index comes from the List length"))
        .collect()
}
```

List items currently support scalar, external, and custom leaves plus recursive
tuples of those leaves. External items are opaque guards that dereference to the
provider payload without cloning it. Nested Lists and Lists inside tuples remain
outside this authoring slice.

## Custom Value Provider Authoring

An ordinary Gleam custom type maps to one Rust output enum and, when existing
source values are accepted, one explicit generated input enum:

```gleam
pub type Job {
  Pending
  Named(String)
  Scheduled(label: String, attempt: Int)
  Prioritized(Priority)
  Tags(List(String))
}
```

```rust
#[geam::custom(input = JobInput)]
enum Job {
    Pending,
    Named(EcoString),
    Scheduled { label: EcoString, attempt: BigInt },
    Prioritized(Priority),
    Tags(Vec<EcoString>),
}

#[geam::function]
fn describe(job: JobInput) -> EcoString {
    // Match the source constructors directly.
    todo!()
}
```

The owned `Job` form constructs a new source value. `JobInput` decodes only the
active constructor and is call-scoped; nested custom fields use the nested
declaration's input form. A Gleam List field is a lazy `geam::List<T>` in the
generated input enum and a `Vec<T>` in the owned output enum. Unit, tuple, and
named variants preserve lexical constructor order and Rust field names become
Gleam field labels.

The declaration protocol is static across sibling modules and provider crates.
The consuming macro refers to the declaring type's sealed schema and codec; it
does not inspect source files, compare runtime type names, or copy external
payloads. An output-only enum omits `input = ...`, and using it as a source
argument produces a diagnostic that asks for an explicit input type.

## External Value Provider Authoring

The [run-metrics example](../examples/run_metrics/README.md) gives the Rust
provider one constructorless source type and four functions. The Gleam package
owns the visible value flow:

```gleam
@external(erlang, "geam_example_run_metrics", "Metrics")
pub type Metrics

@external(erlang, "geam_example_run_metrics", "new")
pub fn new() -> Metrics

@external(erlang, "geam_example_run_metrics", "record")
pub fn record(metrics: Metrics, name: String, value: Float) -> Metrics

@external(erlang, "geam_example_run_metrics", "count")
pub fn count(metrics: Metrics, name: String) -> Int

@external(erlang, "geam_example_run_metrics", "total")
pub fn total(metrics: Metrics, name: String) -> Float
```

The matching Rust module declares the payload and source semantics at the same
site as its functions:

```rust
use ecow::EcoString;
use geam::provider::ExternalPayload;
use num_bigint::BigInt;
use std::collections::BTreeMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[geam::provider(
    package = "example_run_metrics",
    modules = [metrics],
)]
pub struct Component;

#[geam::module(path = "example_run_metrics")]
mod metrics {
    use super::*;

    #[geam::external(name = "Metrics", manual)]
    #[derive(Clone, Default, PartialEq)]
    struct Metrics {
        entries: BTreeMap<EcoString, Metric>,
    }

    #[derive(Clone, Default, PartialEq)]
    struct Metric {
        count: BigInt,
        total: f64,
    }

    impl ExternalPayload for Metrics {
        fn source_equal(&self, other: &Self) -> bool {
            self == other
        }

        fn source_hash(&self) -> u64 {
            let mut hasher = DefaultHasher::new();
            for (name, metric) in &self.entries {
                name.hash(&mut hasher);
                metric.count.hash(&mut hasher);
                let total = if metric.total == 0.0 {
                    0
                } else {
                    metric.total.to_bits()
                };
                total.hash(&mut hasher);
            }
            hasher.finish()
        }

        fn inspect(&self) -> EcoString {
            let entries = self.entries.iter().map(|(name, metric)| {
                let total = if metric.total == 0.0 { 0.0 } else { metric.total };
                format!("#({name:?}, #({}, {total:?}))", metric.count)
            }).collect::<Vec<_>>().join(", ");
            format!("Metrics([{entries}])").into()
        }
    }

    #[geam::function]
    fn new() -> Metrics {
        Metrics::default()
    }

    #[geam::function]
    fn record(metrics: &Metrics, name: EcoString, value: f64) -> Metrics {
        let mut updated = metrics.clone();
        let metric = updated.entries.entry(name).or_default();
        metric.count += 1u8;
        metric.total += value;
        updated
    }

    #[geam::function]
    fn count(metrics: &Metrics, name: EcoString) -> BigInt {
        metrics.entries.get(&name)
            .map(|metric| metric.count.clone())
            .unwrap_or_default()
    }

    #[geam::function]
    fn total(metrics: &Metrics, name: EcoString) -> f64 {
        metrics.entries.get(&name).map_or(0.0, |metric| metric.total)
    }
}
```

`#[geam::external]` generates one typed schema, payload store, storage adapter,
and provider binding. By default it also implements source equality and hashing
through the payload's `PartialEq` and `Hash` implementations, with sealed
`TypeName(<opaque>)` inspection. The `manual` flag keeps registration generation
but leaves `ExternalPayload` to the provider, as above, when source semantics
need specialized equality, hashing, or inspection. Equal signed-zero totals in
this example must share a hash.

An external source argument is an immutable `&Metrics` payload view in Rust; an
external source return is an owned `Metrics` that Geam seals into the store.
`record` therefore returns a persistent update rather than mutating the old
source value.

Scalar positions still use Geam's existing host types: `EcoString`, `f64`, and
`BigInt` correspond to `String`, `Float`, and `Int`. Native tuples recursively
compose those scalars and declared external payloads. The macro does not parse
Gleam source or maintain another Rust-to-Gleam type table. Erlang annotation
strings only establish external availability; Geam links by source package,
module, function or type, and exact scheme.

Rust compilation validates the macro targets, payload trait, borrowing rules,
and generated typed registrations. `geam prepare` then compiles the complete
Gleam project and links those schemas against the source declarations before
initialization or execution. A provider with no process-local state or
configuration omits both declarations; Geam supplies unit state and rejects
unexpected configuration instead of ignoring it.

The current macro surface supports scalars, native tuples composed from
supported leaves, top-level Lists with lazy item access or Vec construction,
non-recursive custom values, Rust `Result`/`Option` mapped to their standard
source types, constructorless external values, generic retained values, and
typed callbacks. Existential retained values use the explicit
`provider::advanced` API; nested Lists remain unsupported.

## Typed Callback Invocation

[`call_tracing`](../examples/call_tracing/README.md) separates opaque function
pass-through from invocation. `Value<fn(...) -> ...>` remains an opaque source
handle; `Callback<fn(...) -> ...>` grants one active `&mut Call` permission to
invoke the function:

```rust
fn around<Item>(
    #[geam::call] call: &mut Call<RunState>,
    callback: Callback<fn() -> Value<Item>>,
) -> HostResult<Value<Item>> {
    call.state_mut().entries.push("before".into());
    let returned = call.invoke(callback, ())?;
    call.state_mut().entries.push("after".into());
    Ok(returned)
}
```

Callback arguments use provider output types and callback results use provider
input views. The generated adapter registers any required constructions once,
then invokes the existing typed host ABI without materializing generic values.
`Call::invoke` preserves nested source panics and provider failures. A live
state borrow prevents callback re-entry through Rust's borrow checker, so state
must be released before invoking source code.

## Generic Values And Retention

[`generic_box`](../examples/generic_box/README.md) shows the ordinary generic
retention path. `Value<Item>` is an opaque value for the current call;
`Stored<Item>` is the non-cloneable field of one source-visible external
payload. The external declaration fixes the source parameter position once, so
`Box(Int)` and `Box(String)` share one Rust external store while each restore
uses its exact specialization:

```rust
#[geam::external(
    name = "Box",
    parameters = [Item],
    input = BoxInput,
)]
pub struct BoxValue<Item> {
    #[geam::stored]
    value: Stored<Item>,
}

fn get<Item>(
    #[geam::call] call: &mut Call<()>,
    boxed: BoxInput<Item>,
) -> Value<Item> {
    call.restore(boxed.value())
}
```

`Call::store` retains the existing runtime value without converting it to an
eager Rust representation. `Call::restore` recreates only a call-scoped typed
handle. Neither operation clones the source payload, and returning an old box
does not reconstruct its external value.

Providers with a persistent Rust collection of retained entries use the
explicit advanced form instead of pretending that collection is a generic Rust
payload:

```rust
#[geam::external(
    name = "PriorityQueue",
    parameters = [Item],
    input = PriorityQueueInput,
    payload = PriorityQueuePayload,
    manual,
)]
pub struct PriorityQueue<Item>;
```

The non-generic payload stores
`provider::advanced::Retained<PriorityQueuePayload, Index0>` inside its own
immutable persistent structure. The generated input exposes the payload and a
typed `stored_item` selector; source equality, hashing, and inspection are
implemented with the narrow `RetainedExternalPayload` operation contexts.
This advanced form exposes no runtime type name, downcast, mutable graph, or
per-specialization store.

## Generated Component Boundary

Each provider crate exports one marker that implements
`HostProviderComponent`. The component owns its store and run-state types.
Provider crates that consume configuration implement the separate
`HostProviderComponentInitialization` contract. Authoring macros generate these
implementations together with module registrations and external stores. The
explicit typed-host form remains the low-level SDK boundary and its canonical
fixture, rather than boilerplate required by ordinary provider authors.

The provider component identity defaults to the Cargo package name and can be
overridden with `id = "..."` when diagnostics need a distinct stable identity.
This identity is separate from the required Gleam `package` and module paths.
When `state` is omitted the component uses unit state. When `state = RunState`
is present without `initialize`, empty configuration constructs
`RunState::default()`. Both default forms reject non-empty configuration. A
configured provider supplies both `state` and `initialize`; an initializer
without a state declaration is rejected by the macro.

A function may inject its active provider call as the first parameter with
`#[geam::call]`. Use `&Call<RunState>` for read-only state access and
`&mut Call<RunState>` for mutation or call-scoped capabilities. Read state with
`call.state()` and mutate it with `call.state_mut()`. The injected parameter is
not part of the Gleam function signature; all following parameters remain
ordinary source arguments.

A provider function that can stop execution returns `HostResult<T>` and creates
the failure with `HostFailure::new(reason)`. This outer envelope is not part of
the Gleam function shape. Rust `Result<T, E>` remains the source-visible Gleam
`Result(T, E)`, so recoverable source errors and host execution failures cannot
be confused.

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

## Advanced Provider Example

[`geam-example-text-pattern`](../examples/text_pattern/provider) is a compact
provider intended to be read as normal crate source. It maps the ordinary
`example_text_pattern` Gleam package to Rust `regex` without adding package-side
Geam metadata. The component demonstrates:

- a constructorless `Pattern` external backed by immutable provider storage;
- source equality, hashing, and canonical inspection for opaque values;
- a named `CompileError` custom value and ordinary Rust `Result` mapping;
- scalar arguments and returns plus `List(String)` output; and
- generated stateless component initialization and typed registration.

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
Its [provider README](../examples/text_pattern/provider/README.md) explains the
complete macro-authored Rust mapping and why `Pattern` owns manual source
semantics.

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
