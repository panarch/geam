# Rust Embedding

Geam can generate the mechanical Rust bindings for one public Gleam module
inside an ordinary Cargo package. The Rust application continues to own source
loading, execution sealing, capabilities, provider configuration, mutable
state, Echo, and call order.

The complete managed example is
[`examples/rust_embedding_application`](../examples/rust_embedding_application).
It combines imported Gleam source, `gleam/io`, and the text-pattern provider in
one independently locked Rust application. Its inventory workflow passes rows
from Rust into Gleam, retains validation results, and reuses them across calls.

## Project Layout

Keep the resolved Gleam project inside the Rust package and commit the generated
Rust source:

```text
application/
  Cargo.toml
  Cargo.lock
  src/
    main.rs
    geam_bindings.rs
  gleam/
    gleam.toml
    manifest.toml
    src/
      application.gleam
```

Cargo build output and downloaded Gleam package source are local artifacts;
ignore `target/` and `gleam/build/`. Keep both lockfiles and
`src/geam_bindings.rs` in source control.

Start the conventional project from the Cargo package directory:

```sh
geam embedding init
```

Init creates a pure Gleam starter and generated bindings without changing
handwritten Rust. When Geam is absent it adds an exact matching dependency with
defaults disabled and `embedding` enabled. Existing dependency sources, aliases,
features, and unrelated Cargo content are preserved.

Sync enables the built-ins reached by the selected Gleam source closure:

- `gleam-stdlib` exposes `geam::gleam_stdlib`;
- `gleam-json` exposes JSON and its stdlib dependency;
- `gleam-time` exposes Time and its stdlib dependency.

For example, the canonical application enables `embedding,gleam-stdlib`.
Its text-pattern provider enables `provider` on the same Geam package identity,
so Cargo unifies the authoring macros without restoring Geam defaults or CLI.
An application that authors providers directly can add `provider` itself.

The nested project is always `gleam/`. Its package name and public boundary
module use the Cargo package name with hyphens replaced by underscores.
For Cargo package `inventory-app`, use:

```toml
# gleam/gleam.toml
name = "inventory_app"
```

Public functions in `gleam/src/inventory_app.gleam` form the Rust boundary.
Internal modules can live under `gleam/src/inventory_app/`. There is no
embedding selector metadata in Cargo.toml or special `lib.gleam` file.

The package must have one enabled direct dependency on `geam`. Each external
provider required by the selected Gleam source closure must also be an enabled
direct Cargo dependency with valid Geam provider metadata. The dependency's
actual Cargo alias is retained in generated source.

When a required provider is missing, `sync` discovers metadata-verified registry
candidates and asks for explicit native-code approval. Only approved providers
are added, at exact versions. Existing compatible declarations, including path
or Git sources, are reused without another prompt. Incompatible declarations
and alias collisions fail instead of being replaced. Noninteractive runs cannot
approve new providers; prepare them interactively before committing the manifest
and lockfile.

`sync` adds missing embedding and built-in features without upgrading the
existing Geam dependency. `check` instead rejects missing required features
without modifying the manifest. Shared workspace declarations are not edited;
inherited dependencies receive only supported member-local feature additions.

## Synchronize Bindings

After editing Gleam source or adding a Gleam dependency, synchronize from the
Cargo package directory:

```sh
geam embedding sync
```

`sync` prepares dependencies through Gleam and Cargo, retaining their ordinary
lockfiles rather than requesting version updates. It compiles the selected
Gleam source and import closure, validates its public Rust boundary and host
requirements, then writes
`src/geam_bindings.rs`. It atomically replaces changed bytes and leaves an
identical file untouched. It refuses to overwrite a handwritten file at that
path. Unused dependencies and externals with a Gleam fallback do not trigger
native-provider discovery. Preparation does not execute provider code or Rust
build scripts; the application's Cargo build remains a separate step.

Use the read-only command in review and CI:

```sh
geam embedding check
```

All embedding commands use the nearest Cargo package manifest. At a virtual
workspace root, move into the intended member directory. Custom project layouts use the
manual Rust API rather than selector flags on these commands.

`check` runs the same package selection, validation, and rendering path as
`sync`, then compares the expected bytes in memory. Missing or stale output
fails with the exact sync command needed to restore it and never edits the
file.

## Run The Module

The generated module exposes the selected `project`, a typed `Functions`
aggregate, and plain or hosted `bind` support. Compile a provider-free project
with:

```rust
let program = geam_bindings::project().compile()?;
```

Hosted composition performs static provider registration separately before
the same project compilation boundary:

```rust
let program = geam_bindings::project()?.compile()?;
```

For hosted execution, generated `RunStateInputs` lists every runtime value the
caller must choose. The canonical application supplies stdlib state and its
external provider configuration explicitly:

```rust
let mut state = geam_bindings::RunStateInputs {
    stdlib: GleamStdlibRunState::from_seed([7; 32]),
    example_text_pattern: HostProviderConfiguration::empty(),
}
.initialize()?;
```

Time-backed source closures add a caller-owned `time` field. Component-owned
unit state, including JSON state, is initialized internally and does not become
a synthetic input. Initialization returns `RunState` directly when every
selected component is total, and preserves `HostProviderInitializationError`
when an external provider can reject its configuration.

The application remains explicit about the runtime lifecycle:

1. Compile the generated project selection through the existing read-only
   plain or hosted project loader.
2. Build and bind all selected functions into one owner.
3. Seal that owner once.
4. Construct caller-owned capabilities, provider configuration, state, and
   Echo storage.
5. Reuse the typed function handles and sealed module for repeated calls.

See the canonical application's
[`main.rs`](../examples/rust_embedding_application/src/main.rs) for preparation,
caller-owned state, and input/output. Its
[`inventory.rs`](../examples/rust_embedding_application/src/inventory.rs) keeps
the three typed calls and application report together; the adjacent tests fix
exact data, repeated-call, IO, and Echo behavior.

The lower-level `compile_typed_project` and `compile_typed_host_project`
functions remain available when an application deliberately owns project
selection or host registration instead of generated bindings.

## Data Boundary

Function arguments and returns support this recursive ordinary-data grammar:

```text
Data = Scalar | Tuple(Data...) | Result(Data, Data) | Option(Data) | List(Data)
```

| Gleam | Rust |
| --- | --- |
| `Int` | `BigInt` |
| `Float` | `f64` |
| `String` | `EcoString` |
| `BitArray` | `BitArrayValue` |
| `UtfCodepoint` | `char` |
| `Bool` | `bool` |
| `Nil` | `()` |
| `#(A, ...)` | `(A, ...)` |
| prelude `Result(A, B)` | `Result<A, B>` |
| stdlib `Option(A)` | `Option<A>` |
| `List(A)` | consumed `Vec<A>` or borrowed `&List<A>` input; retained `List<A>` output |

`BigInt`, `EcoString`, `BitArrayValue`, and the embedding `List` are available
from `geam::embedding`. Tuple values have one through seven elements; `(T,)`
is a one-element Tuple, whereas `()` is Nil. A function has zero through seven
arguments, passed as a Rust tuple independently of any Tuple-valued argument.

All compound types recurse, including `List(List(String))` and Lists inside
Tuple, Result, or Option. Only the prelude Result and `gleam/option.Option`
from `gleam_stdlib` map to Rust's standard variants. Aliases resolving to those
types work; custom types with matching names or constructors do not.

A Gleam `Error` is an ordinary Rust `Err` inside the function's return value.
For the example's batch function, each List item is a Result. These data errors
are separate from the outer `Result` returned by `module.call`, whose
`CallError` reports a foreign handle/value or an execution failure:

```rust
let rows: Vec<(EcoString, BigInt)> = vec![("invalid".into(), 2.into())];
let checked = module.call(
    &functions.validate_batch,
    (rows,),
    &mut state,
    &mut echo,
)?;
assert_eq!(checked.get(0), Some(Err("invalid code".into())));
```

### Retained Lists

A consumed Vec constructs a new Gleam List. A borrowed List from the same
loaded module reuses its retained handle without traversing or reconstructing
items. The canonical application uses both modes:

```rust
use geam::embedding::{BigInt, EcoString};

let rows: Vec<(EcoString, BigInt)> = vec![
    ("AB-12".into(), 3.into()),
    ("invalid".into(), 2.into()),
];
let checked = module.call(
    &functions.validate_batch,
    (rows,),
    &mut state,
    &mut echo,
)?;
assert_eq!(checked.len(), 2);
assert_eq!(checked.get(1), Some(Err("invalid code".into())));

let total = module.call(
    &functions.total_quantity,
    (&checked,),
    &mut state,
    &mut echo,
)?;
assert_eq!(total, BigInt::from(3));
```

The read-only List API makes materialization explicit:

- `len` and `is_empty` are O(1) and decode no items.
- `get` returns one owned Rust item, or `None` when the index is out of range.
- `iter` yields owned items lazily as they are requested.
- `to_vec` decodes every item into a new Vec.

Retained Lists own the immutable storage needed for reading. They remain
readable after the call, mutable state, Echo, and module have been dropped.
They do not borrow or recreate mutable provider state, and do not implement
`Send` or `Sync`.

Passing a retained List back is restricted to its original live module. A
different load is a different owner, even for identical source and signatures.
`CallError::ForeignValue` is returned before source execution or host-state
mutation. For a List of scalar or non-List compound items, an explicit
`to_vec()` followed by a fresh Vec input transfers the data to another owner.

Nested Lists can use fresh `Vec<Vec<T>>` input or retained
`&List<List<T>>` input. A fresh outer Vec cannot contain retained children:
`Vec<List<T>>` and `Vec<&List<T>>` are not accepted. `to_vec` materializes
one List layer, so nested items still contain retained handles. Explicitly
materialize each nested List for a fresh cross-owner input, for example:

```rust
let rows: Vec<Vec<EcoString>> = nested.iter().map(|row| row.to_vec()).collect();
```

### Input Inference

Generated bindings fix non-List positions and allow an independent carrier for
each List position. Callers pass Vecs or borrowed Lists directly; there is no
public mode wrapper. Ordinary scalar calls keep their `.into()` syntax.

An absent Option/Result branch may not give Rust enough information to choose
a List carrier. Use an ordinary local type annotation. For a function taking
`Option(List(#(String, Int)))`, for example:

```rust
let rows: Option<Vec<(EcoString, BigInt)>> = None;
module.call(&functions.optional_batch, (rows,), &mut state, &mut echo)?;
```

### Gleam Boundary Modules

Arbitrary records and custom enums, external values, callbacks, and generic
signatures are not Rust binding types. Public constants are rejected.
Unsupported function signatures fail synchronization with their function and
nested type position instead of being omitted. A domain type can remain in an
imported Gleam module; a thin root boundary projects it into the supported data
grammar.

In the canonical application, `inventory_rules` owns normalization, validation,
and an opaque Stock type, and uses the text-pattern provider. The selected
`geam_rust_embedding_application` module exposes only ordinary data through batch validation,
total quantity, and first valid row. Rust never needs to represent Stock or
the provider's external Pattern type. Imported modules may use the ordinary
supported Gleam profile; only public functions of the selected root become
Rust bindings.

## Manual Binding

For provider-free direct control over declarations and binding,
[`rust_embedding.rs`](../examples/rust_embedding.rs) remains the low-level
typed API reference. Source closures that require built-in or external
providers should use the managed sync and check workflow shown by the canonical
application. The lower-level provider assembly contract remains documented in
[host provider components](host-providers.md); it is not a second application
workflow.
