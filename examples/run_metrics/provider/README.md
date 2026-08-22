# geam-example-run-metrics

`geam-example-run-metrics` is the macro-authored Rust provider for the
[`example_run_metrics`](../project/packages/example_run_metrics) Gleam package.
It backs the constructorless `Metrics` type with immutable, Rust-owned payloads.

`#[geam::external]` generates the source schema, typed store, storage adapter,
and provider binding. The four `#[geam::function]` declarations then accept
`&Metrics` payload views and return owned updated values through the same static
host registration. The provider needs no configuration or process-local state,
so both declarations are omitted from `#[geam::provider]`.

See the [complete example](../README.md) for the matching Gleam declarations,
persistent value assertions, and standalone commands.
