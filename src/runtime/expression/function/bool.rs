use crate::plan::ValueType;
use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::{BoolFunctionExpr, BoolFunctionExprKind, FunctionReturnFamily};
use crate::runtime::expression::{
    eval_bool_expr, eval_float_expr, eval_int_expr, eval_panic_expr, eval_string_expr,
    project_function_list_expr, project_tuple_expr,
};
use crate::runtime::frame::Frame;
use crate::runtime::function;
use crate::runtime::{BoolFunctionValue, ExecutionError, FunctionValueKind, Value};

pub(in crate::runtime) fn eval_bool_function_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &BoolFunctionExpr,
) -> Result<BoolFunctionValue, ExecutionError> {
    match expression.kind() {
        BoolFunctionExprKind::Reference(reference) => Ok(BoolFunctionValue::new_with_captures(
            *reference.function(),
            reference.params().to_vec(),
            Vec::new(),
            plan.function_value_type(reference.params(), ValueType::Bool),
        )),
        BoolFunctionExprKind::Closure(template) => Ok(BoolFunctionValue::new_with_captures(
            *template.function(),
            template.params().to_vec(),
            function::eval_capture_args(plan, frame, template.captures())?,
            plan.function_value_type(template.params(), ValueType::Bool),
        )),
        BoolFunctionExprKind::LocalGet { local, .. } => Ok(frame.get_bool_function(*local)),
        BoolFunctionExprKind::Call { function, args, .. } => {
            function::run_bool_function_returning_function_call(plan, *function, args, frame)
        }
        BoolFunctionExprKind::FunctionCall {
            function: callee,
            args,
            ..
        } => function::run_bool_function_function_call(plan, callee.as_ref(), args, frame),
        BoolFunctionExprKind::TupleIndex {
            tuple,
            index,
            type_,
        } => {
            let expected = ValueType::Function(Box::new(plan.function_type(type_)));
            let value = project_tuple_expr(plan, frame, tuple, *index, expected.clone())?;
            let actual = value.value_type();
            match value {
                Value::Function(function) => match function.kind() {
                    FunctionValueKind::Bool(value) => Ok(value.clone()),
                    _ => Err(ExecutionError::tuple_index_family_mismatch(
                        expected, actual,
                    )),
                },
                _ => Err(ExecutionError::tuple_index_family_mismatch(
                    expected, actual,
                )),
            }
        }
        BoolFunctionExprKind::ListIndex { list, index, type_ } => {
            let type_ = plan.function_type(type_);
            let function = project_function_list_expr(plan, frame, list, *index, &type_)?;
            match function.kind() {
                FunctionValueKind::Bool(value) => Ok(value.clone()),
                _ => Err(ExecutionError::function_return_family_mismatch(
                    FunctionReturnFamily::Bool,
                    function.kind().family(),
                )),
            }
        }
        BoolFunctionExprKind::Panic(panic) => {
            eval_panic_expr(plan, frame, panic).map(|never| match never {})
        }
        BoolFunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, frame, subject)? {
                eval_bool_function_expr(plan, frame, true_)
            } else {
                eval_bool_function_expr(plan, frame, false_)
            }
        }
        BoolFunctionExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_bool_function_expr(plan, frame, branch);
                }
            }
            eval_bool_function_expr(plan, frame, fallback)
        }
        BoolFunctionExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_bool_function_expr(plan, frame, branch);
                }
            }
            eval_bool_function_expr(plan, frame, fallback)
        }
        BoolFunctionExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_bool_function_expr(plan, frame, branch);
                }
            }
            eval_bool_function_expr(plan, frame, fallback)
        }
        BoolFunctionExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, steps, frame)?;
            eval_bool_function_expr(plan, frame, return_)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        BoolExpr, BoolFunctionExpr, BoolFunctionFunctionId, BoolFunctionId, CaptureArg, Expr,
        FloatExpr, FunctionId, FunctionPlan, FunctionType, IntExpr, IntLocalId, ListExpr,
        ModulePlan, PanicExpr, PanicSite, ReturnExpr, Step, StringExpr, TupleExpr, ValueType,
    };
    use crate::runtime::{ExecutionError, run_main};

    #[test]
    fn source_bool_function_expression_variants_evaluate_exact_values() {
        let source = r#"
fn invert(value: Bool) -> Bool { !value }
fn identity(value: Bool) -> Bool { value }
fn make_inverter() -> fn(Bool) -> Bool { invert }

pub fn main() {
  let local = invert
  let maker = make_inverter
  #(
    invert(False),
    { let captured = True fn(value) { value != captured } }(False),
    local(False),
    make_inverter()(False),
    maker()(True),
    #(invert).0(False),
    case [invert] { [function] -> function(False) _ -> False },
    case True { True -> invert False -> identity }(False),
    case False { True -> identity False -> invert }(True),
    case 1 { 1 -> invert _ -> identity }(False),
    case 0 { 1 -> identity _ -> invert }(True),
    case "hit" { "hit" -> invert _ -> identity }(False),
    case "miss" { "hit" -> identity _ -> invert }(True),
    case 1.0 { 1.0 -> invert _ -> identity }(False),
    case 0.0 { 1.0 -> identity _ -> invert }(True),
    { let _ = 0 invert }(False),
  )
}
"#;

        assert_eq!(
            crate::runtime::run_src(source),
            crate::runtime::Value::Tuple(
                vec![
                    true, true, true, true, false, true, true, true, false, true, false, true,
                    false, true, false, true,
                ]
                .into_iter()
                .map(crate::runtime::Value::Bool)
                .collect(),
            ),
        );
    }

    #[test]
    fn module_expression_errors_propagate_through_bool_function_wrappers() {
        let type_ = FunctionType::new(Vec::new(), ValueType::Bool);
        let panic = |message: &str| {
            PanicExpr::panic_at(
                Some(StringExpr::value(message.into())),
                PanicSite::unknown(),
            )
        };
        let fallback = || BoolFunctionExpr::panic(panic("fallback"), type_.clone());
        let expressions = [
            (
                BoolFunctionExpr::closure(
                    BoolFunctionId(1),
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
                BoolFunctionExpr::tuple_index(
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
                BoolFunctionExpr::list_index(
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
                BoolFunctionExpr::bool_case(
                    BoolExpr::panic(panic("bool subject")),
                    fallback(),
                    fallback(),
                ),
                "bool subject",
            ),
            (
                BoolFunctionExpr::int_case(
                    IntExpr::panic(panic("int subject")),
                    Vec::new(),
                    fallback(),
                ),
                "int subject",
            ),
            (
                BoolFunctionExpr::string_case(
                    StringExpr::panic(panic("string subject")),
                    Vec::new(),
                    fallback(),
                ),
                "string subject",
            ),
            (
                BoolFunctionExpr::float_case(
                    FloatExpr::panic(panic("float subject")),
                    Vec::new(),
                    fallback(),
                ),
                "float subject",
            ),
            (
                BoolFunctionExpr::block(
                    vec![Step::evaluate(Expr::int(IntExpr::panic(panic("step"))))],
                    fallback(),
                ),
                "step",
            ),
        ];

        for (expression, message) in expressions {
            assert_eq!(
                run_module_bool_function_expression(expression).to_string(),
                format!("panic: {message}"),
            );
        }
    }

    fn run_module_bool_function_expression(expression: BoolFunctionExpr) -> ExecutionError {
        let main = FunctionPlan::new(
            FunctionId::new(0),
            "main".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::bool_function(BoolFunctionFunctionId(0), expression),
        );
        let module = ModulePlan::new("main".into(), main, Vec::new());
        let plan = crate::ExecutionPlan::from_module_plan(module);

        run_main(&plan).expect_err("module expression should fail at runtime")
    }
}
