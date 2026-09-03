# Host Provider: Run Metrics

The tag set example derives ordinary source behavior from its Rust payload.
This example takes control of that boundary: a provider defines how a
Rust-owned `Metrics` value compares, hashes, and appears when inspected from
Gleam.

## Read The Example

1. [`project/packages/example_run_metrics/src/example_run_metrics.gleam`](project/packages/example_run_metrics/src/example_run_metrics.gleam)
   declares the constructorless type and source-visible operations.
2. [`provider/src/lib.rs`](provider/src/lib.rs) defines the payload, persistent
   update, and custom external behavior.
3. [`project/src/run_metrics_example.gleam`](project/src/run_metrics_example.gleam)
   checks old values, aggregate results, and equality.

The Gleam API is:

```gleam
pub type Metrics

pub fn new() -> Metrics
pub fn record(metrics: Metrics, name: String, value: Float) -> Metrics
pub fn count(metrics: Metrics, name: String) -> Int
pub fn total(metrics: Metrics, name: String) -> Float
```

`Metrics` is an ordinary immutable Gleam value. The Rust provider stores its
opaque payload, and `record` clones that payload before returning an updated
version. Earlier values remain readable and independently constructed values
with the same entries compare equal in Gleam source.

## Run

With `geam` available on `PATH`, select the local provider and run the project:

```sh
cd examples/provider/run_metrics/project
geam provider add --path ../provider
geam prepare
geam run
```

No provider configuration file is needed, so the component omits both state and
initialization. All metrics data lives in the source-visible `Metrics` values.
The entrypoint checks counts and totals for empty, one-sample, multi-sample,
and missing-key cases. It also checks that an update preserves the old value
and that independently built metrics with the same entries compare equal. A
successful run is silent because all assertions pass.

Continue with [call tracing](../call_tracing/README.md) when provider code needs
to invoke a typed Gleam callback.
