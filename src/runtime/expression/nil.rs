use super::{
    eval_bool_expr, eval_float_expr, eval_int_expr, eval_panic_expr, eval_string_expr,
    project_nil_list_expr, project_tuple_expr,
};
use crate::plan::ValueType;
use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::{NilExpr, NilExprKind};
use crate::runtime::ExecutionError;
use crate::runtime::evaluated::EvaluatedValue;
use crate::runtime::frame::Frame;
use crate::runtime::function;
use crate::runtime::state::RuntimeState;

pub(in crate::runtime) fn eval_nil_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &NilExpr,
) -> Result<(), ExecutionError> {
    match expression.kind() {
        NilExprKind::Value => Ok(()),
        NilExprKind::LocalGet { local, .. } => {
            frame.get_nil(*local);
            Ok(())
        }
        NilExprKind::Call { function, args } => {
            function::run_nil_call(plan, state, *function, args, frame)
        }
        NilExprKind::FunctionCall { function, args } => {
            function::run_nil_function_call(plan, state, function, args, frame)
        }
        NilExprKind::TupleIndex { tuple, index } => {
            match project_tuple_expr(plan, state, frame, tuple, *index, ValueType::Nil)? {
                EvaluatedValue::Nil => Ok(()),
                other => Err(ExecutionError::TupleIndexFamilyMismatch {
                    expected: ValueType::Nil,
                    actual: other.value_type(plan),
                }),
            }
        }
        NilExprKind::ListIndex { list, index } => {
            project_nil_list_expr(plan, state, frame, list, *index)
        }
        NilExprKind::Panic(panic) => {
            eval_panic_expr(plan, state, frame, panic).map(|never| match never {})
        }
        NilExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, state, frame, subject)? {
                eval_nil_expr(plan, state, frame, true_)
            } else {
                eval_nil_expr(plan, state, frame, false_)
            }
        }
        NilExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_nil_expr(plan, state, frame, branch);
                }
            }
            eval_nil_expr(plan, state, frame, fallback)
        }
        NilExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_nil_expr(plan, state, frame, branch);
                }
            }
            eval_nil_expr(plan, state, frame, fallback)
        }
        NilExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_nil_expr(plan, state, frame, branch);
                }
            }
            eval_nil_expr(plan, state, frame, fallback)
        }
        NilExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, state, steps, frame)?;
            eval_nil_expr(plan, state, frame, return_)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        BoolExpr, Expr, FloatExpr, FunctionId, FunctionPlan, IntExpr, ModulePlan, NilExpr,
        NilFunctionId, PanicExpr, PanicSite, ReturnExpr, Step, StringExpr, TupleExpr, ValueType,
    };
    use crate::runtime::{ExecutionError, run_main};

    #[test]
    fn source_nil_expression_variants_evaluate_exact_values() {
        let source = r#"
fn nil_value() -> Nil { Nil }

pub fn main() {
  let local = Nil
  let function = nil_value
  #(
    local,
    nil_value(),
    function(),
    #(Nil).0,
    case [Nil] { [value] -> value _ -> Nil },
    case True { True -> Nil False -> Nil },
    case False { True -> Nil False -> Nil },
    case 1 { 1 -> Nil _ -> Nil },
    case 2 { 1 -> Nil _ -> Nil },
    case "one" { "one" -> Nil _ -> Nil },
    case "two" { "one" -> Nil _ -> Nil },
    case 1.0 { 1.0 -> Nil _ -> Nil },
    case 2.0 { 1.0 -> Nil _ -> Nil },
    { let _ = 0 Nil },
  )
}
"#;

        assert_eq!(
            crate::runtime::run_src(source),
            crate::runtime::Value::Tuple(vec![crate::runtime::Value::Nil; 14]),
        );
    }

    #[test]
    fn source_operand_errors_propagate_through_nil_expressions() {
        let expressions = [
            "case fail_bool() { True -> Nil False -> Nil }",
            "case fail_int() { 0 -> Nil _ -> Nil }",
            "case fail_string() { \"zero\" -> Nil _ -> Nil }",
            "case fail_float() { 0.0 -> Nil _ -> Nil }",
            "{ let _ = fail_bool() Nil }",
            "{ let function = fail_nil function() }",
        ];

        for expression in expressions {
            let source = format!(
                r#"
fn fail_bool() -> Bool {{ panic }}
fn fail_int() -> Int {{ panic }}
fn fail_string() -> String {{ panic }}
fn fail_float() -> Float {{ panic }}
fn fail_nil() -> Nil {{ panic }}
pub fn main() -> Nil {{ {expression} }}
"#,
            );

            assert_eq!(
                crate::runtime::run_src_error(&source).to_string(),
                "panic: `panic` expression evaluated.",
            );
        }
    }

    #[test]
    fn module_expression_errors_propagate_through_nil_wrappers() {
        let panic = || PanicExpr::panic_at(None, PanicSite::unknown());
        let expressions = [
            NilExpr::tuple_index(TupleExpr::panic(panic(), vec![ValueType::Nil]), 0),
            NilExpr::bool_case(BoolExpr::panic(panic()), NilExpr::value(), NilExpr::value()),
            NilExpr::int_case(IntExpr::panic(panic()), Vec::new(), NilExpr::value()),
            NilExpr::string_case(StringExpr::panic(panic()), Vec::new(), NilExpr::value()),
            NilExpr::float_case(FloatExpr::panic(panic()), Vec::new(), NilExpr::value()),
            NilExpr::block(
                vec![Step::evaluate(Expr::bool(BoolExpr::panic(panic())))],
                NilExpr::value(),
            ),
        ];

        for expression in expressions {
            assert_eq!(
                run_module_nil_expression(expression).to_string(),
                "panic: `panic` expression evaluated.",
            );
        }
    }

    fn run_module_nil_expression(expression: NilExpr) -> ExecutionError {
        let main = FunctionPlan::new(
            FunctionId::new(0),
            "main".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::nil(NilFunctionId(0), expression),
        );
        let module = ModulePlan::new("main".into(), main, Vec::new());
        let plan = crate::ExecutionPlan::from_module_plan(module);

        run_main(&plan).expect_err("module expression should fail at runtime")
    }
}
