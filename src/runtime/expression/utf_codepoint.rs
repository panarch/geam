use super::{
    eval_bool_expr, eval_custom_field, eval_float_expr, eval_int_expr, eval_panic_expr,
    eval_string_expr, project_tuple_expr, project_utf_codepoint_list_expr,
};
use crate::plan::ValueType;
use crate::plan::execution::{ExecutionPlan, UtfCodepointExpr, UtfCodepointExprKind};
use crate::runtime::evaluated::EvaluatedValue;
use crate::runtime::frame::Frame;
use crate::runtime::state::RuntimeState;
use crate::runtime::{ExecutionError, function};

pub(in crate::runtime) fn eval_utf_codepoint_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &UtfCodepointExpr,
) -> Result<char, ExecutionError> {
    match expression.kind() {
        UtfCodepointExprKind::LocalGet { local } => Ok(frame.get_utf_codepoint(*local)),
        UtfCodepointExprKind::Call { function, args } => {
            function::run_utf_codepoint_call(plan, state, *function, args, frame)
        }
        UtfCodepointExprKind::FunctionCall { function, args } => {
            function::run_utf_codepoint_function_call(plan, state, function, args, frame)
        }
        UtfCodepointExprKind::TupleIndex { tuple, index } => {
            match project_tuple_expr(plan, state, frame, tuple, *index, ValueType::UtfCodepoint)? {
                EvaluatedValue::UtfCodepoint(value) => Ok(value),
                other => Err(ExecutionError::TupleIndexFamilyMismatch {
                    expected: ValueType::UtfCodepoint,
                    actual: other.value_type(plan),
                }),
            }
        }
        UtfCodepointExprKind::CustomField(access) => {
            let (constructor, value) = eval_custom_field(plan, state, frame, access)?;
            match value {
                EvaluatedValue::UtfCodepoint(value) => Ok(value),
                other => {
                    let descriptor = plan.custom_constructor(constructor);
                    Err(ExecutionError::CustomFieldFamilyMismatch {
                        custom_type: plan.custom_value_type(constructor.type_id()),
                        constructor: descriptor.name().clone(),
                        field_index: access.index(),
                        expected: ValueType::UtfCodepoint,
                        actual: other.value_type(plan),
                    })
                }
            }
        }
        UtfCodepointExprKind::ListIndex { list, index } => {
            project_utf_codepoint_list_expr(plan, state, frame, list, *index)
        }
        UtfCodepointExprKind::Panic(panic) => {
            eval_panic_expr(plan, state, frame, panic).map(|never| match never {})
        }
        UtfCodepointExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, state, frame, subject)? {
                eval_utf_codepoint_expr(plan, state, frame, true_)
            } else {
                eval_utf_codepoint_expr(plan, state, frame, false_)
            }
        }
        UtfCodepointExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_utf_codepoint_expr(plan, state, frame, branch);
                }
            }
            eval_utf_codepoint_expr(plan, state, frame, fallback)
        }
        UtfCodepointExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_utf_codepoint_expr(plan, state, frame, branch);
                }
            }
            eval_utf_codepoint_expr(plan, state, frame, fallback)
        }
        UtfCodepointExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_utf_codepoint_expr(plan, state, frame, branch);
                }
            }
            eval_utf_codepoint_expr(plan, state, frame, fallback)
        }
        UtfCodepointExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, state, steps, frame)?;
            eval_utf_codepoint_expr(plan, state, frame, return_)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        BoolExpr, Expr, FloatExpr, FunctionTemplate, FunctionTemplateId, IntExpr, ModulePlan,
        PanicExpr, PanicSite, ReturnBody, ReturnExpr, Step, StringExpr, TupleExpr,
        UtfCodepointExpr, ValueType,
    };
    use crate::runtime::{BitArrayValue, ExecutionError, Value, run_main};

    #[test]
    fn source_utf_codepoint_expression_paths_evaluate_exact_values() {
        let bytes = [
            1, 2, 3, 11, 12, 3, 4, 5, 6, 7, 8, 9, 10, 14, 15, 16, 17, 18, 19, 20, 21, 22, 13,
        ];

        assert_eq!(
            crate::runtime::run_src(include_str!(
                "../../../tests/fixtures/execution/values/utf_codepoint_expression_paths.gleam"
            )),
            Value::Tuple(
                bytes
                    .into_iter()
                    .map(|byte| Value::BitArray(BitArrayValue::from_bytes(vec![byte])))
                    .collect(),
            ),
        );
    }

    #[test]
    fn module_expression_errors_propagate_through_utf_codepoint_wrappers() {
        let panic = || PanicExpr::panic_at(None, PanicSite::unknown());
        let fallback = || UtfCodepointExpr::panic(panic());
        let expressions = [
            UtfCodepointExpr::tuple_index(
                TupleExpr::panic(panic(), vec![ValueType::UtfCodepoint]),
                0,
            ),
            UtfCodepointExpr::bool_case(BoolExpr::panic(panic()), fallback(), fallback()),
            UtfCodepointExpr::int_case(IntExpr::panic(panic()), Vec::new(), fallback()),
            UtfCodepointExpr::string_case(StringExpr::panic(panic()), Vec::new(), fallback()),
            UtfCodepointExpr::float_case(FloatExpr::panic(panic()), Vec::new(), fallback()),
            UtfCodepointExpr::block(
                vec![Step::evaluate(Expr::bool(BoolExpr::panic(panic())))],
                fallback(),
            ),
        ];

        for expression in expressions {
            assert_eq!(
                run_module_utf_codepoint_expression(expression).to_string(),
                "panic: `panic` expression evaluated.",
            );
        }
    }

    fn run_module_utf_codepoint_expression(expression: UtfCodepointExpr) -> ExecutionError {
        let main = FunctionTemplate::new(
            FunctionTemplateId::new(0),
            "main".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::utf_codepoint_body(ReturnBody::expr(expression)),
        );
        let module = ModulePlan::new("main".into(), main, Vec::new());
        let plan = crate::ExecutionPlan::from_module_plan(module);

        run_main(&plan).expect_err("module expression should fail at runtime")
    }
}
