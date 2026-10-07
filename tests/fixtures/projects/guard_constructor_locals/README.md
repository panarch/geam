# Guard constructor locals

This Gleam-only project locks original `gleam_stdlib` 1.0.3 and `clip` 1.2.2.
Downloaded package sources are unchanged and stay under the ignored `build/`
directory. The application checks local values inside guard constructors and
nested containers, pattern bindings, generic calls, captures, and the original
clip option/flag paths, including exact errors and remaining arguments.
Nested fallthrough cases also bind previously ignored fields, list heads/tails,
and aliases, including guarded alternatives and retained captures.
Earlier clauses also narrow Bool siblings in tuples and custom fields before
the remaining branch reads the second list element.
Arithmetic results also pass through captured constructor guards across small
and arbitrary-precision integer boundaries, including promotion and demotion.
Record guards also preserve constructor narrowing for generic, nongeneric and
reordered variants. Source assertions cover pattern-first OR guards, aliases,
clause scope, shadowing, nested record/tuple fields and shared-field controls.
Typed dynamic and prepared consumers repeat True/False inputs, including a
Fragment whose fields differ from the matched Element. These cases reproduce
the field layout and guard used by Lustre; they do not establish full Lustre
support.

From the repository root, run:

```sh
cargo test --package geam --test guard_constructor_locals --locked
cargo test --package geam --test prepared_embedding --locked
cargo test --package geam --test standalone_build --locked -- --test-threads=1
```

The root target exercises hosted execution and typed Rust embedding. The
prepared target compiles generated Rust data and runs after removing the Gleam
project. The standalone target runs the checkout CLI and relocates debug and
release executables without source, build directories, or development tools.
These downstream checks complement core owner tests; they do not establish full
clip CLI or provider support.
