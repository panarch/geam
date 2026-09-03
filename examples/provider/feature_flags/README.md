# Host Provider: Feature Flags

This example initializes provider state from application-owned configuration.
The Rust provider reads an environment name and enabled feature flags once,
then Gleam functions observe that shared state through a small API.

## Read The Example

1. [`project/packages/example_feature_flags/src/example_feature_flags.gleam`](project/packages/example_feature_flags/src/example_feature_flags.gleam)
   declares the two functions available to Gleam.
2. [`provider/src/lib.rs`](provider/src/lib.rs) validates configuration and
   initializes the provider state.
3. [`project/config/feature_flags.toml`](project/config/feature_flags.toml)
   supplies the application-owned values for this run.
4. [`project/src/feature_flags_example.gleam`](project/src/feature_flags_example.gleam)
   checks the configured environment and flags.

The complete Gleam API is:

```gleam
pub fn environment() -> String
pub fn enabled(name: String) -> Bool
```

The checked-in configuration is:

```toml
environment = "staging"
enabled = ["new_checkout", "audit_log"]
```

## Run

Pass the checked-in configuration when running the project:

```sh
cd examples/provider/feature_flags/project
geam provider add --path ../provider
geam prepare
geam run --provider-config example_feature_flags=config/feature_flags.toml
```

The entrypoint checks that the environment is `"staging"`, that
`"new_checkout"` and `"audit_log"` are enabled, and that an absent flag is
disabled. A successful run is silent because all assertions pass.

## Configuration Errors

Omitting the configuration fails with
`configuration key \`environment\` must be a String`. Supplying a non-array or
a non-string array item for `enabled` fails with
`configuration key \`enabled\` must be an Array of Strings`. Geam reports these
configuration errors before application code runs.

Continue with [run metrics](../run_metrics/README.md) to customize source
equality, hashing, and inspection for a Rust-owned value.
