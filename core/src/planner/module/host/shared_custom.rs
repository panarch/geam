use crate::host::HostCustomTypeSchema;
use crate::plan::{CustomTypeDefinition, CustomTypeName};
use crate::planner::{PlanError, SharedCustomTypeProviderLinkReason};
use ecow::EcoString;

pub(super) fn validate(
    package: &EcoString,
    module: &EcoString,
    definitions: &[CustomTypeDefinition],
    schemas: Vec<HostCustomTypeSchema>,
) -> Result<Vec<CustomTypeName>, PlanError> {
    schemas
        .into_iter()
        .map(|actual| {
            let failure = |reason| PlanError::SharedCustomTypeProviderLink {
                package: package.clone(),
                module: module.clone(),
                type_: actual.name().clone(),
                reason: Box::new(reason),
            };
            let definition = definitions
                .iter()
                .find(|definition| definition.name().name() == actual.name())
                .ok_or_else(|| failure(SharedCustomTypeProviderLinkReason::MissingDeclaration))?;
            let expected =
                super::link::host_custom_type_schema(definition).with_shared_access(true);
            if expected != actual {
                return Err(failure(
                    SharedCustomTypeProviderLinkReason::SchemaMismatch {
                        expected,
                        actual: actual.clone(),
                    },
                ));
            }
            Ok(definition.name().clone())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::validate;
    use crate::embedding::HostPreparation;
    use crate::host::{
        HostCustomConstructorDefinition, HostCustomConstructorList, HostCustomConstructorListEnd,
        HostCustomField, HostCustomFieldList, HostCustomFieldListEnd, HostCustomSchema,
        HostCustomType, HostCustomTypeArgument, HostCustomTypeSchema, HostDeclarations,
        HostFunctionDeclaration, HostProviderModuleDeclaration, HostTypeIndex0, HostTypeList,
        HostTypeListEnd, HostTypeParameter,
    };
    use crate::plan::{
        CustomConstructorDefinition, CustomTypeDefinition, CustomTypeName, CustomTypePublicity,
    };
    use crate::planner::{PlanError, SharedCustomTypeProviderLinkReason};
    use crate::{ModuleSource, PackageSource};

    struct Handle;
    struct Constructor;
    struct Field;
    impl HostCustomSchema for Handle {
        const PACKAGE: &'static str = "producer";
        const MODULE: &'static str = "handles";
        const NAME: &'static str = "Handle";
        const PARAMETER_COUNT: usize = 1;
        const SHARED: bool = true;
        type Constructors = HostCustomConstructorList<Constructor, HostCustomConstructorListEnd>;
    }
    impl HostCustomConstructorDefinition for Constructor {
        const NAME: &'static str = "Handle";
        type Fields = HostCustomFieldList<Field, HostCustomFieldListEnd>;
    }
    impl HostCustomField for Field {
        const LABEL: Option<&'static str> = None;
        type Type = HostCustomTypeArgument<HostTypeIndex0>;
    }
    type Shared = HostCustomType<Handle, HostTypeList<HostTypeParameter<0>, HostTypeListEnd>>;

    #[test]
    fn producer_grant_is_validated_even_without_native_functions() {
        let declarations = |shared| {
            let producer = HostProviderModuleDeclaration::new("producer", "handles").unwrap();
            let producer = if shared {
                producer.with_shared_custom_type::<Handle>().unwrap()
            } else {
                producer
            };
            HostDeclarations::from_providers([
                producer,
                HostProviderModuleDeclaration::new("consumer", "consumer")
                    .unwrap()
                    .with_function(HostFunctionDeclaration::<(Shared,), Shared>::new("retain"))
                    .unwrap(),
            ])
            .unwrap()
        };
        let plan = |shared| {
            let typed = crate::compile_declared_host_program(
                "consumer",
                "consumer",
                [
                    PackageSource::new(
                        "producer",
                        Vec::<&str>::new(),
                        [ModuleSource::new(
                            "handles",
                            "handles.gleam",
                            r#"
pub opaque type Handle(a) { Handle(a) }
pub fn make(value: a) { Handle(value) }
pub fn get(value: Handle(a)) { case value { Handle(item) -> item } }
"#,
                        )],
                    ),
                    PackageSource::new(
                        "consumer",
                        ["producer"],
                        [ModuleSource::new(
                            "consumer",
                            "consumer.gleam",
                            r#"
import handles
@external(erlang, "native", "retain")
fn retain(value: handles.Handle(a)) -> handles.Handle(a)
pub fn main() { handles.get(retain(handles.make(42))) }
"#,
                        )],
                    ),
                ],
                declarations(shared),
            )
            .unwrap();
            HostPreparation::new(typed)
        };
        assert!(plan(true).is_ok());
        assert_eq!(
            plan(false).err(),
            Some(PlanError::HostProviderLink {
                package: "consumer".into(),
                module: "consumer".into(),
                function: "retain".into(),
                reason: Box::new(crate::HostProviderLinkReason::MissingSharedCustomType {
                    custom_type: CustomTypeName::new(
                        "producer".into(),
                        "handles".into(),
                        "Handle".into()
                    ),
                }),
            })
        );
    }

    #[test]
    fn sharing_checks_the_exact_source_declaration_before_consumers() {
        let name = CustomTypeName::new("producer".into(), "handles".into(), "Handle".into());
        let definition = CustomTypeDefinition::new(
            name.clone(),
            CustomTypePublicity::Public,
            true,
            Vec::new(),
            vec![CustomConstructorDefinition::new(
                "Handle".into(),
                0,
                Vec::new(),
            )],
        );
        let matching =
            super::super::link::host_custom_type_schema(&definition).with_shared_access(true);
        assert_eq!(
            validate(
                &"producer".into(),
                &"handles".into(),
                std::slice::from_ref(&definition),
                vec![matching.clone()]
            ),
            Ok(vec![name])
        );
        let missing = validate(
            &"producer".into(),
            &"handles".into(),
            &[],
            vec![matching.clone()],
        )
        .unwrap_err();
        assert_eq!(
            missing,
            PlanError::SharedCustomTypeProviderLink {
                package: "producer".into(),
                module: "handles".into(),
                type_: "Handle".into(),
                reason: Box::new(SharedCustomTypeProviderLinkReason::MissingDeclaration),
            }
        );
        assert_eq!(
            missing.to_string(),
            "shared custom type provider producer::handles.Handle: custom type declaration is missing"
        );
        let actual = HostCustomTypeSchema::of::<Handle>();
        assert_eq!(
            validate(
                &"producer".into(),
                &"handles".into(),
                &[definition],
                vec![actual.clone()]
            ),
            Err(PlanError::SharedCustomTypeProviderLink {
                package: "producer".into(),
                module: "handles".into(),
                type_: "Handle".into(),
                reason: Box::new(SharedCustomTypeProviderLinkReason::SchemaMismatch {
                    expected: matching.clone(),
                    actual: actual.clone()
                }),
            })
        );
        for (source, reason) in [
            (
                "pub fn main() { 42 }",
                SharedCustomTypeProviderLinkReason::MissingDeclaration,
            ),
            (
                "pub opaque type Handle { Handle }\npub fn main() { 42 }",
                SharedCustomTypeProviderLinkReason::SchemaMismatch {
                    expected: matching,
                    actual,
                },
            ),
        ] {
            let owner = HostProviderModuleDeclaration::new("producer", "handles")
                .unwrap()
                .with_shared_custom_type::<Handle>()
                .unwrap();
            let typed = crate::compile_declared_host_program(
                "producer",
                "handles",
                [PackageSource::new(
                    "producer",
                    Vec::<&str>::new(),
                    [ModuleSource::new("handles", "handles.gleam", source)],
                )],
                HostDeclarations::from_providers([owner]).unwrap(),
            )
            .unwrap();
            assert_eq!(
                HostPreparation::new(typed).err(),
                Some(PlanError::SharedCustomTypeProviderLink {
                    package: "producer".into(),
                    module: "handles".into(),
                    type_: "Handle".into(),
                    reason: Box::new(reason),
                })
            );
        }
    }
}
