# Create a Rust callback in an embedding application

This one Cargo package declares a native function, captures an offset in Rust,
passes the function to Gleam and invokes the returned alias. It also stores the
function inside private `Saved` data, restores it, and calls a source wrapper
whose internal implementation is generic. Public Rust roots remain concrete;
Rust receives an opaque `Saved` handle rather than its private fields. The ordinary
application module `pricing` implements the operation and owns a call counter. Both dynamic source loading
and immutable prepared loading use the same declaration and body.

```rust
let factory = bindings.callable::<declarations::AddOffset>()?;
let mut module = bindings.seal()?;
// Inside module.with_execution(...):
let callback = scope.construct(&factory, (BigInt::from(10), ()))?;
let value = scope.call(&functions.calculate, (BigInt::from(5), &callback)).await?;
let alias = scope.call(&functions.keep, (&callback,)).await?;
let next = scope.invoke(&alias, (BigInt::from(7),)).await?;
```

`src/declarations.rs` contains the signature and capture layout. It also exposes
`declare` and `select` for preparation. Cargo metadata names that file explicitly:

```toml
[package.metadata.geam.embedding]
generate = "both"
declarations = "src/declarations.rs"
```

The independent preparation helper compiles this declaration module and the
package's dependencies. It does not import `callbacks`, `pricing` or `main`, or
run the application's build script or initialize its state. Keep declarations
independent of application bodies and generated bindings. For child modules,
use an explicit path such as `#[path = "declarations/native.rs"] mod native;`
so the application and independent helper resolve the same source file. The application binds
its real bodies in `src/callbacks.rs` and supplies them to `project` or `load`.
The generated `project`, `bind` and `load` functions accept the application's
concrete `HostProfile`. This example uses caller-owned `Pricing` state containing
a `Cell` counter; it requires `Send`, without `Sync` or payload cloning. The helper
does not import that state type. The generated default profile and dependency
registration remain available to applications that use them.

Select the native factory before sealing the returned binding owner.

Each `scope.construct` creates a distinct function identity. Cloning or passing a
function retains its captures and identity. Captures follow their declared
recursive sequence `(head, tail)`, ending in `()`. Calls and aliases belong to the
original execution scope; returned work still requires explicit observation.

Run from the repository root with the current Geam installed:

```sh
cd examples/embedding/callables
geam embedding sync
geam embedding check
cargo test --locked
cargo run --quiet --locked
```

Expected output:

```text
dynamic: 15, 17, 19
prepared: 15, 17, 19
```

The prepared path uses the checked-in `src/geam_bindings/program.rs` and fresh
native implementations. The dynamic path additionally reads the local Gleam
project. `cargo run --quiet --locked -- --prepared` runs only the prepared path,
which can be moved away from the source tree. Generated files are maintained by `geam embedding sync`.

Back to the [embedding examples](../README.md).
