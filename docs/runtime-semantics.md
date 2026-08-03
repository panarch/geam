# Runtime Semantics

Geam is a Rust embedding runtime for Gleam programs. It follows Gleam semantics
for accepted source, but it does not try to emulate every backend detail when
Gleam itself leaves behavior to the target runtime.

## Compatibility Policy

Geam uses this rule when runtime behavior reaches an implementation-specific
edge:

- If Gleam defines target-independent behavior, Geam follows that behavior.
- If Gleam normalizes backend differences, Geam follows the normalized Gleam
  behavior instead of Rust's default behavior.
- If Gleam leaves an edge to backend numeric or runtime semantics, Geam keeps
  the Rust-native representation and avoids adding a separate compatibility
  wrapper unless the Geam profile explicitly needs one.

This keeps accepted Gleam source compatible where Gleam defines compatibility,
while preserving Geam's primary role as a Rust runtime that exposes Rust-friendly
values.

## Rust Value Representation

Geam public runtime values are intentionally close to Rust values. For example,
`Float` is exposed as `f64` rather than a custom Gleam float wrapper.

That means Geam does not currently redefine all floating-point edge behavior:

- Float arithmetic and ordering use Rust `f64` operations unless a narrower
  Gleam rule applies.
- Float equality and `case` literal matching use Rust `f64` equality.
- Float-containing values use `PartialEq`, not `Eq`, because `f64` does not
  provide lawful total equality.

Do not add a custom wrapper or semantic normalization only to hide Rust floating
point behavior. Add one only when it is required to match a documented Gleam
semantic boundary or an explicit Geam runtime policy.

## Function Equality

Function equality follows runtime identity rather than comparing function code,
types, or captured values structurally.

- Repeated evaluation of the same top-level function reference produces the
  same identity.
- Evaluating an anonymous closure produces a fresh identity, including closures
  with no captures.
- Evaluating a custom type constructor as a first-class function produces a
  fresh identity.
- Moving or cloning a function value through locals, arguments, captures,
  containers, calls, or returns preserves its identity.
- Tuple, list, custom, and `Result` equality applies these rules recursively to
  contained function values.

The public Rust `FunctionValue` representation is an owned materialized value;
its `PartialEq` implementation is not the identity used by Gleam source-level
function equality.

After module linkage, a top-level reference identity includes its
module-qualified function template target. Equal local indices in different
modules are distinct references, while qualified and unqualified imports of
the same target share one identity.

## Rust Host Functions

Rust host functions enter through package-qualified source-less host modules
or providers for existing source external declarations. The ordinary
`ExecutionPlan` has an uninhabited host target and remains host-free. Hosted
source instead follows this separate pipeline:

```text
HostProviderSet + PackageSource[]
-> HostedTypedProgram
-> HostedModulePlan
-> HostedExecution
```

The same pipeline can start from `compile_typed_host_project`, which reads an
already resolved filesystem project and combines its selected import closure
with an explicit `HostProviderSet`. Plain and hosted project loading share the
manifest, source catalog, closure selection, and parse owner. Provider linkage
remains a planning concern rather than a filesystem-loader fallback.

### Linking And Identity

Registration seals a callable schema and implementation together, so missing
implementations and signature mismatches cannot become runtime states. The
plan retains only package, module, function, scheme, shape, and callable target
metadata. Rust callback objects are carried separately and retained only for
host functions reached by execution specialization.

Provider selection is completed before source body planning. An exact provider
selects a host template, an external declaration with no provider uses its
Gleam body when one exists, and an external-only declaration without a
provider is rejected. A provider cannot replace an ordinary Gleam function.
Dependency functions that Gleam analysis marks unavailable on the selected
Erlang target are omitted before hosted function indexing when no provider was
registered for them. This preserves explicit provider ownership while avoiding
missing-provider failures for unselected target-only declarations.

Host calls use the same family-specific runtime function IDs as Gleam
functions. Direct calls, tail calls, function-value calls, and top-level
reference equality therefore do not introduce a parallel dispatch or identity
model. Qualified and unqualified references to one host function compare
equal; Rust closure addresses and captures do not participate in language
equality.

### Host Values And State

The direct host boundary accepts infallible and fallible Rust closures with
zero through seven arguments. Each argument and return is one of `BigInt`,
`f64`, `EcoString`, `BitArrayValue`, `char`, `bool`, or `()`. A provider that
never succeeds may use `Infallible` as its return; `Infallible` is not an
argument or materialized value family. Seven is an intentional profile limit
aligned with Clippy's default `too_many_arguments` threshold. Registration
derives the public schema, family-local parameter slots, and callback adapter
together; unsupported Rust types and arities have no `HostFunction`
implementation.

Scoped providers describe compound values through `HostListType`,
`HostTupleType`, and `HostCustomType`. Lists, tuples, and ordinary custom
values cross one invocation as typed handles rather than materialized
`Value`s. A provider can inspect those handles through `HostCall` and can
construct a return only through the return-family-specific call builder.
Custom schemas are checked against the selected source definition before
lowering, so runtime trusts their constructor positions and field shapes.
Schema fields refer to their enclosing custom parameters with
`HostCustomTypeArgument`; selecting a concrete `HostCustomType` substitutes
those arguments before `HostCall` reads or constructs constructor fields.
Function-scheme parameters remain the separate `HostTypeParameter` namespace.

Each `HostProfile` defines caller-owned `RunState`. A scoped callback can
project only its declared `HostProvider::State` through the active `HostCall`;
callback objects and mutable state are not stored in canonical plan nodes.
The explicit `GleamStdlibProfile` uses caller-created `GleamStdlibRunState` for
random operations. Construction requires either a seed or fallible system
entropy; the runtime provides no hidden seed, global generator, or `Default`
state.

### External Values And Retained Storage

A source-backed provider can register a constructorless external type with
`HostExternalStorage`. Planning links its exact package, module, type name, and
parameter count. Canonical plan and graph nodes retain nominal type and storage
IDs only. `HostedExecution` owns the profile's typed stores used for payload
creation and access, while `HostExternalStorage` supplies payload source
equality, hashing, and inspection behavior. These operations receive narrow
contexts that can compare, hash, or inspect retained typed and existential
Gleam values without exposing runtime storage. Neither Rust payloads nor
storage behavior become canonical plan metadata.

Source hashing follows Gleam equality: equal values always have equal hashes,
while a matching hash still requires source equality to resolve collisions.
Hashes are runtime indexes and are not stable across processes or releases.
An immutable external payload's source hash and inspection are computed on
first demand and cached in its lease. Payload creation does not traverse
retained values merely to prepare either semantic. Before an external value is
materialized beyond the runtime, its canonical inspection is sealed into the
owned public value. Neither semantic is derived from Rust `TypeId`, allocation
addresses, public opaque identity, or inspection text.

`HostExternal` is an invocation-scoped typed handle. A provider can create or
inspect its Rust payload only through the active `HostCall`. The materialized
public `ExternalValue` exposes nominal type, opaque instance identity, and
canonical inspection, but never the Rust payload. Gleam equality remains owned
by `HostExternalStorage` and is distinct from public Rust `PartialEq`. The
payload lease is self-contained, so the value can be cloned, inspected, and
dropped after the run state and `HostedExecution` have been dropped.

An external payload can retain an exact Gleam value as `HostStoredValue`.
Monomorphic fields preserve their declared host type marker. Generic payload
fields use a stable external type-argument position that each provider
function maps to its own local type parameters. Restoration is available only
from the payload view of an active `HostCall`; provider run state and public
`Value` have no retain or restore API. Lists, tuples, ordinary customs,
functions, nested externals, and their runtime identities are retained without
`Any`, Rust `TypeId`, downcasts, or runtime shape validation.

An external payload can instead retain `HostStoredDynamic`, which seals the
exact specialized Gleam shape beside the retained runtime value. A payload
view may request a typed decode through the active `HostCall`. An exact shape
match restores the typed handle; a mismatch or a type parameter not supplied
by that host specialization returns `None` as ordinary provider semantics.
Dynamic identity is the recursive Gleam shape, including nominal custom and
external identities, rather than a Rust payload type. Public `Value` does not
expose this decode surface.

External leases determine payload lifetime. The profile store keeps a typed
index only while at least one lease exists; dropping the final lease removes
the index entry, so the store cannot extend payload lifetime beyond its leases.
Retained list and capture graphs continue to use the shared iterative release
queue, including after the original runtime state has been dropped. Geam does
not support cyclic evaluated graphs or moving stored values between hosted
executions.

Providers that model private transient-style builders use persistent external
payload versions. Each operation may share immutable retained entries with its
input, but it returns a new payload and never mutates a version already visible
to Gleam. Aliases therefore continue to observe their original values, and the
retained graph remains acyclic. Geam does not enforce a consumed-token state at
runtime and does not provide general mutable external references or cycle
collection.

The explicit official `gleam/dict` provider is one concrete use of this model.
It selects persistent buckets by Gleam source hash and resolves every collision
with source equality. `Dict` and private `TransientDict` remain nominally
distinct immutable payload versions, official Gleam fallback bodies remain the
source owner of their operations, and dictionary iteration order is not a
runtime contract. Canonical inspection sorts rendered entries only to make
escaped values deterministic to read; that display order does not define
iteration semantics.

The separate `gleam_json` provider represents encoded Json in immutable shared
external storage. Source equality, hashing, and inspection derive from the
encoded representation rather than Rust payload identity. Parsing constructs
the existing Dynamic, List, and Dict value families while preserving the
acyclic external-value ownership model; it does not introduce a separate JSON
runtime family.

### Specialization And Re-entry

A generic provider registers one source `TypeScheme`; first-use
specialization derives concrete parameter locals, return-family storage, and
host targets. `HostedModulePlan` owns the linked generic program;
`HostedExecution::try_from_module_plan` seals only entry-reachable
specializations. A reachable value-producing specialization whose successful
return storage remains unresolved returns `HostSpecializationError`; an
unused declaration does not. A function exposed through `HostFunctionType`
must also have inhabited runtime argument storage. If its argument family
remains symbolic, sealing rejects that invocation capability. The same
function value may still cross an opaque `HostTypeParameter` position for
pass-through or equality because that position does not expose invocation.

A non-returning provider with a concrete result context enters that concrete
function family; an unresolved result enters the Never family. Neither path
fabricates a success value. Runtime performs no `Value` downcast, signature
lookup, generic type lookup, callback shape validation, symbolic callback
dispatch, or fallback selection.

`HostFunctionType<Arguments, Return>` exposes an invocation capability as a
call-scoped `HostCallable`. `HostCall::invoke` routes it through the same typed
function loops used by ordinary calls. Runtime-owned handles may remain live
across a nested call, but they cannot escape the invocation that supplied
them. A provider-state or payload borrow must end before re-entry.

Nested execution preserves its original failure domain. A source panic remains
a source panic. A nested host failure names the provider that actually failed
and records the invoking host as its caller; the outer provider does not wrap
it in a new `HostFailure`. Retained arguments and scoped values are released
before either error returns to the caller.

## Generic Values

A bare type parameter has no successful runtime value representation. A
generic computation whose result remains bare can only stop through existing
source-level behavior such as `panic`, `todo`, or non-returning recursion; Geam
does not fabricate a value or recover one through runtime type checks.

Containers may preserve a parameter in their public type metadata when their
runtime payload does not require a value of that parameter, for example an empty
`List(parameter)` or a phantom custom constructor.

## Constant Evaluation

A constant initializer is lowered into a reusable zero-argument typed graph
program. Each source reference executes that program, so an unselected branch
does not evaluate the constants it contains. Top-level function references
retain their stable reference identity, while each evaluation of a closure or
constructor-function constant creates a fresh instance identity as described
above.

## Value Inspection And Echo

`Value::inspect()` is the canonical language-facing rendering of a materialized
runtime value. It borrows only the `Value`: formatting does not consult an
execution plan, runtime state, typed AST, or source environment. Functions are
summarized by arity, without exposing captures, runtime identity, or function
body metadata.

Echo output crosses an explicit host boundary:

```rust
run_main(&plan, &mut echo_sink)
```

For an ordinary echo expression, execution evaluates the value, then its
optional message, emits one owned `EchoOutput`, and continues with the original
stored value. Pipeline echo receives the already evaluated pipeline value,
evaluates its optional message, emits through the same boundary, and passes
that original value to the next pipeline stage. A failing value or message
emits nothing. Output emitted before a later source panic remains in the
caller-owned sink.

`EchoOutput` contains the materialized `Value`, optional message, and a compact
source location. When the plan owns source context, the location includes its
path and one-based line. Otherwise it retains the module, function, and span as
a site-only fallback. It does not retain full source text, execution state, or
runtime handles.

`EchoSink::emit` is infallible. Geam does not provide a hidden no-op sink,
global output queue, or default stdout/stderr destination, and sink failures
are not part of `ExecutionError`. A future build profile that omits source
metadata is a separate design concern.

## Standard-Library IO

Official `gleam/io` functions emit through the `IoSink` projected by the
caller's `GleamStdlibHostProfile`. Each owned `IoOutput` records either stdout
or stderr and one exact text chunk. `print` operations preserve their input;
`println` operations append exactly one newline before emitting. All four
operations emit before returning `Nil`, so output remains caller-owned if a
later source expression panics.

The default `GleamStdlibRunState` collects events in cross-stream source order.
A custom profile may project another concrete sink, including an adapter that
streams to an outer host. Delivery is infallible at the Geam boundary: writer
failure policy belongs to that adapter and does not become an `ExecutionError`
or `HostFailure`.

Stdlib IO and language Echo remain separate capabilities. They do not share a
global queue or select a process output destination. A host that needs one
combined transcript can compose both adapters over its own shared recorder.

## Execution Graph

An executable function owns its entry bindings and a `FunctionBody`. The body
owns a shared `BlockGraph` topology plus function-specific return and tail-call
exits. A reusable constant program owns the same block-graph topology plus its
typed return values. In this control-flow graph, only `Block` is a node; values,
instructions, terminators, edges, IDs, and metadata describe or connect those
nodes.

Each block contains ordered typed parameters, instructions, and one terminator.
Branches, switches, matches, source stops, returns, and tail calls are explicit
edges or terminators rather than recursive runtime expression or return nodes.

The runtime evaluates a block iteratively. On an edge it retains the ordered
edge arguments, drops the old block environment, drains queued list releases,
and constructs the target environment from those arguments. No function-wide
default frame is allocated, and a block can only read entry values, block
parameters, or instruction outputs that dominate the read.

Tail calls return control to the typed function-family loop, which replaces the
current activation without growing the Rust stack. Non-tail call instructions
currently invoke the callee graph through the Rust stack because the caller has
a continuation after the instruction; explicit activation-stack execution is a
separate runtime concern.

`ExecutionPlan::explain()` reads this frozen representation directly. It shows
typed entry values, block parameters, instructions, operands, edge arguments,
terminators, and reusable constant programs in deterministic order. The output
is intended for human inspection and does not define a stable serialization or
an additional execution model.

## Linked Source Diagnostics

An execution plan retains one source context per planned module. A source-level
panic site carries its owning module identity, so diagnostics select the
dependency source and path when execution stops inside an imported function.
Linking does not rewrite dependency sites to the root source.

## Numeric Division By Zero

Integer division and remainder by zero are normalized because Gleam defines
that behavior across its targets. Geam therefore returns `0` for both:

```gleam
1 / 0
1 % 0
```

Float division by zero is also not left as raw Rust division. Rust `f64`
division would produce infinities or `NaN`, but Gleam lowers float division by
zero through target-specific helper logic rather than exposing raw backend
division behavior.

Geam's current policy is:

```text
left /. 0.0  -> 0.0
left /. -0.0 -> 0.0
```

This is an intentional Geam normalization at the zero-division boundary. It does
not imply that Geam normalizes every signed-zero edge for floats. Outside this
division-by-zero rule, `Float` behavior remains Rust `f64` behavior unless a
future runtime policy says otherwise.

## Updating This Document

When a new value family or host/runtime feature reaches an implementation edge,
record the decision here if it affects observable execution behavior. Keep
review rules in `review-policy.md`; keep runtime meaning decisions here.
