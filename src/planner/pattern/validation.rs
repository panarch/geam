use crate::plan::{ValueShape, ValueType};
use crate::planner::context::PlanContext;
use crate::planner::error::{
    InvalidPatternShapeReason, InvalidTypedAstReason, PatternKind, PlanError,
};
use ecow::EcoString;
use gleam_core::analyse::Inferred;
use gleam_core::ast::{Pattern, TailPattern, TypedPattern};
use gleam_core::type_::{PRELUDE_MODULE_NAME, PatternConstructor, Type};

pub(in crate::planner) fn validate_pattern(
    pattern: &TypedPattern,
    expected: &ValueShape,
    context: &PlanContext<'_>,
) -> Result<(), PlanError> {
    match pattern {
        Pattern::Int { .. } => validate_pattern_type(expected, ValueShape::Int),
        Pattern::Float { .. } => validate_pattern_type(expected, ValueShape::Float),
        Pattern::String { .. } | Pattern::StringPrefix { .. } => {
            validate_pattern_type(expected, ValueShape::String)
        }
        Pattern::Variable { type_, .. } | Pattern::Discard { type_, .. } => {
            validate_pattern_type(expected, context.value_shape_in_scope(type_.as_ref()))
        }
        Pattern::Assign { pattern, .. } => validate_pattern(pattern, expected, context),
        Pattern::Tuple { .. } => validate_tuple_pattern(pattern, expected, context).map(drop),
        Pattern::List { .. } => validate_list_pattern(pattern, expected, context).map(drop),
        Pattern::Constructor {
            arguments,
            constructor,
            spread,
            type_,
            ..
        } => {
            let actual = context.value_shape_in_scope(type_.as_ref());
            validate_pattern_type(expected, actual.clone())?;
            let constructor = resolved_constructor(constructor)?;
            match &actual {
                ValueShape::Custom(_) => {
                    let field_types = arguments
                        .iter()
                        .map(|argument| {
                            pattern_value_shape(&argument.value, context)
                                .map(|shape| shape.value_type())
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                    let resolved = context.custom_pattern_constructor(
                        type_.as_ref(),
                        constructor,
                        field_types,
                    )?;
                    let constructor = resolved.into_constructor();
                    for (argument, field) in arguments.iter().zip(constructor.fields()) {
                        validate_pattern(
                            &argument.value,
                            &ValueShape::from_value_type(field.type_().clone()),
                            context,
                        )?;
                    }
                }
                _ => validate_constructor_pattern(
                    constructor,
                    arguments.len(),
                    spread.is_some(),
                    &actual.value_type(),
                )?,
            }
            Ok(())
        }
        Pattern::BitArray { segments, .. } => {
            validate_pattern_type(expected, ValueShape::BitArray)?;
            for segment in segments {
                let expected = context.value_shape_in_scope(segment.type_.as_ref());
                validate_pattern(segment.value.as_ref(), &expected, context)?;
            }
            Ok(())
        }
        Pattern::BitArraySize(_) | Pattern::Invalid { .. } => {
            pattern_value_shape(pattern, context).map(drop)
        }
    }
}

pub(in crate::planner) fn unexpected_pattern(
    pattern: &TypedPattern,
    expected: &ValueShape,
    context: &PlanContext<'_>,
) -> PlanError {
    match validate_pattern(pattern, expected, context) {
        Err(error) => error,
        Ok(()) => pattern_kind_mismatch(pattern, expected),
    }
}

pub(in crate::planner) fn pattern_value_shape(
    pattern: &TypedPattern,
    context: &PlanContext<'_>,
) -> Result<ValueShape, PlanError> {
    let shape = match pattern {
        Pattern::Int { .. } => ValueShape::Int,
        Pattern::Float { .. } => ValueShape::Float,
        Pattern::String { .. } | Pattern::StringPrefix { .. } => ValueShape::String,
        Pattern::Variable { type_, .. }
        | Pattern::Discard { type_, .. }
        | Pattern::List { type_, .. }
        | Pattern::Constructor { type_, .. } => context.value_shape_in_scope(type_.as_ref()),
        Pattern::Tuple { elements, .. } => tuple_pattern_shape(elements, context)?,
        Pattern::BitArray { .. } => ValueShape::BitArray,
        Pattern::Assign { pattern, .. } => pattern_value_shape(pattern, context)?,
        Pattern::BitArraySize(_) => {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: InvalidPatternShapeReason::BitArraySizeNode,
                },
            });
        }
        Pattern::Invalid { .. } => {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: InvalidPatternShapeReason::InvalidNode,
                },
            });
        }
    };
    Ok(shape)
}

pub(in crate::planner) fn pattern_kind(pattern: &TypedPattern) -> PatternKind {
    match pattern {
        Pattern::Int { .. } => PatternKind::Int,
        Pattern::Float { .. } => PatternKind::Float,
        Pattern::String { .. } => PatternKind::String,
        Pattern::Variable { .. } => PatternKind::Variable,
        Pattern::BitArraySize(_) => PatternKind::BitArraySize,
        Pattern::Assign { .. } => PatternKind::Assign,
        Pattern::Discard { .. } => PatternKind::Discard,
        Pattern::List { .. } => PatternKind::List,
        Pattern::Constructor { .. } => PatternKind::Constructor,
        Pattern::Tuple { .. } => PatternKind::Tuple,
        Pattern::BitArray { .. } => PatternKind::BitArray,
        Pattern::StringPrefix { .. } => PatternKind::StringPrefix,
        Pattern::Invalid { .. } => PatternKind::Invalid,
    }
}

fn tuple_pattern_shape(
    elements: &[TypedPattern],
    context: &PlanContext<'_>,
) -> Result<ValueShape, PlanError> {
    Ok(ValueShape::Tuple(
        elements
            .iter()
            .map(|element| pattern_value_shape(element, context))
            .collect::<Result<Vec<_>, _>>()?
            .into_boxed_slice(),
    ))
}

pub(in crate::planner) fn validate_pattern_type(
    expected: &ValueShape,
    actual: ValueShape,
) -> Result<(), PlanError> {
    validate_pattern_value_type(expected.value_type(), actual.value_type())
}

pub(in crate::planner) fn validate_pattern_value_type(
    expected: ValueType,
    actual: ValueType,
) -> Result<(), PlanError> {
    if expected != actual {
        return Err(pattern_type_mismatch(expected, actual));
    }
    Ok(())
}

pub(in crate::planner) fn pattern_value_type_from_gleam(
    type_: &Type,
) -> Result<ValueType, PlanError> {
    ValueType::from_gleam(type_).ok_or(PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::PatternShape {
            reason: InvalidPatternShapeReason::UnsupportedType,
        },
    })
}

pub(in crate::planner) struct ValidatedTuplePattern {
    pub(in crate::planner) element_shapes: Box<[ValueShape]>,
}

pub(in crate::planner) fn validate_tuple_pattern(
    pattern: &TypedPattern,
    expected: &ValueShape,
    context: &PlanContext<'_>,
) -> Result<ValidatedTuplePattern, PlanError> {
    let Pattern::Tuple { elements, .. } = pattern else {
        return Err(pattern_kind_mismatch(pattern, expected));
    };
    let ValueShape::Tuple(expected_elements) = expected else {
        return Err(pattern_type_mismatch(
            expected.value_type(),
            tuple_pattern_shape(elements, context)?.value_type(),
        ));
    };
    validate_tuple_arity(expected_elements.len(), elements.len())?;
    for (element, expected) in elements.iter().zip(expected_elements) {
        validate_pattern(element, expected, context)?;
    }
    Ok(ValidatedTuplePattern {
        element_shapes: expected_elements.clone(),
    })
}

pub(in crate::planner) fn validate_tuple_arity(
    expected: usize,
    actual: usize,
) -> Result<(), PlanError> {
    if expected != actual {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PatternShape {
                reason: InvalidPatternShapeReason::TupleArity { expected, actual },
            },
        });
    }
    Ok(())
}

pub(in crate::planner) enum ValidatedListTail {
    Named(EcoString),
    Discard,
}

pub(in crate::planner) struct ValidatedListPattern {
    pub(in crate::planner) item_shape: ValueShape,
    pub(in crate::planner) tail: Option<ValidatedListTail>,
}

pub(in crate::planner) fn validate_list_pattern(
    pattern: &TypedPattern,
    expected: &ValueShape,
    context: &PlanContext<'_>,
) -> Result<ValidatedListPattern, PlanError> {
    let Pattern::List {
        elements,
        tail,
        type_,
        ..
    } = pattern
    else {
        return Err(pattern_kind_mismatch(pattern, expected));
    };
    let ValueShape::List(expected_item) = expected else {
        return Err(pattern_kind_mismatch(pattern, expected));
    };
    let actual = context.value_shape_in_scope(type_.as_ref());
    validate_pattern_type(expected, actual)?;
    for element in elements {
        validate_pattern(element, expected_item, context)?;
    }
    let tail = tail
        .as_deref()
        .map(|tail: &TailPattern<_>| validate_list_tail(&tail.pattern, expected, context))
        .transpose()?;
    Ok(ValidatedListPattern {
        item_shape: expected_item.as_ref().clone(),
        tail,
    })
}

pub(in crate::planner) fn validate_list_tail(
    pattern: &TypedPattern,
    expected: &ValueShape,
    context: &PlanContext<'_>,
) -> Result<ValidatedListTail, PlanError> {
    match pattern {
        Pattern::Variable { name, .. } => {
            validate_pattern(pattern, expected, context)?;
            Ok(ValidatedListTail::Named(name.clone()))
        }
        Pattern::Discard { .. } => {
            validate_pattern(pattern, expected, context)?;
            Ok(ValidatedListTail::Discard)
        }
        _ => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PatternShape {
                reason: InvalidPatternShapeReason::ListTailKind {
                    actual: pattern_kind(pattern),
                },
            },
        }),
    }
}

pub(in crate::planner) fn resolved_constructor(
    constructor: &Inferred<PatternConstructor>,
) -> Result<&PatternConstructor, PlanError> {
    let Inferred::Known(constructor) = constructor else {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PatternShape {
                reason: InvalidPatternShapeReason::UnresolvedConstructor,
            },
        });
    };
    Ok(constructor)
}

fn pattern_kind_mismatch(pattern: &TypedPattern, expected: &ValueShape) -> PlanError {
    pattern_kind_mismatch_for_kind(pattern_kind(pattern), expected)
}

fn pattern_kind_mismatch_for_kind(actual: PatternKind, expected: &ValueShape) -> PlanError {
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::PatternShape {
            reason: InvalidPatternShapeReason::KindMismatch {
                expected: expected.value_type(),
                actual,
            },
        },
    }
}

pub(in crate::planner) fn pattern_type_mismatch(
    expected: ValueType,
    actual: ValueType,
) -> PlanError {
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::PatternShape {
            reason: InvalidPatternShapeReason::TypeMismatch { expected, actual },
        },
    }
}

fn validate_constructor_pattern(
    constructor: &PatternConstructor,
    argument_count: usize,
    has_spread: bool,
    type_: &ValueType,
) -> Result<(), PlanError> {
    match type_ {
        ValueType::Bool | ValueType::Nil => {
            validate_constructor_module(constructor, PRELUDE_MODULE_NAME.into())?;
            let family = if matches!(type_, ValueType::Bool) {
                PreludeConstructorFamily::Bool
            } else {
                PreludeConstructorFamily::Nil
            };
            let expected_index = constructor_identity(constructor, family)?;
            validate_constructor_index(constructor, expected_index)?;
            validate_constructor_arity(0, argument_count)?;
            validate_constructor_spread(has_spread, type_)?;
        }
        ValueType::Custom(_) => {}
        ValueType::Parameter(_)
        | ValueType::Int
        | ValueType::Float
        | ValueType::String
        | ValueType::BitArray
        | ValueType::UtfCodepoint
        | ValueType::External(_)
        | ValueType::Tuple(_)
        | ValueType::List(_)
        | ValueType::Function(_) => {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: InvalidPatternShapeReason::ConstructorType {
                        type_: type_.clone(),
                    },
                },
            });
        }
    }
    Ok(())
}

#[derive(Clone, Copy)]
enum PreludeConstructorFamily {
    Bool,
    Nil,
}

fn constructor_identity(
    constructor: &PatternConstructor,
    family: PreludeConstructorFamily,
) -> Result<usize, PlanError> {
    match (family, constructor.name.as_str()) {
        (PreludeConstructorFamily::Bool, "True") | (PreludeConstructorFamily::Nil, "Nil") => Ok(0),
        (PreludeConstructorFamily::Bool, "False") => Ok(1),
        (family, _) => {
            let expected = match family {
                PreludeConstructorFamily::Bool => "True or False",
                PreludeConstructorFamily::Nil => "Nil",
            };
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: InvalidPatternShapeReason::ConstructorName {
                        expected: expected.into(),
                        actual: constructor.name.clone(),
                    },
                },
            })
        }
    }
}

fn validate_constructor_spread(has_spread: bool, type_: &ValueType) -> Result<(), PlanError> {
    if has_spread {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PatternShape {
                reason: InvalidPatternShapeReason::ConstructorSpread {
                    type_: type_.clone(),
                },
            },
        });
    }
    Ok(())
}

fn validate_constructor_module(
    constructor: &PatternConstructor,
    expected: EcoString,
) -> Result<(), PlanError> {
    if constructor.module != expected {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PatternShape {
                reason: InvalidPatternShapeReason::ConstructorModule {
                    expected,
                    actual: constructor.module.clone(),
                },
            },
        });
    }
    Ok(())
}

fn validate_constructor_index(
    constructor: &PatternConstructor,
    expected: usize,
) -> Result<(), PlanError> {
    let actual = usize::from(constructor.constructor_index);
    if actual != expected {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PatternShape {
                reason: InvalidPatternShapeReason::ConstructorIndex { expected, actual },
            },
        });
    }
    Ok(())
}

pub(in crate::planner) fn validate_constructor_arity(
    expected: usize,
    actual: usize,
) -> Result<(), PlanError> {
    if actual != expected {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PatternShape {
                reason: InvalidPatternShapeReason::ConstructorArity { expected, actual },
            },
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        pattern_kind, validate_constructor_pattern, validate_list_pattern, validate_pattern,
        validate_pattern_value_type, validate_tuple_arity, validate_tuple_pattern,
    };
    use crate::plan::{CustomType, CustomTypeName, ValueShape, ValueType};
    use crate::planner::context::{AnonymousFunctions, FunctionInfo, PlanContext};
    use crate::planner::error::{
        InvalidCustomTypeReason, InvalidPatternShapeReason, InvalidTypedAstReason, PatternKind,
        PlanError,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{compile, dummy_span};
    use ecow::EcoString;
    use gleam_core::analyse::Inferred;
    use gleam_core::ast::{BitArraySize, CallArg, Pattern, TailPattern, TypedPattern};
    use gleam_core::type_::{self, PRELUDE_MODULE_NAME, PatternConstructor};
    use num_bigint::BigInt;
    use std::collections::HashMap;

    fn with_context<T>(run: impl FnOnce(&PlanContext<'_>) -> T) -> T {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let context = PlanContext::new(&module, &functions, &mut anonymous);
        run(&context)
    }

    fn int_pattern() -> TypedPattern {
        Pattern::Int {
            location: dummy_span(),
            value: "1".into(),
            int_value: BigInt::from(1),
        }
    }

    fn discard(type_: std::sync::Arc<type_::Type>) -> TypedPattern {
        Pattern::Discard {
            name: "_".into(),
            location: dummy_span(),
            type_,
        }
    }

    fn pattern_error(reason: InvalidPatternShapeReason) -> PlanError {
        PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PatternShape { reason },
        }
    }

    fn constructor(name: &str, module: &str, index: u16) -> PatternConstructor {
        PatternConstructor {
            name: name.into(),
            field_map: None,
            documentation: None,
            module: module.into(),
            location: dummy_span(),
            constructor_index: index,
        }
    }

    fn result_pattern(
        argument: TypedPattern,
        value_type: std::sync::Arc<type_::Type>,
    ) -> TypedPattern {
        Pattern::Constructor {
            location: dummy_span(),
            name_location: dummy_span(),
            name: "Ok".into(),
            arguments: vec![CallArg {
                label: None,
                location: dummy_span(),
                value: argument,
                implicit: None,
            }],
            module: None,
            constructor: Inferred::Known(constructor("Ok", PRELUDE_MODULE_NAME, 0)),
            spread: None,
            type_: type_::result(value_type, type_::string()),
        }
    }

    #[test]
    fn pattern_annotations_are_validated_recursively() {
        with_context(|context| {
            assert_eq!(
                validate_pattern(&int_pattern(), &ValueShape::String, context),
                Err(pattern_error(InvalidPatternShapeReason::TypeMismatch {
                    expected: ValueType::String,
                    actual: ValueType::Int,
                })),
            );

            let nested = Pattern::Tuple {
                location: dummy_span(),
                elements: vec![int_pattern()],
            };
            assert_eq!(
                validate_pattern(
                    &nested,
                    &ValueShape::Tuple(vec![ValueShape::String].into_boxed_slice()),
                    context,
                ),
                Err(pattern_error(InvalidPatternShapeReason::TypeMismatch {
                    expected: ValueType::String,
                    actual: ValueType::Int,
                })),
            );
            assert_eq!(
                validate_pattern(
                    &nested,
                    &ValueShape::Tuple(
                        vec![ValueShape::Int, ValueShape::String].into_boxed_slice(),
                    ),
                    context,
                ),
                Err(pattern_error(InvalidPatternShapeReason::TupleArity {
                    expected: 2,
                    actual: 1,
                })),
            );
        });
    }

    #[test]
    fn custom_pattern_fields_are_validated_recursively() {
        assert!(
            plan_module(compile(
                r#"
type Boxed {
  Boxed(Int)
}

pub fn main() {
  case Boxed(1) {
    Boxed(value) -> value
  }
}
"#,
            ))
            .is_ok(),
        );

        with_context(|context| {
            let result_type = CustomType::new(
                CustomTypeName::new("".into(), "gleam".into(), "Result".into()),
                vec![ValueType::Int, ValueType::String],
            );
            let result_shape = ValueShape::from_value_type(ValueType::Custom(result_type.clone()));
            let missing_argument = Pattern::Constructor {
                location: dummy_span(),
                name_location: dummy_span(),
                name: "Ok".into(),
                arguments: Vec::new(),
                module: None,
                constructor: Inferred::Known(constructor("Ok", PRELUDE_MODULE_NAME, 0)),
                spread: None,
                type_: type_::result(type_::int(), type_::string()),
            };
            assert_eq!(
                validate_pattern(&missing_argument, &result_shape, context),
                Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::CustomType {
                        package: "".into(),
                        module: "gleam".into(),
                        name: "Result".into(),
                        reason: Box::new(InvalidCustomTypeReason::ConstructorArity {
                            expected: 1,
                            actual: 0,
                        }),
                    },
                }),
            );

            let nested = result_pattern(
                result_pattern(discard(type_::string()), type_::int()),
                type_::result(type_::int(), type_::string()),
            );
            let nested_shape = ValueShape::from_value_type(ValueType::Custom(CustomType::new(
                CustomTypeName::new("".into(), "gleam".into(), "Result".into()),
                vec![ValueType::Custom(result_type), ValueType::String],
            )));
            assert_eq!(
                validate_pattern(&nested, &nested_shape, context),
                Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::CustomType {
                        package: "".into(),
                        module: "gleam".into(),
                        name: "Result".into(),
                        reason: Box::new(InvalidCustomTypeReason::FieldType {
                            index: 0,
                            expected: ValueType::Int,
                            actual: ValueType::String,
                        }),
                    },
                }),
            );
        });
    }

    #[test]
    fn typed_pattern_carriers_reject_the_wrong_pattern_family() {
        with_context(|context| {
            assert_eq!(
                validate_tuple_pattern(&int_pattern(), &ValueShape::Int, context).map(|_| ()),
                Err(pattern_error(InvalidPatternShapeReason::KindMismatch {
                    expected: ValueType::Int,
                    actual: PatternKind::Int,
                })),
            );
            assert_eq!(
                validate_list_pattern(&int_pattern(), &ValueShape::Int, context).map(|_| ()),
                Err(pattern_error(InvalidPatternShapeReason::KindMismatch {
                    expected: ValueType::Int,
                    actual: PatternKind::Int,
                })),
            );
            assert_eq!(
                pattern_kind(&Pattern::BitArraySize(BitArraySize::Int {
                    location: dummy_span(),
                    value: "1".into(),
                    int_value: BigInt::from(1),
                })),
                PatternKind::BitArraySize,
            );
            assert_eq!(
                pattern_kind(&result_pattern(discard(type_::int()), type_::int())),
                PatternKind::Constructor,
            );
        });
    }

    #[test]
    fn list_annotations_and_tail_kind_are_validated_once() {
        with_context(|context| {
            let expected = ValueShape::List(Box::new(ValueShape::Int));
            let invalid_annotation = Pattern::List {
                location: dummy_span(),
                elements: Vec::new(),
                tail: None,
                type_: type_::int(),
            };
            assert_eq!(
                validate_pattern(&invalid_annotation, &expected, context),
                Err(pattern_error(InvalidPatternShapeReason::TypeMismatch {
                    expected: ValueType::List(Box::new(ValueType::Int)),
                    actual: ValueType::Int,
                })),
            );

            let invalid_tail = Pattern::List {
                location: dummy_span(),
                elements: vec![discard(type_::int())],
                tail: Some(Box::new(TailPattern {
                    location: dummy_span(),
                    pattern: int_pattern(),
                })),
                type_: type_::list(type_::int()),
            };
            assert_eq!(
                validate_pattern(&invalid_tail, &expected, context),
                Err(pattern_error(InvalidPatternShapeReason::ListTailKind {
                    actual: PatternKind::Int,
                })),
            );
        });
    }

    #[test]
    fn malformed_pattern_nodes_are_rejected_by_the_recursive_owner() {
        with_context(|context| {
            let invalid = Pattern::Invalid {
                location: dummy_span(),
                type_: type_::int(),
            };
            assert_eq!(
                validate_pattern(&invalid, &ValueShape::Int, context),
                Err(pattern_error(InvalidPatternShapeReason::InvalidNode)),
            );
            assert_eq!(
                super::pattern_value_shape(&invalid, context),
                Err(pattern_error(InvalidPatternShapeReason::InvalidNode)),
            );

            let size = Pattern::BitArraySize(BitArraySize::Int {
                location: dummy_span(),
                value: "1".into(),
                int_value: BigInt::from(1),
            });
            assert_eq!(
                validate_pattern(&size, &ValueShape::Int, context),
                Err(pattern_error(InvalidPatternShapeReason::BitArraySizeNode)),
            );
            assert_eq!(
                super::pattern_value_shape(&size, context),
                Err(pattern_error(InvalidPatternShapeReason::BitArraySizeNode)),
            );

            let nested_invalid = || Pattern::Invalid {
                location: dummy_span(),
                type_: type_::int(),
            };
            let result_type = CustomType::new(
                CustomTypeName::new("".into(), "gleam".into(), "Result".into()),
                vec![ValueType::Int, ValueType::String],
            );
            assert_eq!(
                validate_pattern(
                    &result_pattern(nested_invalid(), type_::int()),
                    &ValueShape::Custom(crate::plan::CustomValueShape::any(result_type)),
                    context,
                ),
                Err(pattern_error(InvalidPatternShapeReason::InvalidNode)),
            );

            let tuple = Pattern::Tuple {
                location: dummy_span(),
                elements: vec![nested_invalid()],
            };
            assert_eq!(
                super::pattern_value_shape(&tuple, context),
                Err(pattern_error(InvalidPatternShapeReason::InvalidNode)),
            );
            assert_eq!(
                validate_tuple_pattern(&tuple, &ValueShape::Int, context).map(|_| ()),
                Err(pattern_error(InvalidPatternShapeReason::InvalidNode)),
            );

            let alias = Pattern::Assign {
                location: dummy_span(),
                name: "value".into(),
                pattern: Box::new(nested_invalid()),
            };
            assert_eq!(
                super::pattern_value_shape(&alias, context),
                Err(pattern_error(InvalidPatternShapeReason::InvalidNode)),
            );
        });
    }

    #[test]
    fn prelude_constructor_metadata_has_exact_failures() {
        let bool_type = ValueType::Bool;
        let nil_type = ValueType::Nil;
        assert_eq!(
            validate_constructor_pattern(
                &constructor("True", PRELUDE_MODULE_NAME, 0),
                0,
                false,
                &bool_type,
            ),
            Ok(()),
        );
        assert_eq!(
            validate_constructor_pattern(
                &constructor("False", PRELUDE_MODULE_NAME, 1),
                0,
                false,
                &bool_type,
            ),
            Ok(()),
        );
        assert_eq!(
            validate_constructor_pattern(
                &constructor("Nil", PRELUDE_MODULE_NAME, 0),
                0,
                false,
                &nil_type,
            ),
            Ok(()),
        );

        assert_eq!(
            validate_constructor_pattern(&constructor("True", "other", 0), 0, false, &bool_type),
            Err(pattern_error(
                InvalidPatternShapeReason::ConstructorModule {
                    expected: PRELUDE_MODULE_NAME.into(),
                    actual: "other".into(),
                },
            )),
        );
        assert_eq!(
            validate_constructor_pattern(
                &constructor("Other", PRELUDE_MODULE_NAME, 0),
                0,
                false,
                &bool_type,
            ),
            Err(pattern_error(InvalidPatternShapeReason::ConstructorName {
                expected: "True or False".into(),
                actual: "Other".into(),
            })),
        );
        assert_eq!(
            validate_constructor_pattern(
                &constructor("Other", PRELUDE_MODULE_NAME, 0),
                0,
                false,
                &nil_type,
            ),
            Err(pattern_error(InvalidPatternShapeReason::ConstructorName {
                expected: "Nil".into(),
                actual: "Other".into(),
            })),
        );
        assert_eq!(
            validate_constructor_pattern(
                &constructor("True", PRELUDE_MODULE_NAME, 1),
                0,
                false,
                &bool_type,
            ),
            Err(pattern_error(InvalidPatternShapeReason::ConstructorIndex {
                expected: 0,
                actual: 1,
            })),
        );
        assert_eq!(
            validate_constructor_pattern(
                &constructor("True", PRELUDE_MODULE_NAME, 0),
                1,
                false,
                &bool_type,
            ),
            Err(pattern_error(InvalidPatternShapeReason::ConstructorArity {
                expected: 0,
                actual: 1,
            })),
        );
        assert_eq!(
            validate_constructor_pattern(
                &constructor("True", PRELUDE_MODULE_NAME, 0),
                0,
                true,
                &bool_type,
            ),
            Err(pattern_error(
                InvalidPatternShapeReason::ConstructorSpread {
                    type_: ValueType::Bool,
                },
            )),
        );
        assert_eq!(
            validate_constructor_pattern(
                &constructor("True", PRELUDE_MODULE_NAME, 0),
                0,
                false,
                &ValueType::Int,
            ),
            Err(pattern_error(InvalidPatternShapeReason::ConstructorType {
                type_: ValueType::Int,
            })),
        );

        let custom = ValueType::Custom(CustomType::new(
            CustomTypeName::new("app".into(), "main".into(), "Box".into()),
            Vec::new(),
        ));
        assert_eq!(
            validate_constructor_pattern(&constructor("Box", "main", 0), 1, true, &custom,),
            Ok(()),
        );
    }

    #[test]
    fn scalar_and_tuple_helpers_preserve_structured_context() {
        assert_eq!(
            validate_pattern_value_type(ValueType::Int, ValueType::Int),
            Ok(())
        );
        assert_eq!(
            validate_pattern_value_type(ValueType::Int, ValueType::String),
            Err(pattern_error(InvalidPatternShapeReason::TypeMismatch {
                expected: ValueType::Int,
                actual: ValueType::String,
            })),
        );
        assert_eq!(validate_tuple_arity(2, 2), Ok(()));
        assert_eq!(
            validate_tuple_arity(2, 1),
            Err(pattern_error(InvalidPatternShapeReason::TupleArity {
                expected: 2,
                actual: 1,
            })),
        );
        assert_eq!(
            super::pattern_value_type_from_gleam(type_::generic_var(0).as_ref()),
            Err(pattern_error(InvalidPatternShapeReason::UnsupportedType)),
        );
        assert_eq!(
            super::resolved_constructor(&gleam_core::analyse::Inferred::Unknown),
            Err(pattern_error(
                InvalidPatternShapeReason::UnresolvedConstructor,
            )),
        );

        for (pattern, kind) in [
            (
                Pattern::Assign {
                    name: "alias".into(),
                    location: dummy_span(),
                    pattern: Box::new(int_pattern()),
                },
                PatternKind::Assign,
            ),
            (
                Pattern::Tuple {
                    location: dummy_span(),
                    elements: Vec::new(),
                },
                PatternKind::Tuple,
            ),
            (
                Pattern::BitArray {
                    location: dummy_span(),
                    segments: Vec::new(),
                },
                PatternKind::BitArray,
            ),
        ] {
            assert_eq!(super::pattern_kind(&pattern), kind);
        }
    }
}
