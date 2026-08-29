# Rust Embedding

Geam can generate the mechanical Rust bindings for one public Gleam module
inside an ordinary Cargo package. The Rust application continues to own source
loading, execution sealing, capabilities, provider configuration, mutable
state, Echo, and call order.

The complete managed example is
[`examples/rust_embedding_application`](../examples/rust_embedding_application).
It combines imported Gleam source, `gleam/io`, and the text-pattern provider in
one independently locked Rust application.

## Project Layout

Keep the resolved Gleam project inside the Rust package and commit the generated
Rust source:

```text
application/
  Cargo.toml
  Cargo.lock
  src/
    main.rs
    geam_bindings.rs
  gleam/
    gleam.toml
    manifest.toml
    src/
```

Cargo build output and downloaded Gleam package source are local artifacts;
ignore `target/` and `gleam/build/`. Keep both lockfiles and
`src/geam_bindings.rs` in source control.

Add Geam without its command-line or provider-authoring surface:

```sh
cargo add geam --no-default-features --features embedding
```

Enable only the built-ins reached by the selected Gleam source closure:

- `gleam-stdlib` exposes `geam::gleam_stdlib`;
- `gleam-json` exposes JSON and its stdlib dependency;
- `gleam-time` exposes Time and its stdlib dependency.

For example, the canonical application enables `embedding,gleam-stdlib`.
Its text-pattern provider enables `provider` on the same Geam package identity,
so Cargo unifies the authoring macros without restoring Geam defaults or CLI.
An application that authors providers directly can add `provider` itself.

Select the nested project and one boundary module in `Cargo.toml`:

```toml
[package.metadata.geam.embedding]
project = "gleam"
module = "rust_embedding"
```

The package must have one enabled direct dependency on `geam`. Each external
provider required by the selected Gleam source closure must also be an enabled
direct Cargo dependency with valid Geam provider metadata. The dependency's
actual Cargo alias is retained in generated source.

`sync` and `check` inspect Cargo's locked resolved feature graph. A missing
`embedding` or required built-in feature fails with the owning Cargo manifest
and exact feature name before generated Rust compilation. Geam does not edit
the dependency declaration.

## Synchronize Bindings

Resolve both package graphs before synchronization. Geam deliberately does not
download dependencies or edit either manifest or lockfile:

```sh
cd gleam
gleam deps download
cd ..
cargo generate-lockfile
geam embedding sync
```

`sync` compiles the selected Gleam source and import closure, validates its
public Rust boundary and host requirements, then writes
`src/geam_bindings.rs`. It atomically replaces changed bytes and leaves an
identical file untouched.

Use the read-only command in review and CI:

```sh
geam embedding check
```

Both commands accept `--manifest-path PATH`. Without it, Geam selects the
nearest Cargo package manifest. A virtual workspace is not guessed; select a
member manifest explicitly.

`check` runs the same package selection, validation, and rendering path as
`sync`, then compares the expected bytes in memory. Missing or stale output
fails with the exact sync command needed to restore it and never edits the
file.

## Run The Module

The generated module exposes `ROOT_MODULE`, a typed `Functions` aggregate, and
plain or hosted `bind` support. The application remains explicit about the
runtime lifecycle:

1. Load the resolved project with `compile_typed_project` or
   `compile_typed_host_project`.
2. Build and bind all selected functions into one owner.
3. Seal that owner once.
4. Construct caller-owned capabilities, provider configuration, state, and
   Echo storage.
5. Reuse the typed function handles and sealed module for repeated calls.

See the canonical application's
[`main.rs`](../examples/rust_embedding_application/src/main.rs) for the complete
hosted sequence and exact state and output assertions.

The first managed boundary supports public functions with zero through seven
scalar arguments and a scalar return: Int, Float, String, BitArray,
UtfCodepoint, Bool, or Nil. Public constants, generic signatures, compound
values, callbacks, and other unsupported exports fail synchronization instead
of being omitted. Imported modules may contain ordinary supported Gleam code;
only public functions of the selected root module become Rust bindings.

For direct control over declarations and binding, the
[`rust_embedding.rs`](../examples/rust_embedding.rs) and
[`rust_hosted_embedding.rs`](../examples/rust_hosted_embedding.rs) examples
remain the low-level typed API references.
