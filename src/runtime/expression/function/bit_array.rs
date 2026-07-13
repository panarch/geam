use crate::plan::ValueType;
use crate::plan::execution::{
    BitArrayFunctionExpr, BitArrayFunctionExprKind, ExecutionPlan, FunctionReturnFamily,
};
use crate::runtime::expression::{
    eval_bool_expr, eval_float_expr, eval_int_expr, eval_panic_expr, eval_string_expr,
    project_function_list_expr, project_tuple_expr,
};
use crate::runtime::frame::Frame;
use crate::runtime::function;
use crate::runtime::state::RuntimeState;
use crate::runtime::{
    EvaluatedBitArrayFunction, EvaluatedFunctionValueKind, EvaluatedValue, ExecutionError,
};

pub(in crate::runtime) fn eval_bit_array_function_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &BitArrayFunctionExpr,
) -> Result<EvaluatedBitArrayFunction, ExecutionError> {
    match expression.kind() {
        BitArrayFunctionExprKind::Reference(reference) => Ok(EvaluatedBitArrayFunction::new(
            *reference.function(),
            reference.params().to_vec(),
            Vec::new(),
            crate::runtime::evaluated::function_type(
                reference.params(),
                crate::plan::execution::ValueType::BitArray,
            ),
        )),
        BitArrayFunctionExprKind::Closure(template) => Ok(EvaluatedBitArrayFunction::new(
            *template.function(),
            template.params().to_vec(),
            function::eval_capture_args(plan, state, frame, template.captures())?,
            crate::runtime::evaluated::function_type(
                template.params(),
                crate::plan::execution::ValueType::BitArray,
            ),
        )),
        BitArrayFunctionExprKind::LocalGet { local, .. } => {
            Ok(frame.get_bit_array_function(*local))
        }
        BitArrayFunctionExprKind::Call { function, args, .. } => {
            function::run_bit_array_function_returning_function_call(
                plan, state, *function, args, frame,
            )
        }
        BitArrayFunctionExprKind::FunctionCall {
            function: callee,
            args,
            ..
        } => function::run_bit_array_function_function_call(
            plan,
            state,
            callee.as_ref(),
            args,
            frame,
        ),
        BitArrayFunctionExprKind::TupleIndex {
            tuple,
            index,
            type_,
        } => {
            let expected = ValueType::Function(Box::new(plan.function_type(type_)));
            let value = project_tuple_expr(plan, state, frame, tuple, *index, expected.clone())?;
            let actual = value.value_type(plan);
            match value {
                EvaluatedValue::Function(function) => match function.kind() {
                    EvaluatedFunctionValueKind::BitArray(value) => Ok(value.clone()),
                    _ => Err(ExecutionError::TupleIndexFamilyMismatch { expected, actual }),
                },
                _ => Err(ExecutionError::TupleIndexFamilyMismatch { expected, actual }),
            }
        }
        BitArrayFunctionExprKind::ListIndex { list, index, type_ } => {
            let type_ = plan.function_type(type_);
            let function = project_function_list_expr(plan, state, frame, list, *index, &type_)?;
            match function.kind() {
                EvaluatedFunctionValueKind::BitArray(value) => Ok(value.clone()),
                _ => Err(ExecutionError::FunctionReturnFamilyMismatch {
                    expected: FunctionReturnFamily::BitArray,
                    actual: function.kind().family(),
                }),
            }
        }
        BitArrayFunctionExprKind::Panic(panic) => {
            eval_panic_expr(plan, state, frame, panic).map(|never| match never {})
        }
        BitArrayFunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, state, frame, subject)? {
                eval_bit_array_function_expr(plan, state, frame, true_)
            } else {
                eval_bit_array_function_expr(plan, state, frame, false_)
            }
        }
        BitArrayFunctionExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_bit_array_function_expr(plan, state, frame, branch);
                }
            }
            eval_bit_array_function_expr(plan, state, frame, fallback)
        }
        BitArrayFunctionExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_bit_array_function_expr(plan, state, frame, branch);
                }
            }
            eval_bit_array_function_expr(plan, state, frame, fallback)
        }
        BitArrayFunctionExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_bit_array_function_expr(plan, state, frame, branch);
                }
            }
            eval_bit_array_function_expr(plan, state, frame, fallback)
        }
        BitArrayFunctionExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, state, steps, frame)?;
            eval_bit_array_function_expr(plan, state, frame, return_)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        BitArrayFunctionExpr, BitArrayFunctionFunctionId, BitArrayFunctionId, BoolExpr, CaptureArg,
        Expr, FloatExpr, FunctionId, FunctionPlan, FunctionType, IntExpr, IntLocalId, ListExpr,
        ModulePlan, PanicExpr, PanicSite, ReturnExpr, Step, StringExpr, TupleExpr, ValueType,
    };
    use crate::runtime::{ExecutionError, run_main};

    #[test]
    fn module_expression_errors_propagate_through_bit_array_function_wrappers() {
        let type_ = FunctionType::new(Vec::new(), ValueType::BitArray);
        let panic = |message: &str| {
            PanicExpr::panic_at(
                Some(StringExpr::value(message.into())),
                PanicSite::unknown(),
            )
        };
        let fallback = || BitArrayFunctionExpr::panic(panic("fallback"), type_.clone());
        let expressions = [
            (
                BitArrayFunctionExpr::closure(
                    BitArrayFunctionId(1),
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
                BitArrayFunctionExpr::tuple_index(
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
                BitArrayFunctionExpr::list_index(
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
                BitArrayFunctionExpr::bool_case(
                    BoolExpr::panic(panic("bool subject")),
                    fallback(),
                    fallback(),
                ),
                "bool subject",
            ),
            (
                BitArrayFunctionExpr::int_case(
                    IntExpr::panic(panic("int subject")),
                    Vec::new(),
                    fallback(),
                ),
                "int subject",
            ),
            (
                BitArrayFunctionExpr::string_case(
                    StringExpr::panic(panic("string subject")),
                    Vec::new(),
                    fallback(),
                ),
                "string subject",
            ),
            (
                BitArrayFunctionExpr::float_case(
                    FloatExpr::panic(panic("float subject")),
                    Vec::new(),
                    fallback(),
                ),
                "float subject",
            ),
            (
                BitArrayFunctionExpr::block(
                    vec![Step::evaluate(Expr::int(IntExpr::panic(panic("step"))))],
                    fallback(),
                ),
                "step",
            ),
        ];

        for (expression, message) in expressions {
            assert_eq!(
                run_module_bit_array_function_expression(expression).to_string(),
                format!("panic: {message}"),
            );
        }
    }

    #[test]
    fn bit_array_function_tuple_projection_reports_direct_mutated_family_mismatch() {
        let type_ = FunctionType::new(Vec::new(), ValueType::BitArray);
        let tuple = TupleExpr::value(
            vec![Expr::function(crate::plan::FunctionExpr::int(
                crate::plan::IntFunctionExpr::reference(crate::plan::IntFunctionReference::new(
                    crate::plan::IntFunctionId(0),
                    Vec::new(),
                )),
            ))],
            vec![ValueType::Function(Box::new(type_.clone()))],
        );
        let expression = BitArrayFunctionExpr::tuple_index(tuple, 0, type_);

        assert_eq!(
            run_module_bit_array_function_expression(expression),
            ExecutionError::TupleIndexFamilyMismatch {
                expected: ValueType::Function(Box::new(FunctionType::new(
                    Vec::new(),
                    ValueType::BitArray,
                ))),
                actual: ValueType::Function(Box::new(FunctionType::new(
                    Vec::new(),
                    ValueType::Int,
                ))),
            },
        );

        let tuple = TupleExpr::value(
            vec![Expr::int(IntExpr::value(1.into()))],
            vec![ValueType::Function(Box::new(FunctionType::new(
                Vec::new(),
                ValueType::BitArray,
            )))],
        );
        let expression = BitArrayFunctionExpr::tuple_index(
            tuple,
            0,
            FunctionType::new(Vec::new(), ValueType::BitArray),
        );
        assert_eq!(
            run_module_bit_array_function_expression(expression),
            ExecutionError::TupleIndexFamilyMismatch {
                expected: ValueType::Function(Box::new(FunctionType::new(
                    Vec::new(),
                    ValueType::BitArray,
                ))),
                actual: ValueType::Int,
            },
        );
    }

    fn run_module_bit_array_function_expression(
        expression: BitArrayFunctionExpr,
    ) -> ExecutionError {
        let main = FunctionPlan::new(
            FunctionId::new(0),
            "main".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::bit_array_function(BitArrayFunctionFunctionId(0), expression),
        );
        let module = ModulePlan::new("main".into(), main, Vec::new());
        let plan = crate::ExecutionPlan::from_module_plan(module);

        run_main(&plan).expect_err("module expression should fail at runtime")
    }
}
