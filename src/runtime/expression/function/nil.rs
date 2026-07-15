use super::eval_custom_field_function;
use crate::plan::ValueType;
use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::{FunctionReturnFamily, NilFunctionExpr, NilFunctionExprKind};
use crate::runtime::expression::{
    eval_bool_expr, eval_float_expr, eval_int_expr, eval_panic_expr, eval_string_expr,
    project_function_list_expr, project_tuple_expr,
};
use crate::runtime::frame::Frame;
use crate::runtime::function;
use crate::runtime::state::RuntimeState;
use crate::runtime::{
    EvaluatedFunctionValueKind, EvaluatedNilFunction, EvaluatedValue, ExecutionError,
};

pub(in crate::runtime) fn eval_nil_function_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &NilFunctionExpr,
) -> Result<EvaluatedNilFunction, ExecutionError> {
    match expression.kind() {
        NilFunctionExprKind::Reference(reference) => Ok(EvaluatedNilFunction::new(
            *reference.function(),
            reference.param_locals(),
            Vec::new(),
            crate::runtime::evaluated::function_type_from_slots(
                plan,
                reference.params(),
                crate::plan::execution::ValueType::Nil,
            ),
        )),
        NilFunctionExprKind::Closure(template) => Ok(EvaluatedNilFunction::new(
            *template.function(),
            template.param_locals(),
            function::eval_capture_args(plan, state, frame, template.captures())?,
            crate::runtime::evaluated::function_type_from_slots(
                plan,
                template.params(),
                crate::plan::execution::ValueType::Nil,
            ),
        )),
        NilFunctionExprKind::LocalGet { local, .. } => Ok(frame.get_nil_function(*local)),
        NilFunctionExprKind::Call { function, args, .. } => {
            function::run_nil_function_returning_function_call(plan, state, *function, args, frame)
        }
        NilFunctionExprKind::FunctionCall {
            function: callee,
            args,
            ..
        } => function::run_nil_function_function_call(plan, state, callee.as_ref(), args, frame),
        NilFunctionExprKind::TupleIndex {
            tuple,
            index,
            type_,
        } => {
            let expected = ValueType::Function(Box::new(plan.function_type(type_)));
            let value = project_tuple_expr(plan, state, frame, tuple, *index, expected.clone())?;
            let actual = value.value_type(plan);
            match value {
                EvaluatedValue::Function(function) => match function.kind() {
                    EvaluatedFunctionValueKind::Nil(value) => Ok(value.clone()),
                    _ => Err(ExecutionError::TupleIndexFamilyMismatch { expected, actual }),
                },
                _ => Err(ExecutionError::TupleIndexFamilyMismatch { expected, actual }),
            }
        }
        NilFunctionExprKind::CustomField(access) => {
            let (constructor, expected, function) =
                eval_custom_field_function(plan, state, frame, access)?;
            match function.kind() {
                EvaluatedFunctionValueKind::Nil(value) => Ok(value.clone()),
                _ => {
                    let descriptor = plan.custom_constructor(constructor);
                    Err(ExecutionError::CustomFieldFamilyMismatch {
                        custom_type: plan.custom_value_type(constructor.type_id()),
                        constructor: descriptor.name().clone(),
                        field_index: access.index(),
                        expected,
                        actual: ValueType::Function(Box::new(plan.function_type(function.type_()))),
                    })
                }
            }
        }
        NilFunctionExprKind::ListIndex { list, index, type_ } => {
            let type_ = plan.function_type(type_);
            let function = project_function_list_expr(plan, state, frame, list, *index, &type_)?;
            match function.kind() {
                EvaluatedFunctionValueKind::Nil(value) => Ok(value.clone()),
                _ => Err(ExecutionError::FunctionReturnFamilyMismatch {
                    expected: FunctionReturnFamily::Nil,
                    actual: function.kind().family(),
                }),
            }
        }
        NilFunctionExprKind::Panic(panic) => {
            eval_panic_expr(plan, state, frame, panic).map(|never| match never {})
        }
        NilFunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, state, frame, subject)? {
                eval_nil_function_expr(plan, state, frame, true_)
            } else {
                eval_nil_function_expr(plan, state, frame, false_)
            }
        }
        NilFunctionExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_nil_function_expr(plan, state, frame, branch);
                }
            }
            eval_nil_function_expr(plan, state, frame, fallback)
        }
        NilFunctionExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_nil_function_expr(plan, state, frame, branch);
                }
            }
            eval_nil_function_expr(plan, state, frame, fallback)
        }
        NilFunctionExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_nil_function_expr(plan, state, frame, branch);
                }
            }
            eval_nil_function_expr(plan, state, frame, fallback)
        }
        NilFunctionExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, state, steps, frame)?;
            eval_nil_function_expr(plan, state, frame, return_)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        BoolExpr, CaptureArg, Expr, FloatExpr, FunctionId, FunctionPlan, FunctionType, IntExpr,
        IntLocalId, ListExpr, ModulePlan, NilFunctionExpr, NilFunctionFunctionId, NilFunctionId,
        PanicExpr, PanicSite, ReturnExpr, Step, StringExpr, TupleExpr, ValueType,
    };
    use crate::runtime::{ExecutionError, run_main};

    #[test]
    fn source_nil_function_expression_variants_evaluate_exact_values() {
        let source = r#"
fn nil_value(_value: Int) -> Nil { Nil }
fn other_nil(_value: Int) -> Nil { Nil }
fn make_nil() -> fn(Int) -> Nil { nil_value }

pub fn main() {
  let local = nil_value
  let maker = make_nil
  #(
    nil_value(0),
    { let captured = 1 fn(_value) { let _ = captured Nil } }(1),
    local(2),
    make_nil()(3),
    maker()(4),
    #(nil_value).0(5),
    case [nil_value] { [function] -> function(6) _ -> Nil },
    case True { True -> nil_value False -> other_nil }(7),
    case False { True -> other_nil False -> nil_value }(8),
    case 1 { 1 -> nil_value _ -> other_nil }(9),
    case 0 { 1 -> other_nil _ -> nil_value }(10),
    case "hit" { "hit" -> nil_value _ -> other_nil }(11),
    case "miss" { "hit" -> other_nil _ -> nil_value }(12),
    case 1.0 { 1.0 -> nil_value _ -> other_nil }(13),
    case 0.0 { 1.0 -> other_nil _ -> nil_value }(14),
    { let _ = 0 nil_value }(15),
  )
}
"#;

        assert_eq!(
            crate::runtime::run_src(source),
            crate::runtime::Value::Tuple(vec![crate::runtime::Value::Nil; 16]),
        );
    }

    #[test]
    fn module_expression_errors_propagate_through_nil_function_wrappers() {
        let type_ = FunctionType::new(Vec::new(), ValueType::Nil);
        let panic = |message: &str| {
            PanicExpr::panic_at(
                Some(StringExpr::value(message.into())),
                PanicSite::unknown(),
            )
        };
        let fallback = || NilFunctionExpr::panic(panic("fallback"), type_.clone());
        let expressions = [
            (
                NilFunctionExpr::closure(
                    NilFunctionId(1),
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
                NilFunctionExpr::tuple_index(
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
                NilFunctionExpr::list_index(
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
                NilFunctionExpr::bool_case(
                    BoolExpr::panic(panic("bool subject")),
                    fallback(),
                    fallback(),
                ),
                "bool subject",
            ),
            (
                NilFunctionExpr::int_case(
                    IntExpr::panic(panic("int subject")),
                    Vec::new(),
                    fallback(),
                ),
                "int subject",
            ),
            (
                NilFunctionExpr::string_case(
                    StringExpr::panic(panic("string subject")),
                    Vec::new(),
                    fallback(),
                ),
                "string subject",
            ),
            (
                NilFunctionExpr::float_case(
                    FloatExpr::panic(panic("float subject")),
                    Vec::new(),
                    fallback(),
                ),
                "float subject",
            ),
            (
                NilFunctionExpr::block(
                    vec![Step::evaluate(Expr::int(IntExpr::panic(panic("step"))))],
                    fallback(),
                ),
                "step",
            ),
        ];

        for (expression, message) in expressions {
            assert_eq!(
                run_module_nil_function_expression(expression).to_string(),
                format!("panic: {message}"),
            );
        }
    }

    fn run_module_nil_function_expression(expression: NilFunctionExpr) -> ExecutionError {
        let main = FunctionPlan::new(
            FunctionId::new(0),
            "main".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::nil_function(NilFunctionFunctionId(0), expression),
        );
        let module = ModulePlan::new("main".into(), main, Vec::new());
        let plan = crate::ExecutionPlan::from_module_plan(module);

        run_main(&plan).expect_err("module expression should fail at runtime")
    }
}
