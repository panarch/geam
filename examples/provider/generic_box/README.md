# Host Provider: Generic Box

This example lets a Rust-owned external value retain a generic Gleam value
across provider calls. `Box(String)` and `Box(Int)` keep their exact Gleam
specializations; the provider does not convert them into one universal Rust
value.

## Read The Example

1. [`project/packages/example_generic_box/src/example_generic_box.gleam`](project/packages/example_generic_box/src/example_generic_box.gleam)
   declares the generic `Box(item)` API.
2. [`provider/src/lib.rs`](provider/src/lib.rs) stores, restores, replaces, and
   maps retained Gleam values.
3. [`project/src/generic_box_example.gleam`](project/src/generic_box_example.gleam)
   checks identity, cross-type replacement, equality, and callback mapping.

The Rust payload marks the source value as retained storage:

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
```

`Value<Item>` exists only during an active call. `Call::store` moves it into the
new `BoxValue`, while `Call::restore` recovers the exact specialization from a
generated `BoxInput<Item>`:

```rust
#[geam::function]
fn new<Item>(
    #[geam::call] call: &mut Call<()>,
    value: Value<Item>,
) -> BoxValue<Item> {
    BoxValue { value: call.store(value) }
}

#[geam::function]
fn get<Item>(
    #[geam::call] call: &mut Call<()>,
    boxed: BoxInput<Item>,
) -> Value<Item> {
    call.restore(boxed.value())
}
```

The complete Gleam API is:

```gleam
pub type Box(item)
pub fn new(value: item) -> Box(item)
pub fn get(boxed: Box(item)) -> item
pub fn replace(boxed: Box(old), value: new) -> Box(new)
pub fn contains(boxed: Box(item), expected: item) -> Bool
pub fn map(boxed: Box(input), mapper: fn(input) -> output) -> Box(output)
```

## Run

Run it twice to verify fresh standalone execution as well as retained values:

```sh
cd examples/provider/generic_box/project
geam provider add --path ../provider
geam prepare
geam run
geam run
```

Each run checks that the original box still contains `"alpha"`, a replacement
can change its item type to `Int`, a Gleam callback maps that value, and equal
source values compare equal. Both runs are silent when all assertions pass.

Continue with [text pattern](../text_pattern/README.md) to see a Hex package and
crates.io provider prepared as a separately published pair.
