# External Charlist service consumer

This independent manual provider constructs original `gleam_erlang` Charlists
through `geam::gleam_erlang::service::charlist_from_string`. The producer owns the
binding and retained character list. The consumer has no Charlist storage adapter
or replacement schema and uses only public Geam APIs.

The [provider implementation](provider/src/lib.rs) registers exact constructions
for `Charlist` and `HostListType<char>`. Its three functions show direct text
conversion, a `#(Charlist, Charlist)` header pair, and nested status/header-list
construction. Additional intermediate tuples and lists have their own registered
tokens. The constructor borrows Rust text only for the call and preserves Unicode
scalar values without normalization.

The [Gleam project](project) locks `gleam_erlang` 1.3.0 and `gleam_stdlib` 1.0.3.
It runs their unchanged source to check empty, ASCII, NUL, Unicode and combining
characters, equality, dictionary keys, and exact nested fields. The
[integration target](provider/tests/public_usage.rs) keeps the complete profile
and public component composition visible. It adds an observation-only native
function to compare kind, equality in both directions, and hashes against
explicit integer lists, including nested values. This observer belongs to the
integration test and does not construct Charlist payloads. The target also runs
twice and inspects retained results after dropping execution and state.

From the repository root, with the repository's Rust toolchain and Gleam 1.18.1:

```sh
cargo test --manifest-path tests/fixtures/charlist_service/provider/Cargo.toml --locked
cargo fmt --manifest-path tests/fixtures/charlist_service/provider/Cargo.toml --all --check
cargo clippy --manifest-path tests/fixtures/charlist_service/provider/Cargo.toml --all-targets --locked -- -D warnings
gleam format --check tests/fixtures/charlist_service/project/src
```

The test downloads exact locked Gleam dependencies as needed and verifies that
the manifest remains unchanged. Cargo dependencies use this fixture's own lock.
Downloaded packages, target directories and generated build data are ignored.

The fixture package has an independent 100% line and full-region coverage gate;
Geam dependencies keep separate owner gates. See the
[testing guide](../../../docs/development/testing.md#charlist-service-consumer)
for its fresh-profile commands and the
[public service guide](../../../docs/reference/execution-services.md#constructing-erlang-charlists)
for the authoring contract. Workspace, Acceptance and Coverage run these checks
on ordinary pull requests.

This is a verification consumer. Its fixed HTTP-shaped metadata is not an HTTP
implementation, and it does not expand provider macro return mappings.
