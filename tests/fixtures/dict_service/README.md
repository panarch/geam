# External Dict service consumer

This independent manual provider constructs the original `gleam/dict.Dict`
through `geam::gleam_stdlib::service::dict_from_entries`. The standard library
owns the schema, binding, retained entries and source/native value semantics.
The consumer uses the public `DictOf<Key, Item>` alias and typed host APIs; it
does not implement foreign storage or use `provider_support`.

The [provider](provider/src/lib.rs) registers exact Dict construction tokens with
`with_scoped_function_and_constructions`. `entries` consumes String pairs,
including an empty iterator and a repeated key. `groups` constructs
`Dict(Int, List(String))`, with a separate token for its intermediate Lists.
`nested` returns `#(Dict(String, String), List(Dict(String, String)))`, including
an empty child Dict. Returning a Dict directly is not required.

The same component also registers `lookup` returning `Result(String, Nil)` and
`try_entries` returning `Result(Dict(String, String), Nil)`. Both use the public
`geam::provider::{GleamResult, GleamOk, GleamError}` markers. `lookup` returns
fixed fixture data, while `try_entries` passes a Dict constructed by the same
stdlib service into `Ok`. They use the ordinary typed host construction path;
neither duplicates the Result schema nor imports hidden support.

Keys use Gleam equality and hashing. The last equal pair wins, matching
`dict.from_list`; iteration order is unspecified. Input pairs are consumed and
unique entries retained, so the value does not borrow the input container.
The host handle stays call-scoped. Ordinary returned values retain their data;
embedding a scoped value does not extend that value's execution lifetime.

The [Gleam project](project) pins original `gleam_stdlib` 1.0.3. Assertions use
its unchanged `get`, `size`, `from_list`, `insert` and `delete` bodies to check
each returned value, missing keys, source equality, Dict keys and persistent
aliases. [public_usage.rs](provider/tests/public_usage.rs) composes the public
components, checks locked source acquisition, executes twice, and inspects the
complete nested result after dropping execution and state. Result assertions
fix both variants, empty and Unicode/NUL String payloads, source and stdlib
Result equality, and the original Dict operations on a Result payload.
Component projection unit tests live beside the provider implementation.

From the repository root, with Rust and Gleam 1.18.1:

```sh
cargo fetch --manifest-path tests/fixtures/dict_service/provider/Cargo.toml --locked
cargo test --manifest-path tests/fixtures/dict_service/provider/Cargo.toml --locked
cargo fmt --manifest-path tests/fixtures/dict_service/provider/Cargo.toml --all --check
cargo clippy --manifest-path tests/fixtures/dict_service/provider/Cargo.toml --all-targets --locked -- -D warnings
gleam format --check tests/fixtures/dict_service/project/src
```

This fixture has an independent Cargo lock and enables only Geam's `provider`
and `gleam-stdlib` features. Downloaded source and build products are ignored.
The test acquires locked Gleam dependencies and preserves the manifest bytes.
It does not generate a prepared program or require Erlang services.

Workspace, Acceptance and Coverage run the fixture on normal pull requests.
Its independent 100% line/full-region gate covers the fixture provider package;
Geam dependencies retain their separate owner gates. See the
[testing guide](../../../docs/development/testing.md#dict-service-consumer) and
[service guide](../../../docs/reference/execution-services.md#constructing-standard-library-dicts).

This verifies Dict construction and its composition with canonical Results.
The [Provider SDK fixture](../provider_sdk) separately proves Result-only use
without stdlib. Neither fixture implements `envoy` or reads or changes the
process environment.
