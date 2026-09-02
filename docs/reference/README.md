# Technical Reference

These documents define the public execution and integration contracts behind
the user workflows. Start with a guide when you want to run a project, embed a
module, or author a provider; use this section when you need exact ownership,
type, compatibility, or runtime behavior.

- [Architecture](architecture.md) explains where Geam begins after Gleam
  analysis and how plain, hosted, standalone, and embedding paths relate.
- [Compatibility](compatibility.md) records supported toolchains, verified
  packages, execution-profile limits, and deployment boundaries.
- [Rust embedding boundary](embedding-boundary.md) defines generated and manual
  binding ownership, supported Rust data, retained Lists, and provider state.
- [Host provider boundary](provider-boundary.md) defines provider type mappings,
  components, external storage, state, callbacks, and runner composition.
- [Runtime semantics](runtime-semantics.md) defines values, equality, control
  flow, host re-entry, IO, errors, and execution-plan behavior.

The generated Rust API is published on [docs.rs](https://docs.rs/geam), which
documents individual Rust items. These pages explain how those items compose
across execution, embedding, and provider boundaries.
