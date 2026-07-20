use super::eval_custom_field_function;
use crate::plan::ValueType;
use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::{FunctionReturnFamily, TupleFunctionExpr, TupleFunctionExprKind};
use crate::runtime::expression::{
    eval_bool_expr, eval_direct_call, eval_float_expr, eval_function_call, eval_int_expr,
    eval_panic_expr, eval_string_expr, project_function_list_expr, project_tuple_expr,
};
use crate::runtime::frame::Frame;
use crate::runtime::function;
use crate::runtime::state::RuntimeState;
use crate::runtime::{
    EvaluatedFunctionValueKind, EvaluatedTupleFunction, EvaluatedValue, ExecutionError,
    InvariantError,
};

pub(in crate::runtime) fn eval_tuple_function_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &TupleFunctionExpr,
) -> Result<EvaluatedTupleFunction, ExecutionError> {
    match expression.kind() {
        TupleFunctionExprKind::Constant(value) => {
            eval_tuple_function_expr(plan, state, frame, plan.constant(*value))
        }
        TupleFunctionExprKind::Reference(reference) => Ok(EvaluatedTupleFunction::reference(
            *reference.function(),
            reference.param_locals(),
            Vec::new(),
            expression.type_().clone(),
        )),
        TupleFunctionExprKind::Closure(closure) => Ok(EvaluatedTupleFunction::closure(
            *closure.function(),
            closure.param_locals(),
            function::eval_capture_args(plan, state, frame, closure.captures())?,
            expression.type_().clone(),
        )),
        TupleFunctionExprKind::LocalGet { local, .. } => Ok(frame.get_tuple_function(*local)),
        TupleFunctionExprKind::Call(call) => eval_direct_call(
            plan,
            state,
            frame,
            call,
            |plan, state, function, args, frame| {
                function::run_tuple_function_returning_function_call(
                    plan, state, *function, args, frame,
                )
            },
        ),
        TupleFunctionExprKind::FunctionCall(call) => eval_function_call(
            plan,
            state,
            frame,
            call,
            function::run_tuple_function_function_call,
        ),
        TupleFunctionExprKind::TupleIndex {
            tuple,
            index,
            type_,
        } => {
            match project_tuple_expr(
                plan,
                state,
                frame,
                tuple,
                *index,
                ValueType::Function(Box::new(plan.function_type(type_))),
            )? {
                EvaluatedValue::Function(function) => match function.kind() {
                    crate::runtime::EvaluatedFunctionValueKind::Tuple(value) => Ok(value.clone()),
                    _ => Err(ExecutionError::Invariant(
                        InvariantError::TupleIndexFamilyMismatch {
                            expected: ValueType::Function(Box::new(plan.function_type(type_))),
                            actual: EvaluatedValue::Function(function).value_type(plan),
                        },
                    )),
                },
                other => Err(ExecutionError::Invariant(
                    InvariantError::TupleIndexFamilyMismatch {
                        expected: ValueType::Function(Box::new(plan.function_type(type_))),
                        actual: other.value_type(plan),
                    },
                )),
            }
        }
        TupleFunctionExprKind::CustomField(access) => {
            let (constructor, expected, function) =
                eval_custom_field_function(plan, state, frame, access)?;
            match function.kind() {
                EvaluatedFunctionValueKind::Tuple(value) => Ok(value.clone()),
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
        TupleFunctionExprKind::ListIndex { list, index, type_ } => {
            let type_ = plan.function_type(type_);
            let function = project_function_list_expr(plan, state, frame, list, *index, &type_)?;
            match function.kind() {
                EvaluatedFunctionValueKind::Tuple(value) => Ok(value.clone()),
                _ => Err(ExecutionError::Invariant(
                    InvariantError::FunctionReturnFamilyMismatch {
                        expected: FunctionReturnFamily::Tuple,
                        actual: function.kind().family(),
                    },
                )),
            }
        }
        TupleFunctionExprKind::Panic(panic) => {
            eval_panic_expr(plan, state, frame, panic).map(|never| match never {})
        }
        TupleFunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, state, frame, subject)? {
                eval_tuple_function_expr(plan, state, frame, true_)
            } else {
                eval_tuple_function_expr(plan, state, frame, false_)
            }
        }
        TupleFunctionExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_tuple_function_expr(plan, state, frame, branch);
                }
            }
            eval_tuple_function_expr(plan, state, frame, fallback)
        }
        TupleFunctionExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_tuple_function_expr(plan, state, frame, branch);
                }
            }
            eval_tuple_function_expr(plan, state, frame, fallback)
        }
        TupleFunctionExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_tuple_function_expr(plan, state, frame, branch);
                }
            }
            eval_tuple_function_expr(plan, state, frame, fallback)
        }
        TupleFunctionExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, state, steps, frame)?;
            eval_tuple_function_expr(plan, state, frame, return_)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        BoolExpr, CaptureArg, Expr, FloatExpr, FunctionTemplate, FunctionTemplateId, FunctionType,
        IntExpr, IntLocalId, ListExpr, ModulePlan, PanicExpr, PanicSite, ReturnExpr, Step,
        StringExpr, TupleExpr, TupleFunctionExpr, TupleFunctionFunctionId, TupleFunctionId,
        ValueType,
    };
    use crate::runtime::{ExecutionError, run_main};

    #[test]
    fn source_tuple_function_expression_variants_evaluate_exact_values() {
        let source = r#"
fn pair(value: Int) { #(value) }
fn identity(value: Int) { #(value) }
fn make_pair(offset: Int) -> fn(Int) -> #(Int) {
  fn(value) { #(value + offset) }
}

pub fn main() {
  let local = pair
  let maker = make_pair
  #(
    pair(0),
    { let captured = 1 fn(value) { #(value + captured) } }(0),
    local(2),
    make_pair(1)(2),
    maker(1)(3),
    #(pair).0(5),
    case [pair] { [function] -> function(6) _ -> #(0) },
    case True { True -> pair False -> identity }(7),
    case False { True -> identity False -> pair }(8),
    case 1 { 1 -> pair _ -> identity }(9),
    case 0 { 1 -> identity _ -> pair }(10),
    case "hit" { "hit" -> pair _ -> identity }(11),
    case "miss" { "hit" -> identity _ -> pair }(12),
    case 1.0 { 1.0 -> pair _ -> identity }(13),
    case 0.0 { 1.0 -> identity _ -> pair }(14),
    { let _ = 0 pair }(15),
  )
}
"#;

        assert_eq!(
            crate::runtime::run_src(source),
            crate::runtime::Value::Tuple(
                [0_i64, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
                    .into_iter()
                    .map(|value| {
                        crate::runtime::Value::Tuple(vec![crate::runtime::Value::Int(value.into())])
                    })
                    .collect(),
            ),
        );
    }

    #[test]
    fn module_expression_errors_propagate_through_tuple_function_wrappers() {
        let return_type = vec![ValueType::Int];
        let type_ = FunctionType::new(Vec::new(), ValueType::Tuple(return_type.clone()));
        let panic = |message: &str| {
            PanicExpr::panic_at(
                Some(StringExpr::value(message.into())),
                PanicSite::unknown(),
            )
        };
        let fallback = || TupleFunctionExpr::panic(panic("fallback"), type_.clone());
        let expressions = [
            (
                TupleFunctionExpr::closure(
                    crate::plan::monomorphic_function_instantiation(
                        1,
                        crate::plan::FunctionShape::from_function_type(type_.clone()),
                    ),
                    Vec::new(),
                    vec![CaptureArg::int(
                        IntLocalId(0),
                        IntExpr::panic(panic("capture")),
                    )],
                    type_.clone(),
                    return_type,
                ),
                "capture",
            ),
            (
                TupleFunctionExpr::tuple_index(
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
                TupleFunctionExpr::list_index(
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
                TupleFunctionExpr::bool_case(
                    BoolExpr::panic(panic("bool subject")),
                    fallback(),
                    fallback(),
                ),
                "bool subject",
            ),
            (
                TupleFunctionExpr::bool_case(
                    BoolExpr::not(BoolExpr::value(false)),
                    TupleFunctionExpr::panic(panic("true branch"), type_.clone()),
                    fallback(),
                ),
                "true branch",
            ),
            (
                TupleFunctionExpr::bool_case(
                    BoolExpr::not(BoolExpr::value(true)),
                    fallback(),
                    TupleFunctionExpr::panic(panic("false branch"), type_.clone()),
                ),
                "false branch",
            ),
            (
                TupleFunctionExpr::int_case(
                    IntExpr::panic(panic("int subject")),
                    Vec::new(),
                    fallback(),
                ),
                "int subject",
            ),
            (
                TupleFunctionExpr::string_case(
                    StringExpr::panic(panic("string subject")),
                    Vec::new(),
                    fallback(),
                ),
                "string subject",
            ),
            (
                TupleFunctionExpr::float_case(
                    FloatExpr::panic(panic("float subject")),
                    Vec::new(),
                    fallback(),
                ),
                "float subject",
            ),
            (
                TupleFunctionExpr::block(
                    vec![Step::evaluate(Expr::int(IntExpr::panic(panic("step"))))],
                    fallback(),
                ),
                "step",
            ),
        ];

        for (expression, message) in expressions {
            assert_eq!(
                run_module_tuple_function_expression(expression).to_string(),
                format!("panic: {message}"),
            );
        }
    }

    fn run_module_tuple_function_expression(expression: TupleFunctionExpr) -> ExecutionError {
        let target = FunctionTemplate::new(
            FunctionTemplateId::new(1),
            "target".into(),
            Vec::new(),
            vec![Step::evaluate(Expr::int(IntExpr::local_get(
                IntLocalId(0),
                "capture".into(),
            )))],
            ReturnExpr::tuple(
                TupleFunctionId(0),
                TupleExpr::panic(
                    PanicExpr::panic_at(None, PanicSite::unknown()),
                    vec![ValueType::Int],
                ),
            ),
        );
        let main = FunctionTemplate::new(
            FunctionTemplateId::new(0),
            "main".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::tuple_function(TupleFunctionFunctionId(0), expression),
        );
        let module = ModulePlan::new("main".into(), main, vec![target]);
        let plan = crate::ExecutionPlan::from_module_plan(module);

        run_main(&plan).expect_err("module expression should fail at runtime")
    }
}
