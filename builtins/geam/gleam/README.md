# Geam APIs

Core APIs for Gleam programs running on [Geam](https://github.com/panarch/geam),
the Rust runtime and embedding layer for Gleam.

```sh
gleam add geam
```

## Future

Use `geam/future` to compose work from Rust async functions:

```gleam
import geam/future.{type Future}

pub fn answer() -> Future(Int) {
  use value <- future.map(future.ready(21))
  value * 2
}
```

Calling `answer` returns work. The Rust host explicitly drives it to obtain `42`.
Reusing the same Future shares its result rather than repeating the operation.
`map` transforms a result, `then` continues with another Future, and `all`
combines independent work in input order.

See the [Future guide](https://github.com/panarch/geam/blob/main/docs/future.md)
and [Rust embedding guide](https://github.com/panarch/geam/blob/main/docs/embedding.md#drive-explicit-future-values)
for the complete source and host workflow.

## Runtime Support

These APIs are implemented by Geam. This package is versioned independently of
the Geam runtime.

Implementations for the Erlang and JavaScript runtimes are not yet available.
