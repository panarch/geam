use super::LinkedFunction;
use crate::plan::SourceContext;
use crate::planner::error::{HostProviderLinkReason, PlanError};
use crate::planner::module::registry::ProgramRegistry;
use ecow::EcoString;

#[derive(Clone, Copy)]
enum HostCustomTypeAccess {
    SourceDeclaration,
    SourceLessPublicSurface,
}

pub(in crate::planner::module::host) fn validate_host_custom_schemas(
    registry: &ProgramRegistry,
    source_context: Option<&SourceContext>,
    functions: &[LinkedFunction],
) -> Result<(), PlanError> {
    let access = match source_context {
        Some(_) => HostCustomTypeAccess::SourceDeclaration,
        None => HostCustomTypeAccess::SourceLessPublicSurface,
    };
    for function in functions {
        let LinkedFunction::Host { template, .. } = function else {
            continue;
        };
        for actual in template.custom_schemas() {
            validate_host_custom_schema(
                registry,
                template.package(),
                template.site(),
                template.signature(),
                actual,
                access,
            )?;
        }
    }
    Ok(())
}

fn validate_host_custom_schema(
    registry: &ProgramRegistry,
    package: &EcoString,
    site: &crate::plan::HostCallSite,
    signature: &crate::plan::FunctionTemplateSignature,
    actual: &crate::host::HostCustomTypeSchema,
    access: HostCustomTypeAccess,
) -> Result<(), PlanError> {
    let name = crate::plan::CustomTypeName::new(
        actual.package().clone(),
        actual.module().clone(),
        actual.name().clone(),
    );
    let (visible, expected, parameter_count) = match registry.custom_type(&name) {
        Some(definition) => {
            let visible = match access {
                HostCustomTypeAccess::SourceLessPublicSurface => {
                    !definition.is_opaque()
                        && definition.publicity() == crate::plan::CustomTypePublicity::Public
                }
                HostCustomTypeAccess::SourceDeclaration => {
                    let same_package = definition.name().package() == package;
                    let same_module = same_package && definition.name().module() == site.module();
                    if definition.is_opaque() {
                        same_module
                    } else {
                        match definition.publicity() {
                            crate::plan::CustomTypePublicity::Public => true,
                            crate::plan::CustomTypePublicity::Internal => same_package,
                            crate::plan::CustomTypePublicity::Private => same_module,
                        }
                    }
                }
            };
            (
                visible,
                host_custom_type_schema(definition),
                definition.parameters().len(),
            )
        }
        None if name.package().is_empty()
            && name.module() == "gleam"
            && name.name() == "Result" =>
        {
            (
                true,
                crate::host::HostCustomTypeSchema::new(
                    "",
                    "gleam",
                    "Result",
                    2,
                    [
                        crate::host::HostCustomConstructorSchema::new(
                            "Ok",
                            [crate::host::HostCustomFieldSchema::new(
                                None::<EcoString>,
                                crate::host::HostSchemaType::parameter(0),
                            )],
                        ),
                        crate::host::HostCustomConstructorSchema::new(
                            "Error",
                            [crate::host::HostCustomFieldSchema::new(
                                None::<EcoString>,
                                crate::host::HostSchemaType::parameter(1),
                            )],
                        ),
                    ],
                ),
                2,
            )
        }
        None => {
            return Err(PlanError::HostProviderLink {
                package: package.clone(),
                module: site.module().clone(),
                function: site.function().clone(),
                reason: Box::new(HostProviderLinkReason::MissingCustomType { custom_type: name }),
            });
        }
    };
    if !visible {
        return Err(PlanError::HostProviderLink {
            package: package.clone(),
            module: site.module().clone(),
            function: site.function().clone(),
            reason: Box::new(HostProviderLinkReason::CustomTypeVisibility { custom_type: name }),
        });
    }
    if actual != &expected {
        return Err(PlanError::HostProviderLink {
            package: package.clone(),
            module: site.module().clone(),
            function: site.function().clone(),
            reason: Box::new(HostProviderLinkReason::CustomSchemaMismatch {
                expected,
                actual: actual.clone(),
            }),
        });
    }
    for shape in signature
        .shape()
        .argument_shapes()
        .iter()
        .chain([signature.shape().return_shape()])
    {
        if let Some(actual) = invalid_host_custom_type_argument_count(shape, &name, parameter_count)
        {
            return Err(PlanError::HostProviderLink {
                package: package.clone(),
                module: site.module().clone(),
                function: site.function().clone(),
                reason: Box::new(HostProviderLinkReason::CustomTypeArgumentCount {
                    custom_type: name,
                    expected: parameter_count,
                    actual,
                }),
            });
        }
    }
    Ok(())
}

fn invalid_host_custom_type_argument_count(
    shape: &crate::plan::ValueShape,
    custom_type: &crate::plan::CustomTypeName,
    expected: usize,
) -> Option<usize> {
    let mut pending = vec![shape];
    while let Some(shape) = pending.pop() {
        match shape {
            crate::plan::ValueShape::Tuple(elements) => {
                pending.extend(elements.iter().rev());
            }
            crate::plan::ValueShape::List(item) => pending.push(item),
            crate::plan::ValueShape::Custom(custom) => {
                if custom.type_name() == custom_type && custom.arguments().len() != expected {
                    return Some(custom.arguments().len());
                }
                pending.extend(custom.arguments().iter().rev());
            }
            crate::plan::ValueShape::External(external) => {
                pending.extend(external.arguments().iter().rev());
            }
            crate::plan::ValueShape::Parameter(_)
            | crate::plan::ValueShape::Int
            | crate::plan::ValueShape::Float
            | crate::plan::ValueShape::String
            | crate::plan::ValueShape::BitArray
            | crate::plan::ValueShape::UtfCodepoint
            | crate::plan::ValueShape::Bool
            | crate::plan::ValueShape::Nil
            | crate::plan::ValueShape::Function(_) => {}
        }
    }
    None
}

fn host_custom_type_schema(
    definition: &crate::plan::CustomTypeDefinition,
) -> crate::host::HostCustomTypeSchema {
    crate::host::HostCustomTypeSchema::new(
        definition.name().package().clone(),
        definition.name().module().clone(),
        definition.name().name().clone(),
        definition.parameters().len(),
        definition.constructors().iter().map(|constructor| {
            crate::host::HostCustomConstructorSchema::new(
                constructor.name().clone(),
                constructor.fields().iter().map(|field| {
                    crate::host::HostCustomFieldSchema::new(
                        field.label().cloned(),
                        host_schema_type(field.type_()),
                    )
                }),
            )
        }),
    )
}

fn host_schema_type(type_: &crate::plan::CustomTypeTemplate) -> crate::host::HostSchemaType {
    use crate::host::HostSchemaType as H;
    use crate::plan::CustomTypeTemplate as T;

    match type_ {
        T::Int => H::Int,
        T::Float => H::Float,
        T::String => H::String,
        T::BitArray => H::BitArray,
        T::UtfCodepoint => H::UtfCodepoint,
        T::Bool => H::Bool,
        T::Nil => H::Nil,
        T::Tuple(elements) => H::tuple(elements.iter().map(host_schema_type)),
        T::List(item) => H::list(host_schema_type(item)),
        T::Function { arguments, return_ } => H::function(
            arguments.iter().map(host_schema_type),
            host_schema_type(return_),
        ),
        T::Custom { name, arguments } => H::custom(
            name.package().clone(),
            name.module().clone(),
            name.name().clone(),
            arguments.iter().map(host_schema_type),
        ),
        T::External { name, arguments } => H::External {
            schema: crate::host::HostExternalTypeSchema::new(
                name.package().clone(),
                name.module().clone(),
                name.name().clone(),
                arguments.len(),
            ),
            arguments: arguments
                .iter()
                .map(host_schema_type)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        },
        T::Parameter(parameter) => H::parameter(parameter.0),
    }
}

#[cfg(test)]
mod tests {
    use super::{HostCustomTypeAccess, host_schema_type, validate_host_custom_schema};
    use crate::host::{
        HostCustomConstructorSchema, HostCustomFieldSchema, HostCustomTypeSchema, HostSchemaType,
    };
    use crate::plan::{
        CustomConstructorDefinition, CustomConstructorRefinement, CustomFieldDefinition,
        CustomTypeDefinition, CustomTypeName, CustomTypeParameterId, CustomTypePublicity,
        CustomTypeTemplate, CustomValueShape, ExternalTypeName, ExternalValueShape, FunctionShape,
        FunctionTemplateId, FunctionTemplateSignature, HostCallSite, ModuleId, SourceSpan,
        TypeParameterId, TypeScheme, ValueShape,
    };
    use crate::planner::module::constant::ConstantSignatures;
    use crate::planner::module::registry::{ModuleRegistry, ProgramRegistry};
    use crate::planner::{HostProviderLinkReason, PlanError};
    use ecow::EcoString;

    #[test]
    fn maps_every_planned_custom_field_type_to_the_exact_host_schema() {
        let custom_name = CustomTypeName::new("domain".into(), "domain/tree".into(), "Tree".into());
        let cases = [
            (CustomTypeTemplate::Int, HostSchemaType::Int),
            (CustomTypeTemplate::Float, HostSchemaType::Float),
            (CustomTypeTemplate::String, HostSchemaType::String),
            (CustomTypeTemplate::BitArray, HostSchemaType::BitArray),
            (
                CustomTypeTemplate::UtfCodepoint,
                HostSchemaType::UtfCodepoint,
            ),
            (CustomTypeTemplate::Bool, HostSchemaType::Bool),
            (CustomTypeTemplate::Nil, HostSchemaType::Nil),
            (
                CustomTypeTemplate::Tuple(vec![CustomTypeTemplate::Int, CustomTypeTemplate::Bool]),
                HostSchemaType::tuple([HostSchemaType::Int, HostSchemaType::Bool]),
            ),
            (
                CustomTypeTemplate::List(Box::new(CustomTypeTemplate::String)),
                HostSchemaType::list(HostSchemaType::String),
            ),
            (
                CustomTypeTemplate::Function {
                    arguments: vec![CustomTypeTemplate::Int, CustomTypeTemplate::Bool],
                    return_: Box::new(CustomTypeTemplate::String),
                },
                HostSchemaType::function(
                    [HostSchemaType::Int, HostSchemaType::Bool],
                    HostSchemaType::String,
                ),
            ),
            (
                CustomTypeTemplate::Custom {
                    name: custom_name.clone(),
                    arguments: vec![CustomTypeTemplate::Parameter(CustomTypeParameterId(0))],
                },
                HostSchemaType::custom(
                    "domain",
                    "domain/tree",
                    "Tree",
                    [HostSchemaType::parameter(0)],
                ),
            ),
            (
                CustomTypeTemplate::External {
                    name: ExternalTypeName::new(
                        "storage".into(),
                        "storage/cell".into(),
                        "Cell".into(),
                    ),
                    arguments: vec![CustomTypeTemplate::Int],
                },
                HostSchemaType::External {
                    schema: crate::host::HostExternalTypeSchema::new(
                        "storage",
                        "storage/cell",
                        "Cell",
                        1,
                    ),
                    arguments: vec![HostSchemaType::Int].into_boxed_slice(),
                },
            ),
            (
                CustomTypeTemplate::Parameter(CustomTypeParameterId(1)),
                HostSchemaType::parameter(1),
            ),
        ];

        for (template, expected) in cases {
            assert_eq!(host_schema_type(&template), expected);
        }
    }

    #[test]
    fn source_less_host_custom_type_requires_a_planned_definition() {
        let package = EcoString::from("application");
        let site = HostCallSite::new("host/custom".into(), "accept".into(), SourceSpan::new(0, 0));
        let signature = FunctionTemplateSignature::new(
            FunctionTemplateId::in_module(ModuleId::new(0), 0),
            TypeScheme::new(0),
            FunctionShape::new(Vec::new(), ValueShape::Bool),
        );
        let actual = HostCustomTypeSchema::new("application", "domain/missing", "Missing", 0, []);
        let registry = ProgramRegistry::new(Vec::new());

        assert_eq!(
            validate_host_custom_schema(
                &registry,
                &package,
                &site,
                &signature,
                &actual,
                HostCustomTypeAccess::SourceLessPublicSurface,
            )
            .err(),
            Some(PlanError::HostProviderLink {
                package: "application".into(),
                module: "host/custom".into(),
                function: "accept".into(),
                reason: Box::new(HostProviderLinkReason::MissingCustomType {
                    custom_type: CustomTypeName::new(
                        "application".into(),
                        "domain/missing".into(),
                        "Missing".into(),
                    ),
                }),
            }),
        );
    }

    #[test]
    fn validates_the_compiler_owned_result_schema_without_a_source_definition() {
        let result = CustomTypeName::new("".into(), "gleam".into(), "Result".into());
        let actual = HostCustomTypeSchema::new(
            "",
            "gleam",
            "Result",
            2,
            [
                HostCustomConstructorSchema::new(
                    "Ok",
                    [HostCustomFieldSchema::new(
                        None::<EcoString>,
                        HostSchemaType::parameter(0),
                    )],
                ),
                HostCustomConstructorSchema::new(
                    "Error",
                    [HostCustomFieldSchema::new(
                        None::<EcoString>,
                        HostSchemaType::parameter(1),
                    )],
                ),
            ],
        );
        let signature = FunctionTemplateSignature::new(
            FunctionTemplateId::in_module(ModuleId::new(0), 0),
            TypeScheme::new(1),
            FunctionShape::new(
                Vec::new(),
                ValueShape::Custom(CustomValueShape::new(
                    result,
                    vec![ValueShape::Parameter(TypeParameterId(0)), ValueShape::Nil],
                    CustomConstructorRefinement::Any,
                )),
            ),
        );

        assert_eq!(
            validate_host_custom_schema(
                &ProgramRegistry::new(Vec::new()),
                &"application".into(),
                &HostCallSite::new(
                    "host/result".into(),
                    "produce".into(),
                    SourceSpan::new(0, 0),
                ),
                &signature,
                &actual,
                HostCustomTypeAccess::SourceDeclaration,
            ),
            Ok(()),
        );
    }

    #[test]
    fn rejects_a_malformed_compiler_owned_result_schema_before_argument_count() {
        let result = CustomTypeName::new("".into(), "gleam".into(), "Result".into());
        let expected = HostCustomTypeSchema::new(
            "",
            "gleam",
            "Result",
            2,
            [
                HostCustomConstructorSchema::new(
                    "Ok",
                    [HostCustomFieldSchema::new(
                        None::<EcoString>,
                        HostSchemaType::parameter(0),
                    )],
                ),
                HostCustomConstructorSchema::new(
                    "Error",
                    [HostCustomFieldSchema::new(
                        None::<EcoString>,
                        HostSchemaType::parameter(1),
                    )],
                ),
            ],
        );
        let actual = HostCustomTypeSchema::new(
            "",
            "gleam",
            "Result",
            2,
            [HostCustomConstructorSchema::new(
                "Success",
                [HostCustomFieldSchema::new(
                    None::<EcoString>,
                    HostSchemaType::parameter(0),
                )],
            )],
        );
        let signature = FunctionTemplateSignature::new(
            FunctionTemplateId::in_module(ModuleId::new(0), 0),
            TypeScheme::new(0),
            FunctionShape::new(
                Vec::new(),
                ValueShape::Custom(CustomValueShape::new(
                    result,
                    vec![ValueShape::Int],
                    CustomConstructorRefinement::Any,
                )),
            ),
        );

        assert_eq!(
            validate_host_custom_schema(
                &ProgramRegistry::new(Vec::new()),
                &"application".into(),
                &HostCallSite::new(
                    "host/result".into(),
                    "produce".into(),
                    SourceSpan::new(0, 0),
                ),
                &signature,
                &actual,
                HostCustomTypeAccess::SourceDeclaration,
            ),
            Err(PlanError::HostProviderLink {
                package: "application".into(),
                module: "host/result".into(),
                function: "produce".into(),
                reason: Box::new(HostProviderLinkReason::CustomSchemaMismatch { expected, actual }),
            }),
        );
    }

    #[test]
    fn validates_a_custom_schema_referenced_through_a_nested_type_argument() {
        let custom_type = CustomTypeName::new("application".into(), "main".into(), "Boxed".into());
        let definition = CustomTypeDefinition::new(
            custom_type.clone(),
            CustomTypePublicity::Public,
            false,
            vec![CustomTypeParameterId(0)],
            vec![CustomConstructorDefinition::new(
                "Boxed".into(),
                0,
                vec![CustomFieldDefinition::new(
                    Some("value".into()),
                    CustomTypeTemplate::Parameter(CustomTypeParameterId(0)),
                )],
            )],
        );
        let registry = ProgramRegistry::new(vec![ModuleRegistry::new(
            "main".into(),
            vec![definition],
            Vec::new(),
            std::collections::HashMap::new(),
            ConstantSignatures::default(),
        )]);
        let schema = HostCustomTypeSchema::new(
            "application",
            "main",
            "Boxed",
            1,
            [HostCustomConstructorSchema::new(
                "Boxed",
                [HostCustomFieldSchema::new(
                    Some("value"),
                    HostSchemaType::parameter(0),
                )],
            )],
        );
        let parameter = TypeParameterId(0);
        let shape = FunctionShape::new(
            vec![ValueShape::External(ExternalValueShape::new(
                ExternalTypeName::new("storage".into(), "storage/cell".into(), "Cell".into()),
                vec![ValueShape::List(Box::new(ValueShape::Custom(
                    CustomValueShape::new(
                        custom_type,
                        vec![ValueShape::Parameter(parameter)],
                        CustomConstructorRefinement::Any,
                    ),
                )))],
            ))],
            ValueShape::Bool,
        );
        let module = ModuleId::new(0);
        let signature = FunctionTemplateSignature::new(
            FunctionTemplateId::in_module(module, 0),
            TypeScheme::new(1),
            shape,
        );
        let package = EcoString::from("application");
        let site = HostCallSite::new("main".into(), "accept".into(), SourceSpan::new(0, 0));

        assert_eq!(
            validate_host_custom_schema(
                &registry,
                &package,
                &site,
                &signature,
                &schema,
                HostCustomTypeAccess::SourceLessPublicSurface,
            ),
            Ok(()),
        );
    }

    #[test]
    fn source_provider_rejects_a_custom_schema_mismatch_before_shape_validation() {
        let custom_type = CustomTypeName::new(
            "application".into(),
            "domain/marker".into(),
            "Marker".into(),
        );
        let definition = CustomTypeDefinition::new(
            custom_type,
            CustomTypePublicity::Public,
            false,
            Vec::new(),
            vec![CustomConstructorDefinition::new(
                "Expected".into(),
                0,
                Vec::new(),
            )],
        );
        let registry = ProgramRegistry::new(vec![ModuleRegistry::new(
            "domain/marker".into(),
            vec![definition],
            Vec::new(),
            std::collections::HashMap::new(),
            ConstantSignatures::default(),
        )]);
        let package = EcoString::from("application");
        let site = HostCallSite::new("host/marker".into(), "accept".into(), SourceSpan::new(0, 0));
        let signature = FunctionTemplateSignature::new(
            FunctionTemplateId::in_module(ModuleId::new(0), 0),
            TypeScheme::new(0),
            FunctionShape::new(Vec::new(), ValueShape::Bool),
        );
        let actual = HostCustomTypeSchema::new(
            "application",
            "domain/marker",
            "Marker",
            0,
            [HostCustomConstructorSchema::new(
                "Actual",
                Vec::<HostCustomFieldSchema>::new(),
            )],
        );

        assert_eq!(
            validate_host_custom_schema(
                &registry,
                &package,
                &site,
                &signature,
                &actual,
                HostCustomTypeAccess::SourceDeclaration,
            )
            .err(),
            Some(PlanError::HostProviderLink {
                package: "application".into(),
                module: "host/marker".into(),
                function: "accept".into(),
                reason: Box::new(HostProviderLinkReason::CustomSchemaMismatch {
                    expected: HostCustomTypeSchema::new(
                        "application",
                        "domain/marker",
                        "Marker",
                        0,
                        [HostCustomConstructorSchema::new(
                            "Expected",
                            Vec::<HostCustomFieldSchema>::new(),
                        )],
                    ),
                    actual,
                }),
            }),
        );
    }

    #[test]
    fn host_custom_types_preserve_source_visibility() {
        let custom_type =
            CustomTypeName::new("domain".into(), "domain/marker".into(), "Marker".into());
        let actual = HostCustomTypeSchema::new(
            "domain",
            "domain/marker",
            "Marker",
            0,
            [HostCustomConstructorSchema::new(
                "Marker",
                Vec::<HostCustomFieldSchema>::new(),
            )],
        );
        let signature = FunctionTemplateSignature::new(
            FunctionTemplateId::in_module(ModuleId::new(0), 0),
            TypeScheme::new(0),
            FunctionShape::new(Vec::new(), ValueShape::Bool),
        );
        let cases = [
            (
                CustomTypePublicity::Public,
                false,
                EcoString::from("application"),
                EcoString::from("host/custom"),
                Ok(()),
            ),
            (
                CustomTypePublicity::Internal,
                false,
                EcoString::from("domain"),
                EcoString::from("host/custom"),
                Ok(()),
            ),
            (
                CustomTypePublicity::Private,
                false,
                EcoString::from("domain"),
                EcoString::from("domain/marker"),
                Ok(()),
            ),
            (
                CustomTypePublicity::Public,
                true,
                EcoString::from("domain"),
                EcoString::from("domain/marker"),
                Ok(()),
            ),
            (
                CustomTypePublicity::Internal,
                false,
                EcoString::from("application"),
                EcoString::from("host/custom"),
                Err(PlanError::HostProviderLink {
                    package: "application".into(),
                    module: "host/custom".into(),
                    function: "accept".into(),
                    reason: Box::new(HostProviderLinkReason::CustomTypeVisibility {
                        custom_type: custom_type.clone(),
                    }),
                }),
            ),
            (
                CustomTypePublicity::Private,
                false,
                EcoString::from("domain"),
                EcoString::from("host/custom"),
                Err(PlanError::HostProviderLink {
                    package: "domain".into(),
                    module: "host/custom".into(),
                    function: "accept".into(),
                    reason: Box::new(HostProviderLinkReason::CustomTypeVisibility {
                        custom_type: custom_type.clone(),
                    }),
                }),
            ),
            (
                CustomTypePublicity::Public,
                true,
                EcoString::from("domain"),
                EcoString::from("host/custom"),
                Err(PlanError::HostProviderLink {
                    package: "domain".into(),
                    module: "host/custom".into(),
                    function: "accept".into(),
                    reason: Box::new(HostProviderLinkReason::CustomTypeVisibility {
                        custom_type: custom_type.clone(),
                    }),
                }),
            ),
        ];

        for (publicity, opaque, package, module, expected) in cases {
            let definition = CustomTypeDefinition::new(
                custom_type.clone(),
                publicity,
                opaque,
                Vec::new(),
                vec![CustomConstructorDefinition::new(
                    "Marker".into(),
                    0,
                    Vec::new(),
                )],
            );
            let registry = ProgramRegistry::new(vec![ModuleRegistry::new(
                "domain/marker".into(),
                vec![definition],
                Vec::new(),
                std::collections::HashMap::new(),
                ConstantSignatures::default(),
            )]);
            let site = HostCallSite::new(module, "accept".into(), SourceSpan::new(0, 0));

            assert_eq!(
                validate_host_custom_schema(
                    &registry,
                    &package,
                    &site,
                    &signature,
                    &actual,
                    HostCustomTypeAccess::SourceDeclaration,
                ),
                expected,
            );
        }
    }

    #[test]
    fn source_less_host_custom_types_require_a_public_non_opaque_surface() {
        let custom_type =
            CustomTypeName::new("domain".into(), "domain/marker".into(), "Marker".into());
        let actual = HostCustomTypeSchema::new(
            "domain",
            "domain/marker",
            "Marker",
            0,
            [HostCustomConstructorSchema::new(
                "Marker",
                Vec::<HostCustomFieldSchema>::new(),
            )],
        );
        let signature = FunctionTemplateSignature::new(
            FunctionTemplateId::in_module(ModuleId::new(0), 0),
            TypeScheme::new(0),
            FunctionShape::new(Vec::new(), ValueShape::Bool),
        );

        for (publicity, opaque) in [
            (CustomTypePublicity::Internal, false),
            (CustomTypePublicity::Private, false),
            (CustomTypePublicity::Public, true),
        ] {
            let definition = CustomTypeDefinition::new(
                custom_type.clone(),
                publicity,
                opaque,
                Vec::new(),
                vec![CustomConstructorDefinition::new(
                    "Marker".into(),
                    0,
                    Vec::new(),
                )],
            );
            let registry = ProgramRegistry::new(vec![ModuleRegistry::new(
                "domain/marker".into(),
                vec![definition],
                Vec::new(),
                std::collections::HashMap::new(),
                ConstantSignatures::default(),
            )]);
            let package = EcoString::from("domain");
            let site =
                HostCallSite::new("host/custom".into(), "accept".into(), SourceSpan::new(0, 0));

            assert_eq!(
                validate_host_custom_schema(
                    &registry,
                    &package,
                    &site,
                    &signature,
                    &actual,
                    HostCustomTypeAccess::SourceLessPublicSurface,
                ),
                Err(PlanError::HostProviderLink {
                    package: "domain".into(),
                    module: "host/custom".into(),
                    function: "accept".into(),
                    reason: Box::new(HostProviderLinkReason::CustomTypeVisibility {
                        custom_type: custom_type.clone(),
                    }),
                }),
            );
        }
    }

    #[test]
    fn opaque_custom_type_visibility_precedes_schema_matching() {
        let custom_type =
            CustomTypeName::new("domain".into(), "domain/marker".into(), "Marker".into());
        let definition = CustomTypeDefinition::new(
            custom_type.clone(),
            CustomTypePublicity::Public,
            true,
            Vec::new(),
            vec![CustomConstructorDefinition::new(
                "Expected".into(),
                0,
                Vec::new(),
            )],
        );
        let registry = ProgramRegistry::new(vec![ModuleRegistry::new(
            "domain/marker".into(),
            vec![definition],
            Vec::new(),
            std::collections::HashMap::new(),
            ConstantSignatures::default(),
        )]);
        let package = EcoString::from("domain");
        let site = HostCallSite::new("host/custom".into(), "accept".into(), SourceSpan::new(0, 0));
        let signature = FunctionTemplateSignature::new(
            FunctionTemplateId::in_module(ModuleId::new(0), 0),
            TypeScheme::new(0),
            FunctionShape::new(Vec::new(), ValueShape::Bool),
        );
        let actual = HostCustomTypeSchema::new(
            "domain",
            "domain/marker",
            "Marker",
            0,
            [HostCustomConstructorSchema::new(
                "Actual",
                Vec::<HostCustomFieldSchema>::new(),
            )],
        );

        assert_eq!(
            validate_host_custom_schema(
                &registry,
                &package,
                &site,
                &signature,
                &actual,
                HostCustomTypeAccess::SourceDeclaration,
            ),
            Err(PlanError::HostProviderLink {
                package: "domain".into(),
                module: "host/custom".into(),
                function: "accept".into(),
                reason: Box::new(HostProviderLinkReason::CustomTypeVisibility { custom_type }),
            }),
        );
    }

    #[test]
    fn source_less_host_custom_type_applies_every_declared_type_argument() {
        let custom_type =
            CustomTypeName::new("application".into(), "domain/box".into(), "Boxed".into());
        let definition = CustomTypeDefinition::new(
            custom_type.clone(),
            CustomTypePublicity::Public,
            false,
            vec![CustomTypeParameterId(0)],
            vec![CustomConstructorDefinition::new(
                "Boxed".into(),
                0,
                vec![CustomFieldDefinition::new(
                    None,
                    CustomTypeTemplate::Parameter(CustomTypeParameterId(0)),
                )],
            )],
        );
        let registry = ProgramRegistry::new(vec![ModuleRegistry::new(
            "domain/box".into(),
            vec![definition],
            Vec::new(),
            std::collections::HashMap::new(),
            ConstantSignatures::default(),
        )]);
        let package = EcoString::from("application");
        let site = HostCallSite::new("host/box".into(), "accept".into(), SourceSpan::new(0, 0));
        let signature = FunctionTemplateSignature::new(
            FunctionTemplateId::in_module(ModuleId::new(0), 0),
            TypeScheme::new(0),
            FunctionShape::new(
                vec![ValueShape::Tuple(
                    vec![ValueShape::Custom(CustomValueShape::new(
                        custom_type.clone(),
                        Vec::new(),
                        CustomConstructorRefinement::Any,
                    ))]
                    .into_boxed_slice(),
                )],
                ValueShape::Bool,
            ),
        );
        let actual = HostCustomTypeSchema::new(
            "application",
            "domain/box",
            "Boxed",
            1,
            [HostCustomConstructorSchema::new(
                "Boxed",
                [HostCustomFieldSchema::new(
                    None::<EcoString>,
                    HostSchemaType::parameter(0),
                )],
            )],
        );

        assert_eq!(
            validate_host_custom_schema(
                &registry,
                &package,
                &site,
                &signature,
                &actual,
                HostCustomTypeAccess::SourceLessPublicSurface,
            )
            .err(),
            Some(PlanError::HostProviderLink {
                package: "application".into(),
                module: "host/box".into(),
                function: "accept".into(),
                reason: Box::new(HostProviderLinkReason::CustomTypeArgumentCount {
                    custom_type: CustomTypeName::new(
                        "application".into(),
                        "domain/box".into(),
                        "Boxed".into(),
                    ),
                    expected: 1,
                    actual: 0,
                }),
            }),
        );
    }
}
