# Geam Callable Provider

This crate implements the `example_callables` Gleam package using ordinary
Geam provider attributes. Private `#[geam::callable]` bodies create capturing
function values; the exported factories return them to Gleam. Generic wrappers
and callbacks held by a custom `Reply(item)` preserve the declared types.

See the [complete example](https://github.com/panarch/geam/tree/main/examples/provider/callables)
for the Gleam declarations and standalone commands. The provider needs no
runtime configuration. Each execution starts with fresh default state.
