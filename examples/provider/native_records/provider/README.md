# Native Records Provider

The Rust half of the [native records example](../README.md). It declares a
symbol-backed `Key` and a tuple-backed `Record`, exposes them as Gleam Dynamic
values, and invokes a typed Gleam callback after checking a record's fields.
