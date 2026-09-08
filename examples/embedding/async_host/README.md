# Async Rust Functions

This example reads a file through an async Rust provider and returns a
`Future(Result(String, String))` to Gleam. Rust drives the work using its own
executor. The `double` function in the same module remains an ordinary call.

```sh
cd examples/embedding/async_host
geam embedding sync
cargo test --locked
cargo run --quiet --locked
```

```text
double: 42
created
Hello from Rust
again: Hello from Rust
```

## Gleam

The [Gleam module](gleam/src/geam_rust_embedding_async_host.gleam) combines the
file provider's work with `future.map`:

```gleam
pub fn greeting(path: String) -> Future(Result(String, String)) {
  use contents <- future.map(files.read(path))
  case contents {
    Ok(text) -> Ok("Hello " <> text)
    Error(error) -> Error(error)
  }
}
```

The [independent provider](../../provider/async_files/provider/src/lib.rs) uses an
ordinary Rust `async fn`. Its Gleam declaration is
`read(String) -> Future(Result(String, String))`; file errors remain source
`Error` values.

## Rust

[main.rs](src/main.rs) loads the generated project, binds both functions and
attaches the module to caller-owned state and an Echo sink. It then calls
`greeting` to obtain work and awaits `scope.observe(&work)` to drive it.

Observing the same work again shares its completion; it does not read the file
again. `Completed::read` borrows the result so sharing does not require cloning
its payload.

Ordinary entries return their values directly and Future entries return work.
Both use the same module and `Send` provider state, with no storage setting.
The host remains responsible for state, Echo and its executor.

## Source Package

The example uses the repository's ordinary
[geam package](../../../builtins/geam/gleam) as a local Gleam dependency.
In your own application's `gleam/` directory, add the package with:

```sh
gleam add geam
```

The Rust dependency enables `geam-builtin` for the built-in implementation.
See the [Future guide](../../../docs/future.md) for composition and lifetime
examples.
