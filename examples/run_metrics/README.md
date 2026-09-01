# Run Metrics Provider Example

This example pairs one constructorless Gleam type with a macro-authored Rust
payload:

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

```text
project/
  packages/example_run_metrics/  ordinary local Gleam package
provider/                          geam-example-run-metrics crate
```

## Run The Example

With `geam` available on `PATH`, select the local provider and run the project:

```sh
cd examples/run_metrics/project
geam provider add --path ../provider
geam prepare
geam run
```

No provider configuration file is needed, so the component omits both state and
initialization. All metrics data lives in the source-visible `Metrics` values.
The entrypoint checks empty, one-sample, multi-sample, missing-key, old-value
preservation, and equality behavior. A successful run produces no application output.

The tracked `.cargo/config.toml` files only redirect the unreleased Geam
authoring API to this repository checkout. They are development wiring for this
example, not consumer metadata.

## What To Read

- [`project/packages/example_run_metrics/src/example_run_metrics.gleam`](project/packages/example_run_metrics/src/example_run_metrics.gleam)
  is the complete source-visible API.
- [`provider/src/lib.rs`](provider/src/lib.rs) is the matching Rust payload and
  component.
- [`project/src/run_metrics_example.gleam`](project/src/run_metrics_example.gleam)
  exercises every function through the standalone runner.

Together they show generated external schema and storage ownership, mixed
external/scalar signatures, persistent updates, source equality, and canonical
inspection without low-level host registration boilerplate.
