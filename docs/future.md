# Future

Use `geam/future` to compose work from Rust async functions in Gleam. The module
belongs to the `geam` Gleam package; Geam provides its Rust implementation as a
built-in.

A `Future(a)` represents one operation and its shared result of type `a`.
Start with a result that is already available:

```gleam
import geam/future.{type Future}

pub fn answer() -> Future(Int) {
  future.ready(42)
}
```

`ready` puts an existing value into a completed Future. Calling `answer()`
returns that Future; the Rust application obtains `42` by observing it.

## Add the package

From the Gleam package directory containing `gleam.toml`, add `geam`:

```sh
gleam add geam
```

For a Rust embedding application, run this inside its nested `gleam/` directory.
This is an ordinary dependency, so the Gleam language server can provide type
checking, completion and navigation for `geam/future`.

The Gleam package is versioned independently of the Rust runtime; their version
numbers do not need to match.

The Rust implementation is included with Geam; no external provider selection is
needed for this package. Use `geam run` in a standalone project, or run
`geam embedding sync` in a Rust embedding application. An Erlang implementation
is not currently available, and calling these operations on Erlang reports that
limitation.

## Run a Future-returning program

When a standalone program returns a Future from `main`, `geam run` drives it to
completion:

```gleam
import geam/future.{type Future}
import gleam/int
import gleam/io

pub fn main() -> Future(Nil) {
  use value <- future.map(future.ready(42))
  io.println(int.to_string(value))
}
```

The standalone runner supplies the Tokio runtime. Only the outer Future returned
by `main` is observed: returning a List of Futures does not drive them, and
`Future(Future(a))` is not implicitly flattened. Use `all` or `then` to express
that composition. A Future completing with `Error(...)` returns source data;
the application decides how to handle it.

## Transform a result with map

`map` creates work that transforms another Future's result. Its callback runs
when the Rust host drives the returned work:

```gleam
import geam/future.{type Future}

pub fn doubled() -> Future(Int) {
  use value <- future.map(future.ready(21))
  value * 2
}
```

Observing `doubled()` produces `42`. The `use` expression passes the following
code as the callback to `map`, so `value` is an ordinary `Int` inside it.

The same pattern works with a Future returned by an async provider. The
[async files provider](../examples/provider/async_files) supplies
`read(String) -> Future(Result(String, String))`. The next examples use that
package; the provider example includes a standalone application, and its
[embedding application](../examples/embedding/async_host) shows the Rust setup.

## Continue with another operation

Use `then` when the next step returns another Future. This example tries a
second file if the first read fails:

```gleam
import example_async_files as files
import geam/future.{type Future}

pub fn read_with_fallback(
  path: String,
  fallback_path: String,
) -> Future(Result(String, String)) {
  use result <- future.then(files.read(path))
  case result {
    Ok(text) -> future.ready(Ok(text))
    Error(_) -> files.read(fallback_path)
  }
}
```

The second read is created only when the first result is `Error`. On success,
`ready` returns the text already read. Both branches return a Future, and
`then` follows the selected work to its result.

Use `map` for a callback returning a result value, and `then` for a callback
returning another Future. Using `map` for the latter keeps the extra layer:
`Future(Future(a))`.

## Combine independent operations

Use `all` when the operations can proceed independently:

```gleam
import example_async_files as files
import geam/future.{type Future}

pub fn read_both(
  first: String,
  second: String,
) -> Future(List(Result(String, String))) {
  future.all([files.read(first), files.read(second)])
}
```

When Rust observes this work, `all` drives both reads so one can make progress
while the other is waiting. Results stay in input order, regardless of which
read finishes first.

Each file read returns a `Result`, so a failed read appears as an `Error` item
in the list. `all` does not turn those source values into one combined error.
It composes the supplied work; it does not create Gleam processes or an
executor.

## Reuse a result

A Future is one operation, not a recipe to run again. If you bind
`let work = files.read(path)`, then `future.all([work, work])` shares the result
of one read. Calling `files.read(path)` twice instead creates two operations.

Passing a Future through a function or container preserves that operation.
Observing it again from Rust shares its completed success or failure without
repeating its effects.

## Drive work from Rust

Once the Rust host has loaded and bound the Gleam `doubled` function, it calls
the function to get work and observes that work to get the result:

```rust
let work = scope.call(&functions.doubled, ())?;
let result = scope.observe(&work).await?;
result.read(|value| println!("{value}"));
```

This prints `42`. The host's executor drives the Rust Future returned by
`observe`; creating or combining Gleam Future values does not start one.

The [embedding guide](embedding.md#drive-explicit-future-values) shows the full
loading, state, and execution-scope setup. To implement an async function for
Gleam, follow the [provider guide](host-providers.md#return-async-rust-work).

## Lifetimes and cancellation

Work can outlive the Gleam function that returned it, while remaining tied to
the Rust execution scope that supplies its state and capabilities.

- Dropping one `observe` call ends that observation. Work retained elsewhere
  can still be observed within the same live scope.
- Dropping the last reference to unfinished work releases what it owns,
  including its inputs and captured values.
- Ending the execution scope cancels pending work. Completed plain results
  remain available; cancellation does not undo I/O that already happened.

See [runtime semantics](reference/runtime-semantics.md#explicit-work) for the
complete sharing and cancellation behavior, and the
[embedding reference](reference/embedding-boundary.md#explicit-future-execution)
for nested Future values and Rust completion errors.
