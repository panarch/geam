use super::{
    eval_bool_expr, eval_float_expr, eval_int_expr, eval_panic_expr, eval_string_expr,
    project_tuple_list_expr,
};
use crate::plan::ValueType;
use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::{TupleExpr, TupleExprKind};
use crate::runtime::ExecutionError;
use crate::runtime::evaluated::EvaluatedValue;
use crate::runtime::frame::Frame;
use crate::runtime::function;
use crate::runtime::state::RuntimeState;

pub(in crate::runtime) fn eval_tuple_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &TupleExpr,
) -> Result<Vec<EvaluatedValue>, ExecutionError> {
    match expression.kind() {
        TupleExprKind::Value(elements) => {
            let mut values = Vec::with_capacity(elements.len());
            for element in elements {
                values.push(super::eval_expr(plan, state, frame, element)?);
            }
            Ok(values)
        }
        TupleExprKind::LocalGet { local, .. } => Ok(frame.get_tuple(*local)),
        TupleExprKind::Call { function, args } => {
            function::run_tuple_call(plan, state, *function, args, frame)
        }
        TupleExprKind::FunctionCall { function, args } => {
            function::run_tuple_function_call(plan, state, function, args, frame)
        }
        TupleExprKind::TupleIndex { tuple, index } => {
            let expected = ValueType::Tuple(
                expression
                    .type_()
                    .iter()
                    .map(|type_| plan.value_type(type_))
                    .collect(),
            );
            match project_tuple_expr(plan, state, frame, tuple, *index, expected.clone())? {
                EvaluatedValue::Tuple(values) => Ok(values),
                other => Err(ExecutionError::TupleIndexFamilyMismatch {
                    expected,
                    actual: other.value_type(plan),
                }),
            }
        }
        TupleExprKind::ListIndex { list, index } => {
            let expected = expression
                .type_()
                .iter()
                .map(|type_| plan.value_type(type_))
                .collect::<Vec<_>>();
            project_tuple_list_expr(plan, state, frame, list, *index, &expected)
        }
        TupleExprKind::Panic(panic) => {
            eval_panic_expr(plan, state, frame, panic).map(|never| match never {})
        }
        TupleExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, state, frame, subject)? {
                eval_tuple_expr(plan, state, frame, true_)
            } else {
                eval_tuple_expr(plan, state, frame, false_)
            }
        }
        TupleExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_tuple_expr(plan, state, frame, branch);
                }
            }
            eval_tuple_expr(plan, state, frame, fallback)
        }
        TupleExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_tuple_expr(plan, state, frame, branch);
                }
            }
            eval_tuple_expr(plan, state, frame, fallback)
        }
        TupleExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_tuple_expr(plan, state, frame, branch);
                }
            }
            eval_tuple_expr(plan, state, frame, fallback)
        }
        TupleExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, state, steps, frame)?;
            eval_tuple_expr(plan, state, frame, return_)
        }
    }
}

pub(in crate::runtime) fn project_tuple_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    tuple: &TupleExpr,
    index: usize,
    expected: ValueType,
) -> Result<EvaluatedValue, ExecutionError> {
    let values = eval_tuple_expr(plan, state, frame, tuple)?;
    let Some(value) = values.get(index).cloned() else {
        return Err(ExecutionError::TupleIndexFamilyMismatch {
            expected,
            actual: ValueType::Tuple(values.iter().map(|value| value.value_type(plan)).collect()),
        });
    };
    Ok(value)
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        BoolExpr, Expr, FloatExpr, FunctionId, FunctionPlan, IntExpr, ModulePlan, PanicExpr,
        PanicSite, ReturnExpr, Step, StringExpr, TupleExpr, TupleFunctionId, ValueType,
    };
    use crate::runtime::{ExecutionError, run_main};

    #[test]
    fn source_tuple_expression_variants_evaluate_exact_values() {
        let source = r#"
fn pair(value: Int) { #(value) }

pub fn main() {
  let local = #(1)
  let function = pair
  #(
    #(0),
    local,
    pair(2),
    function(3),
    #(#(4)).0,
    case [#(5)] { [value] -> value _ -> #(0) },
    case True { True -> #(6) False -> #(0) },
    case False { True -> #(0) False -> #(7) },
    case 1 { 1 -> #(8) _ -> #(0) },
    case 2 { 1 -> #(0) _ -> #(9) },
    case "one" { "one" -> #(10) _ -> #(0) },
    case "two" { "one" -> #(0) _ -> #(11) },
    case 1.0 { 1.0 -> #(12) _ -> #(0) },
    case 2.0 { 1.0 -> #(0) _ -> #(13) },
    { let _ = 0 #(14) },
  )
}
"#;

        assert_eq!(
            crate::runtime::run_src(source),
            crate::runtime::Value::Tuple(
                (0_i64..=14)
                    .map(|value| {
                        crate::runtime::Value::Tuple(vec![crate::runtime::Value::Int(value.into())])
                    })
                    .collect(),
            ),
        );
    }

    #[test]
    fn source_operand_errors_propagate_through_tuple_expressions() {
        let expressions = [
            "#(fail_int())",
            "case fail_bool() { True -> #(1) False -> #(0) }",
            "case fail_int() { 0 -> #(0) _ -> #(1) }",
            "case fail_string() { \"zero\" -> #(0) _ -> #(1) }",
            "case fail_float() { 0.0 -> #(0) _ -> #(1) }",
            "{ let _ = fail_bool() #(1) }",
            "{ let function = fail_tuple function() }",
        ];

        for expression in expressions {
            let source = format!(
                r#"
fn fail_bool() -> Bool {{ panic }}
fn fail_int() -> Int {{ panic }}
fn fail_string() -> String {{ panic }}
fn fail_float() -> Float {{ panic }}
fn fail_tuple() -> #(Int) {{ panic }}
pub fn main() -> #(Int) {{ {expression} }}
"#,
            );

            assert_eq!(
                crate::runtime::run_src_error(&source).to_string(),
                "panic: `panic` expression evaluated.",
            );
        }
    }

    #[test]
    fn module_expression_errors_propagate_through_tuple_wrappers() {
        let panic = || PanicExpr::panic_at(None, PanicSite::unknown());
        let value = || {
            TupleExpr::value(
                vec![Expr::int(IntExpr::value(0.into()))],
                vec![ValueType::Int],
            )
        };
        let expressions = [
            TupleExpr::tuple_index(
                TupleExpr::panic(panic(), vec![ValueType::Tuple(vec![ValueType::Int])]),
                0,
                vec![ValueType::Int],
            ),
            TupleExpr::bool_case(BoolExpr::panic(panic()), value(), value()),
            TupleExpr::int_case(IntExpr::panic(panic()), Vec::new(), value()),
            TupleExpr::string_case(StringExpr::panic(panic()), Vec::new(), value()),
            TupleExpr::float_case(FloatExpr::panic(panic()), Vec::new(), value()),
            TupleExpr::block(
                vec![Step::evaluate(Expr::bool(BoolExpr::panic(panic())))],
                value(),
            ),
        ];

        for expression in expressions {
            assert_eq!(
                run_module_tuple_expression(expression).to_string(),
                "panic: `panic` expression evaluated.",
            );
        }
    }

    #[test]
    fn tuple_index_failure_reports_the_actual_nested_list_type() {
        let list = crate::plan::ListExpr::value(Vec::new(), ValueType::Int);
        let expression = TupleExpr::tuple_index(
            TupleExpr::value(
                vec![Expr::list(list)],
                vec![ValueType::List(Box::new(ValueType::Int))],
            ),
            1,
            vec![ValueType::Int],
        );

        assert_eq!(
            run_module_tuple_expression(expression),
            ExecutionError::TupleIndexFamilyMismatch {
                expected: ValueType::Tuple(vec![ValueType::Int]),
                actual: ValueType::Tuple(vec![ValueType::List(Box::new(ValueType::Int))]),
            },
        );
    }

    fn run_module_tuple_expression(expression: TupleExpr) -> ExecutionError {
        let main = FunctionPlan::new(
            FunctionId::new(0),
            "main".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::tuple(TupleFunctionId(0), expression),
        );
        let module = ModulePlan::new("main".into(), main, Vec::new());
        let plan = crate::ExecutionPlan::from_module_plan(module);

        run_main(&plan).expect_err("module expression should fail at runtime")
    }
}
