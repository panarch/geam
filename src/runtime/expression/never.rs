use super::{eval_bool_expr, eval_float_expr, eval_int_expr, eval_panic_expr, eval_string_expr};
use crate::plan::execution::{ExecutionPlan, NeverExpr, NeverExprKind};
use crate::runtime::error::ExecutionResult;
use crate::runtime::frame::Frame;
use crate::runtime::function;
use crate::runtime::state::RuntimeState;
use std::convert::Infallible;

pub(in crate::runtime) fn eval_never_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &NeverExpr,
) -> ExecutionResult<Infallible> {
    match expression.kind() {
        NeverExprKind::Call { function, args } => {
            function::run_never_call(plan, state, *function, args, frame)
        }
        NeverExprKind::Arguments { prefix, diverging } => {
            function::eval_call_argument_values(plan, state, prefix, frame)?;
            eval_never_expr(plan, state, frame, diverging)
        }
        NeverExprKind::FunctionCall {
            function: callee,
            args,
        } => function::run_never_function_call(plan, state, callee, args, frame),
        NeverExprKind::FunctionArguments {
            function: callee,
            prefix,
            diverging,
        } => {
            super::function::eval_function_expr(plan, state, frame, callee)?;
            function::eval_call_argument_values(plan, state, prefix, frame)?;
            eval_never_expr(plan, state, frame, diverging)
        }
        NeverExprKind::Values { prefix, diverging } => {
            for value in prefix {
                super::eval_expr(plan, state, frame, value)?;
            }
            eval_never_expr(plan, state, frame, diverging)
        }
        NeverExprKind::LetAssert {
            subject,
            message,
            site,
            pattern_span,
        } => function::fail_let_assert(
            plan,
            state,
            frame,
            subject,
            message.as_deref(),
            site,
            *pattern_span,
        ),
        NeverExprKind::Panic(expression) => eval_panic_expr(plan, state, frame, expression),
        NeverExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, state, frame, subject)? {
                eval_never_expr(plan, state, frame, true_)
            } else {
                eval_never_expr(plan, state, frame, false_)
            }
        }
        NeverExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_never_expr(plan, state, frame, branch);
                }
            }
            eval_never_expr(plan, state, frame, fallback)
        }
        NeverExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_never_expr(plan, state, frame, branch);
                }
            }
            eval_never_expr(plan, state, frame, fallback)
        }
        NeverExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_never_expr(plan, state, frame, branch);
                }
            }
            eval_never_expr(plan, state, frame, fallback)
        }
        NeverExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, state, steps, frame)?;
            eval_never_expr(plan, state, frame, return_)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn never_expressions_preserve_call_and_compound_evaluation_order() {
        let cases = [
            (
                r#"
fn stop() -> value { panic as "unreached direct argument" }
fn keep(_prefix: Int, _value: value) { 1 }
pub fn main() { keep(panic as "direct prefix", stop()) }
"#,
                "panic: direct prefix",
            ),
            (
                r#"
fn stop() -> value { panic as "direct call" }
fn keep(prefix: Int, _value: value) { prefix }
pub fn main() { keep(1, stop()) }
"#,
                "panic: direct call",
            ),
            (
                r#"
fn stop() -> value { panic as "function call" }
fn invoke(function: fn() -> value) -> value { function() }
fn keep(prefix: Int, _value: value) { prefix }
pub fn main() { keep(1, invoke(stop)) }
"#,
                "panic: function call",
            ),
            (
                r#"
fn stop() -> value { panic as "unreached function argument" }
fn keep(_prefix: Int, _value: value) { 1 }
fn fail() -> fn(Int, value) -> Int { panic as "function callee" }
pub fn main() { fail()(1, stop()) }
"#,
                "panic: function callee",
            ),
            (
                r#"
fn stop() -> value { panic as "unreached function argument" }
fn keep(_prefix: Int, _value: value) { 1 }
pub fn main() {
  let function = keep
  function(panic as "function prefix", stop())
}
"#,
                "panic: function prefix",
            ),
            (
                r#"
fn stop() -> value { panic as "function argument" }
fn keep(prefix: Int, _value: value) { prefix }
pub fn main() {
  let function = keep
  function(1, stop())
}
"#,
                "panic: function argument",
            ),
            (
                r#"
pub type Boxed(value) { Boxed(prefix: Int, value: value) }
fn stop() -> value { panic as "compound value" }
fn keep(prefix: Int, _value: value) { prefix }
pub fn main() { keep(1, Boxed(2, stop())) }
"#,
                "panic: compound value",
            ),
            (
                r#"
fn stop() -> value { panic as "unreached Bool branch" }
fn keep(_value: value) { 1 }
pub fn main() {
  keep(case panic as "Bool subject" { True -> stop() False -> stop() })
}
"#,
                "panic: Bool subject",
            ),
            (
                r#"
fn stop() -> value { panic as "unreached Int branch" }
fn keep(_value: value) { 1 }
pub fn main() {
  keep(case panic as "Int subject" { 1 -> stop() _ -> stop() })
}
"#,
                "panic: Int subject",
            ),
            (
                r#"
fn stop() -> value { panic as "unreached String branch" }
fn keep(_value: value) { 1 }
pub fn main() {
  keep(case panic as "String subject" { "one" -> stop() _ -> stop() })
}
"#,
                "panic: String subject",
            ),
            (
                r#"
fn stop() -> value { panic as "unreached Float branch" }
fn keep(_value: value) { 1 }
pub fn main() {
  keep(case panic as "Float subject" { 1.0 -> stop() _ -> stop() })
}
"#,
                "panic: Float subject",
            ),
            (
                r#"
fn first(values: List(value)) -> value {
  let assert [value, ..] = values
  value
}
fn keep(prefix: Int, _value: value) { prefix }
pub fn main() { keep(1, first([])) }
"#,
                "let_assert: Pattern match failed, no pattern matched the value.",
            ),
            (
                r#"
fn stop() -> value {
  let _ = Nil
  panic as "never block"
}
fn keep(prefix: Int, _value: value) { prefix }
pub fn main() { keep(1, stop()) }
"#,
                "panic: never block",
            ),
            (
                r#"
fn keep(_value: #(value)) { 1 }
pub fn main() {
  keep(#({
    let _ = panic as "generic block step"
    panic as "unreached generic block result"
  }))
}
"#,
                "panic: generic block step",
            ),
            (
                r#"
fn keep(_value: #(#(value))) { 1 }
pub fn main() {
  keep(#({
    let _ = panic as "tuple block step"
    #(panic as "unreached tuple block result")
  }))
}
"#,
                "panic: tuple block step",
            ),
            (
                r#"
pub type Boxed(value) { Boxed(value) }
fn keep(_value: #(Boxed(value))) { 1 }
pub fn main() {
  keep(#({
    let _ = panic as "custom block step"
    Boxed(panic as "unreached custom block result")
  }))
}
"#,
                "panic: custom block step",
            ),
            (
                r#"
fn keep(_value: #(value)) { 1 }
pub fn main() {
  keep(#(case True {
    True -> {
      let _ = panic as "generic branch block step"
      panic as "unreached generic branch result"
    }
    False -> panic as "unreached generic fallback"
  }))
}
"#,
                "panic: generic branch block step",
            ),
            (
                r#"
fn keep(_value: #(#(value))) { 1 }
pub fn main() {
  keep(#(case True {
    True -> {
      let _ = panic as "tuple branch block step"
      #(panic as "unreached tuple branch result")
    }
    False -> #(panic as "unreached tuple fallback")
  }))
}
"#,
                "panic: tuple branch block step",
            ),
            (
                r#"
pub type Boxed(value) { Boxed(value) }
fn keep(_value: #(Boxed(value))) { 1 }
pub fn main() {
  keep(#(case True {
    True -> {
      let _ = panic as "custom branch block step"
      Boxed(panic as "unreached custom branch result")
    }
    False -> Boxed(panic as "unreached custom fallback")
  }))
}
"#,
                "panic: custom branch block step",
            ),
        ];

        for (source, expected) in cases {
            assert_eq!(crate::runtime::run_src_error(source).to_string(), expected);
        }
    }

    #[test]
    fn never_cases_select_matching_and_fallback_branches() {
        let cases = [
            (
                r#"case False { True -> panic as "unselected" False -> panic as "bool fallback" }"#,
                "panic: bool fallback",
            ),
            (
                r#"case 1 { 1 -> panic as "int matching" _ -> panic as "unselected" }"#,
                "panic: int matching",
            ),
            (
                r#"case 0 { 1 -> panic as "unselected" _ -> panic as "int fallback" }"#,
                "panic: int fallback",
            ),
            (
                r#"case "selected" { "selected" -> panic as "string matching" _ -> panic as "unselected" }"#,
                "panic: string matching",
            ),
            (
                r#"case "fallback" { "selected" -> panic as "unselected" _ -> panic as "string fallback" }"#,
                "panic: string fallback",
            ),
            (
                r#"case 1.0 { 1.0 -> panic as "float matching" _ -> panic as "unselected" }"#,
                "panic: float matching",
            ),
            (
                r#"case 0.0 { 1.0 -> panic as "unselected" _ -> panic as "float fallback" }"#,
                "panic: float fallback",
            ),
        ];

        for (expression, expected) in cases {
            let source = format!(
                r#"
fn first(value: Int, _other: other) {{ value }}
pub fn main() {{ first(1, {expression}) }}
"#,
            );

            assert_eq!(crate::runtime::run_src_error(&source).to_string(), expected);
        }

        for (selector, expected) in [
            ("True", "panic: runtime bool matching"),
            ("False", "panic: runtime bool fallback"),
        ] {
            let source = format!(
                r#"
fn first(value: Int, _other: other) {{ value }}
pub fn main() {{
  let selector = {selector}
  first(1, case selector {{
    True -> panic as "runtime bool matching"
    False -> panic as "runtime bool fallback"
  }})
}}
"#,
            );

            assert_eq!(crate::runtime::run_src_error(&source).to_string(), expected);
        }
    }
}
