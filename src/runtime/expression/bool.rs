use super::{
    eval_bit_array_expr, eval_custom_expr, eval_expr, eval_float_expr, eval_int_expr,
    eval_list_expr, eval_panic_expr, eval_string_expr, project_bool_list_expr, project_tuple_expr,
};
use crate::plan::ValueType;
use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::{BoolExpr, BoolExprKind};
use crate::runtime::ExecutionError;
use crate::runtime::evaluated::EvaluatedValue;
use crate::runtime::frame::Frame;
use crate::runtime::function;
use crate::runtime::state::RuntimeState;

pub(in crate::runtime) fn eval_bool_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &BoolExpr,
) -> Result<bool, ExecutionError> {
    match expression.kind() {
        BoolExprKind::Value(value) => Ok(*value),
        BoolExprKind::LocalGet { local, .. } => Ok(frame.get_bool(*local)),
        BoolExprKind::Call { function, args } => {
            function::run_bool_call(plan, state, *function, args, frame)
        }
        BoolExprKind::FunctionCall { function, args } => {
            function::run_bool_function_call(plan, state, function, args, frame)
        }
        BoolExprKind::TupleIndex { tuple, index } => {
            match project_tuple_expr(plan, state, frame, tuple, *index, ValueType::Bool)? {
                EvaluatedValue::Bool(value) => Ok(value),
                other => Err(ExecutionError::TupleIndexFamilyMismatch {
                    expected: ValueType::Bool,
                    actual: other.value_type(plan),
                }),
            }
        }
        BoolExprKind::ListIndex { list, index } => {
            project_bool_list_expr(plan, state, frame, list, *index)
        }
        BoolExprKind::Panic(panic) => {
            eval_panic_expr(plan, state, frame, panic).map(|never| match never {})
        }
        BoolExprKind::Not(value) => Ok(!eval_bool_expr(plan, state, frame, value)?),
        BoolExprKind::LtInt { left, right } => Ok(
            eval_int_expr(plan, state, frame, left)? < eval_int_expr(plan, state, frame, right)?
        ),
        BoolExprKind::LtEqInt { left, right } => {
            Ok(eval_int_expr(plan, state, frame, left)?
                <= eval_int_expr(plan, state, frame, right)?)
        }
        BoolExprKind::GtInt { left, right } => Ok(
            eval_int_expr(plan, state, frame, left)? > eval_int_expr(plan, state, frame, right)?
        ),
        BoolExprKind::GtEqInt { left, right } => {
            Ok(eval_int_expr(plan, state, frame, left)?
                >= eval_int_expr(plan, state, frame, right)?)
        }
        BoolExprKind::LtFloat { left, right } => Ok(eval_float_expr(plan, state, frame, left)?
            < eval_float_expr(plan, state, frame, right)?),
        BoolExprKind::LtEqFloat { left, right } => Ok(eval_float_expr(plan, state, frame, left)?
            <= eval_float_expr(plan, state, frame, right)?),
        BoolExprKind::GtFloat { left, right } => Ok(eval_float_expr(plan, state, frame, left)?
            > eval_float_expr(plan, state, frame, right)?),
        BoolExprKind::GtEqFloat { left, right } => Ok(eval_float_expr(plan, state, frame, left)?
            >= eval_float_expr(plan, state, frame, right)?),
        BoolExprKind::Equal { left, right } => {
            let left = eval_expr(plan, state, frame, left)?;
            let right = eval_expr(plan, state, frame, right)?;
            Ok(crate::runtime::evaluated::values_equal(
                plan, state, &left, &right,
            ))
        }
        BoolExprKind::NotEqual { left, right } => {
            let left = eval_expr(plan, state, frame, left)?;
            let right = eval_expr(plan, state, frame, right)?;
            Ok(!crate::runtime::evaluated::values_equal(
                plan, state, &left, &right,
            ))
        }
        BoolExprKind::StringStartsWith { value, prefix } => {
            Ok(eval_string_expr(plan, state, frame, value)?.starts_with(prefix.as_str()))
        }
        BoolExprKind::ListLengthEquals { value, length } => {
            let value = eval_list_expr(plan, state, frame, value)?;
            Ok(state.list_len(&value) == *length)
        }
        BoolExprKind::ListLengthAtLeast { value, length } => {
            let value = eval_list_expr(plan, state, frame, value)?;
            Ok(state.list_len(&value) >= *length)
        }
        BoolExprKind::BitArrayMatches { value, pattern } => {
            let value = eval_bit_array_expr(plan, state, frame, value)?;
            Ok(crate::runtime::pattern::match_bit_array_pattern(
                frame, &value, pattern,
            ))
        }
        BoolExprKind::CustomMatches { value, pattern } => {
            let value = eval_custom_expr(plan, state, frame, value)?;
            crate::runtime::function::match_and_apply_assert_pattern(
                plan,
                state,
                frame,
                pattern,
                &EvaluatedValue::Custom(value),
            )
        }
        BoolExprKind::And { left, right } => {
            let left = eval_bool_expr(plan, state, frame, left)?;
            eval_and(left, || eval_bool_expr(plan, state, frame, right))
        }
        BoolExprKind::Or { left, right } => {
            let left = eval_bool_expr(plan, state, frame, left)?;
            eval_or(left, || eval_bool_expr(plan, state, frame, right))
        }
        BoolExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, state, frame, subject)? {
                eval_bool_expr(plan, state, frame, true_)
            } else {
                eval_bool_expr(plan, state, frame, false_)
            }
        }
        BoolExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_bool_expr(plan, state, frame, branch);
                }
            }
            eval_bool_expr(plan, state, frame, fallback)
        }
        BoolExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_bool_expr(plan, state, frame, branch);
                }
            }
            eval_bool_expr(plan, state, frame, fallback)
        }
        BoolExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_bool_expr(plan, state, frame, branch);
                }
            }
            eval_bool_expr(plan, state, frame, fallback)
        }
        BoolExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, state, steps, frame)?;
            eval_bool_expr(plan, state, frame, return_)
        }
    }
}

fn eval_and(
    left: bool,
    right: impl FnOnce() -> Result<bool, ExecutionError>,
) -> Result<bool, ExecutionError> {
    if left { right() } else { Ok(false) }
}

fn eval_or(
    left: bool,
    right: impl FnOnce() -> Result<bool, ExecutionError>,
) -> Result<bool, ExecutionError> {
    if left { Ok(true) } else { right() }
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        AssertPattern, BitArrayExpr, BitArrayPattern, BoolExpr, BoolFunctionId, CustomExpr,
        CustomType, CustomTypeDefinition, CustomTypeName, CustomTypePublicity, Expr, FloatExpr,
        FunctionId, FunctionPlan, IntExpr, ListExpr, ModulePlan, PanicExpr, PanicSite, ReturnExpr,
        Step, StringExpr, TupleExpr, ValueType,
    };
    use crate::runtime::{ExecutionError, run_main};

    #[test]
    fn source_bool_expression_variants_evaluate_exact_values() {
        let source = r#"
fn invert(value: Bool) -> Bool { !value }

pub fn main() {
  let local = True
  let function = invert
  #(
    local,
    invert(True),
    function(False),
    #(True).0,
    case [True] { [value] -> value _ -> False },
    !False,
    1 < 2,
    1 <= 1,
    2 > 1,
    2 >= 2,
    1.0 <. 2.0,
    1.0 <=. 1.0,
    2.0 >. 1.0,
    2.0 >=. 2.0,
    #(1, "one") == #(1, "one"),
    [1] != [2],
    case "prefix-rest" { "prefix-" <> _ -> True _ -> False },
    case [1, 2] { [_, _] -> True _ -> False },
    case [1, 2] { [_, ..] -> True _ -> False },
    True && True,
    False && True,
    True || False,
    False || True,
    case True { True -> True False -> False },
    case False { True -> True False -> False },
    case 1 { 1 -> True _ -> False },
    case 2 { 1 -> True _ -> False },
    case "one" { "one" -> True _ -> False },
    case "two" { "one" -> True _ -> False },
    case 1.0 { 1.0 -> True _ -> False },
    case 2.0 { 1.0 -> True _ -> False },
    { let _ = 0 True },
  )
}
"#;

        assert_eq!(
            crate::runtime::run_src(source),
            crate::runtime::Value::Tuple(
                vec![
                    true, false, true, true, true, true, true, true, true, true, true, true, true,
                    true, true, true, true, true, true, true, false, true, true, true, false, true,
                    false, true, false, true, false, true,
                ]
                .into_iter()
                .map(crate::runtime::Value::Bool)
                .collect(),
            ),
        );
    }

    #[test]
    fn source_operand_errors_propagate_through_bool_expressions() {
        let expressions = [
            "!fail_bool()",
            "fail_int() < 1",
            "1 < fail_int()",
            "fail_int() <= 1",
            "1 <= fail_int()",
            "fail_int() > 1",
            "1 > fail_int()",
            "fail_int() >= 1",
            "1 >= fail_int()",
            "fail_float() <. 1.0",
            "1.0 <. fail_float()",
            "fail_float() <=. 1.0",
            "1.0 <=. fail_float()",
            "fail_float() >. 1.0",
            "1.0 >. fail_float()",
            "fail_float() >=. 1.0",
            "1.0 >=. fail_float()",
            "fail_int() == 1",
            "1 == fail_int()",
            "fail_int() != 1",
            "1 != fail_int()",
            "case fail_string() { \"prefix\" <> _ -> True _ -> False }",
            "case fail_int_list() { [_, _] -> True _ -> False }",
            "case fail_int_list() { [_, ..] -> True _ -> False }",
            "fail_bool() && True",
            "True && fail_bool()",
            "fail_bool() || False",
            "False || fail_bool()",
            "case fail_bool() { True -> True False -> False }",
            "case fail_int() { 0 -> False _ -> True }",
            "case fail_string() { \"zero\" -> False _ -> True }",
            "case fail_float() { 0.0 -> False _ -> True }",
            "case fail_bit_array() { <<1>> -> True _ -> False }",
            "case fail_custom() { Full(_) -> True Empty -> False }",
            "{ let _ = fail_int() True }",
            "{ let function = fail_bool function() }",
        ];

        for expression in expressions {
            let source = format!(
                r#"
fn fail_bool() -> Bool {{ panic }}
fn fail_int() -> Int {{ panic }}
fn fail_string() -> String {{ panic }}
fn fail_float() -> Float {{ panic }}
fn fail_bit_array() -> BitArray {{ panic }}
fn fail_int_list() -> List(Int) {{ panic }}
pub type Choice {{ Empty Full(Int) }}
fn fail_custom() -> Choice {{ panic }}
pub fn main() -> Bool {{ {expression} }}
"#,
            );

            assert_eq!(
                crate::runtime::run_src_error(&source).to_string(),
                "panic: `panic` expression evaluated.",
            );
        }
    }

    #[test]
    fn module_expression_errors_propagate_through_bool_wrappers() {
        let panic = || PanicExpr::panic_at(None, PanicSite::unknown());
        let expressions = [
            BoolExpr::tuple_index(TupleExpr::panic(panic(), vec![ValueType::Bool]), 0),
            BoolExpr::string_starts_with(StringExpr::panic(panic()), "prefix".into()),
            BoolExpr::list_length_equals(ListExpr::panic(panic(), ValueType::Int), 1),
            BoolExpr::list_length_at_least(ListExpr::panic(panic(), ValueType::Int), 1),
            BoolExpr::bit_array_matches(
                BitArrayExpr::panic(panic()),
                BitArrayPattern::new(Vec::new()),
            ),
            BoolExpr::bool_case(
                BoolExpr::panic(panic()),
                BoolExpr::value(true),
                BoolExpr::value(false),
            ),
            BoolExpr::int_case(IntExpr::panic(panic()), Vec::new(), BoolExpr::value(false)),
            BoolExpr::string_case(
                StringExpr::panic(panic()),
                Vec::new(),
                BoolExpr::value(false),
            ),
            BoolExpr::float_case(
                FloatExpr::panic(panic()),
                Vec::new(),
                BoolExpr::value(false),
            ),
            BoolExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::panic(panic())))],
                BoolExpr::value(false),
            ),
        ];

        for expression in expressions {
            assert_eq!(
                run_module_bool_expression(expression, Vec::new()).to_string(),
                "panic: `panic` expression evaluated.",
            );
        }

        let custom_name = CustomTypeName::new("geam".into(), "main".into(), "Empty".into());
        let custom_type = CustomType::new(custom_name.clone(), Vec::new());
        let custom_definition = CustomTypeDefinition::new(
            custom_name,
            CustomTypePublicity::Public,
            false,
            Vec::new(),
            Vec::new(),
        );
        assert_eq!(
            run_module_bool_expression(
                BoolExpr::custom_matches(
                    CustomExpr::panic(panic(), custom_type),
                    AssertPattern::Discard,
                ),
                vec![custom_definition],
            )
            .to_string(),
            "panic: `panic` expression evaluated.",
        );
    }

    fn run_module_bool_expression(
        expression: BoolExpr,
        custom_types: Vec<CustomTypeDefinition>,
    ) -> ExecutionError {
        let main = FunctionPlan::new(
            FunctionId::new(0),
            "main".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::bool(BoolFunctionId(0), expression),
        );
        let module =
            ModulePlan::new("main".into(), main, Vec::new()).with_custom_types(custom_types);
        let plan = crate::ExecutionPlan::from_module_plan(module);

        run_main(&plan).expect_err("module expression should fail at runtime")
    }
}
