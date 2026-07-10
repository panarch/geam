use crate::plan::ValueType;
use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::{FunctionReturnFamily, StringFunctionExpr, StringFunctionExprKind};
use crate::runtime::expression::{
    eval_bool_expr, eval_float_expr, eval_int_expr, eval_panic_expr, eval_string_expr,
    project_function_list_expr, project_tuple_expr,
};
use crate::runtime::frame::Frame;
use crate::runtime::function;
use crate::runtime::{ExecutionError, FunctionValueKind, StringFunctionValue, Value};

pub(in crate::runtime) fn eval_string_function_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &StringFunctionExpr,
) -> Result<StringFunctionValue, ExecutionError> {
    match expression.kind() {
        StringFunctionExprKind::Reference(reference) => Ok(StringFunctionValue::new_with_captures(
            *reference.function(),
            reference.params().to_vec(),
            Vec::new(),
        )),
        StringFunctionExprKind::Closure(template) => Ok(StringFunctionValue::new_with_captures(
            *template.function(),
            template.params().to_vec(),
            function::eval_capture_args(plan, frame, template.captures())?,
        )),
        StringFunctionExprKind::LocalGet { local, .. } => Ok(frame.get_string_function(*local)),
        StringFunctionExprKind::Call { function, args, .. } => {
            function::run_string_function_returning_function_call(plan, *function, args, frame)
        }
        StringFunctionExprKind::FunctionCall {
            function: callee,
            args,
            ..
        } => function::run_string_function_function_call(plan, callee.as_ref(), args, frame),
        StringFunctionExprKind::TupleIndex {
            tuple,
            index,
            type_,
        } => {
            let expected = ValueType::Function(Box::new(type_.clone()));
            let value = project_tuple_expr(plan, frame, tuple, *index, expected.clone())?;
            let actual = value.value_type();
            match value {
                Value::Function(function) => match function.kind() {
                    FunctionValueKind::String(value) => Ok(value.clone()),
                    _ => Err(ExecutionError::tuple_index_family_mismatch(
                        expected, actual,
                    )),
                },
                _ => Err(ExecutionError::tuple_index_family_mismatch(
                    expected, actual,
                )),
            }
        }
        StringFunctionExprKind::ListIndex { list, index, type_ } => {
            let function = project_function_list_expr(plan, frame, list, *index, type_)?;
            match function.kind() {
                FunctionValueKind::String(value) => Ok(value.clone()),
                _ => Err(ExecutionError::function_return_family_mismatch(
                    FunctionReturnFamily::String,
                    function.kind().family(),
                )),
            }
        }
        StringFunctionExprKind::Panic(panic) => {
            eval_panic_expr(plan, frame, panic).map(|never| match never {})
        }
        StringFunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, frame, subject)? {
                eval_string_function_expr(plan, frame, true_)
            } else {
                eval_string_function_expr(plan, frame, false_)
            }
        }
        StringFunctionExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_string_function_expr(plan, frame, branch);
                }
            }
            eval_string_function_expr(plan, frame, fallback)
        }
        StringFunctionExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_string_function_expr(plan, frame, branch);
                }
            }
            eval_string_function_expr(plan, frame, fallback)
        }
        StringFunctionExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_string_function_expr(plan, frame, branch);
                }
            }
            eval_string_function_expr(plan, frame, fallback)
        }
        StringFunctionExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, steps, frame)?;
            eval_string_function_expr(plan, frame, return_)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        BoolExpr, CaptureArg, Expr, FloatExpr, FunctionId, FunctionPlan, FunctionType, IntExpr,
        IntLocalId, ListExpr, ModulePlan, PanicExpr, PanicSite, ReturnExpr, Step, StringExpr,
        StringFunctionExpr, StringFunctionFunctionId, StringFunctionId, TupleExpr, ValueType,
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
                    StringFunctionId(1),
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
        let main = FunctionPlan::new(
            FunctionId::new(0),
            "main".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::string_function(StringFunctionFunctionId(0), expression),
        );
        let module = ModulePlan::new("main".into(), main, Vec::new());
        let plan = crate::ExecutionPlan::from_module_plan(module);

        run_main(&plan).expect_err("module expression should fail at runtime")
    }
}
