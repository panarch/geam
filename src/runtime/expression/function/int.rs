use super::eval_custom_field_function;
use crate::plan::ValueType;
use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::{FunctionReturnFamily, IntFunctionExpr, IntFunctionExprKind};
use crate::runtime::expression::{
    eval_bool_expr, eval_float_expr, eval_int_expr, eval_panic_expr, eval_string_expr,
    project_function_list_expr, project_tuple_expr,
};
use crate::runtime::frame::Frame;
use crate::runtime::function;
use crate::runtime::state::RuntimeState;
use crate::runtime::{
    EvaluatedFunctionValueKind, EvaluatedIntFunction, EvaluatedValue, ExecutionError,
};

pub(in crate::runtime) fn eval_int_function_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &IntFunctionExpr,
) -> Result<EvaluatedIntFunction, ExecutionError> {
    match expression.kind() {
        IntFunctionExprKind::Reference(reference) => Ok(EvaluatedIntFunction::new(
            *reference.function(),
            reference.params().to_vec(),
            Vec::new(),
            crate::runtime::evaluated::function_type(
                reference.params(),
                crate::plan::execution::ValueType::Int,
            ),
        )),
        IntFunctionExprKind::Closure(template) => Ok(EvaluatedIntFunction::new(
            *template.function(),
            template.params().to_vec(),
            function::eval_capture_args(plan, state, frame, template.captures())?,
            crate::runtime::evaluated::function_type(
                template.params(),
                crate::plan::execution::ValueType::Int,
            ),
        )),
        IntFunctionExprKind::LocalGet { local, .. } => Ok(frame.get_int_function(*local)),
        IntFunctionExprKind::Call { function, args, .. } => {
            function::run_int_function_returning_function_call(plan, state, *function, args, frame)
        }
        IntFunctionExprKind::FunctionCall {
            function: callee,
            args,
            ..
        } => function::run_int_function_function_call(plan, state, callee.as_ref(), args, frame),
        IntFunctionExprKind::TupleIndex {
            tuple,
            index,
            type_,
        } => {
            let expected = ValueType::Function(Box::new(plan.function_type(type_)));
            let value = project_tuple_expr(plan, state, frame, tuple, *index, expected.clone())?;
            let actual = value.value_type(plan);
            match value {
                EvaluatedValue::Function(function) => match function.kind() {
                    EvaluatedFunctionValueKind::Int(value) => Ok(value.clone()),
                    _ => Err(ExecutionError::TupleIndexFamilyMismatch { expected, actual }),
                },
                _ => Err(ExecutionError::TupleIndexFamilyMismatch { expected, actual }),
            }
        }
        IntFunctionExprKind::CustomField(access) => {
            let (constructor, expected, function) =
                eval_custom_field_function(plan, state, frame, access)?;
            match function.kind() {
                EvaluatedFunctionValueKind::Int(value) => Ok(value.clone()),
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
        IntFunctionExprKind::ListIndex { list, index, type_ } => {
            let type_ = plan.function_type(type_);
            let function = project_function_list_expr(plan, state, frame, list, *index, &type_)?;
            match function.kind() {
                EvaluatedFunctionValueKind::Int(value) => Ok(value.clone()),
                _ => Err(ExecutionError::FunctionReturnFamilyMismatch {
                    expected: FunctionReturnFamily::Int,
                    actual: function.kind().family(),
                }),
            }
        }
        IntFunctionExprKind::Panic(panic) => {
            eval_panic_expr(plan, state, frame, panic).map(|never| match never {})
        }
        IntFunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, state, frame, subject)? {
                eval_int_function_expr(plan, state, frame, true_)
            } else {
                eval_int_function_expr(plan, state, frame, false_)
            }
        }
        IntFunctionExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_int_function_expr(plan, state, frame, branch);
                }
            }
            eval_int_function_expr(plan, state, frame, fallback)
        }
        IntFunctionExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_int_function_expr(plan, state, frame, branch);
                }
            }
            eval_int_function_expr(plan, state, frame, fallback)
        }
        IntFunctionExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_int_function_expr(plan, state, frame, branch);
                }
            }
            eval_int_function_expr(plan, state, frame, fallback)
        }
        IntFunctionExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, state, steps, frame)?;
            eval_int_function_expr(plan, state, frame, return_)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        BoolExpr, CaptureArg, Expr, FloatExpr, FunctionId, FunctionPlan, FunctionType, IntExpr,
        IntFunctionExpr, IntFunctionFunctionId, IntFunctionId, IntLocalId, ListExpr, ModulePlan,
        PanicExpr, PanicSite, ReturnExpr, Step, StringExpr, TupleExpr, ValueType,
    };
    use crate::runtime::{ExecutionError, run_main};

    #[test]
    fn source_int_function_expression_variants_evaluate_exact_values() {
        let source = r#"
fn add_one(value: Int) -> Int { value + 1 }
fn identity(value: Int) -> Int { value }
fn make_adder(offset: Int) -> fn(Int) -> Int {
  fn(value) { value + offset }
}

pub fn main() {
  let local = add_one
  let maker = make_adder
  #(
    add_one(0),
    { let captured = 1 fn(value) { value + captured } }(1),
    local(2),
    make_adder(1)(3),
    maker(1)(4),
    #(add_one).0(5),
    case [add_one] { [function] -> function(6) _ -> 0 },
    case True { True -> add_one False -> identity }(7),
    case False { True -> identity False -> add_one }(8),
    case 1 { 1 -> add_one _ -> identity }(9),
    case 0 { 1 -> identity _ -> add_one }(10),
    case "hit" { "hit" -> add_one _ -> identity }(11),
    case "miss" { "hit" -> identity _ -> add_one }(12),
    case 1.0 { 1.0 -> add_one _ -> identity }(13),
    case 0.0 { 1.0 -> identity _ -> add_one }(14),
    { let _ = 0 add_one }(15),
  )
}
"#;

        assert_eq!(
            crate::runtime::run_src(source),
            crate::runtime::Value::Tuple(
                (1_i64..=16)
                    .map(|value| crate::runtime::Value::Int(value.into()))
                    .collect(),
            ),
        );
    }

    #[test]
    fn module_expression_errors_propagate_through_int_function_wrappers() {
        let type_ = FunctionType::new(Vec::new(), ValueType::Int);
        let panic = |message: &str| {
            PanicExpr::panic_at(
                Some(StringExpr::value(message.into())),
                PanicSite::unknown(),
            )
        };
        let fallback = || IntFunctionExpr::panic(panic("fallback"), type_.clone());
        let expressions = [
            (
                IntFunctionExpr::closure(
                    IntFunctionId(1),
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
                IntFunctionExpr::tuple_index(
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
                IntFunctionExpr::list_index(
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
                IntFunctionExpr::bool_case(
                    BoolExpr::panic(panic("bool subject")),
                    fallback(),
                    fallback(),
                ),
                "bool subject",
            ),
            (
                IntFunctionExpr::int_case(
                    IntExpr::panic(panic("int subject")),
                    Vec::new(),
                    fallback(),
                ),
                "int subject",
            ),
            (
                IntFunctionExpr::string_case(
                    StringExpr::panic(panic("string subject")),
                    Vec::new(),
                    fallback(),
                ),
                "string subject",
            ),
            (
                IntFunctionExpr::float_case(
                    FloatExpr::panic(panic("float subject")),
                    Vec::new(),
                    fallback(),
                ),
                "float subject",
            ),
            (
                IntFunctionExpr::block(
                    vec![Step::evaluate(Expr::int(IntExpr::panic(panic("step"))))],
                    fallback(),
                ),
                "step",
            ),
        ];

        for (expression, message) in expressions {
            assert_eq!(
                run_module_int_function_expression(expression).to_string(),
                format!("panic: {message}"),
            );
        }
    }

    fn run_module_int_function_expression(expression: IntFunctionExpr) -> ExecutionError {
        let main = FunctionPlan::new(
            FunctionId::new(0),
            "main".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::int_function(IntFunctionFunctionId(0), expression),
        );
        let module = ModulePlan::new("main".into(), main, Vec::new());
        let plan = crate::ExecutionPlan::from_module_plan(module);

        run_main(&plan).expect_err("module expression should fail at runtime")
    }
}
