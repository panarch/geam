use super::{
    eval_bool_expr, eval_custom_field, eval_float_expr, eval_panic_expr, eval_string_expr,
    project_int_list_expr, project_tuple_expr,
};
use crate::plan::ValueType;
use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::{IntExpr, IntExprKind};
use crate::runtime::ExecutionError;
use crate::runtime::evaluated::EvaluatedValue;
use crate::runtime::frame::Frame;
use crate::runtime::function;
use crate::runtime::state::RuntimeState;
use num_bigint::BigInt;

pub(in crate::runtime) fn eval_int_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &IntExpr,
) -> Result<BigInt, ExecutionError> {
    match expression.kind() {
        IntExprKind::Value(value) => Ok(value.clone()),
        IntExprKind::LocalGet { local, .. } => Ok(frame.get_int(*local)),
        IntExprKind::Call { function, args } => {
            function::run_int_call(plan, state, *function, args, frame)
        }
        IntExprKind::FunctionCall { function, args } => {
            function::run_int_function_call(plan, state, function, args, frame)
        }
        IntExprKind::TupleIndex { tuple, index } => {
            match project_tuple_expr(plan, state, frame, tuple, *index, ValueType::Int)? {
                EvaluatedValue::Int(value) => Ok(value),
                other => Err(ExecutionError::TupleIndexFamilyMismatch {
                    expected: ValueType::Int,
                    actual: other.value_type(plan),
                }),
            }
        }
        IntExprKind::CustomField(access) => {
            let (constructor, value) = eval_custom_field(plan, state, frame, access)?;
            match value {
                EvaluatedValue::Int(value) => Ok(value),
                other => {
                    let descriptor = plan.custom_constructor(constructor);
                    Err(ExecutionError::CustomFieldFamilyMismatch {
                        custom_type: plan.custom_value_type(constructor.type_id()),
                        constructor: descriptor.name().clone(),
                        field_index: access.index(),
                        expected: ValueType::Int,
                        actual: other.value_type(plan),
                    })
                }
            }
        }
        IntExprKind::ListIndex { list, index } => {
            project_int_list_expr(plan, state, frame, list, *index)
        }
        IntExprKind::Panic(panic) => {
            eval_panic_expr(plan, state, frame, panic).map(|never| match never {})
        }
        IntExprKind::Add { left, right } => Ok(
            eval_int_expr(plan, state, frame, left)? + eval_int_expr(plan, state, frame, right)?
        ),
        IntExprKind::Sub { left, right } => Ok(
            eval_int_expr(plan, state, frame, left)? - eval_int_expr(plan, state, frame, right)?
        ),
        IntExprKind::Mult { left, right } => Ok(
            eval_int_expr(plan, state, frame, left)? * eval_int_expr(plan, state, frame, right)?
        ),
        IntExprKind::Div { left, right } => Ok(eval_div_int(
            eval_int_expr(plan, state, frame, left)?,
            eval_int_expr(plan, state, frame, right)?,
        )),
        IntExprKind::Remainder { left, right } => Ok(eval_remainder_int(
            eval_int_expr(plan, state, frame, left)?,
            eval_int_expr(plan, state, frame, right)?,
        )),
        IntExprKind::Negate(value) => Ok(-eval_int_expr(plan, state, frame, value)?),
        IntExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, state, frame, subject)? {
                eval_int_expr(plan, state, frame, true_)
            } else {
                eval_int_expr(plan, state, frame, false_)
            }
        }
        IntExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_int_expr(plan, state, frame, branch);
                }
            }
            eval_int_expr(plan, state, frame, fallback)
        }
        IntExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_int_expr(plan, state, frame, branch);
                }
            }
            eval_int_expr(plan, state, frame, fallback)
        }
        IntExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_int_expr(plan, state, frame, branch);
                }
            }
            eval_int_expr(plan, state, frame, fallback)
        }
        IntExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, state, steps, frame)?;
            eval_int_expr(plan, state, frame, return_)
        }
    }
}

fn eval_div_int(left: BigInt, right: BigInt) -> BigInt {
    // Gleam defines Int division by zero as 0 across its targets.
    if right == BigInt::from(0) {
        return BigInt::from(0);
    }

    left / right
}

fn eval_remainder_int(left: BigInt, right: BigInt) -> BigInt {
    // Gleam defines Int remainder by zero as 0 across its targets.
    if right == BigInt::from(0) {
        return BigInt::from(0);
    }

    left % right
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        BoolExpr, Expr, FloatExpr, FunctionTemplate, FunctionTemplateId, IntExpr, IntFunctionId,
        ModulePlan, PanicExpr, PanicSite, ReturnExpr, Step, StringExpr, TupleExpr, ValueType,
    };
    use crate::runtime::{ExecutionError, run_main};
    use num_bigint::BigInt;

    #[test]
    fn source_int_expression_variants_evaluate_exact_values() {
        let source = r#"
fn add_one(value: Int) -> Int { value + 1 }

pub fn main() {
  let local = 1
  let function = add_one
  #(
    local,
    add_one(1),
    function(1),
    #(3).0,
    case [4] { [value] -> value _ -> 0 },
    1 + 2,
    5 - 2,
    2 * 3,
    7 / 2,
    7 / 0,
    7 % 3,
    7 % 0,
    -1,
    case True { True -> 1 False -> 0 },
    case False { True -> 1 False -> 0 },
    case 1 { 1 -> 2 _ -> 0 },
    case 2 { 1 -> 2 _ -> 3 },
    case "one" { "one" -> 1 _ -> 0 },
    case "two" { "one" -> 1 _ -> 2 },
    case 1.0 { 1.0 -> 1 _ -> 0 },
    case 2.0 { 1.0 -> 1 _ -> 2 },
    { let _ = 0 4 },
  )
}
"#;

        assert_eq!(
            crate::runtime::run_src(source),
            crate::runtime::Value::Tuple(
                vec![
                    1_i64, 2, 2, 3, 4, 3, 3, 6, 3, 0, 1, 0, -1, 1, 0, 2, 3, 1, 2, 1, 2, 4
                ]
                .into_iter()
                .map(|value| crate::runtime::Value::Int(value.into()))
                .collect(),
            ),
        );
    }

    #[test]
    fn source_operand_errors_propagate_through_int_expressions() {
        let expressions = [
            "fail_int() + 1",
            "1 + fail_int()",
            "fail_int() - 1",
            "1 - fail_int()",
            "fail_int() * 1",
            "1 * fail_int()",
            "fail_int() / 1",
            "1 / fail_int()",
            "fail_int() % 1",
            "1 % fail_int()",
            "-fail_int()",
            "case fail_bool() { True -> 1 False -> 0 }",
            "case fail_int() { 0 -> 0 _ -> 1 }",
            "case fail_string() { \"zero\" -> 0 _ -> 1 }",
            "case fail_float() { 0.0 -> 0 _ -> 1 }",
            "{ let _ = fail_bool() 1 }",
            "{ let function = fail_int function() }",
        ];

        for expression in expressions {
            let source = format!(
                r#"
fn fail_bool() -> Bool {{ panic }}
fn fail_int() -> Int {{ panic }}
fn fail_string() -> String {{ panic }}
fn fail_float() -> Float {{ panic }}
pub fn main() -> Int {{ {expression} }}
"#,
            );

            assert_eq!(
                crate::runtime::run_src_error(&source).to_string(),
                "panic: `panic` expression evaluated.",
            );
        }
    }

    #[test]
    fn module_expression_errors_propagate_through_int_wrappers() {
        let panic = || PanicExpr::panic_at(None, PanicSite::unknown());
        let expressions = [
            IntExpr::tuple_index(TupleExpr::panic(panic(), vec![ValueType::Int]), 0),
            IntExpr::bool_case(
                BoolExpr::panic(panic()),
                IntExpr::value(BigInt::from(1)),
                IntExpr::value(BigInt::from(0)),
            ),
            IntExpr::int_case(
                IntExpr::panic(panic()),
                Vec::new(),
                IntExpr::value(BigInt::from(0)),
            ),
            IntExpr::string_case(
                StringExpr::panic(panic()),
                Vec::new(),
                IntExpr::value(BigInt::from(0)),
            ),
            IntExpr::float_case(
                FloatExpr::panic(panic()),
                Vec::new(),
                IntExpr::value(BigInt::from(0)),
            ),
            IntExpr::block(
                vec![Step::evaluate(Expr::bool(BoolExpr::panic(panic())))],
                IntExpr::value(BigInt::from(0)),
            ),
        ];

        for expression in expressions {
            assert_eq!(
                run_module_int_expression(expression).to_string(),
                "panic: `panic` expression evaluated.",
            );
        }
    }

    fn run_module_int_expression(expression: IntExpr) -> ExecutionError {
        let main = FunctionTemplate::new(
            FunctionTemplateId::new(0),
            "main".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::int(IntFunctionId(0), expression),
        );
        let module = ModulePlan::new("main".into(), main, Vec::new());
        let plan = crate::ExecutionPlan::from_module_plan(module);

        run_main(&plan).expect_err("module expression should fail at runtime")
    }
}
