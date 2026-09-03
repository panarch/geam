# Rust Embedding: Gleam Package

This example adds `gleam_stdlib` to a nested Gleam project. Its public function
uses `gleam/list.first` and converts the Result into `gleam/option.Option`, which
the generated Rust binding exposes as `Option<EcoString>`.

## Read The Example

1. [gleam.toml](gleam/gleam.toml) declares the Gleam package dependency.
2. [geam_rust_embedding_package.gleam](gleam/src/geam_rust_embedding_package.gleam)
   defines the public function selected for the generated Rust binding.
3. [main.rs](src/main.rs) supplies the stdlib state requested by the generated
   bindings and calls the function with populated and empty Lists.

After changing Gleam dependencies, run `geam embedding sync` from the Cargo
package directory. Sync resolves the Gleam dependencies, enables the required
Geam features, and regenerates the typed Rust bindings. The generated API asks
for `GleamStdlibRunState`; this example shows how to supply it before the next
example uses stdlib IO.

## Run

With Geam, Rust, and Gleam installed, run from the repository root:

```sh
cd examples/embedding/package
geam embedding check
cargo test --locked
cargo run --quiet --locked
```

The application prints:

```text
first: Gleam
empty: none
```

Continue with [Gleam IO](../io) to route stdlib output through Rust.
