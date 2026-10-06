# Native function signature views

This independent provider keeps the original generic Gleam identity declaration
and uses `NativeRules::retained_views` with the public stdlib
`service::with_native_dynamic` helper. It exercises the pinned Gleam stdlib
1.0.3 source rather than defining a replacement Dynamic type or store.

The project checks anonymous, named, captured, and nested functions, exact
controls, tuple/list fields, and a typed → Dynamic → typed round trip. The
provider test and embedding application also check input/result conversion
failures followed by successful calls. A mismatched arity is refused by the
native conversion before invocation.

```sh
cargo test --manifest-path tests/fixtures/native_function_views/provider/Cargo.toml --locked
cd tests/fixtures/native_function_views/project
geam provider add --path ../provider
geam run
geam build --release
```

For live and prepared embedding, run `geam embedding sync` in `embedding`,
then `cargo run --locked`. Pass `-- --prepared` to use only the generated
source-free artifact. Successful runs print `native function views: 42` for
each call. Generated application `program.rs` files and build data are ignored;
the ordinary public bindings and independent lock files are tracked.
