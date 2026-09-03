# Rust Embedding: Gleam Package

This example adds `gleam_stdlib` to a nested Gleam project. Its public function
uses `gleam/list.first` and converts the Result into `gleam/option.Option`, which
the generated Rust binding exposes as `Option<EcoString>`.

## Read The Example

1. [gleam.toml](gleam/gleam.toml) declares the Gleam package dependency.
2. [geam_rust_embedding_package.gleam](gleam/src/geam_rust_embedding_package.gleam)
   defines the public function selected for the generated Rust binding.
3. [main.rs](src/main.rs) initializes the generated stdlib host profile and
   calls the same function with populated and empty Lists.

After changing Gleam dependencies, run `geam embedding sync` from the Cargo
package directory. Sync resolves the Gleam lock, enables the required Geam
features, and regenerates the typed Rust bindings. In this example the selected
stdlib closure produces hosted bindings, so Rust supplies explicit stdlib run
state even though the boundary function itself performs no IO.

## Run

From the repository root:

```sh
cargo build --package geam --bin geam --locked
cd examples/embedding/package
../../../target/debug/geam embedding check
cargo test --locked
cargo run --quiet --locked
```

The application prints:

```text
first: Gleam
empty: none
```

Continue with [caller-owned IO](../io) to use that hosted state
for an observable Rust capability.
