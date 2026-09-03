# Feature Flags Provider Example

This example owns configuration explicitly. The initializer requires one
environment string and an array of enabled flag names, then functions observe
the resulting state through `&RunState`.

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

Run it with:

```sh
cd examples/provider/feature_flags/project
geam provider add --path ../provider
geam prepare
geam run --provider-config example_feature_flags=config/feature_flags.toml
```

Omitting the configuration fails with
`configuration key \`environment\` must be a String`. Supplying a non-array or
a non-string array item for `enabled` fails with
`configuration key \`enabled\` must be an Array of Strings`. These are provider
initialization failures before hosted planning or execution.

Read [`project/packages/example_feature_flags/src/example_feature_flags.gleam`](project/packages/example_feature_flags/src/example_feature_flags.gleam),
[`provider/src/lib.rs`](provider/src/lib.rs),
[`project/config/feature_flags.toml`](project/config/feature_flags.toml), and
[`project/src/feature_flags_example.gleam`](project/src/feature_flags_example.gleam)
together.
