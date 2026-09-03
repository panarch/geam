# Host Provider: Request IDs

This example adds process-local state without configuration. The provider owns
one request counter for the duration of a run, while the Gleam API remains two
argument-free functions. Omitting an initializer makes Geam construct
`RunState::default()` for every execution.

## Read The Example

1. [`project/packages/example_request_ids/src/example_request_ids.gleam`](project/packages/example_request_ids/src/example_request_ids.gleam)
   declares the small source API.
2. [`provider/src/lib.rs`](provider/src/lib.rs) owns the counter and distinguishes
   mutable from read-only calls.
3. [`project/src/request_ids_example.gleam`](project/src/request_ids_example.gleam)
   checks the counter before and after issuing two IDs.

The Rust signatures make mutation and observation explicit:

```rust
#[geam::function]
fn next(#[geam::call] call: &mut Call<RunState>) -> EcoString;

#[geam::function]
fn issued(#[geam::call] call: &Call<RunState>) -> BigInt;
```

The complete Gleam API is:

```gleam
pub fn next() -> String
pub fn issued() -> Int
```

## Run

Run the example twice to see that each standalone execution starts with fresh
default state:

```sh
cd examples/provider/request_ids/project
geam provider add --path ../provider
geam prepare
geam run
geam run
```

Each run checks `issued() == 0`, then receives `"request-1"` and `"request-2"`
while observing the count after each mutation. Both runs are silent when every
assertion passes, showing that state is not shared between executions.

Continue with [feature flags](../feature_flags/README.md) to initialize provider
state from application-supplied configuration.
