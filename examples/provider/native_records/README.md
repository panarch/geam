# Host Provider: Native Records

A provider can expose the native representation of an external value to
Gleam's Dynamic decoders. This example gives `Record` a tagged tuple shape and
`Key` a symbol shape. The original external types remain distinct.

## Read The Example

1. [The Gleam API](project/packages/example_native_records/src/example_native_records.gleam)
   declares `Key`, `Record`, Dynamic conversion, and the callback operation.
2. [The Rust provider](provider/src/lib.rs) declares each `native_view` and
   checks record fields before invoking the callback.
3. [The application](project/src/native_records_example.gleam) reads the same
   record through Dynamic decoders and the Rust callback operation.

```gleam
let data = records.erase(records.record("visits", 42))
let decoder = {
  use label <- decode.field(1, decode.string)
  use count <- decode.field(2, decode.int)
  decode.success(#(label, count))
}
assert decode.run(data, decoder) == Ok(#("visits", 42))
```

The record's native tuple is `#(records.key("record"), "visits", 42)`.
Both representations compare equally as Dynamic values, work as dictionary
keys, and have the same inspection output. A symbol is distinct from a String.

## Run

```sh
cd examples/provider/native_records/project
geam provider add --path ../provider
geam prepare
geam run
```

The application checks decoding, equality, dictionary lookup, inspection,
callback results, and rejection of malformed records. A successful run is silent.

Next: [Async Rust work](../async_files/README.md).
