# Async File Reading

A Rust `async fn` supplies explicit work to Gleam:

```rust
#[geam::function]
async fn read(path: EcoString) -> Result<EcoString, EcoString> {
    async_fs::read_to_string(path.as_str())
        .await
        .map(EcoString::from)
        .map_err(|error| EcoString::from(error.to_string()))
}
```

The [ordinary Gleam package](project/packages/example_async_files) declares:

```gleam
import geam/future.{type Future}

@external(erlang, "example_async_files", "read")
pub fn read(path: String) -> Future(Result(String, String))
```

The [Rust provider crate](provider) uses the same provider and module attributes
as synchronous providers. Its async return becomes `Future(Result(...))`, not
an implicitly awaited `Result`.

The [async embedding example](../../embedding/async_host) supplies the Rust
executor, combines the returned work in Gleam and observes its shared completion.
Run that example for the complete source-to-Rust workflow.

`async-fs` owns the filesystem operation. Geam does not create an executor or
choose the application's async runtime. Failed reads become ordinary Gleam
`Error(String)` values.
