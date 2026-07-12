use super::{
    eval_bool_expr, eval_int_expr, eval_panic_expr, eval_string_expr, project_float_list_expr,
    project_tuple_expr,
};
use crate::plan::ValueType;
use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::{FloatExpr, FloatExprKind};
use crate::runtime::ExecutionError;
use crate::runtime::evaluated::EvaluatedValue;
use crate::runtime::frame::Frame;
use crate::runtime::function;
use crate::runtime::state::RuntimeState;

pub(in crate::runtime) fn eval_float_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &FloatExpr,
) -> Result<f64, ExecutionError> {
    match expression.kind() {
        FloatExprKind::Value(value) => Ok(*value),
        FloatExprKind::LocalGet { local, .. } => Ok(frame.get_float(*local)),
        FloatExprKind::Call { function, args } => {
            function::run_float_call(plan, state, *function, args, frame)
        }
        FloatExprKind::FunctionCall { function, args } => {
            function::run_float_function_call(plan, state, function, args, frame)
        }
        FloatExprKind::TupleIndex { tuple, index } => {
            match project_tuple_expr(plan, state, frame, tuple, *index, ValueType::Float)? {
                EvaluatedValue::Float(value) => Ok(value),
                other => Err(ExecutionError::TupleIndexFamilyMismatch {
                    expected: ValueType::Float,
                    actual: other.value_type(plan),
                }),
            }
        }
        FloatExprKind::ListIndex { list, index } => {
            project_float_list_expr(plan, state, frame, list, *index)
        }
        FloatExprKind::Panic(panic) => {
            eval_panic_expr(plan, state, frame, panic).map(|never| match never {})
        }
        FloatExprKind::Add { left, right } => Ok(eval_float_expr(plan, state, frame, left)?
            + eval_float_expr(plan, state, frame, right)?),
        FloatExprKind::Sub { left, right } => Ok(eval_float_expr(plan, state, frame, left)?
            - eval_float_expr(plan, state, frame, right)?),
        FloatExprKind::Mult { left, right } => Ok(eval_float_expr(plan, state, frame, left)?
            * eval_float_expr(plan, state, frame, right)?),
        FloatExprKind::Div { left, right } => Ok(eval_div_float(
            eval_float_expr(plan, state, frame, left)?,
            eval_float_expr(plan, state, frame, right)?,
        )),
        FloatExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, state, frame, subject)? {
                eval_float_expr(plan, state, frame, true_)
            } else {
                eval_float_expr(plan, state, frame, false_)
            }
        }
        FloatExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_float_expr(plan, state, frame, branch);
                }
            }
            eval_float_expr(plan, state, frame, fallback)
        }
        FloatExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_float_expr(plan, state, frame, branch);
                }
            }
            eval_float_expr(plan, state, frame, fallback)
        }
        FloatExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_float_expr(plan, state, frame, branch);
                }
            }
            eval_float_expr(plan, state, frame, fallback)
        }
        FloatExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, state, steps, frame)?;
            eval_float_expr(plan, state, frame, return_)
        }
    }
}

fn eval_div_float(left: f64, right: f64) -> f64 {
    // Geam normalizes Gleam float division by zero instead of exposing raw Rust
    // f64 infinities or NaN.
    if right == 0.0 { 0.0 } else { left / right }
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        BoolExpr, Expr, FloatExpr, FloatFunctionId, FunctionId, FunctionPlan, IntExpr, ModulePlan,
        PanicExpr, PanicSite, ReturnExpr, Step, StringExpr, TupleExpr, ValueType,
    };
    use crate::runtime::{ExecutionError, run_main};

    #[test]
    fn source_float_expression_variants_evaluate_exact_values() {
        let source = r#"
fn add_half(value: Float) -> Float { value +. 0.5 }

pub fn main() {
  let local = 1.0
  let function = add_half
  #(
    local,
    add_half(1.0),
    function(1.0),
    #(2.0).0,
    case [3.0] { [value] -> value _ -> 0.0 },
    1.0 +. 2.0,
    5.0 -. 2.0,
    2.0 *. 3.0,
    7.0 /. 2.0,
    7.0 /. 0.0,
    case True { True -> 1.0 False -> 0.0 },
    case False { True -> 1.0 False -> 0.0 },
    case 1 { 1 -> 2.0 _ -> 0.0 },
    case 2 { 1 -> 2.0 _ -> 3.0 },
    case "one" { "one" -> 1.0 _ -> 0.0 },
    case "two" { "one" -> 1.0 _ -> 2.0 },
    case 1.0 { 1.0 -> 1.0 _ -> 0.0 },
    case 2.0 { 1.0 -> 1.0 _ -> 2.0 },
    { let _ = 0 4.0 },
  )
}
"#;

        assert_eq!(
            crate::runtime::run_src(source),
            crate::runtime::Value::Tuple(
                vec![
                    1.0, 1.5, 1.5, 2.0, 3.0, 3.0, 3.0, 6.0, 3.5, 0.0, 1.0, 0.0, 2.0, 3.0, 1.0, 2.0,
                    1.0, 2.0, 4.0,
                ]
                .into_iter()
                .map(crate::runtime::Value::Float)
                .collect(),
            ),
        );
    }

    #[test]
    fn source_operand_errors_propagate_through_float_expressions() {
        let expressions = [
            "fail_float() +. 1.0",
            "1.0 +. fail_float()",
            "fail_float() -. 1.0",
            "1.0 -. fail_float()",
            "fail_float() *. 1.0",
            "1.0 *. fail_float()",
            "fail_float() /. 1.0",
            "1.0 /. fail_float()",
            "case fail_bool() { True -> 1.0 False -> 0.0 }",
            "case fail_int() { 0 -> 0.0 _ -> 1.0 }",
            "case fail_string() { \"zero\" -> 0.0 _ -> 1.0 }",
            "case fail_float() { 0.0 -> 0.0 _ -> 1.0 }",
            "{ let _ = fail_bool() 1.0 }",
            "{ let function = fail_float function() }",
        ];

        for expression in expressions {
            let source = format!(
                r#"
fn fail_bool() -> Bool {{ panic }}
fn fail_int() -> Int {{ panic }}
fn fail_string() -> String {{ panic }}
fn fail_float() -> Float {{ panic }}
pub fn main() -> Float {{ {expression} }}
"#,
            );

            assert_eq!(
                crate::runtime::run_src_error(&source).to_string(),
                "panic: `panic` expression evaluated.",
            );
        }
    }

    #[test]
    fn module_expression_errors_propagate_through_float_wrappers() {
        let panic = || PanicExpr::panic_at(None, PanicSite::unknown());
        let expressions = [
            FloatExpr::tuple_index(TupleExpr::panic(panic(), vec![ValueType::Float]), 0),
            FloatExpr::bool_case(
                BoolExpr::panic(panic()),
                FloatExpr::value(1.0),
                FloatExpr::value(0.0),
            ),
            FloatExpr::int_case(IntExpr::panic(panic()), Vec::new(), FloatExpr::value(0.0)),
            FloatExpr::string_case(
                StringExpr::panic(panic()),
                Vec::new(),
                FloatExpr::value(0.0),
            ),
            FloatExpr::float_case(FloatExpr::panic(panic()), Vec::new(), FloatExpr::value(0.0)),
            FloatExpr::block(
                vec![Step::evaluate(Expr::bool(BoolExpr::panic(panic())))],
                FloatExpr::value(0.0),
            ),
        ];

        for expression in expressions {
            assert_eq!(
                run_module_float_expression(expression).to_string(),
                "panic: `panic` expression evaluated.",
            );
        }
    }

    fn run_module_float_expression(expression: FloatExpr) -> ExecutionError {
        let main = FunctionPlan::new(
            FunctionId::new(0),
            "main".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::float(FloatFunctionId(0), expression),
        );
        let module = ModulePlan::new("main".into(), main, Vec::new());
        let plan = crate::ExecutionPlan::from_module_plan(module);

        run_main(&plan).expect_err("module expression should fail at runtime")
    }
}
