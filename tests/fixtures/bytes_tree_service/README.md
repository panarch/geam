# BytesTree Service Consumer

This independent provider constructs and receives the original opaque
`gleam_stdlib` 1.0.3 `BytesTree` through ordinary `#[geam::function]` declarations
and `geam::gleam_stdlib::service::{BytesTreeOutput, BytesTreeInput}`. It does not
declare foreign constructors, storage, hidden codecs, or manual HostCalls. The
provider enables only `provider,gleam-stdlib`; its dev dependency adds `embedding`
for public tests.

The project pins the unchanged original stdlib package. Native output moves an
owned BitArray into `BytesTreeOutput::from_bit_array`, preserving its byte storage
and selected byte range. Returning the adapter creates one original binary
leaf with fixed-size node/field metadata. The caller composes this component with
`geam::gleam_stdlib::Component<Io>` and supplies `GleamStdlibHostProfile` and both
component projections; stdlib owns the schema and sharing grant.

Direct, tuple, Result (both branches), and nested List/tuple/Result returns are
consumed through original `bytes_tree.to_bit_array`, append, prepend, and concat.
Source assertions cover empty data, NUL/non-UTF-8 bytes, a 16 KiB input, byte-aligned
partial views, repeated calls, and immutable aliases. Non-byte-aligned leaves
receive zero padding on construction, like original `bytes_tree.from_bit_array`:
`<<5:size(3)>>` becomes `<<160>>`. Padding extends the logical bit length while
sharing storage; aliases keep their original bit lengths.

Original public APIs also construct the existing native-read inputs. Native
reads cover empty branches, non-UTF-8 binary, NUL, Unicode and combining text,
nested StringTree/BytesTree, wide Lists, byte padding,
append/prepend order, and preserved aliases. Source compares both explicit bytes
and original `bytes_tree.to_bit_array` results.

`provider/tests/public_usage.rs` composes the public stdlib and consumer
components and binds typed functions with `HostedModuleBuilder`. It repeats
calls, retains both original and native-generated inputs in caller-owned state,
and reads those inputs and owned results after execution closure. Controlled
native Pending work reads before suspension and after resumption on another
worker. A weak reference to
the actual retained input observes its release after completion or cancellation;
cancellation also releases the gate and leaves the module usable.

The default standalone state uses one self-waking suspension instead of a timer.
`tests/standalone_build.rs` includes this exact provider/project in its existing
complete application. Both debug and release executables run repeatedly after
relocation and deletion of the fixture's source, provider, and build directories,
with an empty PATH. Source assertions check the byte results while the enclosing
application retains its exact stdout/stderr checks. Downloaded packages, targets,
and generated Rust stay outside Git.

```sh
cargo fetch --manifest-path tests/fixtures/bytes_tree_service/provider/Cargo.toml --locked
cargo test --manifest-path tests/fixtures/bytes_tree_service/provider/Cargo.toml --locked
cargo fmt --manifest-path tests/fixtures/bytes_tree_service/provider/Cargo.toml --all --check
cargo clippy --manifest-path tests/fixtures/bytes_tree_service/provider/Cargo.toml --all-targets --locked -- -D warnings
gleam format --check tests/fixtures/bytes_tree_service/project/src
cargo test --package geam --test standalone_build --locked -- --test-threads=1
cargo llvm-cov clean --manifest-path tests/fixtures/bytes_tree_service/provider/Cargo.toml --workspace
cargo llvm-cov --manifest-path tests/fixtures/bytes_tree_service/provider/Cargo.toml --no-report --locked
cargo llvm-cov report --manifest-path tests/fixtures/bytes_tree_service/provider/Cargo.toml --package geam-bytes-tree-service-fixture --summary-only --fail-under-lines 100 --fail-under-regions 100
```

Coverage is measured for this fixture provider package. Geam dependencies retain
their separate production-owner coverage gates. The fixture proves BytesTree
binary-leaf construction and consumption. It does not implement recursive native
tree builders, compression, or networking.
