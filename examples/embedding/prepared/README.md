# Rust Embedding: Prepared Program

This application calls the same `double` function as [First Call](../first_call),
but loads a program prepared before the Rust build. The executable does not
read the Gleam project when it starts.

## Read The Example

[Cargo.toml](Cargo.toml) selects the generated loading interface:

```toml
[package.metadata.geam.embedding]
generate = "prepared"
```

The [Gleam module](gleam/src/geam_rust_embedding_prepared.gleam) defines:

```gleam
pub fn double(value: Int) -> Int {
  value * 2
}
```

The complete [Rust application](src/main.rs) loads the prepared program and
calls its typed function:

```rust
mod geam_bindings;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (module, functions) = geam_bindings::load()?;
    let mut echo = Vec::new();

    let value = module.call(&functions.double, (21.into(),), &mut echo)?;
    println!("{value}");
    Ok(())
}
```

`geam embedding sync` generates `src/geam_bindings.rs` and its private
`src/geam_bindings/program.rs` data. Cargo compiles both as ordinary Rust source.
`load()` creates a fresh module and its function handles; it does not call
`double`. Calls use the same runtime and value types as dynamic embedding.

## Run

With Geam, Rust, and Gleam installed, run from the repository root:

```sh
cd examples/embedding/prepared
geam embedding sync
geam embedding check
cargo test --locked
cargo run --quiet --locked
```

The application prints:

```text
42
```

The generated `src/geam_bindings/program.rs` is ignored by Git, so sync is
required on a fresh checkout. After editing Gleam source or changing
dependencies, run sync before building again. `check` verifies the prepared
program as well as its bindings; it can compile and run a preparation helper.
Ordinary Cargo builds use the existing generated files and do not regenerate them.

The Cargo patch selects this repository checkout. An ordinary application
receives its Geam dependency from `embedding init` and does not need that patch.

See [generation choices](../../../docs/embedding.md#prepare-a-program-before-building)
to expose both dynamic and prepared loading in one application. Provider
configuration, mutable state, IO and execution hosts remain caller-owned in
either workflow.
