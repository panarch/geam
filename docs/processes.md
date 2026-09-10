# Gleam processes

Use [`gleam_erlang`](https://hexdocs.pm/gleam_erlang/) to run concurrent Gleam
code in Geam. Its processes have their own identities and mailboxes. A process
can wait for a message while other processes continue running.

## Send and receive

Add the package to a Gleam project:

```sh
gleam add gleam_erlang
```

Use this as the project's main module:

```gleam
import gleam/erlang/process
import gleam/io

pub fn main() {
  let reply = process.new_subject()
  let _worker = process.spawn(fn() {
    process.send(reply, "Hello from a Gleam process")
  })
  process.receive_forever(reply) |> io.println
}
```

```sh
geam run
```

The program prints `Hello from a Gleam process`. The `Subject(String)` identifies
the receiving process and the messages it accepts. Sending does not wait for
the receiver; receiving waits for the next matching message and leaves other
messages in the mailbox.

`process.spawn` links the worker to its caller. `process.spawn_unlinked` creates
an independent process. Selectors choose among multiple Subjects, monitors, and
exit messages. The package's [process API](https://hexdocs.pm/gleam_erlang/gleam/erlang/process.html)
describes these operations, including request/reply calls and timers.

The [service example](../examples/embedding/processes) keeps a counter in a
Gleam process, updates it through several requests, and observes its shutdown.
Its Gleam program runs on both Erlang and Geam.

## Call a service from Rust

In an embedding project, add `gleam_erlang` under `gleam/` and run
`geam embedding sync`. Sync enables Geam's `gleam-erlang` feature and generates
the required component and configuration inputs.

Public Gleam functions can return `Pid` and `Subject` values. Rust retains them
as typed opaque handles and passes them back to other Gleam functions:

```rust
let (pid, requests) = scope.call(&functions.start, ()).await?;
let total = scope.call(&functions.add, (&requests, 42.into())).await?;
let stopped = scope.call(&functions.stop, (&pid, &requests)).await?;
```

These are the calls from the [complete Rust service example](../examples/embedding/processes).
They share one `with_execution` scope. Returning from `start` ends that function's
execution, not the unlinked service it created. The service can continue
responding between Rust calls while the enclosing Rust Future is driven.

The Rust application selects an `ExecutionHost`. `TokioHost` adapts an existing
Tokio runtime; another host can supply its own worker scheduling and monotonic
clock. Process code in Gleam is unchanged by that choice.

## Execution lifetime

Standalone execution ends when ordinary `main` returns, or when a
[Future](future.md) returned directly by `main` completes. Remaining processes
are cancelled and their worker cleanup is awaited.

In Rust, the enclosing `with_execution` scope owns this lifetime. Normal scope
completion cancels remaining processes and awaits cleanup. Dropping its Rust
Future requests cancellation without blocking. Retaining a Pid or Subject does
not keep the execution scope alive.

Processes are logical runtime units, not operating-system processes. The host
may schedule them across multiple workers. Geam does not connect to distributed
Erlang nodes. See [native process semantics](reference/runtime-semantics.md#processes-and-native-environment)
for node identity, atom tables, package resources, and exit reasons.
