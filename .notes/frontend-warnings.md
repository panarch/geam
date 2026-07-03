# Frontend Warnings

Gleam warnings are produced during the Gleam parse/analyse frontend phase, not
by the Geam planner or runtime.

Current Geam frontend uses `WarningEmitter::null()`, so warnings such as
redundant case pattern warnings can be produced by Gleam but are not exposed
through Geam's public frontend API yet.

Future work:

- Add a frontend output type that carries both `TypedModule` and warnings.
- Keep planner behavior focused on preserving Gleam runtime semantics for
  accepted source.
- Do not reimplement Gleam warning logic in the planner unless Geam later
  introduces Geam-specific warnings.
