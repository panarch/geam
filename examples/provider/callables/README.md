# Host Provider: Rust-created Functions

This example returns Rust-created functions to ordinary Gleam code, invokes
source functions from a generic Rust wrapper, and retains a callback inside
`Reply(item)`. No OTP package or Erlang runtime is needed for these functions.

## Read the example

1. [Gleam declarations](project/packages/example_callables/src/example_callables.gleam)
   expose `make_adder`, `make_constant`, `wrap`, `reply`, `deliver`, and `calls`.
2. [Rust provider](provider/src/lib.rs) declares private callable bodies and
   the factories allowed to construct them.
3. [Gleam application](project/src/callables_example.gleam) keeps aliases and
   Lists of functions, uses different generic input/output types, and delivers
   a custom-held reply.

```rust
#[geam::callable(factory = Add)]
fn add(
    #[geam::call] call: &mut Call<RunState>,
    #[geam::capture] offset: BigInt,
    value: BigInt,
) -> BigInt {
    call.state_mut().calls += 1;
    offset + value
}

#[geam::function]
fn make_adder(
    #[geam::call] call: &mut Call<RunState>,
    #[geam::factory] factory: Factory<Add>,
    offset: BigInt,
) -> HostResult<Callback<fn(BigInt) -> BigInt>> {
    call.create(&factory, (offset,))
}
```

`offset` is captured when the factory runs. `value` is supplied on each call.
The private `add` body needs no matching Gleam external. Each construction has
its own function identity; cloning a handle or passing it through a List keeps
that identity. Captures stay alive while a function retains them.

`wrap` captures a typed callback with independent argument and result types.
Its `await` body continues on the same execution and returns the ordinary
Gleam result. It does not change the result into an explicit source Future.
`Value<Item>` retains an opaque generic value; `Callback<fn(...) -> ...>`
grants the declared invocation capability. One does not become the other by
inspecting a runtime value.

## Run

```sh
cd examples/provider/callables/project
geam provider add --path ../provider
geam prepare
geam run
geam run
```

Both runs are silent when all assertions pass. The application checks `15`,
`16`, and `17` from capturing functions, exactly three state effects, a retained
Tuple/List constant, a generic tuple result, and a custom reply returning `42`.
State belongs to each run; callable aliases use that original execution.

Continue with [generic box](../generic_box/README.md) to retain values in an
external Rust payload. For app-local Rust construction without a provider
package, see [embedding callables](../../embedding/callables/README.md).
