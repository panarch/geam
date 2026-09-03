# Call Tracing Provider Example

This example shows one provider function invoking a typed Gleam callback. The
callback re-enters the same provider, while one `Call` keeps the provider state
and callback capability in the same active invocation:

```rust
#[geam::function]
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

The complete Gleam API is:

```gleam
pub fn record(String) -> Nil
pub fn around(fn() -> item) -> item
pub fn entries() -> List(String)
```

Run the example twice to verify callback return identity, `before` / `inside` /
`after` ordering, and fresh state for every standalone execution:

```sh
cd examples/provider/call_tracing/project
geam provider add --path ../provider
geam prepare
geam run
geam run
```

A successful run produces no application output. Read
[`project/packages/example_call_tracing/src/example_call_tracing.gleam`](project/packages/example_call_tracing/src/example_call_tracing.gleam),
[`provider/src/lib.rs`](provider/src/lib.rs), and
[`project/src/call_tracing_example.gleam`](project/src/call_tracing_example.gleam)
together.
