use crate::host::HostExternalTypeSchema;
use crate::plan::{ExternalTypeDefinition, ExternalTypeName};
use crate::planner::error::{ExternalTypeProviderLinkReason, PlanError};
use ecow::EcoString;
use gleam_core::ast::TypedCustomType;
use std::collections::{BTreeMap, BTreeSet};

pub(super) struct HostedTypeDefinitions {
    pub(super) custom_types: Vec<crate::plan::CustomTypeDefinition>,
    pub(super) external_types: Vec<ExternalTypeDefinition>,
}

pub(super) fn plan_hosted_types(
    package: &EcoString,
    module: &EcoString,
    types: Vec<TypedCustomType>,
    registrations: Vec<HostExternalTypeSchema>,
    registered_external_types: &std::collections::HashSet<ExternalTypeName>,
) -> Result<HostedTypeDefinitions, PlanError> {
    let mut source_types = types
        .into_iter()
        .map(|type_| (type_.name.clone(), type_))
        .collect::<BTreeMap<_, _>>();
    let mut registrations = registrations
        .into_iter()
        .map(|schema| (schema.name().clone(), schema))
        .collect::<BTreeMap<_, _>>();
    let mut linked_external_types = BTreeSet::new();
    let mut custom_types = Vec::new();
    let mut external_types = Vec::new();

    for (name, type_) in &source_types {
        let has_backend_external =
            type_.external_erlang.is_some() || type_.external_javascript.is_some();
        let Some(schema) = registrations.remove(name) else {
            if has_backend_external {
                return Err(PlanError::ExternalTypeProviderLink {
                    package: package.clone(),
                    module: module.clone(),
                    type_: name.clone(),
                    reason: Box::new(ExternalTypeProviderLinkReason::MissingRegistration),
                });
            }
            continue;
        };
        if !type_.constructors.is_empty() {
            return Err(PlanError::ExternalTypeProviderLink {
                package: package.clone(),
                module: module.clone(),
                type_: name.clone(),
                reason: Box::new(ExternalTypeProviderLinkReason::ConstructorBackedType),
            });
        }

        let expected = ExternalTypeName::new(package.clone(), module.clone(), name.clone());
        let actual = ExternalTypeName::new(
            schema.package().clone(),
            schema.module().clone(),
            schema.name().clone(),
        );
        if actual != expected {
            return Err(PlanError::ExternalTypeProviderLink {
                package: package.clone(),
                module: module.clone(),
                type_: name.clone(),
                reason: Box::new(ExternalTypeProviderLinkReason::IdentityMismatch {
                    expected,
                    actual,
                }),
            });
        }
        let expected = type_.typed_parameters.len();
        let actual = schema.parameter_count();
        if actual != expected {
            return Err(PlanError::ExternalTypeProviderLink {
                package: package.clone(),
                module: module.clone(),
                type_: name.clone(),
                reason: Box::new(ExternalTypeProviderLinkReason::ParameterCount {
                    expected,
                    actual,
                }),
            });
        }
        linked_external_types.insert(name.clone());
    }

    if let Some((name, _)) = registrations.into_iter().next() {
        return Err(PlanError::ExternalTypeProviderLink {
            package: package.clone(),
            module: module.clone(),
            type_: name,
            reason: Box::new(ExternalTypeProviderLinkReason::MissingDeclaration),
        });
    }

    for (name, type_) in std::mem::take(&mut source_types) {
        if linked_external_types.contains(&name) {
            external_types.push(ExternalTypeDefinition::new(
                ExternalTypeName::new(package.clone(), module.clone(), type_.name),
                type_.typed_parameters.len(),
            ));
        } else {
            custom_types.extend(super::custom_type::plan_custom_types_with_external(
                package,
                module,
                vec![type_],
                registered_external_types,
            )?);
        }
    }

    Ok(HostedTypeDefinitions {
        custom_types,
        external_types,
    })
}

pub(super) fn validate_host_external_schema(
    registry: &super::registry::ProgramRegistry,
    package: &EcoString,
    site: &crate::plan::HostCallSite,
    signature: &crate::plan::FunctionTemplateSignature,
    actual: &HostExternalTypeSchema,
    constructions: &[crate::host::HostTypeDescriptor],
) -> Result<(), PlanError> {
    let name = ExternalTypeName::new(
        actual.package().clone(),
        actual.module().clone(),
        actual.name().clone(),
    );
    let definition = registry
        .external_type(&name)
        .ok_or_else(|| PlanError::HostProviderLink {
            package: package.clone(),
            module: site.module().clone(),
            function: site.function().clone(),
            reason: Box::new(
                crate::planner::error::HostProviderLinkReason::MissingExternalType {
                    external_type: name.clone(),
                },
            ),
        })?;
    let expected = HostExternalTypeSchema::new(
        definition.name().package().clone(),
        definition.name().module().clone(),
        definition.name().name().clone(),
        definition.parameters().len(),
    );
    if actual != &expected {
        return Err(PlanError::HostProviderLink {
            package: package.clone(),
            module: site.module().clone(),
            function: site.function().clone(),
            reason: Box::new(
                crate::planner::error::HostProviderLinkReason::ExternalSchemaMismatch {
                    expected,
                    actual: actual.clone(),
                },
            ),
        });
    }
    let parameter_count = definition.parameters().len();
    let invalid_argument_count = signature
        .shape()
        .argument_shapes()
        .iter()
        .chain([signature.shape().return_shape()])
        .find_map(|shape| invalid_host_external_type_argument_count(shape, &name, parameter_count))
        .or_else(|| {
            constructions.iter().find_map(|construction| {
                invalid_host_external_type_argument_count(
                    &construction.value_shape(),
                    &name,
                    parameter_count,
                )
            })
        });
    if let Some(actual) = invalid_argument_count {
        return Err(PlanError::HostProviderLink {
            package: package.clone(),
            module: site.module().clone(),
            function: site.function().clone(),
            reason: Box::new(
                crate::planner::error::HostProviderLinkReason::ExternalTypeArgumentCount {
                    external_type: name,
                    expected: parameter_count,
                    actual,
                },
            ),
        });
    }
    Ok(())
}

fn invalid_host_external_type_argument_count(
    shape: &crate::plan::ValueShape,
    external_type: &ExternalTypeName,
    expected: usize,
) -> Option<usize> {
    let mut pending = vec![shape];
    while let Some(shape) = pending.pop() {
        match shape {
            crate::plan::ValueShape::Tuple(elements) => pending.extend(elements.iter().rev()),
            crate::plan::ValueShape::List(item) => pending.push(item),
            crate::plan::ValueShape::Custom(custom) => {
                pending.extend(custom.arguments().iter().rev());
            }
            crate::plan::ValueShape::External(external) => {
                if external.type_name() == external_type && external.arguments().len() != expected {
                    return Some(external.arguments().len());
                }
                pending.extend(external.arguments().iter().rev());
            }
            crate::plan::ValueShape::Function(function) => {
                pending.push(function.return_shape());
                pending.extend(function.argument_shapes().iter().rev());
            }
            crate::plan::ValueShape::Parameter(_)
            | crate::plan::ValueShape::Int
            | crate::plan::ValueShape::Float
            | crate::plan::ValueShape::String
            | crate::plan::ValueShape::BitArray
            | crate::plan::ValueShape::UtfCodepoint
            | crate::plan::ValueShape::Bool
            | crate::plan::ValueShape::Nil => {}
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{super::host::plan_host_program, plan_hosted_types, validate_host_external_schema};
    use crate::frontend::{
        ModuleSource, PackageSource, compile_typed_host_program, compile_typed_module,
    };
    use crate::host::{
        ExternalTestProfile, ExternalTestRunState, HostCall, HostCallCompletion, HostCallError,
        HostExternalBinding, HostExternalSchema, HostExternalStorage, HostExternalStore,
        HostExternalType, HostExternalTypeSchema, HostModule, HostProvider, HostProviderModule,
        HostProviderSet, HostTypeDescriptor,
    };
    use crate::plan::{
        CustomConstructorRefinement, CustomTypeName, CustomValueShape, ExternalTypeDefinition,
        ExternalTypeName, ExternalValueShape, FunctionShape, FunctionTemplateId,
        FunctionTemplateSignature, HostCallSite, ModuleId, SourceSpan, TypeScheme, ValueShape,
    };
    use crate::planner::module::constant::ConstantSignatures;
    use crate::planner::module::registry::{ModuleRegistry, ProgramRegistry};
    use crate::planner::{
        ExternalTypeProviderLinkReason, HostProviderLinkReason, InvalidCustomTypeReason,
        InvalidTypedAstReason, PlanError,
    };
    use ecow::EcoString;

    struct ThingSchema;

    struct ThingProvider;

    struct ThingStorage;

    type HostThing = HostExternalType<ThingSchema>;

    impl HostExternalSchema for ThingSchema {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "main";
        const NAME: &'static str = "Thing";
        const PARAMETER_COUNT: usize = 0;
    }

    impl HostExternalStorage<ExternalTestProfile, ThingSchema> for ThingStorage {
        type Payload = ();

        fn store(
            stores: &<ExternalTestProfile as crate::HostProfile>::ExternalStores,
        ) -> &HostExternalStore<Self::Payload> {
            &stores.units
        }

        fn source_equal(
            _: &crate::host::HostExternalEquality<'_>,
            _: &Self::Payload,
            _: &Self::Payload,
        ) -> bool {
            true
        }

        fn source_hash(_: &crate::host::HostExternalHashing<'_>, _: &Self::Payload) -> u64 {
            0
        }

        fn inspect(_: &crate::host::HostExternalInspection<'_>, _: &Self::Payload) -> EcoString {
            "Thing".into()
        }
    }

    impl HostProvider<ExternalTestProfile> for ThingProvider {
        type State = ();

        fn project(state: &mut ExternalTestRunState) -> &mut Self::State {
            &mut state.provider
        }
    }

    impl HostExternalBinding<ExternalTestProfile, ThingSchema> for ThingProvider {
        type Storage = ThingStorage;
    }

    fn new_thing<'call>(
        mut call: HostCall<'call, ExternalTestProfile, ThingProvider, HostThing>,
    ) -> Result<HostCallCompletion<'call, HostThing>, HostCallError> {
        let _ = call.state();
        let thing = call.create_external(());
        Ok(call.return_value(thing))
    }

    #[test]
    fn thing_fixture_source_hash_is_exact() {
        let retained_hash = |_: &crate::runtime::StoredRuntimeValue| 7;
        let hashing = crate::host::HostExternalHashing::new(&retained_hash);

        assert_eq!(
            <ThingStorage as HostExternalStorage<ExternalTestProfile, ThingSchema>>::source_hash(
                &hashing,
                &(),
            ),
            0,
        );
    }

    #[test]
    fn host_external_schema_requires_a_planned_external_type() {
        let registry = ProgramRegistry::new(Vec::new());
        let package = EcoString::from("application");
        let site = HostCallSite::new("main".into(), "accept".into(), SourceSpan::new(0, 0));
        let signature = FunctionTemplateSignature::new(
            FunctionTemplateId::in_module(ModuleId::new(0), 0),
            TypeScheme::new(0),
            FunctionShape::new(Vec::new(), ValueShape::Bool),
        );
        let actual = HostExternalTypeSchema::new("application", "main", "Thing", 0);

        assert_eq!(
            validate_host_external_schema(&registry, &package, &site, &signature, &actual, &[])
                .err(),
            Some(PlanError::HostProviderLink {
                package: "application".into(),
                module: "main".into(),
                function: "accept".into(),
                reason: Box::new(HostProviderLinkReason::MissingExternalType {
                    external_type: ExternalTypeName::new(
                        "application".into(),
                        "main".into(),
                        "Thing".into(),
                    ),
                }),
            }),
        );
    }

    #[test]
    fn host_external_schema_mismatch_precedes_shape_validation() {
        let external_type =
            ExternalTypeName::new("application".into(), "main".into(), "Thing".into());
        let registry = ProgramRegistry::new(vec![ModuleRegistry::new(
            "main".into(),
            Vec::new(),
            vec![ExternalTypeDefinition::new(external_type.clone(), 1)],
            std::collections::HashMap::new(),
            ConstantSignatures::default(),
        )]);
        let package = EcoString::from("application");
        let site = HostCallSite::new("main".into(), "accept".into(), SourceSpan::new(0, 0));
        let signature = FunctionTemplateSignature::new(
            FunctionTemplateId::in_module(ModuleId::new(0), 0),
            TypeScheme::new(0),
            FunctionShape::new(
                vec![ValueShape::External(ExternalValueShape::new(
                    external_type,
                    Vec::new(),
                ))],
                ValueShape::Bool,
            ),
        );
        let actual = HostExternalTypeSchema::new("application", "main", "Thing", 0);

        assert_eq!(
            validate_host_external_schema(&registry, &package, &site, &signature, &actual, &[])
                .err(),
            Some(PlanError::HostProviderLink {
                package: "application".into(),
                module: "main".into(),
                function: "accept".into(),
                reason: Box::new(HostProviderLinkReason::ExternalSchemaMismatch {
                    expected: HostExternalTypeSchema::new("application", "main", "Thing", 1,),
                    actual,
                }),
            }),
        );
    }

    #[test]
    fn host_external_type_applies_every_argument_inside_nested_shapes() {
        let external_type =
            ExternalTypeName::new("application".into(), "main".into(), "Thing".into());
        let registry = ProgramRegistry::new(vec![ModuleRegistry::new(
            "main".into(),
            Vec::new(),
            vec![ExternalTypeDefinition::new(external_type.clone(), 1)],
            std::collections::HashMap::new(),
            ConstantSignatures::default(),
        )]);
        let valid_external = ValueShape::External(ExternalValueShape::new(
            external_type.clone(),
            vec![ValueShape::Int],
        ));
        let invalid_external =
            ValueShape::External(ExternalValueShape::new(external_type.clone(), Vec::new()));
        let signature = FunctionTemplateSignature::new(
            FunctionTemplateId::in_module(ModuleId::new(0), 0),
            TypeScheme::new(0),
            FunctionShape::new(
                vec![
                    ValueShape::Tuple(
                        vec![ValueShape::List(Box::new(ValueShape::Custom(
                            CustomValueShape::new(
                                CustomTypeName::new(
                                    "application".into(),
                                    "main".into(),
                                    "Wrapper".into(),
                                ),
                                vec![valid_external],
                                CustomConstructorRefinement::Any,
                            ),
                        )))]
                        .into_boxed_slice(),
                    ),
                    ValueShape::Function(Box::new(FunctionShape::new(
                        vec![ValueShape::Bool],
                        invalid_external,
                    ))),
                ],
                ValueShape::Nil,
            ),
        );
        let package = EcoString::from("application");
        let site = HostCallSite::new("main".into(), "accept".into(), SourceSpan::new(0, 0));
        let actual = HostExternalTypeSchema::new("application", "main", "Thing", 1);

        assert_eq!(
            validate_host_external_schema(&registry, &package, &site, &signature, &actual, &[])
                .err(),
            Some(PlanError::HostProviderLink {
                package: "application".into(),
                module: "main".into(),
                function: "accept".into(),
                reason: Box::new(HostProviderLinkReason::ExternalTypeArgumentCount {
                    external_type,
                    expected: 1,
                    actual: 0,
                }),
            }),
        );
    }

    #[test]
    fn hidden_construction_external_type_applies_every_declared_type_argument() {
        let external_type =
            ExternalTypeName::new("application".into(), "main".into(), "Thing".into());
        let registry = ProgramRegistry::new(vec![ModuleRegistry::new(
            "main".into(),
            Vec::new(),
            vec![ExternalTypeDefinition::new(external_type.clone(), 1)],
            std::collections::HashMap::new(),
            ConstantSignatures::default(),
        )]);
        let signature = FunctionTemplateSignature::new(
            FunctionTemplateId::in_module(ModuleId::new(0), 0),
            TypeScheme::new(0),
            FunctionShape::new(Vec::new(), ValueShape::Bool),
        );
        let package = EcoString::from("application");
        let site = HostCallSite::new("main".into(), "build".into(), SourceSpan::new(0, 0));
        let actual = HostExternalTypeSchema::new("application", "main", "Thing", 1);
        let constructions = [HostTypeDescriptor::External {
            schema: actual.clone(),
            arguments: Box::new([]),
        }];

        assert_eq!(
            validate_host_external_schema(
                &registry,
                &package,
                &site,
                &signature,
                &actual,
                &constructions,
            ),
            Err(PlanError::HostProviderLink {
                package: "application".into(),
                module: "main".into(),
                function: "build".into(),
                reason: Box::new(HostProviderLinkReason::ExternalTypeArgumentCount {
                    external_type,
                    expected: 1,
                    actual: 0,
                }),
            }),
        );
    }

    #[test]
    fn plans_registered_external_types_inside_ordinary_custom_fields() {
        let source = r#"
pub type Thing

pub type Boxed {
  Boxed(Thing)
}

@external(erlang, "external", "new_thing")
fn new_thing() -> Thing

pub fn main() {
  let thing = new_thing()
  #(Boxed(thing), thing == new_thing())
}
"#;
        let provider = HostProviderModule::<ExternalTestProfile>::new("application", "main")
            .expect("provider module should be valid")
            .with_external_type::<ThingProvider, ThingSchema>()
            .expect("external type should be valid")
            .with_scoped_function::<ThingProvider, (), HostThing, _>("new_thing", new_thing)
            .expect("external constructor should be valid");
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                Vec::<EcoString>::new(),
                [ModuleSource::new("main", "main.gleam", source)],
            )],
            HostProviderSet::with_providers(
                Vec::<HostModule<ExternalTestProfile>>::new(),
                [provider],
            )
            .expect("provider module should be unique"),
        )
        .expect("source should compile");
        let plan = plan_host_program(typed).expect("source should plan");

        assert_eq!(plan.modules()[0].external_types().len(), 1);
        let execution = crate::HostedExecution::try_from_module_plan(plan)
            .expect("external execution should seal");
        let returned = execution
            .run_main(&mut ExternalTestRunState::default(), &mut Vec::new())
            .expect("external source should execute");

        assert_eq!(returned.inspect().to_string(), "#(Boxed(Thing), True)");
    }

    #[test]
    fn hosted_type_planning_preserves_ordinary_custom_type_errors() {
        let mut typed = compile_typed_module(
            "main",
            "main.gleam",
            "pub type Box(value) { Box(value) } pub fn main() { 1 }",
        )
        .expect("generic custom type should analyse");
        typed.definitions.custom_types[0].typed_parameters[0] = gleam_core::type_::int();

        assert_eq!(
            plan_hosted_types(
                &typed.type_info.package,
                &typed.name,
                typed.definitions.custom_types,
                Vec::new(),
                &std::collections::HashSet::new(),
            )
            .err(),
            Some(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    package: "geam".into(),
                    module: "main".into(),
                    name: "Box".into(),
                    reason: Box::new(InvalidCustomTypeReason::DefinitionParameter { index: 0 }),
                },
            }),
        );
    }

    #[test]
    fn preserves_unregistered_constructorless_ordinary_types() {
        let typed =
            compile_typed_module("main", "main.gleam", "pub type Empty pub fn main() { 1 }")
                .expect("constructorless ordinary type should analyse");

        let definitions = plan_hosted_types(
            &typed.type_info.package,
            &typed.name,
            typed.definitions.custom_types,
            Vec::new(),
            &std::collections::HashSet::new(),
        )
        .expect("constructorless ordinary type should plan");

        assert!(definitions.external_types.is_empty());
        assert_eq!(definitions.custom_types.len(), 1);
        assert_eq!(definitions.custom_types[0].name().name(), "Empty");
        assert!(definitions.custom_types[0].constructors().is_empty());
    }

    #[test]
    fn reject_profile_external_registration_for_a_constructor_backed_type() {
        let source = r#"
pub type Thing {
  Thing
}

pub fn main() { Thing }
"#;
        let provider = HostProviderModule::<ExternalTestProfile>::new("application", "main")
            .expect("provider module should be valid")
            .with_external_type::<ThingProvider, ThingSchema>()
            .expect("external type should be valid");
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                Vec::<EcoString>::new(),
                [ModuleSource::new("main", "main.gleam", source)],
            )],
            HostProviderSet::with_providers(
                Vec::<HostModule<ExternalTestProfile>>::new(),
                [provider],
            )
            .expect("provider module should be unique"),
        )
        .expect("source should compile");

        assert_eq!(
            plan_host_program(typed).err(),
            Some(PlanError::ExternalTypeProviderLink {
                package: "application".into(),
                module: "main".into(),
                type_: "Thing".into(),
                reason: Box::new(ExternalTypeProviderLinkReason::ConstructorBackedType),
            }),
        );
    }

    #[test]
    fn constructor_backed_type_precedes_identity_and_parameter_validation() {
        let typed = compile_typed_module(
            "main",
            "main.gleam",
            "pub type Thing(value) { Thing(value) } pub fn main() { 1 }",
        )
        .expect("constructor-backed type should analyse");

        assert_eq!(
            plan_hosted_types(
                &"application".into(),
                &"main".into(),
                typed.definitions.custom_types,
                vec![HostExternalTypeSchema::new("wrong", "main", "Thing", 0)],
                &std::collections::HashSet::new(),
            )
            .err(),
            Some(PlanError::ExternalTypeProviderLink {
                package: "application".into(),
                module: "main".into(),
                type_: "Thing".into(),
                reason: Box::new(ExternalTypeProviderLinkReason::ConstructorBackedType),
            }),
        );
    }

    #[test]
    fn reject_profile_external_registration_identity_before_parameter_count() {
        let source = r#"
pub type Thing

pub fn main() { 1 }
"#;
        let typed =
            compile_typed_module("main", "main.gleam", source).expect("source should compile");
        let package = EcoString::from("application");
        let module = EcoString::from("main");
        let actual = HostExternalTypeSchema::new("wrong", "main", "Thing", 1);

        assert_eq!(
            plan_hosted_types(
                &package,
                &module,
                typed.definitions.custom_types,
                vec![actual],
                &std::collections::HashSet::new(),
            )
            .err(),
            Some(PlanError::ExternalTypeProviderLink {
                package: "application".into(),
                module: "main".into(),
                type_: "Thing".into(),
                reason: Box::new(ExternalTypeProviderLinkReason::IdentityMismatch {
                    expected: ExternalTypeName::new(
                        "application".into(),
                        "main".into(),
                        "Thing".into(),
                    ),
                    actual: ExternalTypeName::new("wrong".into(), "main".into(), "Thing".into(),),
                }),
            }),
        );
    }

    #[test]
    fn reject_profile_external_registration_parameter_count_mismatch() {
        let source = r#"
pub type Thing(a)

pub fn main() { 1 }
"#;
        let provider = HostProviderModule::<ExternalTestProfile>::new("application", "main")
            .expect("provider module should be valid")
            .with_external_type::<ThingProvider, ThingSchema>()
            .expect("external type should be valid");
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                Vec::<EcoString>::new(),
                [ModuleSource::new("main", "main.gleam", source)],
            )],
            HostProviderSet::with_providers(
                Vec::<HostModule<ExternalTestProfile>>::new(),
                [provider],
            )
            .expect("provider module should be unique"),
        )
        .expect("source should compile");

        assert_eq!(
            plan_host_program(typed).err(),
            Some(PlanError::ExternalTypeProviderLink {
                package: "application".into(),
                module: "main".into(),
                type_: "Thing".into(),
                reason: Box::new(ExternalTypeProviderLinkReason::ParameterCount {
                    expected: 1,
                    actual: 0,
                }),
            }),
        );
    }

    #[test]
    fn reject_profile_external_registration_without_a_declaration() {
        let provider = HostProviderModule::<ExternalTestProfile>::new("application", "main")
            .expect("provider module should be valid")
            .with_external_type::<ThingProvider, ThingSchema>()
            .expect("external type should be valid");
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                Vec::<EcoString>::new(),
                [ModuleSource::new(
                    "main",
                    "main.gleam",
                    "pub fn main() { 1 }",
                )],
            )],
            HostProviderSet::with_providers(
                Vec::<HostModule<ExternalTestProfile>>::new(),
                [provider],
            )
            .expect("provider module should be unique"),
        )
        .expect("source should compile");

        assert_eq!(
            plan_host_program(typed).err(),
            Some(PlanError::ExternalTypeProviderLink {
                package: "application".into(),
                module: "main".into(),
                type_: "Thing".into(),
                reason: Box::new(ExternalTypeProviderLinkReason::MissingDeclaration),
            }),
        );
    }

    #[test]
    fn reject_profile_external_registration_without_a_source_module() {
        let provider = HostProviderModule::<ExternalTestProfile>::new("application", "missing")
            .expect("provider module should be valid")
            .with_external_type::<ThingProvider, ThingSchema>()
            .expect("external type should be valid");
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                Vec::<EcoString>::new(),
                [ModuleSource::new(
                    "main",
                    "main.gleam",
                    "pub fn main() { 1 }",
                )],
            )],
            HostProviderSet::with_providers(
                Vec::<HostModule<ExternalTestProfile>>::new(),
                [provider],
            )
            .expect("provider module should be unique"),
        )
        .expect("source should compile");

        assert_eq!(
            plan_host_program(typed).err(),
            Some(PlanError::ExternalTypeProviderLink {
                package: "application".into(),
                module: "missing".into(),
                type_: "Thing".into(),
                reason: Box::new(ExternalTypeProviderLinkReason::MissingModule),
            }),
        );
    }
}
