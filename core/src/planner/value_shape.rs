use crate::plan::{
    CustomConstructorRefinement, CustomTypeName, CustomValueShape, ExternalTypeName,
    ExternalValueShape, FunctionShape, ValueShape, ValueType,
};
use gleam_compiler_core::type_::{Type, TypeVar};
use std::ops::Deref;

use super::type_parameter::TypeParameterScope;

impl ValueType {
    pub(super) fn from_gleam(type_: &Type) -> Option<Self> {
        ValueShape::from_gleam(type_).map(|shape| shape.value_type())
    }
}

impl ValueShape {
    pub(super) fn from_gleam(type_: &Type) -> Option<Self> {
        match type_ {
            Type::Var { type_ } => match type_.borrow().deref() {
                TypeVar::Link { type_ } => Self::from_gleam(type_.as_ref()),
                TypeVar::Unbound { .. } | TypeVar::Generic { .. } => None,
            },
            Type::Tuple { elements } => Some(Self::Tuple(
                elements
                    .iter()
                    .map(|element| Self::from_gleam(element.as_ref()))
                    .collect::<Option<Vec<_>>>()?
                    .into_boxed_slice(),
            )),
            Type::Fn { arguments, return_ } => Some(Self::Function(Box::new(FunctionShape::new(
                arguments
                    .iter()
                    .map(|argument| Self::from_gleam(argument.as_ref()))
                    .collect::<Option<Vec<_>>>()?,
                Self::from_gleam(return_.as_ref())?,
            )))),
            Type::Named {
                package,
                module,
                name,
                arguments,
                ..
            } => {
                if type_.is_int() {
                    Some(Self::Int)
                } else if type_.is_float() {
                    Some(Self::Float)
                } else if type_.is_string() {
                    Some(Self::String)
                } else if type_.is_bit_array() {
                    Some(Self::BitArray)
                } else if type_.is_utf_codepoint() {
                    Some(Self::UtfCodepoint)
                } else if type_.is_bool() {
                    Some(Self::Bool)
                } else if type_.is_nil() {
                    Some(Self::Nil)
                } else if let Some(element) = type_.list_type() {
                    Some(Self::List(Box::new(Self::from_gleam(element.as_ref())?)))
                } else {
                    Some(Self::Custom(CustomValueShape::new(
                        CustomTypeName::new(package.clone(), module.clone(), name.clone()),
                        arguments
                            .iter()
                            .map(|argument| Self::from_gleam(argument.as_ref()))
                            .collect::<Option<Vec<_>>>()?,
                        type_
                            .custom_type_inferred_variant()
                            .map_or(CustomConstructorRefinement::Any, |index| {
                                CustomConstructorRefinement::Exact(usize::from(index))
                            }),
                    )))
                }
            }
        }
    }

    pub(super) fn from_gleam_in_with_external(
        type_: &Type,
        parameters: &mut TypeParameterScope,
        is_external: &impl Fn(&ExternalTypeName) -> bool,
    ) -> Self {
        match type_ {
            Type::Var { type_ } => match type_.borrow().deref() {
                TypeVar::Link { type_ } => {
                    Self::from_gleam_in_with_external(type_.as_ref(), parameters, is_external)
                }
                TypeVar::Unbound { id } | TypeVar::Generic { id } => {
                    Self::Parameter(parameters.resolve(*id))
                }
            },
            Type::Tuple { elements } => Self::Tuple(
                elements
                    .iter()
                    .map(|element| {
                        Self::from_gleam_in_with_external(element.as_ref(), parameters, is_external)
                    })
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            ),
            Type::Fn { arguments, return_ } => Self::Function(Box::new(FunctionShape::new(
                arguments
                    .iter()
                    .map(|argument| {
                        Self::from_gleam_in_with_external(
                            argument.as_ref(),
                            parameters,
                            is_external,
                        )
                    })
                    .collect(),
                Self::from_gleam_in_with_external(return_.as_ref(), parameters, is_external),
            ))),
            Type::Named {
                package,
                module,
                name,
                arguments,
                ..
            } => {
                if type_.is_int() {
                    Self::Int
                } else if type_.is_float() {
                    Self::Float
                } else if type_.is_string() {
                    Self::String
                } else if type_.is_bit_array() {
                    Self::BitArray
                } else if type_.is_utf_codepoint() {
                    Self::UtfCodepoint
                } else if type_.is_bool() {
                    Self::Bool
                } else if type_.is_nil() {
                    Self::Nil
                } else if let Some(element) = type_.list_type() {
                    Self::List(Box::new(Self::from_gleam_in_with_external(
                        element.as_ref(),
                        parameters,
                        is_external,
                    )))
                } else {
                    let name = ExternalTypeName::new(package.clone(), module.clone(), name.clone());
                    let arguments = arguments
                        .iter()
                        .map(|argument| {
                            Self::from_gleam_in_with_external(
                                argument.as_ref(),
                                parameters,
                                is_external,
                            )
                        })
                        .collect();
                    if is_external(&name) {
                        Self::External(ExternalValueShape::new(name, arguments))
                    } else {
                        Self::Custom(CustomValueShape::new(
                            CustomTypeName::new(
                                name.package().clone(),
                                name.module().clone(),
                                name.name().clone(),
                            ),
                            arguments,
                            type_
                                .custom_type_inferred_variant()
                                .map_or(CustomConstructorRefinement::Any, |index| {
                                    CustomConstructorRefinement::Exact(usize::from(index))
                                }),
                        ))
                    }
                }
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::arc_with_non_send_sync)]
mod tests {
    use crate::plan::{
        CustomConstructorRefinement, CustomType, CustomTypeName, CustomValueShape, FunctionType,
        ValueShape, ValueType,
    };
    use ecow::EcoString;
    use gleam_compiler_core::ast::Publicity;
    use gleam_compiler_core::type_::{self, Type};
    use std::sync::Arc;

    #[test]
    fn conversion_preserves_recursive_nominal_types() {
        assert_eq!(
            ValueType::from_gleam(type_::bit_array().as_ref()),
            Some(ValueType::BitArray),
        );
        assert_eq!(
            ValueType::from_gleam(type_::utf_codepoint().as_ref()),
            Some(ValueType::UtfCodepoint),
        );
        assert_eq!(
            ValueType::from_gleam(
                type_::named("package", "main", "BitArray", Publicity::Public, Vec::new(),)
                    .as_ref(),
            ),
            Some(ValueType::Custom(CustomType::new(
                CustomTypeName::new("package".into(), "main".into(), "BitArray".into()),
                Vec::new(),
            ))),
        );
        assert_eq!(
            ValueType::from_gleam(
                type_::fn_(vec![type_::list(type_::int())], type_::list(type_::int())).as_ref(),
            ),
            Some(ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::List(Box::new(ValueType::Int))],
                ValueType::List(Box::new(ValueType::Int)),
            )))),
        );
        assert_eq!(
            ValueType::from_gleam(type_::tuple(vec![type_::list(type_::int())]).as_ref()),
            Some(ValueType::Tuple(vec![ValueType::List(Box::new(
                ValueType::Int
            ))])),
        );
    }

    #[test]
    fn conversion_preserves_nested_custom_refinements() {
        let inner = named("Choice", Vec::new(), Some(1));
        let outer = named("Wrapper", vec![inner], Some(0));

        assert_eq!(
            ValueShape::from_gleam(outer.as_ref()),
            Some(ValueShape::Custom(CustomValueShape::new(
                CustomTypeName::new("geam".into(), "main".into(), "Wrapper".into()),
                vec![ValueShape::Custom(CustomValueShape::new(
                    CustomTypeName::new("geam".into(), "main".into(), "Choice".into()),
                    Vec::new(),
                    CustomConstructorRefinement::Exact(1),
                ))],
                CustomConstructorRefinement::Exact(0),
            ))),
        );
    }

    #[test]
    fn conversion_rejects_unresolved_recursive_member_types() {
        let unsupported = || type_::generic_var(0);

        assert_eq!(ValueType::from_gleam(type_::generic_var(0).as_ref()), None);
        assert_eq!(ValueType::from_gleam(type_::unbound_var(0).as_ref()), None);
        assert_eq!(
            ValueType::from_gleam(type_::tuple(vec![unsupported()]).as_ref()),
            None,
        );
        assert_eq!(
            ValueType::from_gleam(type_::list(unsupported()).as_ref()),
            None,
        );
        assert_eq!(
            ValueType::from_gleam(type_::fn_(vec![unsupported()], type_::int()).as_ref()),
            None,
        );
        assert_eq!(
            ValueType::from_gleam(type_::fn_(Vec::new(), unsupported()).as_ref()),
            None,
        );
        assert_eq!(
            ValueType::from_gleam(
                type_::named(
                    "geam",
                    "main",
                    "Boxed",
                    Publicity::Public,
                    vec![unsupported()],
                )
                .as_ref(),
            ),
            None,
        );
    }

    fn named(name: &str, arguments: Vec<Arc<Type>>, inferred_variant: Option<u16>) -> Arc<Type> {
        Arc::new(Type::Named {
            publicity: Publicity::Private,
            package: EcoString::from("geam"),
            module: EcoString::from("main"),
            name: EcoString::from(name),
            arguments,
            inferred_variant,
        })
    }
}
