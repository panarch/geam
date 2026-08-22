# Request IDs Provider Example

This example adds process-local state without configuration. Omitting an
initializer makes Geam construct `RunState::default()` for every execution.
The Rust signatures distinguish mutation from observation:

```rust
#[geam::function]
fn next(#[geam::state] state: &mut RunState) -> EcoString;

#[geam::function]
fn issued(#[geam::state] state: &RunState) -> BigInt;
```

The complete Gleam API is:

```gleam
pub fn next() -> String
pub fn issued() -> Int
```

Run the example twice to see that each standalone execution starts with an
independent default state:

```sh
cd examples/request_ids/project
geam provider add --path ../provider
geam prepare
geam run
geam run
```

Each run checks the initial count, two generated IDs, and the read-only count
after each mutation. A successful run produces no output.

Read [`project/packages/example_request_ids/src/example_request_ids.gleam`](project/packages/example_request_ids/src/example_request_ids.gleam),
[`provider/src/lib.rs`](provider/src/lib.rs), and
[`project/src/request_ids_example.gleam`](project/src/request_ids_example.gleam) together.
