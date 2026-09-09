# Geam Built-in

`geam-builtin` implements the `geam` Gleam package in Rust. In the repository,
this directory contains both their source declarations and implementation:

- `gleam/` is the `geam` Hex package, with the `geam/future` module and editor-visible types.
- `src/` is the `geam-builtin` Rust crate, which implements those declarations in Geam.

The Hex package is versioned independently of the Rust crate. Its version
changes when the package changes, not with every Rust workspace release.

The built-in owns the Future type, its storage binding, equality, hashing,
inspection, and function registration. `geam-core` owns work execution, shared
completion, scoped access, and cancellation. This crate does not start an executor.

See the [Future guide](https://github.com/panarch/geam/blob/main/docs/future.md)
for usage and the
[testing guide](https://github.com/panarch/geam/blob/main/docs/development/testing.md)
for checkout verification.
