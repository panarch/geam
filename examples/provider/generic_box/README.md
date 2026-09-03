# Generic Box Provider Example

This example shows the lifecycle of a generic Gleam value retained by an
external provider-owned value. The Rust payload stores the source value without
materializing it into a universal Rust representation:

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

Run it twice to verify old-value preservation, cross-type replacement,
source-semantic equality, callback mapping, and fresh standalone execution:

```sh
cd examples/provider/generic_box/project
geam provider add --path ../provider
geam prepare
geam run
geam run
```

A successful run produces no application output. Read
[`project/packages/example_generic_box/src/example_generic_box.gleam`](project/packages/example_generic_box/src/example_generic_box.gleam),
[`provider/src/lib.rs`](provider/src/lib.rs), and
[`project/src/generic_box_example.gleam`](project/src/generic_box_example.gleam)
together.
