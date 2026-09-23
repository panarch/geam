# Host Provider: Process Service

This provider sends requests to a named Gleam process and waits for its replies.
The application and Rust provider use the same process identities, registered
names, mailboxes, and host clock.

## Read the example

1. [`example_process_service.gleam`](project/packages/example_process_service/src/example_process_service.gleam)
   declares `request(name, make_message, timeout_ms)` and its errors. It creates
   a fresh reply Subject for each request.
2. [`provider/src/lib.rs`](provider/src/lib.rs) implements `exchange` with an
   ordinary async provider function. It resolves the name, sends through the
   producer's Subject API, waits on the current process's mailbox, and restores
   the reply's exact source type.
3. [`process_service_example.gleam`](project/src/process_service_example.gleam)
   starts a calculator, receives two replies, handles a silent process, and
   stops the calculator through an ordinary request.

The public call keeps the message and reply types independent:

```gleam
let assert Ok(42) = service.request(name, Add(35, _), 1000)
```

`Unavailable` means no process held the name when sending. `TimedOut` means
the deadline expired while the original target was still alive; `TargetExited`
means that target had exited by the deadline. A zero timeout accepts a queued
reply without waiting. `InvalidTimeout` rejects negative or oversized millisecond
values. `InvalidReply` rejects a native message that does not retain the declared
reply type. Ending the execution cancels pending native work.

The provider uses schema 2 metadata with
`requires-services = ["gleam_erlang"]`. Its component has no additional service
or mutable state. The generated host includes the producer's service once.

## Run

With the checkout Geam CLI, Rust, and Gleam available, run from the repository root:

```sh
cd examples/provider/process_service/project
geam provider add --path ../provider
geam prepare
geam run
```

The complete output is:

```text
named service replied: 42, 17
request timeout and unavailable name handled
worker stopped and name released
```

Running again starts fresh processes and registered names. The independently
locked [provider tests](provider/tests/shared_service.rs) run the same application
with a manually advanced host clock. The [embedding consumer](embedding) uses
the same provider through generated dynamic and prepared bindings:

```sh
cd ../embedding
geam embedding sync
cargo run --locked
cargo run --locked -- --prepared
```

The default embedding command executes both modes, printing the output twice.
`--prepared` executes only the compiled artifact. See the
[execution service reference](../../../docs/reference/execution-services.md)
for the service, value ownership, and lifecycle contracts.

Next: [async file work](../async_files) uses explicit source `Future` values for
work that can be retained and observed separately from an ordinary request.
