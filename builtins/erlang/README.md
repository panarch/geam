# geam-erlang

Built-in Rust implementation of [`gleam_erlang`](https://hexdocs.pm/gleam_erlang/).
Geam runs the package's Gleam source and supplies its native process, mailbox,
selector, timer, atom, reference, Charlist, node, and application-resource
operations through this crate.

Standalone applications use `gleam_erlang` as a Gleam dependency and run with
`geam run`. Rust applications enable Geam's `gleam-erlang` feature and use
`geam embedding sync` to generate bindings. A Rust execution scope keeps
processes running between calls; its host supplies the executor and clock.

See the [process service example](https://github.com/panarch/geam/tree/main/examples/embedding/processes)
for a Rust application retaining typed Pid and Subject handles.

## Native Environment

Processes are logical execution units, not OS processes. Messages retain their
immutable values; Pids and Subjects identify processes without owning their
mailboxes or the execution domain. Normal scope shutdown cancels remaining
units and waits for executor cleanup.

Each execution domain owns its atom table. It starts with the execution's
native constructor tags and the builtin's scalar, selector, node, and signal
tags. Names created by `atom.create` and `process.new_name` may contain up to
255 Unicode codepoints. New names are rejected when the table contains
1,048,576 entries; existing names retain their identities until domain shutdown.

The local node is `nonode@nohost`, `visible()` is empty, and `connect()` returns
`LocalNodeIsNotAlive`. `Port` is an opaque type; this package has no port-creation
API.

`Configuration::resources` maps exact package names to resource directories.
Project-based hosts obtain it from the compiled program's `package_resources()`;
source-only hosts supply their own locations. `priv_directory` returns known
paths without requiring a `priv` directory to exist.
