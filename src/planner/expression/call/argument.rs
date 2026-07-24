use super::CaptureSubstitution;
use crate::plan::{CallArg, CustomConstructor, Expr, ValueShape, ValueType};
use crate::planner::context::PlanContext;
use crate::planner::error::{
    InvalidCallShapeReason, InvalidExpressionType, InvalidTypedAstReason, PlanError,
};
use gleam_core::ast::{CallArg as GleamCallArg, TypedExpr};

pub(super) fn plan_instantiated_call_args(
    arguments: Vec<GleamCallArg<TypedExpr>>,
    instantiated_shapes: &[ValueShape],
    context: &mut PlanContext<'_>,
    capture: Option<&CaptureSubstitution>,
) -> Result<Vec<CallArg>, PlanError> {
    let mut args = Vec::with_capacity(arguments.len());
    for (argument, instantiated_shape) in arguments.into_iter().zip(instantiated_shapes) {
        let expression =
            plan_argument_value(argument, instantiated_shape.clone(), capture, context)?;
        let actual = expression.value_type();
        let expected = instantiated_shape.value_type();
        if actual != expected {
            return Err(call_arg_type_mismatch(expected, actual));
        }
        if !expression.shape().can_flow_to(instantiated_shape) {
            return Err(call_arg_shape_mismatch());
        }
        args.push(CallArg::new(expression));
    }
    Ok(args)
}

pub(super) fn plan_function_call_args(
    arguments: Vec<GleamCallArg<TypedExpr>>,
    params: &[ValueShape],
    context: &mut PlanContext<'_>,
    capture: Option<&CaptureSubstitution>,
) -> Result<Vec<CallArg>, PlanError> {
    let mut args = Vec::with_capacity(arguments.len());
    for (argument, shape) in arguments.into_iter().zip(params) {
        let expression = plan_argument_value(argument, shape.clone(), capture, context)?;
        let actual = expression.value_type();
        let expected = shape.value_type();
        if actual == expected && !expression.shape().can_flow_to(shape) {
            return Err(call_arg_shape_mismatch());
        }
        if actual != expected {
            return Err(call_arg_type_mismatch(expected, actual));
        }
        args.push(CallArg::new(expression));
    }
    Ok(args)
}

pub(super) fn plan_custom_constructor_args(
    arguments: Vec<GleamCallArg<TypedExpr>>,
    constructor: &CustomConstructor,
    context: &mut PlanContext<'_>,
    capture: Option<&CaptureSubstitution>,
) -> Result<Vec<Expr>, PlanError> {
    let actual = arguments.len();
    arguments
        .into_iter()
        .enumerate()
        .map(|(index, argument)| {
            let Some(field) = constructor.fields().get(index) else {
                return Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::CallShape {
                        reason: InvalidCallShapeReason::RecordConstructorExtraArguments {
                            expected: constructor.fields().len(),
                            actual,
                        },
                    },
                });
            };
            if let Some(label) = &argument.label
                && field.label() != Some(label)
            {
                return Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::CallShape {
                        reason: InvalidCallShapeReason::LabelledArguments,
                    },
                });
            }
            let expression = plan_argument_value(
                argument,
                ValueShape::from_value_type(field.type_().clone()),
                capture,
                context,
            )?;
            if expression.value_type() != *field.type_() {
                return Err(call_arg_type_mismatch(
                    field.type_().clone(),
                    expression.value_type(),
                ));
            }
            Ok(expression)
        })
        .collect()
}

fn call_arg_type_mismatch(expected: ValueType, actual: ValueType) -> PlanError {
    if matches!(expected, ValueType::Function(_)) && matches!(actual, ValueType::Function(_)) {
        PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CallShape {
                reason: InvalidCallShapeReason::FunctionCallArgumentTypeMismatch,
            },
        }
    } else {
        PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionType {
                expected: InvalidExpressionType::from_value_type(expected),
                actual: InvalidExpressionType::from_value_type(actual),
            },
        }
    }
}

fn call_arg_shape_mismatch() -> PlanError {
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::CallShape {
            reason: InvalidCallShapeReason::FunctionCallArgumentTypeMismatch,
        },
    }
}

fn plan_argument_value(
    argument: GleamCallArg<TypedExpr>,
    expected: ValueShape,
    capture: Option<&CaptureSubstitution>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    if let Some(capture) = capture
        && super::is_capture_local(&argument.value, &capture.name)
    {
        return Ok(capture.value.clone());
    }

    super::super::plan_expr_with_expected_source_stop_shape(argument.value, expected, context)
}

#[cfg(test)]
#[allow(clippy::arc_with_non_send_sync)]
mod tests {
    use super::{plan_function_call_args, plan_instantiated_call_args};
    use crate::plan::{
        CustomConstructorDefinition, CustomConstructorRefinement, CustomType, CustomTypeDefinition,
        CustomTypeName, CustomTypePublicity, CustomValueShape, ValueShape,
    };
    use crate::planner::context::{AnonymousFunctions, PlanContext};
    use crate::planner::plan_module;
    use crate::planner::support::{compile, expect_plan_error};
    use crate::planner::{
        InvalidCallShapeReason, InvalidTypedAstReason, PlanError, UnsupportedBitArraySegmentReason,
    };
    use ecow::EcoString;
    use gleam_core::ast::{CallArg as GleamCallArg, Statement, TypedExpr, TypedModule};
    use gleam_core::type_::Type;
    use std::collections::HashMap;
    use std::sync::Arc;

    #[test]
    fn custom_constructor_argument_planning_errors_are_preserved() {
        assert_eq!(
            expect_plan_error(
                "pub type Boxed { Boxed(Int) } pub fn main() { Boxed({ <<1:native>> 1 }) }",
            ),
            PlanError::UnsupportedBitArraySegment {
                reason: UnsupportedBitArraySegmentReason::NativeEndianness,
            },
        );
    }

    #[test]
    fn call_arguments_reject_incompatible_constructor_refinements() {
        let expected = PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CallShape {
                reason: InvalidCallShapeReason::FunctionCallArgumentTypeMismatch,
            },
        };
        let source = r#"
pub type Choice {
  First
  Second
}

fn expect_first(value: Choice) {
  Nil
}

pub fn main() {
  expect_first(Second)
}
"#;
        let mut direct = compile(source);
        set_first_param_constructor(&mut direct, "expect_first", 0);
        assert_eq!(plan_module(direct), Err(expected.clone()));

        let mut function_value = compile(
            r#"
pub type Choice {
  First
  Second
}

fn expect_first(value: Choice) {
  Nil
}

pub fn main() {
  let function = expect_first
  function(Second)
}
"#,
        );
        set_first_param_constructor(&mut function_value, "expect_first", 0);
        assert_eq!(
            plan_module(function_value),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "main".into(),
                    name: "expect_first".into(),
                    reason: crate::planner::InvalidModuleReferenceReason::FunctionReferenceShape,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_instantiated_call_argument_constructor_refinement() {
        let module = compile(
            r#"
pub type Choice {
  First
  Second
}

fn consume(value: Choice) {
  Nil
}

pub fn main() {
  consume(Second)
}
"#,
        );
        let arguments = main_call_arguments(&module);
        let module_name = EcoString::from("main");
        let functions = HashMap::new();
        let custom_types = vec![choice_definition()];
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new_with_custom_types(
            &module_name,
            &functions,
            &custom_types,
            &mut anonymous,
        );
        let expected = ValueShape::Custom(CustomValueShape::new(
            choice_type().type_name().clone(),
            Vec::new(),
            CustomConstructorRefinement::Exact(0),
        ));
        assert_eq!(
            plan_instantiated_call_args(arguments, &[expected], &mut context, None,),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::FunctionCallArgumentTypeMismatch,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_function_value_arguments_reject_incompatible_constructor_refinements() {
        let module = compile(
            r#"
pub type Choice {
  First
  Second
}

fn consume(value: Choice) {
  Nil
}

pub fn main() {
  consume(Second)
}
"#,
        );
        let arguments = main_call_arguments(&module);
        let module_name = EcoString::from("main");
        let functions = HashMap::new();
        let custom_types = vec![choice_definition()];
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new_with_custom_types(
            &module_name,
            &functions,
            &custom_types,
            &mut anonymous,
        );
        let expected = ValueShape::Custom(CustomValueShape::new(
            choice_type().type_name().clone(),
            Vec::new(),
            CustomConstructorRefinement::Exact(0),
        ));

        assert_eq!(
            plan_function_call_args(arguments, &[expected], &mut context, None),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::FunctionCallArgumentTypeMismatch,
                },
            }),
        );
    }

    #[test]
    #[should_panic(expected = "test parameter should have a named custom type")]
    fn custom_parameter_fixture_guard_rejects_tuple_parameter() {
        let mut module = compile("fn identity(value: #(Int)) { value } pub fn main() { 1 }");

        set_first_param_constructor(&mut module, "identity", 0);
    }

    #[test]
    #[should_panic(expected = "test main body should be a call")]
    fn main_call_arguments_fixture_guard_rejects_non_call() {
        let module = compile("pub fn main() { Nil }");

        let _ = main_call_arguments(&module);
    }

    fn set_first_param_constructor(module: &mut TypedModule, function_name: &str, index: u16) {
        let function = module
            .definitions
            .functions
            .iter_mut()
            .find(|function| {
                function
                    .name
                    .as_ref()
                    .is_some_and(|name| name.1 == function_name)
            })
            .expect("test function should exist");
        let Type::Named {
            publicity,
            package,
            module,
            name,
            arguments,
            ..
        } = function.arguments[0].type_.as_ref()
        else {
            panic!("test parameter should have a named custom type");
        };
        function.arguments[0].type_ = Arc::new(Type::Named {
            publicity: *publicity,
            package: package.clone(),
            module: module.clone(),
            name: name.clone(),
            arguments: arguments.clone(),
            inferred_variant: Some(index),
        });
    }

    fn main_call_arguments(module: &TypedModule) -> Vec<GleamCallArg<TypedExpr>> {
        let main = module
            .definitions
            .functions
            .iter()
            .find(|function| function.name.as_ref().is_some_and(|name| name.1 == "main"))
            .expect("test main function should exist");
        let Statement::Expression(TypedExpr::Call { arguments, .. }) = &main.body[0] else {
            panic!("test main body should be a call");
        };
        arguments.clone()
    }

    fn choice_type() -> CustomType {
        CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Choice".into()),
            Vec::new(),
        )
    }

    fn choice_definition() -> CustomTypeDefinition {
        CustomTypeDefinition::new(
            choice_type().type_name().clone(),
            CustomTypePublicity::Public,
            false,
            Vec::new(),
            vec![
                CustomConstructorDefinition::new("First".into(), 0, Vec::new()),
                CustomConstructorDefinition::new("Second".into(), 1, Vec::new()),
            ],
        )
    }
}
