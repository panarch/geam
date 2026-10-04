# Application exit consumer

This fixture implements application termination through the public provider
`#[geam::call]` contract. `exit` and `exit_async` request the checked status of
their current execution domain. The source prints before each request and has
an unreachable output afterward. `normal`, `ordinary_int`, `ordinary_error`
and `fail` distinguish fresh scope reuse, ordinary values and source failures.

Run the independent provider's public embedding regression:

```sh
cargo test --manifest-path tests/fixtures/application_exit/provider/Cargo.toml --locked
```

The provider uses the Geam checkout through a path dependency and its own lock.
Its coverage denominator contains this provider's production code, rather than
all code in its Geam dependency:

```sh
cargo llvm-cov --manifest-path tests/fixtures/application_exit/provider/Cargo.toml --locked --summary-only --fail-under-lines 100 --fail-under-regions 100
```

The repository's `standalone_build` suite copies the source and provider to a
temporary project, then checks actual `geam run` and debug/release executables
for statuses 0, 7, 101 and 255. It also checks output failure precedence and
repeated relocated execution with no source, build directory or PATH.
`prepared_embedding` compiles emitted Rust bindings and checks dynamic/prepared
structured exits, host survival and fresh scope reuse. These suites run in the
existing Linux, macOS and Windows acceptance jobs. Generated program data and
build artifacts are not committed.

The complete public contract is in
[application termination](../../../docs/reference/execution-services.md#application-termination).
