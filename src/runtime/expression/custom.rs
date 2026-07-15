use super::{
    eval_bool_expr, eval_custom_field, eval_expr, eval_float_expr, eval_int_expr, eval_panic_expr,
    eval_string_expr, project_custom_list_expr, project_tuple_expr,
};
use crate::plan::ValueType;
use crate::plan::execution::{CustomExpr, CustomExprKind, ExecutionPlan};
use crate::runtime::evaluated::{EvaluatedCustomValue, EvaluatedValue};
use crate::runtime::frame::Frame;
use crate::runtime::state::RuntimeState;
use crate::runtime::{ExecutionError, function};

pub(in crate::runtime) fn eval_custom_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &CustomExpr,
) -> Result<EvaluatedCustomValue, ExecutionError> {
    match expression.kind() {
        CustomExprKind::Constructor {
            constructor,
            arguments,
        } => {
            let fields = arguments
                .iter()
                .map(|argument| eval_expr(plan, state, frame, argument))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(EvaluatedCustomValue::new(*constructor, fields))
        }
        CustomExprKind::LocalGet { local } => Ok(frame.get_custom(*local)),
        CustomExprKind::Call { function, args } => {
            function::run_custom_call(plan, state, *function, args, frame)
        }
        CustomExprKind::FunctionCall { function, args } => {
            function::run_custom_function_call(plan, state, function, args, frame)
        }
        CustomExprKind::TupleIndex { tuple, index } => {
            let expected = ValueType::Custom(plan.custom_value_type(expression.type_id()));
            match project_tuple_expr(plan, state, frame, tuple, *index, expected.clone())? {
                EvaluatedValue::Custom(value) => Ok(value),
                other => Err(ExecutionError::TupleIndexFamilyMismatch {
                    expected,
                    actual: other.value_type(plan),
                }),
            }
        }
        CustomExprKind::CustomField(access) => {
            let expected = ValueType::Custom(plan.custom_value_type(expression.type_id()));
            let (constructor, value) = eval_custom_field(plan, state, frame, access)?;
            match value {
                EvaluatedValue::Custom(value) => Ok(value),
                other => {
                    let descriptor = plan.custom_constructor(constructor);
                    Err(ExecutionError::CustomFieldFamilyMismatch {
                        custom_type: plan.custom_value_type(constructor.type_id()),
                        constructor: descriptor.name().clone(),
                        field_index: access.index(),
                        expected,
                        actual: other.value_type(plan),
                    })
                }
            }
        }
        CustomExprKind::ListIndex { list, index } => {
            project_custom_list_expr(plan, state, frame, list, *index, expression.type_id())
        }
        CustomExprKind::Panic(panic) => {
            eval_panic_expr(plan, state, frame, panic).map(|never| match never {})
        }
        CustomExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, state, frame, subject)? {
                eval_custom_expr(plan, state, frame, true_)
            } else {
                eval_custom_expr(plan, state, frame, false_)
            }
        }
        CustomExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_custom_expr(plan, state, frame, branch);
                }
            }
            eval_custom_expr(plan, state, frame, fallback)
        }
        CustomExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_custom_expr(plan, state, frame, branch);
                }
            }
            eval_custom_expr(plan, state, frame, fallback)
        }
        CustomExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_custom_expr(plan, state, frame, branch);
                }
            }
            eval_custom_expr(plan, state, frame, fallback)
        }
        CustomExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, state, steps, frame)?;
            eval_custom_expr(plan, state, frame, return_)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        BoolExpr, CustomConstructor, CustomConstructorDefinition, CustomConstructorField,
        CustomExpr, CustomFieldDefinition, CustomFunctionId, CustomReturn, CustomType,
        CustomTypeDefinition, CustomTypeName, CustomTypePublicity, CustomTypeTemplate, Expr,
        FloatExpr, FunctionId, FunctionPlan, IntExpr, ModulePlan, PanicExpr, PanicSite, ReturnExpr,
        Step, StringExpr, TupleExpr, ValueType,
    };
    use crate::runtime::{ExecutionError, run_main};

    #[test]
    fn source_custom_expression_variants_evaluate_exact_values() {
        let source = r#"
pub type Boxed {
  Boxed(Int)
}

fn boxed(value: Int) -> Boxed { Boxed(value) }

fn unbox(value: Boxed) -> Int {
  case value { Boxed(inner) -> inner }
}

pub fn main() {
  let local = Boxed(1)
  let function = boxed
  #(
    unbox(local),
    unbox(boxed(2)),
    unbox(function(3)),
    unbox(#(Boxed(4)).0),
    case [Boxed(5)] { [Boxed(value)] -> value _ -> 0 },
    unbox(case True { True -> Boxed(6) False -> Boxed(0) }),
    unbox(case False { True -> Boxed(0) False -> Boxed(7) }),
    unbox(case 1 { 1 -> Boxed(8) _ -> Boxed(0) }),
    unbox(case 0 { 1 -> Boxed(0) _ -> Boxed(9) }),
    unbox(case "hit" { "hit" -> Boxed(10) _ -> Boxed(0) }),
    unbox(case "miss" { "hit" -> Boxed(0) _ -> Boxed(11) }),
    unbox(case 1.0 { 1.0 -> Boxed(12) _ -> Boxed(0) }),
    unbox(case 0.0 { 1.0 -> Boxed(0) _ -> Boxed(13) }),
    unbox({ let _ = 0 Boxed(14) }),
  )
}
"#;

        assert_eq!(
            crate::runtime::run_src(source),
            crate::runtime::Value::Tuple(
                (1_i64..=14)
                    .map(|value| crate::runtime::Value::Int(value.into()))
                    .collect(),
            ),
        );
    }

    #[test]
    fn custom_tuple_projection_reports_direct_mutated_family_mismatch() {
        let type_ = boxed_type();
        let expression = crate::plan::CustomExpr::tuple_index(
            TupleExpr::value(
                vec![Expr::int(IntExpr::value(1.into()))],
                vec![ValueType::Custom(type_.clone())],
            ),
            0,
            type_.clone(),
        );

        assert_eq!(
            run_module_custom_expression(expression, type_.clone()),
            ExecutionError::TupleIndexFamilyMismatch {
                expected: ValueType::Custom(type_),
                actual: ValueType::Int,
            },
        );
    }

    #[test]
    fn module_child_errors_propagate_through_custom_expression_wrappers() {
        let panic = || PanicExpr::panic_at(None, PanicSite::unknown());
        let value = || {
            CustomExpr::constructor(
                boxed_constructor(),
                vec![Expr::int(IntExpr::value(1.into()))],
            )
        };
        let expressions = [
            CustomExpr::constructor(
                boxed_constructor(),
                vec![Expr::int(IntExpr::panic(panic()))],
            ),
            CustomExpr::tuple_index(
                TupleExpr::panic(panic(), vec![ValueType::Custom(boxed_type())]),
                0,
                boxed_type(),
            ),
            CustomExpr::bool_case(BoolExpr::panic(panic()), value(), value()),
            CustomExpr::int_case(IntExpr::panic(panic()), Vec::new(), value()),
            CustomExpr::string_case(StringExpr::panic(panic()), Vec::new(), value()),
            CustomExpr::float_case(FloatExpr::panic(panic()), Vec::new(), value()),
            CustomExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::panic(panic())))],
                value(),
            ),
        ];

        for expression in expressions {
            assert_eq!(
                run_module_custom_expression(expression, boxed_type()).to_string(),
                "panic: `panic` expression evaluated.",
            );
        }
    }

    fn run_module_custom_expression(
        expression: crate::plan::CustomExpr,
        type_: CustomType,
    ) -> ExecutionError {
        let main = FunctionPlan::new(
            FunctionId::new(0),
            "main".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::custom_body(
                CustomFunctionId(0),
                type_.clone(),
                CustomReturn::expr(expression),
            ),
        );
        let module = ModulePlan::new("main".into(), main, Vec::new())
            .with_custom_types(vec![boxed_definition()]);
        let plan = crate::ExecutionPlan::from_module_plan(module);

        run_main(&plan).expect_err("module expression should fail at runtime")
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
            vec![CustomConstructorDefinition::new(
                "Boxed".into(),
                0,
                vec![CustomFieldDefinition::new(None, CustomTypeTemplate::Int)],
            )],
        )
    }

    fn boxed_constructor() -> CustomConstructor {
        CustomConstructor::new(
            boxed_type(),
            "Boxed".into(),
            0,
            vec![CustomConstructorField::new(None, ValueType::Int)],
        )
    }

    fn boxed_name() -> CustomTypeName {
        CustomTypeName::new("geam".into(), "main".into(), "Boxed".into())
    }
}
