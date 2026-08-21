use crate::plan::{
    CustomConstructorDefinition, CustomFieldDefinition, CustomTypeDefinition, CustomTypeName,
    CustomTypeParameterId, CustomTypePublicity, CustomTypeTemplate,
};
use crate::planner::error::{
    InvalidCustomTypeReason, InvalidTypedAstReason, PlanError, UnsupportedTopLevelKind,
};
use ecow::EcoString;
use gleam_compiler_core::ast::{Publicity, TypedCustomType};
use gleam_compiler_core::type_::{Type, TypeVar};
use std::collections::{HashMap, HashSet};
use std::ops::Deref;

pub(super) fn plan_custom_types(
    package: &EcoString,
    module: &EcoString,
    types: Vec<TypedCustomType>,
) -> Result<Vec<CustomTypeDefinition>, PlanError> {
    plan_custom_types_with_external(package, module, types, &HashSet::new())
}

pub(super) fn plan_custom_types_with_external(
    package: &EcoString,
    module: &EcoString,
    types: Vec<TypedCustomType>,
    external_types: &HashSet<crate::plan::ExternalTypeName>,
) -> Result<Vec<CustomTypeDefinition>, PlanError> {
    types
        .into_iter()
        .map(|type_| plan_custom_type(package, module, type_, external_types))
        .collect()
}

fn plan_custom_type(
    package: &EcoString,
    module: &EcoString,
    type_: TypedCustomType,
    external_types: &HashSet<crate::plan::ExternalTypeName>,
) -> Result<CustomTypeDefinition, PlanError> {
    if type_.external_erlang.is_some() || type_.external_javascript.is_some() {
        return Err(PlanError::UnsupportedTopLevel {
            kind: UnsupportedTopLevelKind::ExternalCustomType,
        });
    }

    let name = CustomTypeName::new(package.clone(), module.clone(), type_.name.clone());
    let parameter_ids = parameter_ids(&type_, &name)?;
    let parameters = (0..type_.typed_parameters.len())
        .map(CustomTypeParameterId)
        .collect();
    let constructors = type_
        .constructors
        .into_iter()
        .enumerate()
        .map(|(index, constructor)| {
            let constructor_name = constructor.name.clone();
            let fields = constructor
                .arguments
                .into_iter()
                .enumerate()
                .map(|(field_index, field)| {
                    let type_ = type_template(field.type_.as_ref(), &parameter_ids, external_types)
                        .ok_or_else(|| PlanError::InvalidTypedAst {
                            reason: InvalidTypedAstReason::CustomType {
                                package: name.package().clone(),
                                module: name.module().clone(),
                                name: type_.name.clone(),
                                reason: Box::new(InvalidCustomTypeReason::DefinitionField {
                                    constructor: constructor_name.clone(),
                                    field: field_index,
                                }),
                            },
                        })?;
                    Ok(CustomFieldDefinition::new(
                        field.label.map(|(_, name)| name),
                        type_,
                    ))
                })
                .collect::<Result<Vec<_>, PlanError>>()?;
            Ok(CustomConstructorDefinition::new(
                constructor.name,
                index,
                fields,
            ))
        })
        .collect::<Result<Vec<_>, PlanError>>()?;

    Ok(CustomTypeDefinition::new(
        name,
        publicity(type_.publicity),
        type_.opaque,
        parameters,
        constructors,
    ))
}

fn parameter_ids(
    type_: &TypedCustomType,
    name: &CustomTypeName,
) -> Result<HashMap<u64, CustomTypeParameterId>, PlanError> {
    type_
        .typed_parameters
        .iter()
        .enumerate()
        .map(|(index, parameter)| {
            let id = match parameter.as_ref() {
                Type::Var {
                    type_: parameter_type,
                } => match parameter_type.borrow().deref() {
                    TypeVar::Generic { id } => Some(*id),
                    TypeVar::Link { .. } | TypeVar::Unbound { .. } => None,
                },
                Type::Named { .. } | Type::Fn { .. } | Type::Tuple { .. } => None,
            };
            let Some(id) = id else {
                return Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::CustomType {
                        package: name.package().clone(),
                        module: name.module().clone(),
                        name: name.name().clone(),
                        reason: Box::new(InvalidCustomTypeReason::DefinitionParameter { index }),
                    },
                });
            };
            Ok((id, CustomTypeParameterId(index)))
        })
        .collect()
}

fn type_template(
    type_: &Type,
    parameters: &HashMap<u64, CustomTypeParameterId>,
    external_types: &HashSet<crate::plan::ExternalTypeName>,
) -> Option<CustomTypeTemplate> {
    match type_ {
        Type::Var { type_ } => match type_.borrow().deref() {
            TypeVar::Link { type_ } => type_template(type_.as_ref(), parameters, external_types),
            TypeVar::Generic { id } => parameters
                .get(id)
                .copied()
                .map(CustomTypeTemplate::Parameter),
            TypeVar::Unbound { .. } => None,
        },
        Type::Tuple { elements } => Some(CustomTypeTemplate::Tuple(
            elements
                .iter()
                .map(|element| type_template(element.as_ref(), parameters, external_types))
                .collect::<Option<Vec<_>>>()?,
        )),
        Type::Fn { arguments, return_ } => Some(CustomTypeTemplate::Function {
            arguments: arguments
                .iter()
                .map(|argument| type_template(argument.as_ref(), parameters, external_types))
                .collect::<Option<Vec<_>>>()?,
            return_: Box::new(type_template(return_.as_ref(), parameters, external_types)?),
        }),
        Type::Named {
            package,
            module,
            name,
            arguments,
            ..
        } => {
            if type_.is_int() {
                Some(CustomTypeTemplate::Int)
            } else if type_.is_float() {
                Some(CustomTypeTemplate::Float)
            } else if type_.is_string() {
                Some(CustomTypeTemplate::String)
            } else if type_.is_bit_array() {
                Some(CustomTypeTemplate::BitArray)
            } else if type_.is_utf_codepoint() {
                Some(CustomTypeTemplate::UtfCodepoint)
            } else if type_.is_bool() {
                Some(CustomTypeTemplate::Bool)
            } else if type_.is_nil() {
                Some(CustomTypeTemplate::Nil)
            } else if let Some(element) = type_.list_type() {
                Some(CustomTypeTemplate::List(Box::new(type_template(
                    element.as_ref(),
                    parameters,
                    external_types,
                )?)))
            } else {
                let external_name = crate::plan::ExternalTypeName::new(
                    package.clone(),
                    module.clone(),
                    name.clone(),
                );
                let arguments = arguments
                    .iter()
                    .map(|argument| type_template(argument.as_ref(), parameters, external_types))
                    .collect::<Option<Vec<_>>>()?;
                if external_types.contains(&external_name) {
                    Some(CustomTypeTemplate::External {
                        name: external_name,
                        arguments,
                    })
                } else {
                    Some(CustomTypeTemplate::Custom {
                        name: CustomTypeName::new(package.clone(), module.clone(), name.clone()),
                        arguments,
                    })
                }
            }
        }
    }
}

fn publicity(publicity: Publicity) -> CustomTypePublicity {
    match publicity {
        Publicity::Public => CustomTypePublicity::Public,
        Publicity::Private => CustomTypePublicity::Private,
        Publicity::Internal { .. } => CustomTypePublicity::Internal,
    }
}

#[cfg(test)]
#[allow(clippy::arc_with_non_send_sync)]
mod tests {
    use super::plan_custom_types;
    use crate::plan::{
        CustomTypeName, CustomTypeParameterId, CustomTypePublicity, CustomTypeTemplate,
    };
    use crate::planner::error::{InvalidCustomTypeReason, InvalidTypedAstReason, PlanError};
    use gleam_compiler_core::ast::Publicity;
    use gleam_compiler_core::type_::{self, Type, TypeVar};
    use std::cell::RefCell;
    use std::sync::Arc;

    fn int_type() -> Arc<Type> {
        Arc::new(Type::Named {
            publicity: Publicity::Public,
            name: "Int".into(),
            module: "gleam".into(),
            package: "".into(),
            arguments: Vec::new(),
            inferred_variant: None,
        })
    }

    fn unbound_type(id: u64) -> Arc<Type> {
        Arc::new(Type::Var {
            type_: Arc::new(RefCell::new(TypeVar::Unbound { id })),
        })
    }

    #[test]
    fn plans_publicity_and_every_custom_field_template_exactly() {
        let module = crate::frontend::compile_typed_module(
            "main",
            "main.gleam",
            r#"
type Private {
  Private
}

@internal
pub type Internal {
  Internal
}

pub opaque type Container(value) {
  Container(
    value: value,
    int: Int,
    float: Float,
    string: String,
    bit_array: BitArray,
    codepoint: UtfCodepoint,
    bool: Bool,
    nil: Nil,
    tuple: #(Int, value),
    list: List(value),
    function: fn(value) -> List(value),
    recursive: Container(value),
  )
}

pub fn main() { 1 }
"#,
        )
        .expect("custom type definitions should analyse");
        let definitions = plan_custom_types(
            &module.type_info.package,
            &module.name,
            module.definitions.custom_types,
        )
        .expect("supported custom type definitions should plan");

        assert_eq!(definitions.len(), 3);
        assert_eq!(definitions[0].publicity(), CustomTypePublicity::Private);
        assert_eq!(definitions[1].publicity(), CustomTypePublicity::Internal);

        let container = &definitions[2];
        assert_eq!(
            container.name(),
            &CustomTypeName::new("geam".into(), "main".into(), "Container".into()),
        );
        assert_eq!(container.publicity(), CustomTypePublicity::Public);
        assert!(container.is_opaque());
        assert_eq!(container.parameters(), &[CustomTypeParameterId(0)]);
        assert_eq!(container.constructors().len(), 1);
        let constructor = &container.constructors()[0];
        assert_eq!(constructor.name(), "Container");
        assert_eq!(constructor.index(), 0);
        let fields = constructor.fields();
        assert_eq!(
            fields
                .iter()
                .map(|field| (field.label().cloned(), field.type_().clone()))
                .collect::<Vec<_>>(),
            vec![
                (
                    Some("value".into()),
                    CustomTypeTemplate::Parameter(CustomTypeParameterId(0)),
                ),
                (Some("int".into()), CustomTypeTemplate::Int),
                (Some("float".into()), CustomTypeTemplate::Float),
                (Some("string".into()), CustomTypeTemplate::String),
                (Some("bit_array".into()), CustomTypeTemplate::BitArray),
                (Some("codepoint".into()), CustomTypeTemplate::UtfCodepoint,),
                (Some("bool".into()), CustomTypeTemplate::Bool),
                (Some("nil".into()), CustomTypeTemplate::Nil),
                (
                    Some("tuple".into()),
                    CustomTypeTemplate::Tuple(vec![
                        CustomTypeTemplate::Int,
                        CustomTypeTemplate::Parameter(CustomTypeParameterId(0)),
                    ]),
                ),
                (
                    Some("list".into()),
                    CustomTypeTemplate::List(Box::new(CustomTypeTemplate::Parameter(
                        CustomTypeParameterId(0),
                    ))),
                ),
                (
                    Some("function".into()),
                    CustomTypeTemplate::Function {
                        arguments: vec![CustomTypeTemplate::Parameter(CustomTypeParameterId(0))],
                        return_: Box::new(CustomTypeTemplate::List(Box::new(
                            CustomTypeTemplate::Parameter(CustomTypeParameterId(0)),
                        ))),
                    },
                ),
                (
                    Some("recursive".into()),
                    CustomTypeTemplate::Custom {
                        name: CustomTypeName::new("geam".into(), "main".into(), "Container".into(),),
                        arguments: vec![CustomTypeTemplate::Parameter(CustomTypeParameterId(0))],
                    },
                ),
            ],
        );
    }

    #[test]
    fn rejects_malformed_custom_parameter_and_field_types_exactly() {
        let module = crate::frontend::compile_typed_module(
            "main",
            "main.gleam",
            "pub type Box(value) { Box(value) } pub fn main() { 1 }",
        )
        .expect("generic custom type should analyse");
        let original = module.definitions.custom_types[0].clone();

        let mut concrete_parameter = original.clone();
        concrete_parameter.typed_parameters[0] = int_type();
        assert_eq!(
            plan_custom_types(
                &module.type_info.package,
                &module.name,
                vec![concrete_parameter]
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    package: "geam".into(),
                    module: "main".into(),
                    name: "Box".into(),
                    reason: Box::new(InvalidCustomTypeReason::DefinitionParameter { index: 0 }),
                },
            }),
        );

        let mut unbound_parameter = original.clone();
        unbound_parameter.typed_parameters[0] = unbound_type(99);
        assert_eq!(
            plan_custom_types(
                &module.type_info.package,
                &module.name,
                vec![unbound_parameter]
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    package: "geam".into(),
                    module: "main".into(),
                    name: "Box".into(),
                    reason: Box::new(InvalidCustomTypeReason::DefinitionParameter { index: 0 }),
                },
            }),
        );

        let mut linked_parameter = original.clone();
        linked_parameter.typed_parameters[0] = Arc::new(Type::Var {
            type_: Arc::new(RefCell::new(TypeVar::Link {
                type_: type_::generic_var(99),
            })),
        });
        assert_eq!(
            plan_custom_types(
                &module.type_info.package,
                &module.name,
                vec![linked_parameter]
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    package: "geam".into(),
                    module: "main".into(),
                    name: "Box".into(),
                    reason: Box::new(InvalidCustomTypeReason::DefinitionParameter { index: 0 }),
                },
            }),
        );

        let mut nested_unbound_field = original.clone();
        nested_unbound_field.constructors[0].arguments[0].type_ =
            gleam_compiler_core::type_::list(unbound_type(100));
        assert_eq!(
            plan_custom_types(
                &module.type_info.package,
                &module.name,
                vec![nested_unbound_field]
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    package: "geam".into(),
                    module: "main".into(),
                    name: "Box".into(),
                    reason: Box::new(InvalidCustomTypeReason::DefinitionField {
                        constructor: "Box".into(),
                        field: 0,
                    }),
                },
            }),
        );

        let mut tuple_unbound_field = original.clone();
        tuple_unbound_field.constructors[0].arguments[0].type_ =
            type_::tuple(vec![unbound_type(101)]);
        assert_eq!(
            plan_custom_types(
                &module.type_info.package,
                &module.name,
                vec![tuple_unbound_field]
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    package: "geam".into(),
                    module: "main".into(),
                    name: "Box".into(),
                    reason: Box::new(InvalidCustomTypeReason::DefinitionField {
                        constructor: "Box".into(),
                        field: 0,
                    }),
                },
            }),
        );

        let mut function_argument_unbound_field = original.clone();
        function_argument_unbound_field.constructors[0].arguments[0].type_ =
            type_::fn_(vec![unbound_type(102)], type_::int());
        assert_eq!(
            plan_custom_types(
                &module.type_info.package,
                &module.name,
                vec![function_argument_unbound_field]
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    package: "geam".into(),
                    module: "main".into(),
                    name: "Box".into(),
                    reason: Box::new(InvalidCustomTypeReason::DefinitionField {
                        constructor: "Box".into(),
                        field: 0,
                    }),
                },
            }),
        );

        let mut function_return_unbound_field = original.clone();
        function_return_unbound_field.constructors[0].arguments[0].type_ =
            type_::fn_(Vec::new(), unbound_type(103));
        assert_eq!(
            plan_custom_types(
                &module.type_info.package,
                &module.name,
                vec![function_return_unbound_field]
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    package: "geam".into(),
                    module: "main".into(),
                    name: "Box".into(),
                    reason: Box::new(InvalidCustomTypeReason::DefinitionField {
                        constructor: "Box".into(),
                        field: 0,
                    }),
                },
            }),
        );

        let mut custom_argument_unbound_field = original.clone();
        custom_argument_unbound_field.constructors[0].arguments[0].type_ = Arc::new(Type::Named {
            publicity: Publicity::Private,
            name: "Box".into(),
            module: module.name.clone(),
            package: module.type_info.package.clone(),
            arguments: vec![unbound_type(104)],
            inferred_variant: None,
        });
        assert_eq!(
            plan_custom_types(
                &module.type_info.package,
                &module.name,
                vec![custom_argument_unbound_field]
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    package: "geam".into(),
                    module: "main".into(),
                    name: "Box".into(),
                    reason: Box::new(InvalidCustomTypeReason::DefinitionField {
                        constructor: "Box".into(),
                        field: 0,
                    }),
                },
            }),
        );

        let mut unbound_field = original;
        unbound_field.constructors[0].arguments[0].type_ = unbound_type(100);
        assert_eq!(
            plan_custom_types(&module.type_info.package, &module.name, vec![unbound_field]),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    package: "geam".into(),
                    module: "main".into(),
                    name: "Box".into(),
                    reason: Box::new(InvalidCustomTypeReason::DefinitionField {
                        constructor: "Box".into(),
                        field: 0,
                    }),
                },
            }),
        );
    }

    #[test]
    fn external_custom_types_and_linked_field_templates_are_exact() {
        for source in [
            r#"
@external(erlang, "external", "thing")
pub type Thing
pub fn main() { 1 }
"#,
            r#"
@external(javascript, "external", "thing")
pub type Thing
pub fn main() { 1 }
"#,
        ] {
            let module = crate::frontend::compile_typed_module("main", "main.gleam", source)
                .expect("external custom type should analyse");
            assert_eq!(
                plan_custom_types(
                    &module.type_info.package,
                    &module.name,
                    module.definitions.custom_types,
                ),
                Err(PlanError::UnsupportedTopLevel {
                    kind: crate::planner::UnsupportedTopLevelKind::ExternalCustomType,
                }),
            );
        }

        let module = crate::frontend::compile_typed_module(
            "main",
            "main.gleam",
            "pub type Box(value) { Box(value) } pub fn main() { 1 }",
        )
        .expect("generic custom type should analyse");
        let mut linked_field = module.definitions.custom_types[0].clone();
        let parameter = linked_field.typed_parameters[0].clone();
        linked_field.constructors[0].arguments[0].type_ = Arc::new(Type::Var {
            type_: Arc::new(RefCell::new(TypeVar::Link { type_: parameter })),
        });

        let definitions =
            plan_custom_types(&module.type_info.package, &module.name, vec![linked_field])
                .expect("linked generic field should plan");
        assert_eq!(
            definitions[0].constructors()[0].fields()[0].type_(),
            &CustomTypeTemplate::Parameter(CustomTypeParameterId(0)),
        );
    }
}
