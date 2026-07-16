use super::eval_custom_field_function;
use crate::plan::ValueType;
use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::{FunctionReturnFamily, ListFunctionExpr, ListFunctionExprKind};
use crate::runtime::expression::{
    eval_bool_expr, eval_float_expr, eval_int_expr, eval_panic_expr, eval_string_expr,
    project_function_list_expr, project_tuple_expr,
};
use crate::runtime::frame::Frame;
use crate::runtime::function;
use crate::runtime::state::RuntimeState;
use crate::runtime::{
    EvaluatedFunctionValueKind, EvaluatedListFunction, EvaluatedValue, ExecutionError,
};

pub(in crate::runtime) fn eval_list_function_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &ListFunctionExpr,
) -> Result<EvaluatedListFunction, ExecutionError> {
    match expression.kind() {
        ListFunctionExprKind::Reference(reference) => Ok(EvaluatedListFunction::reference(
            reference.function().clone(),
            reference.param_locals(),
            Vec::new(),
            crate::runtime::evaluated::function_type_from_slots(
                plan,
                reference.params(),
                crate::plan::execution::ValueType::List(reference.function().list_type()),
            ),
        )),
        ListFunctionExprKind::Closure(template) => Ok(EvaluatedListFunction::closure(
            template.function().clone(),
            template.param_locals(),
            function::eval_capture_args(plan, state, frame, template.captures())?,
            crate::runtime::evaluated::function_type_from_slots(
                plan,
                template.params(),
                crate::plan::execution::ValueType::List(template.function().list_type()),
            ),
        )),
        ListFunctionExprKind::LocalGet { local, .. } => Ok(frame.get_list_function(local)),
        ListFunctionExprKind::Call { function, args, .. } => {
            function::run_list_function_returning_function_call(
                plan,
                state,
                function.clone(),
                args,
                frame,
            )
        }
        ListFunctionExprKind::FunctionCall {
            function: callee,
            args,
            ..
        } => function::run_list_function_function_call(plan, state, callee.as_ref(), args, frame),
        ListFunctionExprKind::TupleIndex {
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
                    EvaluatedFunctionValueKind::List(value) => Ok(value.clone()),
                    _ => Err(ExecutionError::TupleIndexFamilyMismatch {
                        expected: ValueType::Function(Box::new(plan.function_type(type_))),
                        actual: EvaluatedValue::Function(function).value_type(plan),
                    }),
                },
                other => Err(ExecutionError::TupleIndexFamilyMismatch {
                    expected: ValueType::Function(Box::new(plan.function_type(type_))),
                    actual: other.value_type(plan),
                }),
            }
        }
        ListFunctionExprKind::CustomField(access) => {
            let (constructor, expected, function) =
                eval_custom_field_function(plan, state, frame, access)?;
            match function.kind() {
                EvaluatedFunctionValueKind::List(value) => Ok(value.clone()),
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
        ListFunctionExprKind::ListIndex { list, index, type_ } => {
            let type_ = plan.function_type(type_);
            let function = project_function_list_expr(plan, state, frame, list, *index, &type_)?;
            match function.kind() {
                EvaluatedFunctionValueKind::List(value) => Ok(value.clone()),
                _ => Err(ExecutionError::FunctionReturnFamilyMismatch {
                    expected: FunctionReturnFamily::List,
                    actual: function.kind().family(),
                }),
            }
        }
        ListFunctionExprKind::Panic(panic) => {
            eval_panic_expr(plan, state, frame, panic).map(|never| match never {})
        }
        ListFunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, state, frame, subject)? {
                eval_list_function_expr(plan, state, frame, true_)
            } else {
                eval_list_function_expr(plan, state, frame, false_)
            }
        }
        ListFunctionExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_list_function_expr(plan, state, frame, branch);
                }
            }
            eval_list_function_expr(plan, state, frame, fallback)
        }
        ListFunctionExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_list_function_expr(plan, state, frame, branch);
                }
            }
            eval_list_function_expr(plan, state, frame, fallback)
        }
        ListFunctionExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_list_function_expr(plan, state, frame, branch);
                }
            }
            eval_list_function_expr(plan, state, frame, fallback)
        }
        ListFunctionExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, state, steps, frame)?;
            eval_list_function_expr(plan, state, frame, return_)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        BoolExpr, CaptureArg, Expr, FloatExpr, FunctionId, FunctionPlan, FunctionType, IntExpr,
        IntListFunctionId, IntLocalId, ListExpr, ListFunctionExpr, ListFunctionFunctionId,
        ListFunctionId, ModulePlan, PanicExpr, PanicSite, ReturnExpr, Step, StringExpr, TupleExpr,
        ValueType,
    };
    use crate::runtime::{ExecutionError, run_main};

    #[test]
    fn source_list_function_expression_variants_evaluate_exact_values() {
        let source = r#"
fn list(value: Int) { [value] }
fn identity(value: Int) { [value] }
fn make_list(offset: Int) -> fn(Int) -> List(Int) {
  fn(value) { [value + offset] }
}

pub fn main() {
  let local = list
  let maker = make_list
  #(
    list(0),
    { let captured = 1 fn(value) { [value + captured] } }(0),
    local(2),
    make_list(1)(2),
    maker(1)(3),
    #(list).0(5),
    case [list] { [function] -> function(6) _ -> [] },
    case True { True -> list False -> identity }(7),
    case False { True -> identity False -> list }(8),
    case 1 { 1 -> list _ -> identity }(9),
    case 0 { 1 -> identity _ -> list }(10),
    case "hit" { "hit" -> list _ -> identity }(11),
    case "miss" { "hit" -> identity _ -> list }(12),
    case 1.0 { 1.0 -> list _ -> identity }(13),
    case 0.0 { 1.0 -> identity _ -> list }(14),
    { let _ = 0 list }(15),
  )
}
"#;

        assert_eq!(
            crate::runtime::run_src(source),
            crate::runtime::Value::Tuple(
                [0_i64, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
                    .into_iter()
                    .map(|value| {
                        crate::runtime::Value::List(crate::runtime::ListValue::int(vec![
                            value.into(),
                        ]))
                    })
                    .collect(),
            ),
        );
    }

    #[test]
    fn module_expression_errors_propagate_through_list_function_wrappers() {
        let item_type = ValueType::Int;
        let type_ = FunctionType::new(Vec::new(), ValueType::List(Box::new(item_type.clone())));
        let panic = |message: &str| {
            PanicExpr::panic_at(
                Some(StringExpr::value(message.into())),
                PanicSite::unknown(),
            )
        };
        let fallback =
            || ListFunctionExpr::panic(panic("fallback"), type_.clone(), item_type.clone());
        let expressions = [
            (
                ListFunctionExpr::closure(
                    ListFunctionId::Int(IntListFunctionId(1)),
                    Vec::new(),
                    vec![CaptureArg::int(
                        IntLocalId(0),
                        IntExpr::panic(panic("capture")),
                    )],
                ),
                "capture",
            ),
            (
                ListFunctionExpr::tuple_index(
                    TupleExpr::panic(
                        panic("tuple"),
                        vec![ValueType::Function(Box::new(type_.clone()))],
                    ),
                    0,
                    type_.clone(),
                    item_type.clone(),
                ),
                "tuple",
            ),
            (
                ListFunctionExpr::list_index(
                    super::super::expect_function_list(ListExpr::panic(
                        panic("list"),
                        ValueType::Function(Box::new(type_.clone())),
                    )),
                    0,
                    type_.clone(),
                    item_type.clone(),
                ),
                "list",
            ),
            (
                ListFunctionExpr::bool_case(
                    BoolExpr::panic(panic("bool subject")),
                    fallback(),
                    fallback(),
                ),
                "bool subject",
            ),
            (
                ListFunctionExpr::int_case(
                    IntExpr::panic(panic("int subject")),
                    Vec::new(),
                    fallback(),
                ),
                "int subject",
            ),
            (
                ListFunctionExpr::string_case(
                    StringExpr::panic(panic("string subject")),
                    Vec::new(),
                    fallback(),
                ),
                "string subject",
            ),
            (
                ListFunctionExpr::float_case(
                    FloatExpr::panic(panic("float subject")),
                    Vec::new(),
                    fallback(),
                ),
                "float subject",
            ),
            (
                ListFunctionExpr::block(
                    vec![Step::evaluate(Expr::int(IntExpr::panic(panic("step"))))],
                    fallback(),
                ),
                "step",
            ),
        ];

        for (expression, message) in expressions {
            assert_eq!(
                run_module_list_function_expression(expression, type_.clone()).to_string(),
                format!("panic: {message}"),
            );
        }
    }

    fn run_module_list_function_expression(
        expression: ListFunctionExpr,
        type_: FunctionType,
    ) -> ExecutionError {
        let runtime_id = ListFunctionFunctionId::from_item_type(0, type_, ValueType::Int);
        let main = FunctionPlan::new(
            FunctionId::new(0),
            "main".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::list_function(runtime_id, expression),
        );
        let module = ModulePlan::new("main".into(), main, Vec::new());
        let plan = crate::ExecutionPlan::from_module_plan(module);

        run_main(&plan).expect_err("module expression should fail at runtime")
    }
}
