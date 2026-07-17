use super::eval_custom_field_function;
use crate::plan::ValueType;
use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::{
    FunctionFunctionExpr, FunctionFunctionExprKind, FunctionFunctionType, FunctionReturnFamily,
};
use crate::runtime::expression::{
    eval_bool_expr, eval_float_expr, eval_int_expr, eval_panic_expr, eval_string_expr,
    project_function_list_expr, project_tuple_expr,
};
use crate::runtime::frame::Frame;
use crate::runtime::function;
use crate::runtime::state::RuntimeState;
use crate::runtime::{
    EvaluatedFunctionFunction, EvaluatedFunctionValueKind, EvaluatedValue, ExecutionError,
};

pub(in crate::runtime) fn eval_function_function_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &FunctionFunctionExpr,
) -> Result<EvaluatedFunctionFunction, ExecutionError> {
    eval_function_function_expr_kind(
        plan,
        state,
        frame,
        expression.function_function_type(),
        expression.kind(),
    )
}

pub(in crate::runtime) fn eval_function_function_expr_kind(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    type_: &FunctionFunctionType,
    kind: &FunctionFunctionExprKind,
) -> Result<EvaluatedFunctionFunction, ExecutionError> {
    match kind {
        FunctionFunctionExprKind::Reference(reference) => Ok(EvaluatedFunctionFunction::reference(
            reference.function().clone(),
            reference.param_locals(),
            Vec::new(),
            type_.to_function_type(),
        )),
        FunctionFunctionExprKind::Closure(template) => Ok(EvaluatedFunctionFunction::closure(
            template.function().clone(),
            template.param_locals(),
            function::eval_capture_args(plan, state, frame, template.captures())?,
            type_.to_function_type(),
        )),
        FunctionFunctionExprKind::LocalGet { local, .. } => Ok(frame.get_function_function(local)),
        FunctionFunctionExprKind::Call { function, args, .. } => {
            function::run_function_function_returning_function_call(
                plan,
                state,
                function.clone(),
                args,
                frame,
            )
        }
        FunctionFunctionExprKind::FunctionCall {
            function: callee,
            args,
            ..
        } => {
            function::run_function_function_function_call(plan, state, callee.as_ref(), args, frame)
        }
        FunctionFunctionExprKind::TupleIndex { tuple, index } => {
            let expected =
                ValueType::Function(Box::new(plan.function_type(&type_.to_function_type())));
            let value = project_tuple_expr(plan, state, frame, tuple, *index, expected.clone())?;
            let actual = value.value_type(plan);
            match value {
                EvaluatedValue::Function(function) => match function.kind() {
                    EvaluatedFunctionValueKind::Function(value) if actual == expected => {
                        Ok(value.clone())
                    }
                    _ => Err(ExecutionError::TupleIndexFamilyMismatch { expected, actual }),
                },
                _ => Err(ExecutionError::TupleIndexFamilyMismatch { expected, actual }),
            }
        }
        FunctionFunctionExprKind::CustomField(access) => {
            let (constructor, expected, function) =
                eval_custom_field_function(plan, state, frame, access)?;
            let actual = ValueType::Function(Box::new(plan.function_type(function.type_())));
            match function.kind() {
                EvaluatedFunctionValueKind::Function(value) if actual == expected => {
                    Ok(value.clone())
                }
                _ => {
                    let descriptor = plan.custom_constructor(constructor);
                    Err(ExecutionError::CustomFieldFamilyMismatch {
                        custom_type: plan.custom_value_type(constructor.type_id()),
                        constructor: descriptor.name().clone(),
                        field_index: access.index(),
                        expected,
                        actual,
                    })
                }
            }
        }
        FunctionFunctionExprKind::ListIndex { list, index } => {
            let public_type = plan.function_type(&type_.to_function_type());
            let function =
                project_function_list_expr(plan, state, frame, list, *index, &public_type)?;
            match function.kind() {
                EvaluatedFunctionValueKind::Function(value) => Ok(value.clone()),
                _ => Err(ExecutionError::FunctionReturnFamilyMismatch {
                    expected: FunctionReturnFamily::Function,
                    actual: function.kind().family(),
                }),
            }
        }
        FunctionFunctionExprKind::Panic(panic) => {
            eval_panic_expr(plan, state, frame, panic).map(|never| match never {})
        }
        FunctionFunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, state, frame, subject)? {
                eval_function_function_expr_kind(plan, state, frame, type_, true_)
            } else {
                eval_function_function_expr_kind(plan, state, frame, type_, false_)
            }
        }
        FunctionFunctionExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_function_function_expr_kind(plan, state, frame, type_, branch);
                }
            }
            eval_function_function_expr_kind(plan, state, frame, type_, fallback)
        }
        FunctionFunctionExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_function_function_expr_kind(plan, state, frame, type_, branch);
                }
            }
            eval_function_function_expr_kind(plan, state, frame, type_, fallback)
        }
        FunctionFunctionExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_function_function_expr_kind(plan, state, frame, type_, branch);
                }
            }
            eval_function_function_expr_kind(plan, state, frame, type_, fallback)
        }
        FunctionFunctionExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, state, steps, frame)?;
            eval_function_function_expr_kind(plan, state, frame, type_, return_)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        BoolExpr, CaptureArg, Expr, FloatExpr, FunctionFunctionExpr, FunctionFunctionLocal,
        FunctionFunctionLocalId, FunctionFunctionReference, FunctionFunctionType, FunctionTemplate,
        FunctionTemplateId, FunctionType, IntExpr, IntFunctionExpr, IntFunctionFunctionId,
        IntLocalId, ListExpr, ModulePlan, PanicExpr, PanicSite, Param, ParamLocal, ReturnExpr,
        Step, StringExpr, TupleExpr, ValueType,
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
        let type_ = FunctionFunctionType::new(Vec::new(), return_type.clone());
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
                    crate::plan::monomorphic_function_instantiation(
                        1,
                        crate::plan::FunctionShape::from_function_type(type_.to_function_type()),
                    ),
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
                FunctionFunctionExpr::tuple_index(
                    TupleExpr::panic(
                        panic("tuple"),
                        vec![ValueType::Function(Box::new(type_.to_function_type()))],
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
                        ValueType::Function(Box::new(type_.to_function_type())),
                    )),
                    2,
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

    #[test]
    fn function_function_tuple_projection_rejects_same_family_signature_mismatch() {
        let returned = FunctionType::new(Vec::new(), ValueType::Int);
        let expected_type = FunctionFunctionType::new(Vec::new(), returned.clone());
        let actual_type = FunctionFunctionType::new(vec![ValueType::Int], returned.clone());
        let actual = FunctionFunctionExpr::reference(
            FunctionFunctionReference::new(
                crate::plan::monomorphic_function_instantiation(
                    2,
                    crate::plan::FunctionShape::from_function_type(actual_type.to_function_type()),
                ),
                vec![ParamLocal::int(IntLocalId(0))],
            ),
            returned,
        );
        let tuple = TupleExpr::value(
            vec![Expr::function(crate::plan::FunctionExpr::function(actual))],
            vec![ValueType::Function(Box::new(
                expected_type.to_function_type(),
            ))],
        );
        let expression = FunctionFunctionExpr::tuple_index(tuple, 0, expected_type.clone());

        assert_eq!(
            run_module_function_function_expression(expression),
            ExecutionError::TupleIndexFamilyMismatch {
                expected: ValueType::Function(Box::new(expected_type.to_function_type())),
                actual: ValueType::Function(Box::new(actual_type.to_function_type())),
            },
        );
    }

    fn run_module_function_function_expression(expression: FunctionFunctionExpr) -> ExecutionError {
        let returned = FunctionType::new(Vec::new(), ValueType::Int);
        let closure_target = FunctionTemplate::new(
            FunctionTemplateId::new(1),
            "closure_target".into(),
            Vec::new(),
            vec![Step::evaluate(Expr::int(IntExpr::local_get(
                IntLocalId(0),
                "capture".into(),
            )))],
            ReturnExpr::int_function(
                IntFunctionFunctionId(0),
                IntFunctionExpr::panic(
                    PanicExpr::panic_at(None, PanicSite::unknown()),
                    returned.clone(),
                ),
            ),
        );
        let argument_target = FunctionTemplate::new(
            FunctionTemplateId::new(2),
            "argument_target".into(),
            vec![Param::named(ParamLocal::int(IntLocalId(0)), "value".into())],
            Vec::new(),
            ReturnExpr::int_function(
                IntFunctionFunctionId(0),
                IntFunctionExpr::panic(PanicExpr::panic_at(None, PanicSite::unknown()), returned),
            ),
        );
        let local = FunctionFunctionLocal::new(
            FunctionFunctionLocalId(0),
            expression.function_function_type().clone(),
        );
        let main = FunctionTemplate::new(
            FunctionTemplateId::new(0),
            "main".into(),
            Vec::new(),
            vec![Step::let_function_function(
                local.id(),
                "value".into(),
                expression,
            )],
            ReturnExpr::function_function(
                0,
                FunctionFunctionExpr::local_get(local, "value".into()),
            ),
        );
        let module = ModulePlan::new("main".into(), main, vec![closure_target, argument_target]);
        let plan = crate::ExecutionPlan::from_module_plan(module);

        run_main(&plan).expect_err("module expression should fail at runtime")
    }
}
