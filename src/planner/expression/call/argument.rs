use super::CaptureSubstitution;
use crate::plan::{CallArg, CustomConstructor, Expr, ValueShape, ValueType};
use crate::planner::context::{FunctionParam, PlanContext};
use crate::planner::error::{InvalidCallShapeReason, InvalidTypedAstReason, PlanError};
use gleam_core::ast::{CallArg as GleamCallArg, TypedExpr};

pub(super) struct NormalizedCallArguments {
    values: Vec<GleamCallArg<TypedExpr>>,
}

impl NormalizedCallArguments {
    pub(super) fn ordinary(values: Vec<GleamCallArg<TypedExpr>>) -> Result<Self, PlanError> {
        if let Some((index, _)) = values
            .iter()
            .enumerate()
            .find(|(_, argument)| argument.implicit.is_some())
        {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::ImplicitArgument { index },
                },
            });
        }
        Ok(Self { values })
    }

    pub(super) fn specialized(values: Vec<GleamCallArg<TypedExpr>>) -> Self {
        Self { values }
    }

    pub(super) fn for_direct(self, params: &[FunctionParam]) -> Result<Self, PlanError> {
        validate_argument_count(params.len(), self.values.len())?;
        validate_argument_labels(
            &self.values,
            params.iter().map(|param| param.label.as_ref()),
        )?;
        Ok(self)
    }

    pub(super) fn for_function_value(self, expected: usize) -> Result<Self, PlanError> {
        validate_argument_count(expected, self.values.len())?;
        validate_argument_labels(&self.values, std::iter::repeat_n(None, expected))?;
        Ok(self)
    }

    pub(super) fn for_constructor(
        self,
        constructor: &CustomConstructor,
    ) -> Result<Self, PlanError> {
        validate_constructor_argument_count(constructor.fields().len(), self.values.len())?;
        validate_argument_labels(
            &self.values,
            constructor.fields().iter().map(|field| field.label()),
        )?;
        Ok(self)
    }

    fn into_values(self) -> Vec<GleamCallArg<TypedExpr>> {
        self.values
    }

    pub(super) fn iter(&self) -> impl Iterator<Item = &GleamCallArg<TypedExpr>> {
        self.values.iter()
    }
}

fn validate_argument_count(expected: usize, actual: usize) -> Result<(), PlanError> {
    if expected == actual {
        return Ok(());
    }
    Err(PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::CallShape {
            reason: InvalidCallShapeReason::ArgumentCount { expected, actual },
        },
    })
}

fn validate_constructor_argument_count(expected: usize, actual: usize) -> Result<(), PlanError> {
    if expected == actual {
        return Ok(());
    }
    Err(PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::CallShape {
            reason: InvalidCallShapeReason::RecordConstructorArgumentCount { expected, actual },
        },
    })
}

fn validate_argument_labels<'a>(
    arguments: &[GleamCallArg<TypedExpr>],
    expected: impl IntoIterator<Item = Option<&'a ecow::EcoString>>,
) -> Result<(), PlanError> {
    for (index, (argument, expected)) in arguments.iter().zip(expected).enumerate() {
        if let Some(actual) = &argument.label
            && Some(actual) != expected
        {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::ArgumentLabel {
                        index,
                        expected: expected.cloned(),
                        actual: actual.clone(),
                    },
                },
            });
        }
    }
    Ok(())
}

pub(super) fn plan_call_args(
    arguments: NormalizedCallArguments,
    shapes: &[ValueShape],
    context: &mut PlanContext<'_>,
    capture: Option<&CaptureSubstitution>,
) -> Result<Vec<CallArg>, PlanError> {
    let mut args = Vec::with_capacity(shapes.len());
    for (index, (argument, shape)) in arguments.into_values().into_iter().zip(shapes).enumerate() {
        let actual = context.value_shape(&argument.value.type_()).value_type();
        validate_call_arg_type(index, &shape.value_type(), &actual)?;
        let expression = plan_argument_value(argument, shape.clone(), capture, context)?;
        validate_call_arg_shape(index, shape, &expression)?;
        args.push(CallArg::new(expression));
    }
    Ok(args)
}

pub(super) fn plan_custom_constructor_args(
    arguments: NormalizedCallArguments,
    constructor: &CustomConstructor,
    context: &mut PlanContext<'_>,
    capture: Option<&CaptureSubstitution>,
) -> Result<Vec<Expr>, PlanError> {
    arguments
        .into_values()
        .into_iter()
        .enumerate()
        .map(|(index, argument)| {
            let field = &constructor.fields()[index];
            let actual = context.value_shape(&argument.value.type_()).value_type();
            validate_call_arg_type(index, field.type_(), &actual)?;
            let expression = plan_argument_value(
                argument,
                ValueShape::from_value_type(field.type_().clone()),
                capture,
                context,
            )?;
            Ok(expression)
        })
        .collect()
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

fn validate_call_arg_shape(
    index: usize,
    expected_shape: &ValueShape,
    expression: &Expr,
) -> Result<(), PlanError> {
    let expected = expected_shape.value_type();
    if expression.shape().can_flow_to(expected_shape) {
        return Ok(());
    }
    Err(PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::CallShape {
            reason: InvalidCallShapeReason::ArgumentShape {
                index,
                type_: expected,
            },
        },
    })
}

fn validate_call_arg_type(
    index: usize,
    expected: &ValueType,
    actual: &ValueType,
) -> Result<(), PlanError> {
    if expected != actual {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CallShape {
                reason: InvalidCallShapeReason::ArgumentType {
                    index,
                    expected: expected.clone(),
                    actual: actual.clone(),
                },
            },
        });
    }
    Ok(())
}

#[cfg(test)]
#[allow(clippy::arc_with_non_send_sync)]
mod tests {
    use super::{NormalizedCallArguments, plan_call_args};
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
                reason: InvalidCallShapeReason::FunctionInstantiationArgumentShape {
                    index: 0,
                    expected: crate::plan::ValueType::Custom(choice_type()),
                    actual: crate::plan::ValueType::Custom(choice_type()),
                },
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
        let expected_type = expected.value_type();
        assert_eq!(
            plan_call_args(
                NormalizedCallArguments::specialized(arguments),
                &[expected],
                &mut context,
                None,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::ArgumentShape {
                        index: 0,
                        type_: expected_type,
                    },
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
        let expected_type = expected.value_type();

        assert_eq!(
            plan_call_args(
                NormalizedCallArguments::specialized(arguments),
                &[expected],
                &mut context,
                None,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::ArgumentShape {
                        index: 0,
                        type_: expected_type,
                    },
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
