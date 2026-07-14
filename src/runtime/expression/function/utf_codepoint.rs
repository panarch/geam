use crate::plan::ValueType;
use crate::plan::execution::{
    ExecutionPlan, FunctionReturnFamily, UtfCodepointFunctionExpr, UtfCodepointFunctionExprKind,
};
use crate::runtime::expression::{
    eval_bool_expr, eval_float_expr, eval_int_expr, eval_panic_expr, eval_string_expr,
    project_function_list_expr, project_tuple_expr,
};
use crate::runtime::frame::Frame;
use crate::runtime::function;
use crate::runtime::state::RuntimeState;
use crate::runtime::{
    EvaluatedFunctionValueKind, EvaluatedUtfCodepointFunction, EvaluatedValue, ExecutionError,
};

pub(in crate::runtime) fn eval_utf_codepoint_function_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &UtfCodepointFunctionExpr,
) -> Result<EvaluatedUtfCodepointFunction, ExecutionError> {
    match expression.kind() {
        UtfCodepointFunctionExprKind::Reference(reference) => {
            Ok(EvaluatedUtfCodepointFunction::new(
                *reference.function(),
                reference.params().to_vec(),
                Vec::new(),
                crate::runtime::evaluated::function_type(
                    reference.params(),
                    crate::plan::execution::ValueType::UtfCodepoint,
                ),
            ))
        }
        UtfCodepointFunctionExprKind::Closure(template) => Ok(EvaluatedUtfCodepointFunction::new(
            *template.function(),
            template.params().to_vec(),
            function::eval_capture_args(plan, state, frame, template.captures())?,
            crate::runtime::evaluated::function_type(
                template.params(),
                crate::plan::execution::ValueType::UtfCodepoint,
            ),
        )),
        UtfCodepointFunctionExprKind::LocalGet { local, .. } => {
            Ok(frame.get_utf_codepoint_function(*local))
        }
        UtfCodepointFunctionExprKind::Call { function, args, .. } => {
            function::run_utf_codepoint_function_returning_function_call(
                plan, state, *function, args, frame,
            )
        }
        UtfCodepointFunctionExprKind::FunctionCall {
            function: callee,
            args,
            ..
        } => function::run_utf_codepoint_function_function_call(
            plan,
            state,
            callee.as_ref(),
            args,
            frame,
        ),
        UtfCodepointFunctionExprKind::TupleIndex {
            tuple,
            index,
            type_,
        } => {
            let expected = ValueType::Function(Box::new(plan.function_type(type_)));
            let value = project_tuple_expr(plan, state, frame, tuple, *index, expected.clone())?;
            let actual = value.value_type(plan);
            match value {
                EvaluatedValue::Function(function) => match function.kind() {
                    EvaluatedFunctionValueKind::UtfCodepoint(value) => Ok(value.clone()),
                    _ => Err(ExecutionError::TupleIndexFamilyMismatch { expected, actual }),
                },
                _ => Err(ExecutionError::TupleIndexFamilyMismatch { expected, actual }),
            }
        }
        UtfCodepointFunctionExprKind::ListIndex { list, index, type_ } => {
            let type_ = plan.function_type(type_);
            let function = project_function_list_expr(plan, state, frame, list, *index, &type_)?;
            match function.kind() {
                EvaluatedFunctionValueKind::UtfCodepoint(value) => Ok(value.clone()),
                _ => Err(ExecutionError::FunctionReturnFamilyMismatch {
                    expected: FunctionReturnFamily::UtfCodepoint,
                    actual: function.kind().family(),
                }),
            }
        }
        UtfCodepointFunctionExprKind::Panic(panic) => {
            eval_panic_expr(plan, state, frame, panic).map(|never| match never {})
        }
        UtfCodepointFunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, state, frame, subject)? {
                eval_utf_codepoint_function_expr(plan, state, frame, true_)
            } else {
                eval_utf_codepoint_function_expr(plan, state, frame, false_)
            }
        }
        UtfCodepointFunctionExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_utf_codepoint_function_expr(plan, state, frame, branch);
                }
            }
            eval_utf_codepoint_function_expr(plan, state, frame, fallback)
        }
        UtfCodepointFunctionExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_utf_codepoint_function_expr(plan, state, frame, branch);
                }
            }
            eval_utf_codepoint_function_expr(plan, state, frame, fallback)
        }
        UtfCodepointFunctionExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_utf_codepoint_function_expr(plan, state, frame, branch);
                }
            }
            eval_utf_codepoint_function_expr(plan, state, frame, fallback)
        }
        UtfCodepointFunctionExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, state, steps, frame)?;
            eval_utf_codepoint_function_expr(plan, state, frame, return_)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        BoolExpr, CaptureArg, Expr, FloatExpr, FunctionId, FunctionPlan, FunctionType, IntExpr,
        IntLocalId, ListExpr, ModulePlan, PanicExpr, PanicSite, ReturnBody, ReturnExpr, Step,
        StringExpr, TupleExpr, UtfCodepointFunctionExpr, UtfCodepointFunctionFunctionId,
        UtfCodepointFunctionId, ValueType,
    };
    use crate::runtime::{BitArrayValue, ExecutionError, Value, run_main};

    #[test]
    fn source_utf_codepoint_function_expression_paths_evaluate_exact_values() {
        let bytes = [
            1, 2, 3, 4, 24, 5, 6, 23, 25, 7, 99, 9, 99, 11, 99, 13, 99, 16, 99, 17, 99, 18, 99, 19,
            99, 15, 20, 21, 22,
        ];

        assert_eq!(
            crate::runtime::run_src(include_str!(
                "../../../../tests/fixtures/execution/values/utf_codepoint_function_value_paths.gleam"
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
    fn module_expression_errors_propagate_through_utf_codepoint_function_wrappers() {
        let type_ = FunctionType::new(Vec::new(), ValueType::UtfCodepoint);
        let panic = |message: &str| {
            PanicExpr::panic_at(
                Some(StringExpr::value(message.into())),
                PanicSite::unknown(),
            )
        };
        let fallback = || UtfCodepointFunctionExpr::panic(panic("fallback"), type_.clone());
        let expressions = [
            (
                UtfCodepointFunctionExpr::closure(
                    UtfCodepointFunctionId(1),
                    Vec::new(),
                    vec![CaptureArg::int(
                        IntLocalId(0),
                        IntExpr::panic(panic("capture")),
                    )],
                    type_.clone(),
                ),
                "capture",
            ),
            (
                UtfCodepointFunctionExpr::tuple_index(
                    TupleExpr::panic(
                        panic("tuple"),
                        vec![ValueType::Function(Box::new(type_.clone()))],
                    ),
                    0,
                    type_.clone(),
                ),
                "tuple",
            ),
            (
                UtfCodepointFunctionExpr::list_index(
                    super::super::expect_function_list(ListExpr::panic(
                        panic("list"),
                        ValueType::Function(Box::new(type_.clone())),
                    )),
                    0,
                    type_.clone(),
                ),
                "list",
            ),
            (
                UtfCodepointFunctionExpr::bool_case(
                    BoolExpr::panic(panic("bool subject")),
                    fallback(),
                    fallback(),
                ),
                "bool subject",
            ),
            (
                UtfCodepointFunctionExpr::int_case(
                    IntExpr::panic(panic("int subject")),
                    Vec::new(),
                    fallback(),
                ),
                "int subject",
            ),
            (
                UtfCodepointFunctionExpr::string_case(
                    StringExpr::panic(panic("string subject")),
                    Vec::new(),
                    fallback(),
                ),
                "string subject",
            ),
            (
                UtfCodepointFunctionExpr::float_case(
                    FloatExpr::panic(panic("float subject")),
                    Vec::new(),
                    fallback(),
                ),
                "float subject",
            ),
            (
                UtfCodepointFunctionExpr::block(
                    vec![Step::evaluate(Expr::int(IntExpr::panic(panic("step"))))],
                    fallback(),
                ),
                "step",
            ),
        ];

        for (expression, message) in expressions {
            assert_eq!(
                run_module_utf_codepoint_function_expression(expression).to_string(),
                format!("panic: {message}"),
            );
        }
    }

    fn run_module_utf_codepoint_function_expression(
        expression: UtfCodepointFunctionExpr,
    ) -> ExecutionError {
        let type_ = FunctionType::new(Vec::new(), ValueType::UtfCodepoint);
        let main = FunctionPlan::new(
            FunctionId::new(0),
            "main".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::utf_codepoint_function_body(
                UtfCodepointFunctionFunctionId(0),
                type_,
                ReturnBody::expr(expression),
            ),
        );
        let module = ModulePlan::new("main".into(), main, Vec::new());
        let plan = crate::ExecutionPlan::from_module_plan(module);

        run_main(&plan).expect_err("module expression should fail at runtime")
    }
}
