# Runtime Semantics Notes

Geam should execute supported Gleam code with Gleam semantics, not with raw Rust,
JavaScript, or Erlang operator semantics.

Gleam defines language behavior first, then each backend lowers to target code
that preserves that behavior. Geam should treat Gleam's typed AST and existing
backends as the semantic reference when adding runtime features.

Important example: integer division and remainder.

- Gleam `/` on `Int` lowers to `BinOp::DivInt`.
- Gleam `%` on `Int` lowers to `BinOp::RemainderInt`.
- Both have type `Int, Int -> Int`.
- Division or remainder by zero returns `0`; it is not a runtime error.
- Negative integer division truncates toward zero.
- Remainder follows the dividend sign, matching the target behavior Gleam uses.

Reference points in the local Gleam checkout:

- `compiler-core/src/erlang.rs`: `int_div` emits `0 -> 0` before `div` or `rem`.
- `compiler-core/templates/prelude.mjs`: `divideInt`, `divideFloat`, and
  `remainderInt` preserve the same zero-denominator behavior for JavaScript.
- `test/javascript_prelude/main.mjs`: prelude tests assert `divideInt(_, 0) == 0`.

When adding a new supported operator or expression, first check how Gleam
preserves its semantics across Erlang and JavaScript. If targets differ at the
raw language level, Geam should follow the Gleam-level behavior, not either raw
target directly.

## Tail Call Optimization Notes

Geam tail-call execution should be compared against Gleam semantics, not against
one backend's incidental implementation limits.

Gleam v1.17.0 has two relevant backend realities:

- Erlang target:
  BEAM naturally supports tail calls. Gleam does not need to rewrite normal
  direct tail calls into a source-level loop for Erlang code generation.
- JavaScript target:
  JavaScript has a call-stack limit, so Gleam's JS backend rewrites a narrow
  class of recursive calls into `while (true)` loops with `loop$arg`
  assignments.

The JavaScript backend's explicit loop rewrite is intentionally conservative:

- It only rewrites calls to the current module function itself.
- The call must be in function tail position.
- Tail position flows through final case branches and final block expressions.
- Non-tail positions remain normal calls:
  - call arguments
  - binary operator operands
  - expression statements before the final statement
  - let/assignment values
  - short-circuit RHS expressions
- Function-value calls are not rewritten as tail recursion.
- If a local variable or argument shadows the current function name, the call is
  not treated as recursive.

Geam has a different execution target: it lowers into an `ExecutionPlan` and
runs it with a Rust runtime loop. This makes some tail-call support easier than
the Gleam JS backend:

- Planner-marked tail calls can be executed by replacing the current runtime
  frame and continuing the loop.
- Supporting current-module direct mutual tail calls is acceptable. It is wider
  than Gleam JS's self-recursion loop rewrite, but it does not create a
  source-visible semantic difference.
- Function-returning-function tail calls can use the same `ReturnBody`
  mechanism, as long as the planner preserves the concrete return family.

Current Geam policy:

- It is OK to optimize more direct current-module tail calls than Gleam's JS
  backend, including mutual recursion.
- It is not OK to change observable Gleam behavior.
- Planner lowering, not runtime inference, decides what is a tail call.
- Function-value callee calls remain outside the current TCO boundary until a
  separate design covers them.
- Shadowing must be respected. A name that resolves to a local/function value is
  not a direct module-function tail call.
- Tail-call argument expressions must be evaluated in the caller frame before
  replacing the frame, preserving Gleam evaluation order.

Useful upstream reference points in the local Gleam checkout:

- `compiler-core/src/javascript/expression.rs`
  - `CurrentFunction::can_recurse`
  - `tail_recursion_used`
  - `tail_call_loop`
  - direct call rewrite in `call_with_doc_arguments`
- `compiler-core/src/javascript/tests/recursion.rs`
  - `tco`
  - `tco_case_block`
  - `not_tco_due_to_assignment`
  - `shadowing_so_not_recursive`
- `compiler-core/src/javascript/tests/functions.rs`
  - `tail_call`
  - `tail_call_doesnt_clobber_tail_position_tracking`
  - `shadowing_current`
  - `recursion_with_discards`
- `compiler-core/src/erlang.rs`
  - `Position::Tail` / `Position::NotTail`

## Function Value Clone Follow-up

Function values are currently owned runtime values. Returning or reading a function value clones a small registered-function reference payload: runtime id, parameter locals, and, for function-returning-function values, the returned FunctionType. This is acceptable for the current profile, but recursive FunctionType can make clone cost grow with return depth. If this becomes noisy or expensive, consider making FunctionType/function values cheap handles through Arc or an arena-backed representation instead of changing runtime semantics.

## Public Raw Value API

`Value` and `FunctionValue` are public because `run_main` returns a runtime
result. Treat them as raw Geam runtime containers, not as the final user-domain
embedding API.

Current `Debug`, `PartialEq`, and `Eq` implementations are convenience surfaces
for raw runtime values and tests. They should not be interpreted as Gleam
semantic equality or as a stable user-facing value comparison API. If Geam later
offers semantic value comparison, expose it through an explicit API such as
`evaluate_eq`/`try_eq`, where function values can be handled deliberately.

For Rust embedding, the likely user-facing path is a decode/extract layer from
`Value` into primitive Rust values or user-owned structs, potentially with
generated/proc-macro assistance. Revisit the public trait surface when that
embedding API is designed, not during closure runtime implementation unless it
causes an actual execution or boundary bug.
