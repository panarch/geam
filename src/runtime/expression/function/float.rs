use crate::plan::ValueType;
use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::{FloatFunctionExpr, FloatFunctionExprKind, FunctionReturnFamily};
use crate::runtime::expression::{
    eval_bool_expr, eval_float_expr, eval_int_expr, eval_panic_expr, eval_string_expr,
    project_function_list_expr, project_tuple_expr,
};
use crate::runtime::frame::Frame;
use crate::runtime::function;
use crate::runtime::{ExecutionError, FloatFunctionValue, FunctionValueKind, Value};

pub(in crate::runtime) fn eval_float_function_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &FloatFunctionExpr,
) -> Result<FloatFunctionValue, ExecutionError> {
    match expression.kind() {
        FloatFunctionExprKind::Reference(reference) => Ok(FloatFunctionValue::new_with_captures(
            *reference.function(),
            reference.params().to_vec(),
            Vec::new(),
        )),
        FloatFunctionExprKind::Closure(template) => Ok(FloatFunctionValue::new_with_captures(
            *template.function(),
            template.params().to_vec(),
            function::eval_capture_args(plan, frame, template.captures())?,
        )),
        FloatFunctionExprKind::LocalGet { local, .. } => Ok(frame.get_float_function(*local)),
        FloatFunctionExprKind::Call { function, args, .. } => {
            function::run_float_function_returning_function_call(plan, *function, args, frame)
        }
        FloatFunctionExprKind::FunctionCall {
            function: callee,
            args,
            ..
        } => function::run_float_function_function_call(plan, callee.as_ref(), args, frame),
        FloatFunctionExprKind::TupleIndex {
            tuple,
            index,
            type_,
        } => {
            let expected = ValueType::Function(Box::new(type_.clone()));
            let value = project_tuple_expr(plan, frame, tuple, *index, expected.clone())?;
            let actual = value.value_type();
            match value {
                Value::Function(function) => match function.kind() {
                    FunctionValueKind::Float(value) => Ok(value.clone()),
                    _ => Err(ExecutionError::tuple_index_family_mismatch(
                        expected, actual,
                    )),
                },
                _ => Err(ExecutionError::tuple_index_family_mismatch(
                    expected, actual,
                )),
            }
        }
        FloatFunctionExprKind::ListIndex { list, index, type_ } => {
            let function = project_function_list_expr(plan, frame, list, *index, type_)?;
            match function.kind() {
                FunctionValueKind::Float(value) => Ok(value.clone()),
                _ => Err(ExecutionError::function_return_family_mismatch(
                    FunctionReturnFamily::Float,
                    function.kind().family(),
                )),
            }
        }
        FloatFunctionExprKind::Panic(panic) => {
            eval_panic_expr(plan, frame, panic).map(|never| match never {})
        }
        FloatFunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, frame, subject)? {
                eval_float_function_expr(plan, frame, true_)
            } else {
                eval_float_function_expr(plan, frame, false_)
            }
        }
        FloatFunctionExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_float_function_expr(plan, frame, branch);
                }
            }
            eval_float_function_expr(plan, frame, fallback)
        }
        FloatFunctionExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_float_function_expr(plan, frame, branch);
                }
            }
            eval_float_function_expr(plan, frame, fallback)
        }
        FloatFunctionExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_float_function_expr(plan, frame, branch);
                }
            }
            eval_float_function_expr(plan, frame, fallback)
        }
        FloatFunctionExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, steps, frame)?;
            eval_float_function_expr(plan, frame, return_)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        BoolExpr, CaptureArg, Expr, FloatExpr, FloatFunctionExpr, FloatFunctionFunctionId,
        FloatFunctionId, FunctionId, FunctionPlan, FunctionType, IntExpr, IntLocalId, ListExpr,
        ModulePlan, PanicExpr, PanicSite, ReturnExpr, Step, StringExpr, TupleExpr, ValueType,
    };
    use crate::runtime::{ExecutionError, run_main};

    #[test]
    fn source_float_function_expression_variants_evaluate_exact_values() {
        let source = r#"
fn add_half(value: Float) -> Float { value +. 0.5 }
fn identity(value: Float) -> Float { value }
fn make_adder(offset: Float) -> fn(Float) -> Float {
  fn(value) { value +. offset }
}

pub fn main() {
  let local = add_half
  let maker = make_adder
  #(
    add_half(0.0),
    { let captured = 1.0 fn(value) { value +. captured } }(1.0),
    local(2.0),
    make_adder(1.0)(3.0),
    maker(1.0)(4.0),
    #(add_half).0(5.0),
    case [add_half] { [function] -> function(6.0) _ -> 0.0 },
    case True { True -> add_half False -> identity }(7.0),
    case False { True -> identity False -> add_half }(8.0),
    case 1 { 1 -> add_half _ -> identity }(9.0),
    case 0 { 1 -> identity _ -> add_half }(10.0),
    case "hit" { "hit" -> add_half _ -> identity }(11.0),
    case "miss" { "hit" -> identity _ -> add_half }(12.0),
    case 1.0 { 1.0 -> add_half _ -> identity }(13.0),
    case 0.0 { 1.0 -> identity _ -> add_half }(14.0),
    { let _ = 0 add_half }(15.0),
  )
}
"#;

        assert_eq!(
            crate::runtime::run_src(source),
            crate::runtime::Value::Tuple(
                [
                    0.5, 2.0, 2.5, 4.0, 5.0, 5.5, 6.5, 7.5, 8.5, 9.5, 10.5, 11.5, 12.5, 13.5, 14.5,
                    15.5,
                ]
                .into_iter()
                .map(crate::runtime::Value::Float)
                .collect(),
            ),
        );
    }

    #[test]
    fn module_expression_errors_propagate_through_float_function_wrappers() {
        let type_ = FunctionType::new(Vec::new(), ValueType::Float);
        let panic = |message: &str| {
            PanicExpr::panic_at(
                Some(StringExpr::value(message.into())),
                PanicSite::unknown(),
            )
        };
        let fallback = || FloatFunctionExpr::panic(panic("fallback"), type_.clone());
        let expressions = [
            (
                FloatFunctionExpr::closure(
                    FloatFunctionId(1),
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
                FloatFunctionExpr::tuple_index(
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
                FloatFunctionExpr::list_index(
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
                FloatFunctionExpr::bool_case(
                    BoolExpr::panic(panic("bool subject")),
                    fallback(),
                    fallback(),
                ),
                "bool subject",
            ),
            (
                FloatFunctionExpr::int_case(
                    IntExpr::panic(panic("int subject")),
                    Vec::new(),
                    fallback(),
                ),
                "int subject",
            ),
            (
                FloatFunctionExpr::string_case(
                    StringExpr::panic(panic("string subject")),
                    Vec::new(),
                    fallback(),
                ),
                "string subject",
            ),
            (
                FloatFunctionExpr::float_case(
                    FloatExpr::panic(panic("float subject")),
                    Vec::new(),
                    fallback(),
                ),
                "float subject",
            ),
            (
                FloatFunctionExpr::block(
                    vec![Step::evaluate(Expr::int(IntExpr::panic(panic("step"))))],
                    fallback(),
                ),
                "step",
            ),
        ];

        for (expression, message) in expressions {
            assert_eq!(
                run_module_float_function_expression(expression).to_string(),
                format!("panic: {message}"),
            );
        }
    }

    fn run_module_float_function_expression(expression: FloatFunctionExpr) -> ExecutionError {
        let main = FunctionPlan::new(
            FunctionId::new(0),
            "main".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::float_function(FloatFunctionFunctionId(0), expression),
        );
        let module = ModulePlan::new("main".into(), main, Vec::new());
        let plan = crate::ExecutionPlan::from_module_plan(module);

        run_main(&plan).expect_err("module expression should fail at runtime")
    }
}
