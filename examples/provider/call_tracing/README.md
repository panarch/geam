# Host Provider: Call Tracing

This example passes a Gleam function into Rust and calls it from the provider.
The callback waits for a delayed record operation in the same provider. The
outer Rust function resumes afterward without losing its state or replaying
the earlier trace entry.

## Read The Example

1. [`project/packages/example_call_tracing/src/example_call_tracing.gleam`](project/packages/example_call_tracing/src/example_call_tracing.gleam)
   declares the callback-taking API.
2. [`provider/src/lib.rs`](provider/src/lib.rs) records entries around one typed
   callback invocation.
3. [`project/src/call_tracing_example.gleam`](project/src/call_tracing_example.gleam)
   supplies the callback and checks its result and call order.

The provider receives a typed callback and invokes it through the active
`Call`:

```rust
#[geam::function(await)]
async fn around<Item>(
    #[geam::call] call: &mut Call<RunState>,
    callback: Callback<fn() -> Value<Item>>,
) -> HostResult<Value<Item>> {
    call.with_state(|state| state.entries.push("before".into())).await?;
    let returned = call.invoke(&callback, ()).await?;
    call.with_state(|state| state.entries.push("after".into())).await?;
    Ok(returned)
}
```

The complete Gleam API is:

```gleam
pub fn record(String) -> Nil
pub fn record_later(String) -> Nil
pub fn around(fn() -> item) -> item
pub fn entries() -> List(String)
```

## Run

Run the example twice to verify callback return identity and fresh state for
every standalone execution:

```sh
cd examples/provider/call_tracing/project
geam provider add --path ../provider
geam prepare
geam run
geam run
```

Each run checks that `around(work) == 42` and that the shared trace is exactly
`["before", "inside", "after"]`. Both runs are silent when all assertions
pass.

`record_later` awaits a Tokio timer before accessing state. The standalone
runner supplies Tokio; a Rust embedding application uses its own Tokio runtime.
Both `around` and `record_later` use `#[geam::function(await)]`: Gleam receives
the completed result, not a source Future.

Continue with [generic box](../generic_box/README.md) to retain a typed Gleam
value inside an external value across provider calls.
