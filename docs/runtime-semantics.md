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

## Execution Graph

Geam lowers each function into immutable typed blocks before runtime. A block
contains ordered typed parameters, instructions, and one terminator. Branches,
switches, matches, source stops, returns, and tail calls are explicit graph
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
