# Raw String provider consumer

This independent consumer uses only the public `geam` facade. Its separately
registered SHA-1 and base64 functions keep their Gleam String signatures:

```gleam
fn crypto_hash(hash: ShaHash, data: String) -> String
fn base64_encode(data: String) -> String
```

The SHA-1 provider returns `StringValue::from_bytes`; the base64 provider consumes
`as_bytes()`. The RFC WebSocket key produces the exact 20-byte digest
`B37A4F2CC0624F1690F64606CF385945B2BEC4EA` and accept value
`s3pPLMBiTxaQ9kYGzzhZRbK+xOo=`. There is no call fusion or intermediate encoding.
This proves the byte-bearing String connection, not a complete WebSocket server
or compatibility with all of gramps.

The source and public embedding test cover empty, NUL, `80`, `FF FE`, truncated
`C3`, Unicode, byte ranges through a codepoint, raw prefix suffixes, nested values
and repeat calls after releasing input owners. Native constructors and byte
operations preserve bytes. Unicode consumers explicitly check UTF-8; they can
fail without changing the stored value. Clone and large ranges share storage;
small ranges and `detached()` release the original parent. Byte equality and
hashing match `Borrow<[u8]>`, not `Borrow<str>`.

The dynamic and prepared embedding caller also checks UTF-16/32 output bytes
and rejects invalid input as a source segment panic with the original module,
function and span. This keeps encoding failures distinct from text-consuming
native provider failures.

From the repository root:

```sh
cargo test --manifest-path tests/fixtures/raw_string_values/provider/Cargo.toml --locked
cargo llvm-cov --manifest-path tests/fixtures/raw_string_values/provider/Cargo.toml --locked --summary-only --fail-under-lines 100 --fail-under-regions 100
(cd tests/fixtures/raw_string_values/embedding && ../../../../target/debug/geam embedding sync)
cargo test --package geam --test prepared_embedding --locked -- raw_strings_run_dynamic
```

Sync generates the ignored `embedding/src/geam_bindings/program.rs`. Bindings
and lockfiles are retained; generated execution data and build caches are not.
The prepared acceptance test checks dynamic and generated execution, debug and
release standalone builds, relocation, repeat execution and execution with the
original source and compilers unavailable. IO comparisons use exact bytes.
SHA-1/base64 dependencies belong solely to this consumer, not Geam built-ins.
