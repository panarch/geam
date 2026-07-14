use crate::plan::ValueType;
use crate::plan::execution::{
    CustomFunctionExpr, CustomFunctionExprKind, ExecutionPlan, FunctionReturnFamily,
};
use crate::runtime::evaluated::{
    EvaluatedCustomFunction, EvaluatedCustomFunctionTarget, EvaluatedFunctionValueKind,
    EvaluatedValue, function_type,
};
use crate::runtime::expression::{
    eval_bool_expr, eval_float_expr, eval_int_expr, eval_panic_expr, eval_string_expr,
    project_function_list_expr, project_tuple_expr,
};
use crate::runtime::frame::Frame;
use crate::runtime::state::RuntimeState;
use crate::runtime::{ExecutionError, function};

pub(in crate::runtime) fn eval_custom_function_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &CustomFunctionExpr,
) -> Result<EvaluatedCustomFunction, ExecutionError> {
    match expression.kind() {
        CustomFunctionExprKind::Constructor(constructor) => Ok(EvaluatedCustomFunction::new(
            EvaluatedCustomFunctionTarget::Constructor(*constructor),
            Vec::new(),
            Vec::new(),
            expression.type_().clone(),
        )),
        CustomFunctionExprKind::Reference(reference) => Ok(EvaluatedCustomFunction::new(
            EvaluatedCustomFunctionTarget::Function(*reference.function()),
            reference.params().to_vec(),
            Vec::new(),
            function_type(reference.params(), expression.type_().return_().clone()),
        )),
        CustomFunctionExprKind::Closure(template) => Ok(EvaluatedCustomFunction::new(
            EvaluatedCustomFunctionTarget::Function(*template.function()),
            template.params().to_vec(),
            function::eval_capture_args(plan, state, frame, template.captures())?,
            expression.type_().clone(),
        )),
        CustomFunctionExprKind::LocalGet { local } => Ok(frame.get_custom_function(*local)),
        CustomFunctionExprKind::Call { function, args } => {
            function::run_custom_function_returning_function_call(
                plan, state, *function, args, frame,
            )
        }
        CustomFunctionExprKind::FunctionCall { function, args } => {
            function::run_custom_function_function_call(plan, state, function, args, frame)
        }
        CustomFunctionExprKind::TupleIndex {
            tuple,
            index,
            type_,
        } => {
            let expected = ValueType::Function(Box::new(plan.function_type(type_)));
            let value = project_tuple_expr(plan, state, frame, tuple, *index, expected.clone())?;
            let actual = value.value_type(plan);
            match value {
                EvaluatedValue::Function(function) => match function.kind() {
                    EvaluatedFunctionValueKind::Custom(value) => Ok(value.clone()),
                    _ => Err(ExecutionError::TupleIndexFamilyMismatch { expected, actual }),
                },
                _ => Err(ExecutionError::TupleIndexFamilyMismatch { expected, actual }),
            }
        }
        CustomFunctionExprKind::ListIndex { list, index, type_ } => {
            let type_ = plan.function_type(type_);
            let function = project_function_list_expr(plan, state, frame, list, *index, &type_)?;
            match function.kind() {
                EvaluatedFunctionValueKind::Custom(value) => Ok(value.clone()),
                _ => Err(ExecutionError::FunctionReturnFamilyMismatch {
                    expected: FunctionReturnFamily::Custom,
                    actual: function.kind().family(),
                }),
            }
        }
        CustomFunctionExprKind::Panic(panic) => {
            eval_panic_expr(plan, state, frame, panic).map(|never| match never {})
        }
        CustomFunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, state, frame, subject)? {
                eval_custom_function_expr(plan, state, frame, true_)
            } else {
                eval_custom_function_expr(plan, state, frame, false_)
            }
        }
        CustomFunctionExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_custom_function_expr(plan, state, frame, branch);
                }
            }
            eval_custom_function_expr(plan, state, frame, fallback)
        }
        CustomFunctionExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_custom_function_expr(plan, state, frame, branch);
                }
            }
            eval_custom_function_expr(plan, state, frame, fallback)
        }
        CustomFunctionExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_custom_function_expr(plan, state, frame, branch);
                }
            }
            eval_custom_function_expr(plan, state, frame, fallback)
        }
        CustomFunctionExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, state, steps, frame)?;
            eval_custom_function_expr(plan, state, frame, return_)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        BoolExpr, CaptureArg, CustomFunctionExpr, CustomFunctionFunctionId, CustomFunctionId,
        CustomFunctionReference, CustomFunctionReturn, CustomType, CustomTypeDefinition,
        CustomTypeName, CustomTypePublicity, Expr, FloatExpr, FunctionExpr, FunctionId,
        FunctionListExpr, FunctionPlan, FunctionType, IntExpr, IntFunctionExpr, IntFunctionId,
        IntFunctionReference, IntLocalId, ListExpr, ModulePlan, PanicExpr, PanicSite, ReturnExpr,
        Step, StringExpr, TupleExpr, ValueType,
    };
    use crate::runtime::{ExecutionError, run_main};

    #[test]
    fn source_custom_function_expression_variants_evaluate_exact_values() {
        let source = r#"
pub type Boxed {
  Boxed(Int)
}

fn boxed(value: Int) -> Boxed { Boxed(value) }
fn identity(value: Int) -> Boxed { Boxed(value) }

fn make_boxer(offset: Int) -> fn(Int) -> Boxed {
  fn(value) { Boxed(value + offset) }
}

fn unbox(value: Boxed) -> Int {
  case value { Boxed(inner) -> inner }
}

pub fn main() {
  let constructor: fn(Int) -> Boxed = Boxed
  let local = boxed
  let maker = make_boxer
  #(
    unbox(constructor(0)),
    unbox({ let captured = 1 fn(value) { Boxed(value + captured) } }(0)),
    unbox(local(2)),
    unbox(make_boxer(1)(2)),
    unbox(maker(1)(3)),
    unbox(#(boxed).0(5)),
    case [boxed] { [function] -> unbox(function(6)) _ -> 0 },
    unbox(case True { True -> boxed False -> identity }(7)),
    unbox(case False { True -> identity False -> boxed }(8)),
    unbox(case 1 { 1 -> boxed _ -> identity }(9)),
    unbox(case 0 { 1 -> identity _ -> boxed }(10)),
    unbox(case "hit" { "hit" -> boxed _ -> identity }(11)),
    unbox(case "miss" { "hit" -> identity _ -> boxed }(12)),
    unbox(case 1.0 { 1.0 -> boxed _ -> identity }(13)),
    unbox(case 0.0 { 1.0 -> identity _ -> boxed }(14)),
    unbox({ let _ = 0 boxed }(15)),
  )
}
"#;

        assert_eq!(
            crate::runtime::run_src(source),
            crate::runtime::Value::Tuple(
                (0_i64..=15)
                    .map(|value| crate::runtime::Value::Int(value.into()))
                    .collect(),
            ),
        );
    }

    #[test]
    fn custom_function_tuple_projection_reports_direct_mutated_family_mismatches() {
        let type_ = boxed_function_type();
        let tuple = TupleExpr::value(
            vec![Expr::function(FunctionExpr::int(
                IntFunctionExpr::reference(IntFunctionReference::new(IntFunctionId(0), Vec::new())),
            ))],
            vec![ValueType::Function(Box::new(type_.clone()))],
        );
        let expression = CustomFunctionExpr::tuple_index(tuple, 0, type_.clone());

        assert_eq!(
            run_module_custom_function_expression(expression),
            ExecutionError::TupleIndexFamilyMismatch {
                expected: ValueType::Function(Box::new(type_.clone())),
                actual: ValueType::Function(Box::new(FunctionType::new(
                    Vec::new(),
                    ValueType::Int,
                ))),
            },
        );

        let tuple = TupleExpr::value(
            vec![Expr::int(IntExpr::value(1.into()))],
            vec![ValueType::Function(Box::new(type_.clone()))],
        );
        let expression = CustomFunctionExpr::tuple_index(tuple, 0, type_.clone());

        assert_eq!(
            run_module_custom_function_expression(expression),
            ExecutionError::TupleIndexFamilyMismatch {
                expected: ValueType::Function(Box::new(type_)),
                actual: ValueType::Int,
            },
        );
    }

    #[test]
    fn module_child_errors_propagate_through_custom_function_wrappers() {
        let panic = || PanicExpr::panic_at(None, PanicSite::unknown());
        let fallback = || {
            CustomFunctionExpr::reference(
                CustomFunctionReference::new(CustomFunctionId(0), Vec::new()),
                boxed_type(),
            )
        };
        let type_ = boxed_function_type();
        let expressions = [
            CustomFunctionExpr::closure(
                CustomFunctionId(0),
                Vec::new(),
                vec![CaptureArg::int(IntLocalId(0), IntExpr::panic(panic()))],
                type_.clone(),
            ),
            CustomFunctionExpr::tuple_index(
                TupleExpr::panic(panic(), vec![ValueType::Function(Box::new(type_.clone()))]),
                0,
                type_.clone(),
            ),
            CustomFunctionExpr::list_index(
                FunctionListExpr::from(ListExpr::panic(
                    panic(),
                    ValueType::Function(Box::new(type_.clone())),
                )),
                0,
                type_.clone(),
            ),
            CustomFunctionExpr::bool_case(BoolExpr::panic(panic()), fallback(), fallback()),
            CustomFunctionExpr::int_case(IntExpr::panic(panic()), Vec::new(), fallback()),
            CustomFunctionExpr::string_case(StringExpr::panic(panic()), Vec::new(), fallback()),
            CustomFunctionExpr::float_case(FloatExpr::panic(panic()), Vec::new(), fallback()),
            CustomFunctionExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::panic(panic())))],
                fallback(),
            ),
        ];

        for expression in expressions {
            assert_eq!(
                run_module_custom_function_expression(expression).to_string(),
                "panic: `panic` expression evaluated.",
            );
        }
    }

    fn run_module_custom_function_expression(expression: CustomFunctionExpr) -> ExecutionError {
        let type_ = expression.type_().clone();
        let main = FunctionPlan::new(
            FunctionId::new(0),
            "main".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::custom_function_body(
                CustomFunctionFunctionId(0),
                type_,
                CustomFunctionReturn::expr(expression),
            ),
        );
        let module = ModulePlan::new("main".into(), main, Vec::new())
            .with_custom_types(vec![boxed_definition()]);
        let plan = crate::ExecutionPlan::from_module_plan(module);

        run_main(&plan).expect_err("module expression should fail at runtime")
    }

    fn boxed_function_type() -> FunctionType {
        FunctionType::new(Vec::new(), ValueType::Custom(boxed_type()))
    }

    fn boxed_type() -> CustomType {
        CustomType::new(boxed_name(), Vec::new())
    }

    fn boxed_definition() -> CustomTypeDefinition {
        CustomTypeDefinition::new(
            boxed_name(),
            CustomTypePublicity::Private,
            false,
            Vec::new(),
            Vec::new(),
        )
    }

    fn boxed_name() -> CustomTypeName {
        CustomTypeName::new("geam".into(), "main".into(), "Boxed".into())
    }
}
