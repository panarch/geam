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

The application combines the work with an ordinary Gleam callback:

```gleam
pub fn main() -> Future(Nil) {
  use result <- future.map(example_async_files.read("message.txt"))
  case result {
    Ok(text) -> io.print(text)
    Error(reason) -> io.println(reason)
  }
}
```

## Run the example

From the repository root:

```sh
cd examples/provider/async_files/project
geam provider add --path ../provider
geam prepare
geam run
```

The program prints `Read by a Rust async function.` from `message.txt`.
When `main` returns a Future, the standalone runner drives that work to
completion using its Tokio runtime. Work nested inside an ordinary return
value is not started automatically.

`async-fs` owns the filesystem operation. Failed reads become ordinary Gleam
`Error(String)` values, handled by the callback above.

The [async embedding example](../../embedding/async_host) uses the same provider
from Rust. In embedding, the Rust application supplies its own executor and
observes completion explicitly.

Next: [Publish a package pair](../text_pattern/README.md).
