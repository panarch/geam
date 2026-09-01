# Value Types Provider Example

This stateless, configuration-free provider is the curated map between Gleam
source values and the Rust types accepted by Geam's authoring macros. It keeps
scalar, tuple, List, custom, Result, and Option declarations in separate modules
so each mapping can be read without unrelated provider state, configuration,
or external-value semantics. As the macro gains more value families, this
example grows with focused sibling modules; state, configuration, and other provider
capabilities stay in their own semantic examples.

## Scalars

The scalar module covers every currently supported ordinary scalar mapping:

```gleam
pub fn join(left: String, right: String) -> String
pub fn add(left: Int, right: Int) -> Int
pub fn multiply(left: Float, right: Float) -> Float
pub fn keep_bits(value: BitArray) -> BitArray
pub fn keep_codepoint(value: UtfCodepoint) -> UtfCodepoint
pub fn invert(value: Bool) -> Bool
pub fn keep_nil(value: Nil) -> Nil
```

The matching Rust signatures import Geam's boundary types from
`geam::provider` and use `EcoString`, `BigInt`, `f64`, `BitArrayValue`, `char`,
`bool`, and `()`, respectively:

```rust
use geam::provider::BigInt;

#[geam::function]
fn add(left: BigInt, right: BigInt) -> BigInt {
    left + right
}

#[geam::function]
fn keep_codepoint(value: char) -> char {
    value
}

#[geam::function]
fn keep_nil(value: ()) -> () {
    value
}
```

## Tuples

The tuple module composes the same scalar leaves recursively:

```gleam
pub fn wrap(value: String) -> #(String)
pub fn unwrap(value: #(String)) -> String
pub fn swap(value: #(String, Int)) -> #(Int, String)
pub fn rotate(value: #(String, Float, Bool)) -> #(Bool, String, Float)
pub fn reassociate(
  value: #(String, #(Int, Bool)),
) -> #(#(String, Int), Bool)
```

Rust uses ordinary native tuples, including one-element and nested tuples:

```rust
#[geam::function]
fn wrap(value: EcoString) -> (EcoString,) {
    (value,)
}

#[geam::function]
fn reassociate(
    value: (EcoString, (BigInt, bool)),
) -> ((EcoString, BigInt), bool) {
    let (label, (count, enabled)) = value;
    ((label, count), enabled)
}
```

## Lists

The List module distinguishes retained source Lists from newly constructed
ones:

```gleam
pub fn length(values: List(Int)) -> Int
pub fn first_or(values: List(String), fallback: String) -> String
pub fn identity(values: List(Int)) -> List(Int)
pub fn reverse(values: List(String)) -> List(String)
pub fn labels(values: List(#(String, Int))) -> List(String)
```

Rust receives an opaque `List<T>` view. `len` does not decode an item,
and `get` decodes only the requested index. Returning the view preserves the
original Gleam List handle; returning `Vec<T>` constructs one new Gleam List:

```rust
use geam::provider::{BigInt, EcoString, List};

#[geam::function]
fn identity(values: List<BigInt>) -> List<BigInt> {
    values
}

#[geam::function]
fn reverse(values: List<EcoString>) -> Vec<EcoString> {
    (0..values.len())
        .rev()
        .map(|index| values.get(index).expect("index comes from the List length"))
        .collect()
}
```

List items currently compose the supported scalar and tuple mappings. The view
is intentionally not an eager `Vec`: provider code chooses whether to inspect
zero, one, or every item.

## Result And Option

Rust's standard `Result<T, E>` and `Option<T>` map directly to Gleam's standard
source types. Output positions use owned values, while input positions apply
the same directional custom decoding used elsewhere:

```rust
#[geam::function]
fn parse(value: EcoString) -> Result<BigInt, ParseError> {
    // ...
}

#[geam::function]
fn describe(value: Result<BigInt, ParseErrorInput>) -> EcoString {
    // ...
}
```

The mapping composes recursively with tuples and custom values. A List of
Result values remains a lazy `List`; returning `Vec<Result<...>>` builds
one new source List.

## Custom Values

The custom module maps ordinary Gleam custom types to Rust enums. The enum used
for outputs mirrors constructors directly, while `input = ...` asks the macro
to generate the directional input enum used when decoding an existing source
value:

```rust
#[geam::custom(input = JobInput)]
enum Job {
    Pending,
    Named(EcoString),
    Scheduled { label: EcoString, attempt: BigInt },
    Prioritized(Priority),
    Tags(Vec<EcoString>),
}
```

Unit, tuple, and named variants retain lexical source order and field labels.
Nested custom values use their generated input type, and a source `List` field
is a lazy `List` view in `JobInput`; the output enum uses `Vec` only when
constructing a new source List.

Run all five modules through the generated standalone runner:

```sh
cd examples/value_types/project
geam provider add --path ../provider
geam prepare
geam run
```

A successful run produces no application output. The entrypoint executes every public
function and checks all scalar mappings, one-, two-, three-element and nested
tuples, empty, indexed, pass-through, reversed and tuple-item Lists, unit,
tuple, named, nested and List-bearing custom constructors, plus source Result
and Option values.

Read
[`project/packages/example_value_types/src/example_value_types/scalars.gleam`](project/packages/example_value_types/src/example_value_types/scalars.gleam),
[`project/packages/example_value_types/src/example_value_types/tuples.gleam`](project/packages/example_value_types/src/example_value_types/tuples.gleam),
[`project/packages/example_value_types/src/example_value_types/lists.gleam`](project/packages/example_value_types/src/example_value_types/lists.gleam),
[`project/packages/example_value_types/src/example_value_types/customs.gleam`](project/packages/example_value_types/src/example_value_types/customs.gleam),
[`project/packages/example_value_types/src/example_value_types/results.gleam`](project/packages/example_value_types/src/example_value_types/results.gleam),
[`provider/src/lib.rs`](provider/src/lib.rs), and
[`project/src/value_types_example.gleam`](project/src/value_types_example.gleam)
together.
