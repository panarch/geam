use super::eval_custom_field_function;
use crate::plan::ValueType;
use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::{FunctionReturnFamily, StringFunctionExpr, StringFunctionExprKind};
use crate::runtime::expression::{
    eval_bool_expr, eval_direct_call, eval_float_expr, eval_function_call, eval_int_expr,
    eval_panic_expr, eval_string_expr, project_function_list_expr, project_tuple_expr,
};
use crate::runtime::frame::Frame;
use crate::runtime::function;
use crate::runtime::state::RuntimeState;
use crate::runtime::{
    EvaluatedFunctionValueKind, EvaluatedStringFunction, EvaluatedValue, ExecutionError,
    InvariantError,
};

pub(in crate::runtime) fn eval_string_function_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &StringFunctionExpr,
) -> Result<EvaluatedStringFunction, ExecutionError> {
    match expression.kind() {
        StringFunctionExprKind::Constant(value) => {
            eval_string_function_expr(plan, state, frame, plan.constant(*value))
        }
        StringFunctionExprKind::Reference(reference) => Ok(EvaluatedStringFunction::reference(
            *reference.function(),
            reference.param_locals(),
            Vec::new(),
            crate::runtime::evaluated::function_type_from_slots(
                plan,
                reference.params(),
                crate::plan::execution::ValueType::String,
            ),
        )),
        StringFunctionExprKind::Closure(closure) => Ok(EvaluatedStringFunction::closure(
            *closure.function(),
            closure.param_locals(),
            function::eval_capture_args(plan, state, frame, closure.captures())?,
            crate::runtime::evaluated::function_type_from_slots(
                plan,
                closure.params(),
                crate::plan::execution::ValueType::String,
            ),
        )),
        StringFunctionExprKind::LocalGet { local, .. } => Ok(frame.get_string_function(*local)),
        StringFunctionExprKind::Call(call) => eval_direct_call(
            plan,
            state,
            frame,
            call,
            |plan, state, function, args, frame| {
                function::run_string_function_returning_function_call(
                    plan, state, *function, args, frame,
                )
            },
        ),
        StringFunctionExprKind::FunctionCall(call) => eval_function_call(
            plan,
            state,
            frame,
            call,
            function::run_string_function_function_call,
        ),
        StringFunctionExprKind::TupleIndex {
            tuple,
            index,
            type_,
        } => {
            let expected = ValueType::Function(Box::new(plan.function_type(type_)));
            let value = project_tuple_expr(plan, state, frame, tuple, *index, expected.clone())?;
            let actual = value.value_type(plan);
            match value {
                EvaluatedValue::Function(function) => match function.kind() {
                    EvaluatedFunctionValueKind::String(value) => Ok(value.clone()),
                    _ => Err(ExecutionError::Invariant(
                        InvariantError::TupleIndexFamilyMismatch { expected, actual },
                    )),
                },
                _ => Err(ExecutionError::Invariant(
                    InvariantError::TupleIndexFamilyMismatch { expected, actual },
                )),
            }
        }
        StringFunctionExprKind::CustomField(access) => {
            let (constructor, expected, function) =
                eval_custom_field_function(plan, state, frame, access)?;
            match function.kind() {
                EvaluatedFunctionValueKind::String(value) => Ok(value.clone()),
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
        StringFunctionExprKind::ListIndex { list, index, type_ } => {
            let type_ = plan.function_type(type_);
            let function = project_function_list_expr(plan, state, frame, list, *index, &type_)?;
            match function.kind() {
                EvaluatedFunctionValueKind::String(value) => Ok(value.clone()),
                _ => Err(ExecutionError::Invariant(
                    InvariantError::FunctionReturnFamilyMismatch {
                        expected: FunctionReturnFamily::String,
                        actual: function.kind().family(),
                    },
                )),
            }
        }
        StringFunctionExprKind::Panic(panic) => {
            eval_panic_expr(plan, state, frame, panic).map(|never| match never {})
        }
        StringFunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, state, frame, subject)? {
                eval_string_function_expr(plan, state, frame, true_)
            } else {
                eval_string_function_expr(plan, state, frame, false_)
            }
        }
        StringFunctionExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_string_function_expr(plan, state, frame, branch);
                }
            }
            eval_string_function_expr(plan, state, frame, fallback)
        }
        StringFunctionExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_string_function_expr(plan, state, frame, branch);
                }
            }
            eval_string_function_expr(plan, state, frame, fallback)
        }
        StringFunctionExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_string_function_expr(plan, state, frame, branch);
                }
            }
            eval_string_function_expr(plan, state, frame, fallback)
        }
        StringFunctionExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, state, steps, frame)?;
            eval_string_function_expr(plan, state, frame, return_)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        BoolExpr, CaptureArg, Expr, FloatExpr, FunctionTemplate, FunctionTemplateId, FunctionType,
        IntExpr, IntLocalId, ListExpr, ModulePlan, PanicExpr, PanicSite, ReturnExpr, Step,
        StringExpr, StringFunctionExpr, StringFunctionFunctionId, StringFunctionId, TupleExpr,
        ValueType,
    };
    use crate::runtime::{ExecutionError, run_main};

    #[test]
    fn source_string_function_expression_variants_evaluate_exact_values() {
        let source = r#"
fn append_bang(value: String) -> String { value <> "!" }
fn identity(value: String) -> String { value }
fn make_prefix(prefix: String) -> fn(String) -> String {
  fn(value) { prefix <> value }
}

pub fn main() {
  let local = append_bang
  let maker = make_prefix
  #(
    append_bang("0"),
    { let captured = "p" fn(value) { captured <> value } }("1"),
    local("2"),
    make_prefix("p")("3"),
    maker("p")("4"),
    #(append_bang).0("5"),
    case [append_bang] { [function] -> function("6") _ -> "missing" },
    case True { True -> append_bang False -> identity }("7"),
    case False { True -> identity False -> append_bang }("8"),
    case 1 { 1 -> append_bang _ -> identity }("9"),
    case 0 { 1 -> identity _ -> append_bang }("10"),
    case "hit" { "hit" -> append_bang _ -> identity }("11"),
    case "miss" { "hit" -> identity _ -> append_bang }("12"),
    case 1.0 { 1.0 -> append_bang _ -> identity }("13"),
    case 0.0 { 1.0 -> identity _ -> append_bang }("14"),
    { let _ = 0 append_bang }("15"),
  )
}
"#;

        assert_eq!(
            crate::runtime::run_src(source),
            crate::runtime::Value::Tuple(
                [
                    "0!", "p1", "2!", "p3", "p4", "5!", "6!", "7!", "8!", "9!", "10!", "11!",
                    "12!", "13!", "14!", "15!",
                ]
                .into_iter()
                .map(|value| crate::runtime::Value::String(value.into()))
                .collect(),
            ),
        );
    }

    #[test]
    fn module_expression_errors_propagate_through_string_function_wrappers() {
        let type_ = FunctionType::new(Vec::new(), ValueType::String);
        let panic = |message: &str| {
            PanicExpr::panic_at(
                Some(StringExpr::value(message.into())),
                PanicSite::unknown(),
            )
        };
        let fallback = || StringFunctionExpr::panic(panic("fallback"), type_.clone());
        let expressions = [
            (
                StringFunctionExpr::closure(
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
                StringFunctionExpr::tuple_index(
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
                StringFunctionExpr::list_index(
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
                StringFunctionExpr::bool_case(
                    BoolExpr::panic(panic("bool subject")),
                    fallback(),
                    fallback(),
                ),
                "bool subject",
            ),
            (
                StringFunctionExpr::bool_case(
                    BoolExpr::not(BoolExpr::value(false)),
                    StringFunctionExpr::panic(panic("true branch"), type_.clone()),
                    fallback(),
                ),
                "true branch",
            ),
            (
                StringFunctionExpr::bool_case(
                    BoolExpr::not(BoolExpr::value(true)),
                    fallback(),
                    StringFunctionExpr::panic(panic("false branch"), type_.clone()),
                ),
                "false branch",
            ),
            (
                StringFunctionExpr::int_case(
                    IntExpr::panic(panic("int subject")),
                    Vec::new(),
                    fallback(),
                ),
                "int subject",
            ),
            (
                StringFunctionExpr::string_case(
                    StringExpr::panic(panic("string subject")),
                    Vec::new(),
                    fallback(),
                ),
                "string subject",
            ),
            (
                StringFunctionExpr::float_case(
                    FloatExpr::panic(panic("float subject")),
                    Vec::new(),
                    fallback(),
                ),
                "float subject",
            ),
            (
                StringFunctionExpr::block(
                    vec![Step::evaluate(Expr::int(IntExpr::panic(panic("step"))))],
                    fallback(),
                ),
                "step",
            ),
        ];

        for (expression, message) in expressions {
            assert_eq!(
                run_module_string_function_expression(expression).to_string(),
                format!("panic: {message}"),
            );
        }
    }

    fn run_module_string_function_expression(expression: StringFunctionExpr) -> ExecutionError {
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
            ReturnExpr::string(
                StringFunctionId(0),
                StringExpr::panic(PanicExpr::panic_at(None, PanicSite::unknown())),
            ),
        );
        let main = FunctionTemplate::new(
            FunctionTemplateId::new(0),
            "main".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::string_function(StringFunctionFunctionId(0), expression),
        );
        let module = ModulePlan::new("main".into(), main, vec![target]);
        let plan = crate::ExecutionPlan::from_module_plan(module);

        run_main(&plan).expect_err("module expression should fail at runtime")
    }
}
