# Value Types Provider Example

This stateless, configuration-free provider is the curated map between Gleam
source values and the Rust types accepted by Geam's authoring macros. It keeps
scalar and tuple declarations in separate modules so each mapping can be read
without unrelated provider state, configuration, or external-value semantics.
As the macro gains more value families, this example grows with focused sibling
modules; state, configuration, and other provider capabilities stay in their
own semantic examples.

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

The matching Rust signatures use `EcoString`, `BigInt`, `f64`,
`BitArrayValue`, `char`, `bool`, and `()`, respectively:

```rust
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

Run both modules through the generated standalone runner:

```sh
cd examples/value_types/project
geam provider add --path ../provider
geam prepare
geam run
```

A successful run produces no output. The entrypoint executes every public
function and checks all scalar mappings plus one-, two-, three-element, and
nested tuple values.

Read
[`project/packages/example_value_types/src/example_value_types/scalars.gleam`](project/packages/example_value_types/src/example_value_types/scalars.gleam),
[`project/packages/example_value_types/src/example_value_types/tuples.gleam`](project/packages/example_value_types/src/example_value_types/tuples.gleam),
[`provider/src/lib.rs`](provider/src/lib.rs), and
[`project/src/value_types_example.gleam`](project/src/value_types_example.gleam)
together.
