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
    eval_custom_expr_kind(plan, state, frame, expression.type_id(), expression.kind())
}

pub(in crate::runtime) fn eval_custom_expr_kind(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    type_id: crate::plan::execution::CustomTypeId,
    kind: &CustomExprKind,
) -> Result<EvaluatedCustomValue, ExecutionError> {
    match kind {
        CustomExprKind::Constructor(construction) => {
            let fields = construction
                .fields()
                .iter()
                .map(|field| eval_expr(plan, state, frame, field))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(EvaluatedCustomValue::from_fields(
                construction.constructor(),
                fields.into_boxed_slice(),
            ))
        }
        CustomExprKind::LocalGet { local } => Ok(frame.get_custom(*local)),
        CustomExprKind::Call { function, args } => {
            function::run_custom_call(plan, state, *function, args, frame)
        }
        CustomExprKind::FunctionCall(call) => {
            function::run_custom_function_call(plan, state, call, frame)
        }
        CustomExprKind::TupleIndex { tuple, index } => {
            let expected = ValueType::Custom(plan.custom_value_type(type_id));
            match project_tuple_expr(plan, state, frame, tuple, *index, expected.clone())? {
                EvaluatedValue::Custom(value) => Ok(value),
                other => Err(ExecutionError::TupleIndexFamilyMismatch {
                    expected,
                    actual: other.value_type(plan),
                }),
            }
        }
        CustomExprKind::CustomField(access) => {
            let expected = ValueType::Custom(plan.custom_value_type(type_id));
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
            project_custom_list_expr(plan, state, frame, list, *index, type_id)
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
                eval_custom_expr_kind(plan, state, frame, type_id, true_)
            } else {
                eval_custom_expr_kind(plan, state, frame, type_id, false_)
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
                    return eval_custom_expr_kind(plan, state, frame, type_id, branch);
                }
            }
            eval_custom_expr_kind(plan, state, frame, type_id, fallback)
        }
        CustomExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_custom_expr_kind(plan, state, frame, type_id, branch);
                }
            }
            eval_custom_expr_kind(plan, state, frame, type_id, fallback)
        }
        CustomExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_custom_expr_kind(plan, state, frame, type_id, branch);
                }
            }
            eval_custom_expr_kind(plan, state, frame, type_id, fallback)
        }
        CustomExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, state, steps, frame)?;
            eval_custom_expr_kind(plan, state, frame, type_id, return_)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        BoolExpr, CustomConstructor, CustomConstructorDefinition, CustomConstructorField,
        CustomExpr, CustomFieldDefinition, CustomReturn, CustomType, CustomTypeDefinition,
        CustomTypeName, CustomTypePublicity, CustomTypeTemplate, Expr, FloatExpr, FunctionTemplate,
        FunctionTemplateId, IntExpr, ModulePlan, PanicExpr, PanicSite, ReturnExpr, Step,
        StringExpr, TupleExpr, ValueType,
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
        let expression = crate::plan::CustomExpr::tuple_index_shape(
            TupleExpr::value(
                vec![Expr::int(IntExpr::value(1.into()))],
                vec![ValueType::Custom(type_.clone())],
            ),
            0,
            crate::plan::CustomValueShape::any(type_.clone()),
        );

        assert_eq!(
            run_module_custom_expression(expression),
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
            CustomExpr::try_constructor(
                boxed_constructor(),
                vec![Expr::int(IntExpr::value(1.into()))],
            )
            .expect("test custom construction should be valid")
        };
        let wrap = |expression| {
            CustomExpr::try_constructor(wrapper_constructor(), vec![Expr::custom(expression)])
                .expect("test custom wrapper construction should be valid")
        };
        let expressions = [
            CustomExpr::try_constructor(
                boxed_constructor(),
                vec![Expr::int(IntExpr::panic(panic()))],
            )
            .expect("test custom construction should be valid"),
            CustomExpr::tuple_index_shape(
                TupleExpr::panic(panic(), vec![ValueType::Custom(boxed_type())]),
                0,
                crate::plan::CustomValueShape::any(boxed_type()),
            ),
            wrap(CustomExpr::bool_case(
                BoolExpr::panic(panic()),
                crate::plan::CustomBoolCaseBranches::try_new(value(), value())
                    .expect("matching custom branches should be valid"),
            )),
            wrap(CustomExpr::int_case(
                IntExpr::panic(panic()),
                crate::plan::CustomCaseBranches::try_new(Vec::new(), value())
                    .expect("matching custom branches should be valid"),
            )),
            wrap(CustomExpr::string_case(
                StringExpr::panic(panic()),
                crate::plan::CustomCaseBranches::try_new(Vec::new(), value())
                    .expect("matching custom branches should be valid"),
            )),
            wrap(CustomExpr::float_case(
                FloatExpr::panic(panic()),
                crate::plan::CustomCaseBranches::try_new(Vec::new(), value())
                    .expect("matching custom branches should be valid"),
            )),
            wrap(CustomExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::panic(panic())))],
                value(),
            )),
            wrap(CustomExpr::bool_case(
                BoolExpr::value(true),
                crate::plan::CustomBoolCaseBranches::try_new(
                    CustomExpr::panic(panic(), boxed_type()),
                    value(),
                )
                .expect("matching custom branches should be valid"),
            )),
            wrap(CustomExpr::int_case(
                IntExpr::value(1.into()),
                crate::plan::CustomCaseBranches::try_new(
                    vec![(1.into(), CustomExpr::panic(panic(), boxed_type()))],
                    value(),
                )
                .expect("matching custom branches should be valid"),
            )),
            wrap(CustomExpr::string_case(
                StringExpr::value("hit".into()),
                crate::plan::CustomCaseBranches::try_new(
                    vec![("hit".into(), CustomExpr::panic(panic(), boxed_type()))],
                    value(),
                )
                .expect("matching custom branches should be valid"),
            )),
            wrap(CustomExpr::float_case(
                FloatExpr::value(1.0),
                crate::plan::CustomCaseBranches::try_new(
                    vec![(1.0, CustomExpr::panic(panic(), boxed_type()))],
                    value(),
                )
                .expect("matching custom branches should be valid"),
            )),
            wrap(CustomExpr::block(
                Vec::new(),
                CustomExpr::panic(panic(), boxed_type()),
            )),
        ];

        for expression in expressions {
            assert_eq!(
                run_module_custom_expression(expression).to_string(),
                "panic: `panic` expression evaluated.",
            );
        }
    }

    fn run_module_custom_expression(expression: crate::plan::CustomExpr) -> ExecutionError {
        let main = FunctionTemplate::new(
            FunctionTemplateId::new(0),
            "main".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::custom_body(CustomReturn::expr(expression)),
        );
        let module = ModulePlan::new("main".into(), main, Vec::new())
            .with_custom_types(vec![boxed_definition(), wrapper_definition()]);
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

    fn wrapper_definition() -> CustomTypeDefinition {
        CustomTypeDefinition::new(
            CustomTypeName::new("geam".into(), "main".into(), "Wrapper".into()),
            CustomTypePublicity::Private,
            false,
            Vec::new(),
            vec![CustomConstructorDefinition::new(
                "Wrapper".into(),
                0,
                vec![CustomFieldDefinition::new(
                    None,
                    CustomTypeTemplate::Custom {
                        name: CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
                        arguments: Vec::new(),
                    },
                )],
            )],
        )
    }

    fn wrapper_constructor() -> CustomConstructor {
        CustomConstructor::new(
            CustomType::new(
                CustomTypeName::new("geam".into(), "main".into(), "Wrapper".into()),
                Vec::new(),
            ),
            "Wrapper".into(),
            0,
            vec![CustomConstructorField::new(
                None,
                ValueType::Custom(CustomType::new(
                    CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
                    Vec::new(),
                )),
            )],
        )
    }

    fn boxed_name() -> CustomTypeName {
        CustomTypeName::new("geam".into(), "main".into(), "Boxed".into())
    }
}
