use super::eval_custom_field_function;
use crate::plan::ValueType;
use crate::plan::execution::{
    BitArrayFunctionExpr, BitArrayFunctionExprKind, ExecutionPlan, FunctionReturnFamily,
};
use crate::runtime::expression::{
    eval_bool_expr, eval_direct_call, eval_float_expr, eval_function_call, eval_int_expr,
    eval_panic_expr, eval_string_expr, project_function_list_expr, project_tuple_expr,
};
use crate::runtime::frame::Frame;
use crate::runtime::function;
use crate::runtime::state::RuntimeState;
use crate::runtime::{
    EvaluatedBitArrayFunction, EvaluatedFunctionValueKind, EvaluatedValue, ExecutionError,
    InvariantError,
};

pub(in crate::runtime) fn eval_bit_array_function_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &BitArrayFunctionExpr,
) -> Result<EvaluatedBitArrayFunction, ExecutionError> {
    match expression.kind() {
        BitArrayFunctionExprKind::Constant(value) => {
            eval_bit_array_function_expr(plan, state, frame, plan.constant(*value))
        }
        BitArrayFunctionExprKind::Reference(reference) => Ok(EvaluatedBitArrayFunction::reference(
            *reference.function(),
            reference.param_locals(),
            Vec::new(),
            crate::runtime::evaluated::function_type_from_slots(
                plan,
                reference.params(),
                crate::plan::execution::ValueType::BitArray,
            ),
        )),
        BitArrayFunctionExprKind::Closure(closure) => Ok(EvaluatedBitArrayFunction::closure(
            *closure.function(),
            closure.param_locals(),
            function::eval_capture_args(plan, state, frame, closure.captures())?,
            crate::runtime::evaluated::function_type_from_slots(
                plan,
                closure.params(),
                crate::plan::execution::ValueType::BitArray,
            ),
        )),
        BitArrayFunctionExprKind::LocalGet { local, .. } => {
            Ok(frame.get_bit_array_function(*local))
        }
        BitArrayFunctionExprKind::Call(call) => eval_direct_call(
            plan,
            state,
            frame,
            call,
            |plan, state, function, args, frame| {
                function::run_bit_array_function_returning_function_call(
                    plan, state, *function, args, frame,
                )
            },
        ),
        BitArrayFunctionExprKind::FunctionCall(call) => eval_function_call(
            plan,
            state,
            frame,
            call,
            function::run_bit_array_function_function_call,
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
                    _ => Err(ExecutionError::Invariant(
                        InvariantError::TupleIndexFamilyMismatch { expected, actual },
                    )),
                },
                _ => Err(ExecutionError::Invariant(
                    InvariantError::TupleIndexFamilyMismatch { expected, actual },
                )),
            }
        }
        BitArrayFunctionExprKind::CustomField(access) => {
            let (constructor, expected, function) =
                eval_custom_field_function(plan, state, frame, access)?;
            match function.kind() {
                EvaluatedFunctionValueKind::BitArray(value) => Ok(value.clone()),
                _ => {
                    let descriptor = plan.custom_constructor(constructor);
                    Err(ExecutionError::Invariant(
                        InvariantError::CustomFieldFamilyMismatch {
                            custom_type: plan.custom_value_type(constructor.type_id()),
                            constructor: descriptor.name().clone(),
                            field_index: access.index(),
                            expected,
                            actual: ValueType::Function(Box::new(
                                plan.function_type(function.type_()),
                            )),
                        },
                    ))
                }
            }
        }
        BitArrayFunctionExprKind::ListIndex { list, index, type_ } => {
            let type_ = plan.function_type(type_);
            let function = project_function_list_expr(plan, state, frame, list, *index, &type_)?;
            match function.kind() {
                EvaluatedFunctionValueKind::BitArray(value) => Ok(value.clone()),
                _ => Err(ExecutionError::Invariant(
                    InvariantError::FunctionReturnFamilyMismatch {
                        expected: FunctionReturnFamily::BitArray,
                        actual: function.kind().family(),
                    },
                )),
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
        BitArrayExpr, BitArrayFunctionExpr, BitArrayFunctionFunctionId, BitArrayFunctionId,
        BoolExpr, CaptureArg, Expr, FloatExpr, FunctionTemplate, FunctionTemplateId, FunctionType,
        IntExpr, IntLocalId, ListExpr, ModulePlan, PanicExpr, PanicSite, ReturnExpr, Step,
        StringExpr, TupleExpr, ValueType,
    };
    use crate::runtime::{BitArrayValue, ExecutionError, InvariantError, Value, run_main};

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
                    crate::plan::monomorphic_function_instantiation(
                        1,
                        crate::plan::FunctionShape::from_function_type(type_.clone()),
                    ),
                    vec![CaptureArg::new(crate::plan::Expr::int(IntExpr::panic(
                        panic("capture"),
                    )))],
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
                BitArrayFunctionExpr::bool_case(
                    BoolExpr::not(BoolExpr::value(false)),
                    BitArrayFunctionExpr::panic(panic("true branch"), type_.clone()),
                    fallback(),
                ),
                "true branch",
            ),
            (
                BitArrayFunctionExpr::bool_case(
                    BoolExpr::not(BoolExpr::value(true)),
                    fallback(),
                    BitArrayFunctionExpr::panic(panic("false branch"), type_.clone()),
                ),
                "false branch",
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
                    crate::plan::monomorphic_function_instantiation(
                        0,
                        crate::plan::FunctionShape::new(Vec::new(), crate::plan::ValueShape::Int),
                    ),
                )),
            ))],
            vec![ValueType::Function(Box::new(type_.clone()))],
        );
        let expression = BitArrayFunctionExpr::tuple_index(tuple, 0, type_);

        assert_eq!(
            run_module_bit_array_function_expression(expression),
            ExecutionError::Invariant(InvariantError::TupleIndexFamilyMismatch {
                expected: ValueType::Function(Box::new(FunctionType::new(
                    Vec::new(),
                    ValueType::BitArray,
                ))),
                actual: ValueType::Function(Box::new(FunctionType::new(
                    Vec::new(),
                    ValueType::Int,
                ))),
            }),
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
            ExecutionError::Invariant(InvariantError::TupleIndexFamilyMismatch {
                expected: ValueType::Function(Box::new(FunctionType::new(
                    Vec::new(),
                    ValueType::BitArray,
                ))),
                actual: ValueType::Int,
            }),
        );
    }

    #[test]
    fn source_function_value_paths_preserve_bit_array_calls() {
        assert_eq!(
            crate::runtime::run_src(include_str!(
                "../../../../tests/fixtures/execution/values/bit_array_function_value_paths.gleam"
            )),
            Value::Tuple(
                [
                    1, 2, 3, 4, 24, 5, 6, 23, 7, 99, 9, 99, 11, 99, 13, 99, 16, 99, 17, 99, 18, 99,
                    19, 99, 15, 20, 21, 22
                ]
                .into_iter()
                .map(|byte| Value::BitArray(BitArrayValue::from_bytes(vec![byte])))
                .collect(),
            ),
        );
    }

    fn run_module_bit_array_function_expression(
        expression: BitArrayFunctionExpr,
    ) -> ExecutionError {
        let target = FunctionTemplate::with_captures(
            FunctionTemplateId::new(1),
            "target".into(),
            Vec::new(),
            vec![crate::plan::ParamSlot::from_local(
                crate::plan::ParamLocal::int(IntLocalId(0)),
            )],
            vec![Step::evaluate(Expr::int(IntExpr::local_get(
                IntLocalId(0),
                "capture".into(),
            )))],
            ReturnExpr::bit_array(
                BitArrayFunctionId(0),
                BitArrayExpr::panic(PanicExpr::panic_at(None, PanicSite::unknown())),
            ),
        );
        let main = FunctionTemplate::new(
            FunctionTemplateId::new(0),
            "main".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::bit_array_function(BitArrayFunctionFunctionId(0), expression),
        );
        let module = ModulePlan::new("main".into(), main, vec![target]);
        let plan = crate::ExecutionPlan::from_module_plan(module);

        run_main(&plan).expect_err("module expression should fail at runtime")
    }
}
