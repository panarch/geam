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

Registration seals a callable schema and implementation together, so missing
implementations and signature mismatches cannot become runtime states. The
plan retains only package, module, function, scheme, shape, and callable target
metadata. Rust callback objects are carried separately and retained only for
host functions reached by execution specialization.

Provider selection is completed before source body planning. An exact provider
selects a host template, an external declaration with no provider uses its
Gleam body when one exists, and an external-only declaration without a
provider is rejected. A provider cannot replace an ordinary Gleam function.

Host calls use the same family-specific runtime function IDs as Gleam
functions. Direct calls, tail calls, function-value calls, and top-level
reference equality therefore do not introduce a parallel dispatch or identity
model. Qualified and unqualified references to one host function compare
equal; Rust closure addresses and captures do not participate in language
equality.

The direct host boundary accepts infallible and fallible Rust closures with
zero through seven arguments. Each argument and return is one of `BigInt`,
`f64`, `EcoString`, `BitArrayValue`, `char`, `bool`, or `()`. A provider that
never succeeds may use `Infallible` as its return; `Infallible` is not an
argument or materialized value family. Seven is an intentional profile limit
aligned with Clippy's default `too_many_arguments` threshold. Registration
derives the public schema, family-local parameter slots, and callback adapter
together; unsupported Rust types and arities have no `HostFunction`
implementation.

One execution profile maps every return-family function body to either a
graph-only entry or a typed graph-or-host entry. A non-returning provider with
a concrete result context enters that concrete function family; an unresolved
result enters the Never family. Neither path fabricates a success value. Plain
execution uses `Infallible` only as its uninhabited host target; it is not a
source-visible host value. Runtime receives the sealed scalar slots and calls
the matching typed function family without a `Value` downcast, signature
check, string lookup, panic translation, or runtime fallback selection. Each
`HostProfile` defines a caller-owned `RunState`; a scoped callback can
project only its declared `HostProvider::State` through the active `HostCall`.
An owned `HostFailure` becomes `ExecutionError::Host` with the failed provider
identity, concrete signature, and preserved source call site.

Scoped providers describe compound values through `HostListType`,
`HostTupleType`, and `HostCustomType`. Lists, tuples, and ordinary custom
values cross one invocation as typed handles rather than materialized
`Value`s. A provider can inspect those handles through `HostCall` and can
construct a return only through the return-family-specific call builder.
Tuple element types use a recursive `HostTypeList`, so tuple size is
independent of the zero-through-seven Rust function argument boundary.

`HostFunctionType<Arguments, Return>` exposes an invocation capability as a
call-scoped `HostCallable`. `HostCall::invoke` routes that callable through the
same family-specific loops used by ordinary direct, tail, and function-value
calls. A nested host call projects its own provider state from the same
caller-owned run state. A provider-state borrow must end before re-entry, which
Rust enforces through the call lifetime. Callables, lists, tuples, and customs
are runtime-owned token handles: they may remain live across a nested call but
cannot escape the invocation that supplied them.

Nested execution preserves its original failure domain. A source panic remains
the existing source panic. A nested host failure names the provider that
actually failed and records the invoking host as its caller; the outer
provider does not wrap it in a new `HostFailure`. Retained arguments and
scoped values are released before either error returns to the caller.

An ordinary custom schema supplies its package, module, type parameters, and
an ordered type-level constructor list. Each constructor owns an ordered
type-level field list with exact labels and recursive field types. Constructor
handles select a position from that sealed list, so the runtime receives the
same index and fields that planning validated. Planning compares every
referenced custom schema exactly with the selected source definition before
execution lowering. Source-backed providers retain public, same-package
internal, and same-module private visibility; source-less public host surfaces
accept only public non-opaque custom types. Opaque custom representations
remain available only to source-backed providers in their defining module,
regardless of declaration publicity.

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
The runtime therefore performs no generic type lookup, callback shape
validation, or symbolic callback dispatch.

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
