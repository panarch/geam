use super::CaptureSubstitution;
use crate::plan::{
    BitArrayListLocalId, BoolListLocalId, CallArg, CustomConstructor, CustomListLocalId, Expr,
    FloatListLocalId, FunctionListLocalId, IntListLocalId, ListFunctionLocal, ListListLocalId,
    ListLocal, NilListLocalId, ParamLocal, StringListLocalId, TupleFunctionLocalId,
    TupleListLocalId, TupleLocalId, UtfCodepointListLocalId, ValueShape, ValueType,
};
use crate::planner::context::{FunctionParam, PlanContext};
use crate::planner::error::{
    InvalidCallShapeReason, InvalidExpressionType, InvalidTypedAstReason, PlanError,
};
use gleam_core::ast::{CallArg as GleamCallArg, TypedExpr};

pub(super) fn plan_instantiated_call_args(
    arguments: Vec<GleamCallArg<TypedExpr>>,
    params: &[FunctionParam],
    instantiated_shapes: &[ValueShape],
    context: &mut PlanContext<'_>,
    capture: Option<&CaptureSubstitution>,
) -> Result<Vec<CallArg>, PlanError> {
    let mut args = Vec::with_capacity(arguments.len());
    for ((argument, param), instantiated_shape) in
        arguments.into_iter().zip(params).zip(instantiated_shapes)
    {
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
        let arg = if param.shape() == instantiated_shape {
            expression.into_call_arg(param.local())
        } else {
            Some(CallArg::parametric(param.slot().clone(), expression))
        };
        let arg = match arg {
            Some(arg) => arg,
            None => return Err(call_arg_type_mismatch(param.local().value_type(), actual)),
        };
        args.push(arg);
    }
    Ok(args)
}

pub(super) fn plan_function_call_args(
    arguments: Vec<GleamCallArg<TypedExpr>>,
    params: &[ValueShape],
    context: &mut PlanContext<'_>,
    capture: Option<&CaptureSubstitution>,
) -> Result<Vec<CallArg>, PlanError> {
    let locals = function_call_param_locals(params);
    let mut args = Vec::with_capacity(arguments.len());
    for ((argument, local), shape) in arguments.into_iter().zip(&locals).zip(params) {
        let expression = plan_argument_value(argument, shape.clone(), capture, context)?;
        let actual = expression.value_type();
        if actual == local.value_type() && !expression.shape().can_flow_to(shape) {
            return Err(call_arg_shape_mismatch());
        }
        let arg = match expression.into_call_arg(local) {
            Some(arg) => arg,
            None => return Err(call_arg_type_mismatch(local.value_type(), actual)),
        };
        args.push(arg);
    }
    Ok(args)
}

fn function_call_param_locals(params: &[ValueShape]) -> Vec<ParamLocal> {
    let mut next_generic = 0;
    let mut next_int = 0;
    let mut next_string = 0;
    let mut next_bit_array = 0;
    let mut next_utf_codepoint = 0;
    let mut next_custom = 0;
    let mut next_float = 0;
    let mut next_bool = 0;
    let mut next_nil = 0;
    let mut next_tuple = 0;
    let mut next_int_list = 0;
    let mut next_string_list = 0;
    let mut next_bit_array_list = 0;
    let mut next_utf_codepoint_list = 0;
    let mut next_custom_list = 0;
    let mut next_float_list = 0;
    let mut next_bool_list = 0;
    let mut next_nil_list = 0;
    let mut next_tuple_list = 0;
    let mut next_list_list = 0;
    let mut next_function_list = 0;
    let mut next_generic_list = 0;
    let mut next_generic_function = 0;
    let mut next_int_function = 0;
    let mut next_string_function = 0;
    let mut next_bit_array_function = 0;
    let mut next_utf_codepoint_function = 0;
    let mut next_custom_function = 0;
    let mut next_float_function = 0;
    let mut next_bool_function = 0;
    let mut next_nil_function = 0;
    let mut next_tuple_function = 0;
    let mut next_list_function = 0;
    let mut next_function_function = 0;

    params
        .iter()
        .map(|shape| match shape {
            ValueShape::Parameter(parameter) => {
                let local = ParamLocal::generic(crate::plan::GenericLocal::new(
                    crate::plan::GenericLocalId(next_generic),
                    *parameter,
                ));
                next_generic += 1;
                local
            }
            ValueShape::Int => {
                let local = ParamLocal::int(crate::plan::IntLocalId(next_int));
                next_int += 1;
                local
            }
            ValueShape::String => {
                let local = ParamLocal::string(crate::plan::StringLocalId(next_string));
                next_string += 1;
                local
            }
            ValueShape::BitArray => {
                let local = ParamLocal::bit_array(crate::plan::BitArrayLocalId(next_bit_array));
                next_bit_array += 1;
                local
            }
            ValueShape::UtfCodepoint => {
                let local =
                    ParamLocal::utf_codepoint(crate::plan::UtfCodepointLocalId(next_utf_codepoint));
                next_utf_codepoint += 1;
                local
            }
            ValueShape::Custom(custom_shape) => {
                let local = ParamLocal::custom_shape(
                    crate::plan::CustomLocalId(next_custom),
                    custom_shape.clone(),
                );
                next_custom += 1;
                local
            }
            ValueShape::Float => {
                let local = ParamLocal::float(crate::plan::FloatLocalId(next_float));
                next_float += 1;
                local
            }
            ValueShape::Bool => {
                let local = ParamLocal::bool(crate::plan::BoolLocalId(next_bool));
                next_bool += 1;
                local
            }
            ValueShape::Nil => {
                let local = ParamLocal::nil(crate::plan::NilLocalId(next_nil));
                next_nil += 1;
                local
            }
            ValueShape::Tuple(elements) => {
                let local = ParamLocal::tuple(
                    TupleLocalId(next_tuple),
                    elements.iter().map(ValueShape::value_type).collect(),
                );
                next_tuple += 1;
                local
            }
            ValueShape::List(item_shape) => {
                let local = match item_shape.as_ref() {
                    ValueShape::Parameter(parameter) => {
                        let local = ListLocal::generic(
                            crate::plan::GenericListLocalId(next_generic_list),
                            *parameter,
                        );
                        next_generic_list += 1;
                        local
                    }
                    ValueShape::Int => {
                        let local = ListLocal::int(IntListLocalId(next_int_list));
                        next_int_list += 1;
                        local
                    }
                    ValueShape::String => {
                        let local = ListLocal::string(StringListLocalId(next_string_list));
                        next_string_list += 1;
                        local
                    }
                    ValueShape::BitArray => {
                        let local = ListLocal::bit_array(BitArrayListLocalId(next_bit_array_list));
                        next_bit_array_list += 1;
                        local
                    }
                    ValueShape::UtfCodepoint => {
                        let local = ListLocal::utf_codepoint(UtfCodepointListLocalId(
                            next_utf_codepoint_list,
                        ));
                        next_utf_codepoint_list += 1;
                        local
                    }
                    ValueShape::Custom(item_shape) => {
                        let local = ListLocal::custom(
                            CustomListLocalId(next_custom_list),
                            item_shape.type_().clone(),
                        );
                        next_custom_list += 1;
                        local
                    }
                    ValueShape::Float => {
                        let local = ListLocal::float(FloatListLocalId(next_float_list));
                        next_float_list += 1;
                        local
                    }
                    ValueShape::Bool => {
                        let local = ListLocal::bool(BoolListLocalId(next_bool_list));
                        next_bool_list += 1;
                        local
                    }
                    ValueShape::Nil => {
                        let local = ListLocal::nil(NilListLocalId(next_nil_list));
                        next_nil_list += 1;
                        local
                    }
                    ValueShape::Tuple(item_shape) => {
                        let local = ListLocal::tuple(
                            TupleListLocalId(next_tuple_list),
                            item_shape.iter().map(ValueShape::value_type).collect(),
                        );
                        next_tuple_list += 1;
                        local
                    }
                    ValueShape::List(item_shape) => {
                        let local = ListLocal::list(
                            ListListLocalId(next_list_list),
                            item_shape.value_type(),
                        );
                        next_list_list += 1;
                        local
                    }
                    ValueShape::Function(item_shape) => {
                        let local = ListLocal::function(
                            FunctionListLocalId(next_function_list),
                            item_shape.type_(),
                        );
                        next_function_list += 1;
                        local
                    }
                };
                ParamLocal::list(local)
            }
            ValueShape::Function(function_shape) => match function_shape.return_shape() {
                ValueShape::Parameter(parameter) => {
                    let local =
                        ParamLocal::generic_function(crate::plan::GenericFunctionLocal::new(
                            crate::plan::GenericFunctionLocalId(next_generic_function),
                            crate::plan::GenericFunctionType::new(
                                function_shape.argument_shapes().to_vec(),
                                *parameter,
                            ),
                        ));
                    next_generic_function += 1;
                    local
                }
                ValueShape::Int => {
                    let local = ParamLocal::int_function(
                        crate::plan::IntFunctionLocalId(next_int_function),
                        function_shape.type_(),
                    );
                    next_int_function += 1;
                    local
                }
                ValueShape::String => {
                    let local = ParamLocal::string_function(
                        crate::plan::StringFunctionLocalId(next_string_function),
                        function_shape.type_(),
                    );
                    next_string_function += 1;
                    local
                }
                ValueShape::BitArray => {
                    let local = ParamLocal::bit_array_function(
                        crate::plan::BitArrayFunctionLocalId(next_bit_array_function),
                        function_shape.type_(),
                    );
                    next_bit_array_function += 1;
                    local
                }
                ValueShape::UtfCodepoint => {
                    let local = ParamLocal::utf_codepoint_function(
                        crate::plan::UtfCodepointFunctionLocalId(next_utf_codepoint_function),
                        function_shape.type_(),
                    );
                    next_utf_codepoint_function += 1;
                    local
                }
                ValueShape::Custom(return_shape) => {
                    let local = ParamLocal::custom_function(crate::plan::CustomFunctionLocal::new(
                        crate::plan::CustomFunctionLocalId(next_custom_function),
                        crate::plan::CustomFunctionType::from_shapes(
                            function_shape.argument_shapes().to_vec(),
                            return_shape.clone(),
                        ),
                    ));
                    next_custom_function += 1;
                    local
                }
                ValueShape::Float => {
                    let local = ParamLocal::float_function(
                        crate::plan::FloatFunctionLocalId(next_float_function),
                        function_shape.type_(),
                    );
                    next_float_function += 1;
                    local
                }
                ValueShape::Bool => {
                    let local = ParamLocal::bool_function(
                        crate::plan::BoolFunctionLocalId(next_bool_function),
                        function_shape.type_(),
                    );
                    next_bool_function += 1;
                    local
                }
                ValueShape::Nil => {
                    let local = ParamLocal::nil_function(
                        crate::plan::NilFunctionLocalId(next_nil_function),
                        function_shape.type_(),
                    );
                    next_nil_function += 1;
                    local
                }
                ValueShape::Tuple(_) => {
                    let local = ParamLocal::tuple_function(
                        TupleFunctionLocalId(next_tuple_function),
                        function_shape.type_(),
                    );
                    next_tuple_function += 1;
                    local
                }
                ValueShape::List(item_shape) => {
                    let local = ParamLocal::list_function(ListFunctionLocal::from_item_type(
                        next_list_function,
                        function_shape.type_(),
                        item_shape.value_type(),
                    ));
                    next_list_function += 1;
                    local
                }
                ValueShape::Function(return_shape) => {
                    let local =
                        ParamLocal::function_function(crate::plan::FunctionFunctionLocal::new(
                            crate::plan::FunctionFunctionLocalId(next_function_function),
                            crate::plan::FunctionFunctionType::from_shapes(
                                function_shape.argument_shapes().to_vec(),
                                return_shape.as_ref().clone(),
                            ),
                        ));
                    next_function_function += 1;
                    local
                }
            },
        })
        .collect()
}

pub(super) fn plan_custom_constructor_args(
    arguments: Vec<GleamCallArg<TypedExpr>>,
    constructor: &CustomConstructor,
    context: &mut PlanContext<'_>,
    capture: Option<&CaptureSubstitution>,
) -> Result<Vec<Expr>, PlanError> {
    arguments
        .into_iter()
        .enumerate()
        .map(|(index, argument)| {
            let Some(field) = constructor.fields().get(index) else {
                return Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::CallShape {
                        reason: InvalidCallShapeReason::FunctionCallArityMismatch,
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
    use super::{function_call_param_locals, plan_function_call_args, plan_instantiated_call_args};
    use crate::plan::{
        BitArrayListLocalId, BoolListLocalId, CustomConstructorDefinition,
        CustomConstructorRefinement, CustomListLocalId, CustomLocalId, CustomType,
        CustomTypeDefinition, CustomTypeName, CustomTypePublicity, CustomValueShape,
        FloatListLocalId, FunctionListLocalId, FunctionType, IntListLocalId, ListListLocalId,
        ListLocal, NilListLocalId, ParamBinding, ParamLocal, StringListLocalId, StringLocalId,
        TupleListLocalId, UtfCodepointListLocalId, ValueShape, ValueType,
    };
    use crate::planner::context::{AnonymousFunctions, FunctionParam, PlanContext};
    use crate::planner::plan_module;
    use crate::planner::support::{compile, expect_plan_error};
    use crate::planner::{
        InvalidCallShapeReason, InvalidExpressionType, InvalidTypedAstReason, PlanError,
        UnsupportedExpressionKind,
    };
    use ecow::EcoString;
    use gleam_core::ast::{CallArg as GleamCallArg, Statement, TypedExpr, TypedModule};
    use gleam_core::type_::Type;
    use std::collections::HashMap;
    use std::sync::Arc;

    #[test]
    fn instantiated_call_argument_rejects_corrupted_parameter_local_family() {
        let module = compile("fn identity(value: Int) { value } pub fn main() { identity(1) }");
        let arguments = main_call_arguments(&module);
        let module_name = EcoString::from("main");
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module_name, &functions, &mut anonymous);
        let param = FunctionParam::new(
            ParamLocal::string(StringLocalId(0)),
            ValueShape::Int,
            ParamBinding::Named("value".into()),
            None,
        );

        assert_eq!(
            plan_instantiated_call_args(
                arguments,
                &[param],
                &[ValueShape::Int],
                &mut context,
                None,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::String,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );
    }

    #[test]
    fn function_call_param_locals_preserve_family_local_order() {
        let shapes = [
            ValueType::Int,
            ValueType::Float,
            ValueType::String,
            ValueType::BitArray,
            ValueType::UtfCodepoint,
            ValueType::Custom(custom_type()),
            ValueType::Bool,
            ValueType::Nil,
            ValueType::Int,
            ValueType::List(Box::new(ValueType::Int)),
            ValueType::List(Box::new(ValueType::String)),
            ValueType::List(Box::new(ValueType::BitArray)),
            ValueType::List(Box::new(ValueType::UtfCodepoint)),
            ValueType::List(Box::new(ValueType::Custom(custom_type()))),
            ValueType::List(Box::new(ValueType::Float)),
            ValueType::List(Box::new(ValueType::Bool)),
            ValueType::List(Box::new(ValueType::Nil)),
            ValueType::List(Box::new(ValueType::Tuple(vec![ValueType::Int]))),
            ValueType::List(Box::new(ValueType::List(Box::new(ValueType::String)))),
            ValueType::List(Box::new(ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::Int],
                ValueType::String,
            ))))),
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::Int],
                ValueType::Int,
            ))),
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::String],
                ValueType::String,
            ))),
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::BitArray],
                ValueType::BitArray,
            ))),
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::UtfCodepoint],
                ValueType::UtfCodepoint,
            ))),
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::Float],
                ValueType::Float,
            ))),
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::Bool],
                ValueType::Bool,
            ))),
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::Nil],
                ValueType::Nil,
            ))),
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::Tuple(vec![ValueType::Int])],
                ValueType::Tuple(vec![ValueType::Int]),
            ))),
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::List(Box::new(ValueType::Int))],
                ValueType::List(Box::new(ValueType::Int)),
            ))),
            ValueType::Function(Box::new(FunctionType::new(
                Vec::new(),
                ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int))),
            ))),
        ]
        .into_iter()
        .map(ValueShape::from_value_type)
        .collect::<Vec<_>>();
        assert_eq!(
            function_call_param_locals(&shapes),
            vec![
                ParamLocal::int(crate::plan::IntLocalId(0)),
                ParamLocal::float(crate::plan::FloatLocalId(0)),
                ParamLocal::string(crate::plan::StringLocalId(0)),
                ParamLocal::bit_array(crate::plan::BitArrayLocalId(0)),
                ParamLocal::utf_codepoint(crate::plan::UtfCodepointLocalId(0)),
                ParamLocal::custom(CustomLocalId(0), custom_type()),
                ParamLocal::bool(crate::plan::BoolLocalId(0)),
                ParamLocal::nil(crate::plan::NilLocalId(0)),
                ParamLocal::int(crate::plan::IntLocalId(1)),
                ParamLocal::list(ListLocal::int(IntListLocalId(0))),
                ParamLocal::list(ListLocal::string(StringListLocalId(0))),
                ParamLocal::list(ListLocal::bit_array(BitArrayListLocalId(0))),
                ParamLocal::list(ListLocal::utf_codepoint(UtfCodepointListLocalId(0))),
                ParamLocal::list(ListLocal::custom(CustomListLocalId(0), custom_type())),
                ParamLocal::list(ListLocal::float(FloatListLocalId(0))),
                ParamLocal::list(ListLocal::bool(BoolListLocalId(0))),
                ParamLocal::list(ListLocal::nil(NilListLocalId(0))),
                ParamLocal::list(ListLocal::tuple(TupleListLocalId(0), vec![ValueType::Int])),
                ParamLocal::list(ListLocal::list(ListListLocalId(0), ValueType::String)),
                ParamLocal::list(ListLocal::function(
                    FunctionListLocalId(0),
                    FunctionType::new(vec![ValueType::Int], ValueType::String),
                )),
                ParamLocal::int_function(
                    crate::plan::IntFunctionLocalId(0),
                    FunctionType::new(vec![ValueType::Int], ValueType::Int),
                ),
                ParamLocal::string_function(
                    crate::plan::StringFunctionLocalId(0),
                    FunctionType::new(vec![ValueType::String], ValueType::String),
                ),
                ParamLocal::bit_array_function(
                    crate::plan::BitArrayFunctionLocalId(0),
                    FunctionType::new(vec![ValueType::BitArray], ValueType::BitArray),
                ),
                ParamLocal::utf_codepoint_function(
                    crate::plan::UtfCodepointFunctionLocalId(0),
                    FunctionType::new(vec![ValueType::UtfCodepoint], ValueType::UtfCodepoint,),
                ),
                ParamLocal::float_function(
                    crate::plan::FloatFunctionLocalId(0),
                    FunctionType::new(vec![ValueType::Float], ValueType::Float),
                ),
                ParamLocal::bool_function(
                    crate::plan::BoolFunctionLocalId(0),
                    FunctionType::new(vec![ValueType::Bool], ValueType::Bool),
                ),
                ParamLocal::nil_function(
                    crate::plan::NilFunctionLocalId(0),
                    FunctionType::new(vec![ValueType::Nil], ValueType::Nil),
                ),
                ParamLocal::tuple_function(
                    crate::plan::TupleFunctionLocalId(0),
                    FunctionType::new(
                        vec![ValueType::Tuple(vec![ValueType::Int])],
                        ValueType::Tuple(vec![ValueType::Int]),
                    ),
                ),
                ParamLocal::list_function(crate::plan::ListFunctionLocal::from_item_type(
                    0,
                    FunctionType::new(
                        vec![ValueType::List(Box::new(ValueType::Int))],
                        ValueType::List(Box::new(ValueType::Int)),
                    ),
                    ValueType::Int,
                )),
                ParamLocal::function_function(crate::plan::FunctionFunctionLocal::new(
                    crate::plan::FunctionFunctionLocalId(0),
                    crate::plan::FunctionFunctionType::new(
                        Vec::new(),
                        FunctionType::new(Vec::new(), ValueType::Int),
                    ),
                )),
            ],
        );
        assert_eq!(
            function_call_param_locals(&[ValueShape::Int])[0],
            ParamLocal::int(crate::plan::IntLocalId(0)),
        );
    }

    #[test]
    fn custom_constructor_argument_planning_errors_are_preserved() {
        assert_eq!(
            expect_plan_error("pub type Boxed { Boxed(Int) } pub fn main() { Boxed(echo 1) }",),
            PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::Echo,
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
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: crate::planner::InvalidExpressionShapeKind::Invalid,
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
        let param = FunctionParam::new(
            ParamLocal::custom(CustomLocalId(0), choice_type()),
            expected.clone(),
            ParamBinding::Named("value".into()),
            None,
        );

        assert_eq!(
            plan_instantiated_call_args(arguments, &[param], &[expected], &mut context, None,),
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

    fn custom_type() -> CustomType {
        CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        )
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
