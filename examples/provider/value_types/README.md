# Host Provider: Value Types

This example keeps the provider stateless and expands the first example's
scalar signatures into the ordinary values most APIs need: tuples, Lists,
custom types, Result, and Option.

## Read The Example

1. [`project/packages/example_value_types/src/example_value_types`](project/packages/example_value_types/src/example_value_types)
   separates the Gleam API by value family.
2. [`provider/src/lib.rs`](provider/src/lib.rs) shows the matching Rust input
   and output types in one provider.
3. [`project/src/value_types_example.gleam`](project/src/value_types_example.gleam)
   constructs each source value and checks the provider result.

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

List items support scalar, external, directional custom, Result, and Option
values plus recursive tuples of those values. A List item cannot itself be a
List or Vec. The view is intentionally not an eager `Vec`: provider code
chooses whether to inspect zero, one, or every item.

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

## Run

Run all five modules through the generated standalone runner:

```sh
cd examples/provider/value_types/project
geam provider add --path ../provider
geam prepare
geam run
```

The entrypoint checks every public function, including `add(20, 22) == 42`,
nested tuple reassociation, lazy List pass-through and traversal, custom
constructors, and successful and failed Result and Option values. A successful
run is silent because all assertions pass.

Continue with [tag set](../tag_set/README.md) when a Gleam value should carry an
opaque payload owned by Rust.
