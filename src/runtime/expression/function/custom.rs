use super::eval_custom_field_function;
use crate::plan::ValueType;
use crate::plan::execution::{
    CustomFunctionExpr, CustomFunctionExprKind, CustomFunctionType, ExecutionPlan,
    FunctionReturnFamily,
};
use crate::runtime::evaluated::{
    EvaluatedCustomFunction, EvaluatedFunctionValueKind, EvaluatedValue,
};
use crate::runtime::expression::{
    eval_bool_expr, eval_direct_call, eval_float_expr, eval_function_call, eval_int_expr,
    eval_panic_expr, eval_string_expr, project_function_list_expr, project_tuple_expr,
};
use crate::runtime::frame::Frame;
use crate::runtime::state::RuntimeState;
use crate::runtime::{ExecutionError, InvariantError, function};

pub(in crate::runtime) fn eval_custom_function_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &CustomFunctionExpr,
) -> Result<EvaluatedCustomFunction, ExecutionError> {
    eval_custom_function_expr_kind(
        plan,
        state,
        frame,
        expression.custom_function_type(),
        expression.kind(),
    )
}

pub(in crate::runtime) fn eval_custom_function_expr_kind(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    type_: &CustomFunctionType,
    kind: &CustomFunctionExprKind,
) -> Result<EvaluatedCustomFunction, ExecutionError> {
    match kind {
        CustomFunctionExprKind::Constant(value) => {
            eval_custom_function_expr(plan, state, frame, plan.constant(*value))
        }
        CustomFunctionExprKind::Constructor(constructor) => Ok(
            EvaluatedCustomFunction::constructor(*constructor, type_.to_function_type()),
        ),
        CustomFunctionExprKind::Reference(reference) => Ok(EvaluatedCustomFunction::reference(
            *reference.function(),
            reference.param_locals(),
            Vec::new(),
            crate::runtime::evaluated::function_type_from_slots(
                plan,
                reference.params(),
                crate::plan::execution::ValueType::Custom(type_.return_().type_id()),
            ),
        )),
        CustomFunctionExprKind::Closure(closure) => Ok(EvaluatedCustomFunction::closure(
            *closure.function(),
            closure.param_locals(),
            function::eval_capture_args(plan, state, frame, closure.captures())?,
            type_.to_function_type(),
        )),
        CustomFunctionExprKind::LocalGet { local } => Ok(frame.get_custom_function(local)),
        CustomFunctionExprKind::Call(call) => eval_direct_call(
            plan,
            state,
            frame,
            call,
            |plan, state, function, args, frame| {
                function::run_custom_function_returning_function_call(
                    plan,
                    state,
                    function.clone(),
                    args,
                    frame,
                )
            },
        ),
        CustomFunctionExprKind::FunctionCall(call) => eval_function_call(
            plan,
            state,
            frame,
            call,
            function::run_custom_function_function_call,
        ),
        CustomFunctionExprKind::TupleIndex { tuple, index } => {
            let expected =
                ValueType::Function(Box::new(plan.function_type(&type_.to_function_type())));
            let value = project_tuple_expr(plan, state, frame, tuple, *index, expected.clone())?;
            let actual = value.value_type(plan);
            match value {
                EvaluatedValue::Function(function) => match function.kind() {
                    EvaluatedFunctionValueKind::Custom(value) if actual == expected => {
                        Ok(value.clone())
                    }
                    _ => Err(ExecutionError::Invariant(
                        InvariantError::TupleIndexFamilyMismatch { expected, actual },
                    )),
                },
                _ => Err(ExecutionError::Invariant(
                    InvariantError::TupleIndexFamilyMismatch { expected, actual },
                )),
            }
        }
        CustomFunctionExprKind::CustomField(access) => {
            let (constructor, expected, function) =
                eval_custom_field_function(plan, state, frame, access)?;
            let actual = ValueType::Function(Box::new(plan.function_type(function.type_())));
            match function.kind() {
                EvaluatedFunctionValueKind::Custom(value) if actual == expected => {
                    Ok(value.clone())
                }
                _ => {
                    let descriptor = plan.custom_constructor(constructor);
                    Err(ExecutionError::Invariant(
                        InvariantError::CustomFieldFamilyMismatch {
                            custom_type: plan.custom_value_type(constructor.type_id()),
                            constructor: descriptor.name().clone(),
                            field_index: access.index(),
                            expected,
                            actual,
                        },
                    ))
                }
            }
        }
        CustomFunctionExprKind::ListIndex { list, index } => {
            let public_type = plan.function_type(&type_.to_function_type());
            let function =
                project_function_list_expr(plan, state, frame, list, *index, &public_type)?;
            match function.kind() {
                EvaluatedFunctionValueKind::Custom(value) => Ok(value.clone()),
                _ => Err(ExecutionError::Invariant(
                    InvariantError::FunctionReturnFamilyMismatch {
                        expected: FunctionReturnFamily::Custom,
                        actual: function.kind().family(),
                    },
                )),
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
                eval_custom_function_expr_kind(plan, state, frame, type_, true_)
            } else {
                eval_custom_function_expr_kind(plan, state, frame, type_, false_)
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
                    return eval_custom_function_expr_kind(plan, state, frame, type_, branch);
                }
            }
            eval_custom_function_expr_kind(plan, state, frame, type_, fallback)
        }
        CustomFunctionExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_custom_function_expr_kind(plan, state, frame, type_, branch);
                }
            }
            eval_custom_function_expr_kind(plan, state, frame, type_, fallback)
        }
        CustomFunctionExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_custom_function_expr_kind(plan, state, frame, type_, branch);
                }
            }
            eval_custom_function_expr_kind(plan, state, frame, type_, fallback)
        }
        CustomFunctionExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, state, steps, frame)?;
            eval_custom_function_expr_kind(plan, state, frame, type_, return_)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        BoolExpr, CaptureArg, CustomConstructorDefinition, CustomFunctionExpr, CustomFunctionLocal,
        CustomFunctionLocalId, CustomFunctionReference, CustomFunctionReturn, CustomFunctionType,
        CustomReturn, CustomType, CustomTypeDefinition, CustomTypeName, CustomTypePublicity, Expr,
        FloatExpr, FunctionExpr, FunctionListExpr, FunctionShape, FunctionTemplate,
        FunctionTemplateId, FunctionType, IntExpr, IntFunctionExpr, IntFunctionId,
        IntFunctionReference, IntLocalId, ListExpr, ModulePlan, PanicExpr, PanicSite, Param,
        ParamLocal, ReturnExpr, Step, StringExpr, TupleExpr, ValueShape, ValueType,
        monomorphic_function_instantiation,
    };
    use crate::runtime::{ExecutionError, InvariantError, run_main};

    fn custom_function_instantiation(
        template: usize,
        type_: &CustomFunctionType,
    ) -> crate::plan::FunctionInstantiation {
        monomorphic_function_instantiation(
            template,
            FunctionShape::new(
                type_.argument_shapes().to_vec(),
                ValueShape::Custom(type_.return_().clone()),
            ),
        )
    }

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
    fn returned_constructor_callables_preserve_arity_through_nested_returns() {
        let source = r#"
pub type Boxed {
  Boxed(Int)
}

fn tail_factory(remaining: Int) -> fn(Int) -> Boxed {
  case remaining {
    0 -> Boxed
    _ -> tail_factory(remaining - 1)
  }
}

fn choose_factory(flag: Bool) -> fn(Int) -> Boxed {
  case flag {
    True -> Boxed
    False -> tail_factory(1)
  }
}

fn nested_factory() -> fn() -> fn(Int) -> Boxed {
  fn() { choose_factory(False) }
}

pub fn main() {
  case nested_factory()()(42) {
    Boxed(value) -> value
  }
}
"#;

        assert_eq!(
            crate::runtime::run_src(source),
            crate::runtime::Value::Int(42.into()),
        );
    }

    #[test]
    fn custom_function_tuple_projection_reports_direct_mutated_family_mismatches() {
        let type_ = boxed_function_type();
        let tuple = TupleExpr::value(
            vec![Expr::function(FunctionExpr::int(
                IntFunctionExpr::reference(IntFunctionReference::new(
                    monomorphic_function_instantiation(
                        2,
                        FunctionShape::from_function_type(FunctionType::new(
                            Vec::new(),
                            ValueType::Int,
                        )),
                    ),
                    Vec::new(),
                )),
            ))],
            vec![ValueType::Function(Box::new(type_.to_function_type()))],
        );
        let expression = CustomFunctionExpr::tuple_index(tuple, 0, type_.clone());

        assert_eq!(
            run_module_custom_function_expression(expression),
            ExecutionError::Invariant(InvariantError::TupleIndexFamilyMismatch {
                expected: ValueType::Function(Box::new(type_.to_function_type())),
                actual: ValueType::Function(Box::new(FunctionType::new(
                    Vec::new(),
                    ValueType::Int,
                ))),
            }),
        );

        let tuple = TupleExpr::value(
            vec![Expr::int(IntExpr::value(1.into()))],
            vec![ValueType::Function(Box::new(type_.to_function_type()))],
        );
        let expression = CustomFunctionExpr::tuple_index(tuple, 0, type_.clone());

        assert_eq!(
            run_module_custom_function_expression(expression),
            ExecutionError::Invariant(InvariantError::TupleIndexFamilyMismatch {
                expected: ValueType::Function(Box::new(type_.to_function_type())),
                actual: ValueType::Int,
            }),
        );

        let actual_type = CustomFunctionType::new(vec![ValueType::Int], boxed_type());
        let tuple = TupleExpr::value(
            vec![Expr::function(FunctionExpr::custom(
                CustomFunctionExpr::reference(
                    CustomFunctionReference::new(
                        custom_function_instantiation(3, &actual_type),
                        vec![ParamLocal::int(IntLocalId(0))],
                    ),
                    actual_type.return_().clone(),
                ),
            ))],
            vec![ValueType::Function(Box::new(type_.to_function_type()))],
        );
        let expression = CustomFunctionExpr::tuple_index(tuple, 0, type_.clone());

        assert_eq!(
            run_module_custom_function_expression(expression),
            ExecutionError::Invariant(InvariantError::TupleIndexFamilyMismatch {
                expected: ValueType::Function(Box::new(type_.to_function_type())),
                actual: ValueType::Function(Box::new(actual_type.to_function_type())),
            }),
        );
    }

    #[test]
    fn module_child_errors_propagate_through_custom_function_wrappers() {
        let panic = || PanicExpr::panic_at(None, PanicSite::unknown());
        let type_ = boxed_function_type();
        let fallback = || {
            CustomFunctionExpr::reference(
                CustomFunctionReference::new(custom_function_instantiation(1, &type_), Vec::new()),
                type_.return_().clone(),
            )
        };
        let expressions = [
            CustomFunctionExpr::closure(
                custom_function_instantiation(1, &type_),
                Vec::new(),
                vec![CaptureArg::int(IntLocalId(0), IntExpr::panic(panic()))],
                type_.clone(),
            ),
            CustomFunctionExpr::tuple_index(
                TupleExpr::panic(
                    panic(),
                    vec![ValueType::Function(Box::new(type_.to_function_type()))],
                ),
                0,
                type_.clone(),
            ),
            CustomFunctionExpr::list_index(
                FunctionListExpr::from(ListExpr::panic(
                    panic(),
                    ValueType::Function(Box::new(type_.to_function_type())),
                )),
                0,
                type_.clone(),
            ),
            CustomFunctionExpr::bool_case(BoolExpr::panic(panic()), fallback(), fallback()),
            CustomFunctionExpr::bool_case(
                BoolExpr::not(BoolExpr::value(false)),
                CustomFunctionExpr::panic(panic(), type_.clone()),
                fallback(),
            ),
            CustomFunctionExpr::bool_case(
                BoolExpr::not(BoolExpr::value(true)),
                fallback(),
                CustomFunctionExpr::panic(panic(), type_.clone()),
            ),
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
        let custom_target = FunctionTemplate::new(
            FunctionTemplateId::new(1),
            "custom_target".into(),
            Vec::new(),
            vec![Step::evaluate(Expr::int(IntExpr::local_get(
                IntLocalId(0),
                "capture".into(),
            )))],
            ReturnExpr::custom_body(CustomReturn::expr(crate::plan::CustomExpr::panic(
                PanicExpr::panic_at(None, PanicSite::unknown()),
                boxed_type(),
            ))),
        );
        let int_target = FunctionTemplate::new(
            FunctionTemplateId::new(2),
            "int_target".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::int(
                IntFunctionId(0),
                IntExpr::panic(PanicExpr::panic_at(None, PanicSite::unknown())),
            ),
        );
        let custom_argument_target = FunctionTemplate::new(
            FunctionTemplateId::new(3),
            "custom_argument_target".into(),
            vec![Param::named(ParamLocal::int(IntLocalId(0)), "value".into())],
            Vec::new(),
            ReturnExpr::custom_body(CustomReturn::expr(crate::plan::CustomExpr::panic(
                PanicExpr::panic_at(None, PanicSite::unknown()),
                boxed_type(),
            ))),
        );
        let local = CustomFunctionLocal::new(
            CustomFunctionLocalId(0),
            expression.custom_function_type().clone(),
        );
        let main = FunctionTemplate::new(
            FunctionTemplateId::new(0),
            "main".into(),
            Vec::new(),
            vec![Step::let_custom_function(
                local.id(),
                "value".into(),
                expression,
            )],
            ReturnExpr::custom_function_body(
                0,
                CustomFunctionReturn::expr(CustomFunctionExpr::local_get(local, "value".into())),
            ),
        );
        let module = ModulePlan::new(
            "main".into(),
            main,
            vec![custom_target, int_target, custom_argument_target],
        )
        .with_custom_types(vec![boxed_definition()]);
        let plan = crate::ExecutionPlan::from_module_plan(module);

        run_main(&plan).expect_err("module expression should fail at runtime")
    }

    fn boxed_function_type() -> CustomFunctionType {
        CustomFunctionType::new(Vec::new(), boxed_type())
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
                Vec::new(),
            )],
        )
    }

    fn boxed_name() -> CustomTypeName {
        CustomTypeName::new("geam".into(), "main".into(), "Boxed".into())
    }
}
