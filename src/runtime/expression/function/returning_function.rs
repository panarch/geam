use crate::plan::ValueType;
use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::{
    FunctionFunctionExpr, FunctionFunctionExprKind, FunctionReturnFamily,
};
use crate::runtime::expression::{
    eval_bool_expr, eval_float_expr, eval_int_expr, eval_panic_expr, eval_string_expr,
    project_function_list_expr, project_tuple_expr,
};
use crate::runtime::frame::Frame;
use crate::runtime::function;
use crate::runtime::{ExecutionError, FunctionFunctionValue, FunctionValueKind, Value};

pub(in crate::runtime) fn eval_function_function_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &FunctionFunctionExpr,
) -> Result<FunctionFunctionValue, ExecutionError> {
    match expression.kind() {
        FunctionFunctionExprKind::Reference(reference) => {
            Ok(FunctionFunctionValue::from_evaluated(
                reference.function().clone(),
                reference.params().to_vec(),
                Vec::new(),
                plan.function_type(expression.type_()),
            ))
        }
        FunctionFunctionExprKind::Closure(template) => Ok(FunctionFunctionValue::from_evaluated(
            template.function().clone(),
            template.params().to_vec(),
            function::eval_capture_args(plan, frame, template.captures())?,
            plan.function_type(expression.type_()),
        )),
        FunctionFunctionExprKind::LocalGet { local, .. } => Ok(frame.get_function_function(*local)),
        FunctionFunctionExprKind::Call { function, args, .. } => {
            function::run_function_function_returning_function_call(plan, *function, args, frame)
        }
        FunctionFunctionExprKind::FunctionCall {
            function: callee,
            args,
            ..
        } => function::run_function_function_function_call(plan, callee.as_ref(), args, frame),
        FunctionFunctionExprKind::TupleIndex {
            tuple,
            index,
            type_,
        } => {
            let expected = ValueType::Function(Box::new(plan.function_type(type_)));
            let value = project_tuple_expr(plan, frame, tuple, *index, expected.clone())?;
            let actual = value.value_type();
            match value {
                Value::Function(function) => match function.kind() {
                    FunctionValueKind::Function(value) => Ok(value.clone()),
                    _ => Err(ExecutionError::tuple_index_family_mismatch(
                        expected, actual,
                    )),
                },
                _ => Err(ExecutionError::tuple_index_family_mismatch(
                    expected, actual,
                )),
            }
        }
        FunctionFunctionExprKind::ListIndex { list, index, type_ } => {
            let type_ = plan.function_type(type_);
            let function = project_function_list_expr(plan, frame, list, *index, &type_)?;
            match function.kind() {
                FunctionValueKind::Function(value) => Ok(value.clone()),
                _ => Err(ExecutionError::function_return_family_mismatch(
                    FunctionReturnFamily::Function,
                    function.kind().family(),
                )),
            }
        }
        FunctionFunctionExprKind::Panic(panic) => {
            eval_panic_expr(plan, frame, panic).map(|never| match never {})
        }
        FunctionFunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, frame, subject)? {
                eval_function_function_expr(plan, frame, true_)
            } else {
                eval_function_function_expr(plan, frame, false_)
            }
        }
        FunctionFunctionExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_function_function_expr(plan, frame, branch);
                }
            }
            eval_function_function_expr(plan, frame, fallback)
        }
        FunctionFunctionExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_function_function_expr(plan, frame, branch);
                }
            }
            eval_function_function_expr(plan, frame, fallback)
        }
        FunctionFunctionExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_function_function_expr(plan, frame, branch);
                }
            }
            eval_function_function_expr(plan, frame, fallback)
        }
        FunctionFunctionExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, steps, frame)?;
            eval_function_function_expr(plan, frame, return_)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        BoolExpr, CaptureArg, Expr, FloatExpr, FunctionFunctionExpr, FunctionFunctionFunctionId,
        FunctionFunctionId, FunctionId, FunctionPlan, FunctionType, IntExpr, IntFunctionFunctionId,
        IntLocalId, ListExpr, ModulePlan, PanicExpr, PanicSite, ReturnExpr, Step, StringExpr,
        TupleExpr, ValueType,
    };
    use crate::runtime::{ExecutionError, run_main};

    #[test]
    fn source_function_function_expression_variants_evaluate_exact_values() {
        let source = r#"
fn add_one(value: Int) -> Int { value + 1 }
fn identity(value: Int) -> Int { value }
fn factory() -> fn(Int) -> Int { add_one }
fn other_factory() -> fn(Int) -> Int { identity }
fn return_factory() -> fn() -> fn(Int) -> Int { factory }
fn pass_factory(value: fn() -> fn(Int) -> Int) { value }

pub fn main() {
  let local = factory
  let pass = pass_factory
  #(
    factory()(0),
    { let captured = 1 fn() { fn(value) { value + captured } } }()(0),
    local()(2),
    return_factory()()(3),
    pass(factory)()(4),
    #(factory).0()(5),
    case [factory] { [value] -> value()(6) _ -> 0 },
    case True { True -> factory False -> other_factory }()(7),
    case False { True -> other_factory False -> factory }()(8),
    case 1 { 1 -> factory _ -> other_factory }()(9),
    case 0 { 1 -> other_factory _ -> factory }()(10),
    case "hit" { "hit" -> factory _ -> other_factory }()(11),
    case "miss" { "hit" -> other_factory _ -> factory }()(12),
    case 1.0 { 1.0 -> factory _ -> other_factory }()(13),
    case 0.0 { 1.0 -> other_factory _ -> factory }()(14),
    { let _ = 0 factory }()(15),
  )
}
"#;

        assert_eq!(
            crate::runtime::run_src(source),
            crate::runtime::Value::Tuple(
                [1_i64, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
                    .into_iter()
                    .map(|value| crate::runtime::Value::Int(value.into()))
                    .collect(),
            ),
        );
    }

    #[test]
    fn module_expression_errors_propagate_through_function_function_wrappers() {
        let return_type = FunctionType::new(Vec::new(), ValueType::Int);
        let type_ = FunctionType::new(
            Vec::new(),
            ValueType::Function(Box::new(return_type.clone())),
        );
        let panic = |message: &str| {
            PanicExpr::panic_at(
                Some(StringExpr::value(message.into())),
                PanicSite::unknown(),
            )
        };
        let fallback = || FunctionFunctionExpr::panic(panic("fallback"), type_.clone());
        let expressions = [
            (
                FunctionFunctionExpr::closure(
                    FunctionFunctionId::Int(IntFunctionFunctionId(1)),
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
                FunctionFunctionExpr::tuple_index(
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
                FunctionFunctionExpr::list_index(
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
                FunctionFunctionExpr::bool_case(
                    BoolExpr::panic(panic("bool subject")),
                    fallback(),
                    fallback(),
                ),
                "bool subject",
            ),
            (
                FunctionFunctionExpr::int_case(
                    IntExpr::panic(panic("int subject")),
                    Vec::new(),
                    fallback(),
                ),
                "int subject",
            ),
            (
                FunctionFunctionExpr::string_case(
                    StringExpr::panic(panic("string subject")),
                    Vec::new(),
                    fallback(),
                ),
                "string subject",
            ),
            (
                FunctionFunctionExpr::float_case(
                    FloatExpr::panic(panic("float subject")),
                    Vec::new(),
                    fallback(),
                ),
                "float subject",
            ),
            (
                FunctionFunctionExpr::block(
                    vec![Step::evaluate(Expr::int(IntExpr::panic(panic("step"))))],
                    fallback(),
                ),
                "step",
            ),
        ];

        for (expression, message) in expressions {
            assert_eq!(
                run_module_function_function_expression(expression).to_string(),
                format!("panic: {message}"),
            );
        }
    }

    fn run_module_function_function_expression(expression: FunctionFunctionExpr) -> ExecutionError {
        let main = FunctionPlan::new(
            FunctionId::new(0),
            "main".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::function_function(FunctionFunctionFunctionId(0), expression),
        );
        let module = ModulePlan::new("main".into(), main, Vec::new());
        let plan = crate::ExecutionPlan::from_module_plan(module);

        run_main(&plan).expect_err("module expression should fail at runtime")
    }
}
