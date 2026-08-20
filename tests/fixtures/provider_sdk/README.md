# External Provider SDK Fixture

This independent Cargo workspace demonstrates how an ordinary Rust crate can
implement source-declared Gleam externals and be composed into a Geam runner.
It is the executable reference for the static provider boundary described in
[`docs/host-providers.md`](../../../docs/host-providers.md).

## Workspace Roles

Read the crates in this order:

1. [`domain`](domain) is an ordinary Rust domain crate. It owns the persistent
   `Catalog` value and has no dependency on Geam.
2. [`provider`](provider) adapts that domain crate to Geam. It exports a
   `HostProviderComponent`, initializes caller-owned state from explicit
   configuration, registers typed callbacks, binds `Catalog` as opaque external
   storage, and constructs a compound Gleam result.
3. [`runner`](runner) represents application-owned embedding code. It manually
   combines component stores and run state into a concrete `HostProfile`,
   collects providers, and executes the complete hosted pipeline through the
   same static component contract emitted by the standalone CLI.

The Cargo dependency direction is:

```text
runner -> provider -> domain
   |          |
   +--------> Geam
```

## Canonical Example

[`runner/tests/public_usage.rs`](runner/tests/public_usage.rs) is intended to be
read as user-facing documentation. In one visible flow it contains:

```text
Gleam external declarations and program source
-> explicit component configuration and initialization
-> generated-like Stores, RunState, and Profile
-> HostProviderSet composition
-> typed compilation, planning, and execution sealing
-> execution with exact returned-value and provider-state assertions
```

The remaining tests in that file show independent run states, opaque external
values that outlive execution, and callback failure propagation. Provider-local
tests cover configuration validation and the domain crate separately verifies
its persistent value semantics.

## Run The Fixture

From the Geam repository root:

```sh
cargo test --manifest-path tests/fixtures/provider_sdk/Cargo.toml --workspace --locked
cargo clippy --manifest-path tests/fixtures/provider_sdk/Cargo.toml --workspace --all-targets --locked -- -D warnings
```

The fixture is deliberately a separate locked workspace rather than a root
development dependency. It proves path-based manual embedding and static
composition only. Standalone discovery, configuration-file parsing, runner
generation, and build ownership are verified by the standalone CLI fixture
rather than duplicated here. Cargo publication remains a distribution concern.
