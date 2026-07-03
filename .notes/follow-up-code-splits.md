# Follow-up Code Split Candidates

These are follow-up cleanup candidates found while expanding Geam value
families. Keep this file synced with main so the next cleanup branch can start
from the current shape instead of stale feature-branch context.

After Float, Tuple, and List, the explicit typed-plan approach has been
validated across scalar, heterogeneous compound, and homogeneous recursive
value families. The method works, but it makes several facade/core modules grow
quickly. Prefer small module splits before introducing broader common
abstractions or generation.

## 1. Split `src/planner/function.rs` return body conversion

Status: completed on main by moving return body conversion into
`src/planner/function/return_body.rs` and then splitting the return-family
conversion details into focused child modules.

Current shape:

- `src/planner/function.rs` remains the facade/core function planning entry
  point.
- `src/planner/function/return_body.rs` owns function return dispatch and the
  return-family mismatch boundary.
- `src/planner/function/return_body/primitive.rs` owns primitive return-family
  conversion.
- `src/planner/function/return_body/function_value.rs` owns function-value
  return-family conversion.

Possible remaining follow-up:

- If `src/planner/function.rs` keeps growing, consider moving anonymous
  function planning into `src/planner/function/anonymous.rs`.
- Keep tests colocated with the owning production module.
- Do not create `mod.rs`.

## 2. Split `src/runtime/function.rs`

Status: completed on main by splitting runtime function execution into focused
child modules.

Current shape:

- Keep `src/runtime/function.rs` as the facade/core runner.
- `src/runtime/function/bind.rs` owns argument/capture binding.
- `src/runtime/function/return_body.rs` owns tail return loop helpers.
- `src/runtime/function/steps.rs` owns step execution.
- Keep tests beside the production module that owns the behavior.
- Do not create `mod.rs`.

## 3. Case subject duplication refactoring candidate

Status: defer for now. Keep this as a refactoring point to revisit, not as an
immediate cleanup task.

`bool_subject.rs`, `int_subject.rs`, `string_subject.rs`, and
`float_subject.rs` are mechanically large. They share repeated
branch/result-family conversion patterns, but the subject semantics are not
identical:

- Bool uses fixed `true_` / `false_` slots.
- Int, String, and Float use ordered literal clauses plus fallback.
- String has string-prefix profile boundaries.
- Float has float literal comparison semantics.

Gleam's scalar primitive set is small, and Geam now supports Bool, Int, String,
Float, Tuple, List, Nil, and function result families across case returns. This
duplication is worth remembering, but it is still not urgent enough to force a
generic abstraction now.

Avoid forcing a generic case abstraction too early. Revisit when
review/navigation starts hurting again. A safer first cleanup would be to
extract only repeated branch/result-family conversion helpers if they remain
obviously duplicated.

Possible future file:

- `src/planner/expression/case/branch_family.rs`

## 4. Split DSL micro modules

Status: completed on main by splitting the two mechanically large DSL files
into smaller result-family and subject-family modules.

Current shape:

- Keep `src/planner/dsl/function/return_body.rs` as the facade/core
  `FunctionReturn` builder.
- `src/planner/dsl/function/return_body/conversion.rs` owns `From`
  conversions.
- `src/planner/dsl/function/return_body/primitive.rs` owns primitive return
  helpers.
- `src/planner/dsl/function/return_body/function.rs` owns function-returning
  helpers and their DSL test.
- Keep `src/planner/dsl/expression/case.rs` as the facade/core case DSL module.
- `src/planner/dsl/expression/case/{bool_,int,string}.rs` own subject-family
  case helpers and tests.

Possible remaining follow-up:

- The current DSL child modules are small enough. Do not split further unless
  navigation or review starts hurting again.

## 5. Larger later cleanup: `src/planner/expression/call.rs`

Status: completed on main by splitting planner call lowering into focused child
modules.

Current shape:

- Keep `src/planner/expression/call.rs` as the facade/core dispatch file.
- `src/planner/expression/call/direct.rs` owns direct local call lowering.
- `src/planner/expression/call/function_value.rs` owns function-value call
  lowering.
- `src/planner/expression/call/implicit.rs` owns use/pipeline implicit argument
  handling.
- `src/planner/expression/call/argument.rs` owns call argument lowering.
- Keep shared split-module helpers in the parent facade or in a child module
  that owns the helper's domain.

Possible remaining follow-up:

- `src/planner/expression/call/function_value.rs` is still mechanically large,
  but it owns a coherent behavior family. Do not split it further unless
  navigation or review starts hurting again.

## 6. Plan frame traversal split is lower priority

`src/plan/frame/return_/function.rs`, `src/plan/frame/function.rs`, and
`src/plan/frame/expression.rs` are growing as result families and control-flow
forms grow.

Do not split these just for line count yet. Revisit if another result family or
pattern family makes traversal logic hard to review.

## 7. Split `src/plan/expression.rs`

Status: completed on main by moving expression argument shapes into
`src/plan/expression/arg.rs`.

Current shape:

- `src/plan/expression.rs` remains the `Expr` facade/core file.
- `src/plan/expression/arg.rs` owns `CallArg`, `CallArgKind`, `CaptureArg`,
  `CaptureArgKind`, and `Expr::into_call_arg`.
- `src/plan/expression/{int,string,float,bool,nil,tuple,list}.rs` and
  `src/plan/expression/function/*.rs` already own typed expression families.

Possible remaining follow-up:

- Keep this stable for now. Do not split `From<Value> for Expr` or case
  constructor helpers unless navigation starts hurting again.

## 8. Split `src/plan/value.rs`

Status: completed on main by splitting function value and capture value shapes
into focused child modules.

Current shape:

- `src/plan/value.rs` owns `ValueType`, `FunctionType`, `Value`,
  `ListValue`, and the value facade/re-exports.
- `src/plan/value/function.rs` owns `FunctionValue`, concrete
  `*FunctionValue` structs, and their tests.
- `src/plan/value/capture.rs` owns `CaptureValue`, `CaptureValueKind`, and
  their tests.

Reason:

- This reduces the broadest public/runtime value surface without changing
  semantics.
- Function value and capture value are meaningful subdomains, not arbitrary
  slices by line count.

## 9. Split anonymous function free-variable analysis

Status: completed on main by moving anonymous function free-variable collection
into a focused child module.

Current shape:

- `src/planner/expression/function.rs` owns anonymous function planning,
  closure expression construction, argument validation, and its tests.
- `src/planner/expression/function/free_variables.rs` owns free-variable
  collection and its traversal tests.

Reason:

- The collector traverses many expression/statement shapes and will keep growing
  as pattern binding expands.
- Splitting it improves reviewability without changing the planner lowering
  boundary.

## 10. Split `src/planner/function/return_body.rs` only after smaller wins

Status: completed on main after the smaller plan/free-variable/value splits.

Current shape:

- `src/planner/function/return_body.rs` remains the facade/core return dispatch
  file.
- `src/planner/function/return_body/primitive.rs` owns primitive return body
  conversion and its shape tests.
- `src/planner/function/return_body/function_value.rs` owns function-valued
  return body conversion and its shape tests.

Result:

- No immediate return-body split follow-up remains.
- Keep the current split stable unless a later value family makes one child
  module hard to review again.
