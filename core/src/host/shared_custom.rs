use super::module::HostModuleIdentity;
use super::{HostCustomTypeSchema, HostRegistrationError};
use gleam_compiler_core::analyse::name::check_name_case;
use gleam_compiler_core::ast::SrcSpan;
use gleam_compiler_core::type_::error::Named;

pub(super) struct RegisteredSharedCustomTypes {
    types: Vec<HostCustomTypeSchema>,
}

impl RegisteredSharedCustomTypes {
    pub(super) fn new() -> Self {
        Self { types: Vec::new() }
    }

    pub(super) fn register(
        &mut self,
        owner: &HostModuleIdentity,
        schema: HostCustomTypeSchema,
    ) -> Result<(), HostRegistrationError> {
        let schema = schema.with_shared_access(true);
        if schema.package() != &owner.package || schema.module() != &owner.module {
            return Err(HostRegistrationError::SharedCustomTypeOwner {
                package: owner.package.clone(),
                module: owner.module.clone(),
                custom_type: crate::plan::CustomTypeName::new(
                    schema.package().clone(),
                    schema.module().clone(),
                    schema.name().clone(),
                ),
            });
        }
        if check_name_case(SrcSpan::new(0, 0), schema.name(), Named::Type).is_err() {
            return Err(HostRegistrationError::InvalidSharedCustomTypeName {
                module: owner.module.clone(),
                type_: schema.name().clone(),
            });
        }
        if self
            .types
            .iter()
            .any(|registered| registered.name() == schema.name())
        {
            return Err(HostRegistrationError::DuplicateSharedCustomType {
                module: owner.module.clone(),
                type_: schema.name().clone(),
            });
        }
        self.types.push(schema);
        Ok(())
    }

    pub(super) fn into_vec(self) -> Vec<HostCustomTypeSchema> {
        self.types
    }
}

#[cfg(test)]
mod tests {
    use super::RegisteredSharedCustomTypes;
    use crate::host::module::HostModuleIdentity;
    use crate::{
        HostCustomConstructorListEnd, HostCustomSchema, HostCustomTypeSchema, HostDeclarations,
        HostProviderModule, HostProviderModuleDeclaration, HostProviderSet, HostRegistrationError,
        StatelessHostProfile,
    };

    struct Shared;
    struct Invalid;

    impl HostCustomSchema for Shared {
        const PACKAGE: &'static str = "producer";
        const MODULE: &'static str = "handles";
        const NAME: &'static str = "Handle";
        const PARAMETER_COUNT: usize = 1;
        const SHARED: bool = true;
        type Constructors = HostCustomConstructorListEnd;
    }

    impl HostCustomSchema for Invalid {
        const PACKAGE: &'static str = "producer";
        const MODULE: &'static str = "handles";
        const NAME: &'static str = "bad";
        const PARAMETER_COUNT: usize = 0;
        type Constructors = HostCustomConstructorListEnd;
    }

    #[test]
    fn sharing_grants_survive_live_and_bodyless_registration() {
        let provider = || {
            HostProviderModule::<StatelessHostProfile>::new("producer", "handles")
                .unwrap()
                .with_shared_custom_type::<Shared>()
                .unwrap()
        };
        let (_, live, _, _) = HostProviderSet::from_providers([provider()])
            .unwrap()
            .into_registered();
        let (_, erased, _, _) = HostProviderSet::from_providers([provider()])
            .unwrap()
            .into_declarations()
            .into_registered();
        let declared = HostProviderModuleDeclaration::new("producer", "handles")
            .unwrap()
            .with_shared_custom_type::<Shared>()
            .unwrap();
        let (_, declarations, _, _) = HostDeclarations::from_providers([declared])
            .unwrap()
            .into_registered();
        let expected = HostCustomTypeSchema::of::<Shared>();
        assert!(expected.requires_shared_access());
        assert_eq!(expected, expected.clone());
        assert!(!HostCustomTypeSchema::of::<Invalid>().requires_shared_access());
        for mut providers in [live, erased, declarations] {
            let (package, module, functions, externals, shared) = providers.remove(0).into_parts();
            assert_eq!(package, "producer");
            assert_eq!(module, "handles");
            assert!(functions.is_empty());
            assert!(externals.is_empty());
            assert_eq!(shared, std::slice::from_ref(&expected));
        }
    }

    #[test]
    fn registration_rejects_wrong_owners_invalid_names_and_duplicates() {
        for (package, module) in [("other", "handles"), ("producer", "other")] {
            let error = HostProviderModule::<StatelessHostProfile>::new(package, module)
                .unwrap()
                .with_shared_custom_type::<Shared>()
                .err()
                .unwrap();
            let declaration_error = HostProviderModuleDeclaration::new(package, module)
                .unwrap()
                .with_shared_custom_type::<Shared>()
                .err()
                .unwrap();
            assert_eq!(declaration_error, error);
            assert_eq!(
                error,
                HostRegistrationError::SharedCustomTypeOwner {
                    package: package.into(),
                    module: module.into(),
                    custom_type: crate::plan::CustomTypeName::new(
                        "producer".into(),
                        "handles".into(),
                        "Handle".into()
                    ),
                }
            );
            assert_eq!(
                error.to_string(),
                format!(
                    "host module {package}::{module} cannot share custom type CustomTypeName {{ package: \"producer\", module: \"handles\", name: \"Handle\" }} owned by another module"
                )
            );
        }
        let owner = HostModuleIdentity::new("producer".into(), "handles".into()).unwrap();
        let mut types = RegisteredSharedCustomTypes::new();
        let invalid = types
            .register(&owner, HostCustomTypeSchema::of::<Invalid>())
            .unwrap_err();
        assert_eq!(
            invalid,
            HostRegistrationError::InvalidSharedCustomTypeName {
                module: "handles".into(),
                type_: "bad".into()
            }
        );
        assert_eq!(
            invalid.to_string(),
            "shared custom type name bad in module handles is invalid"
        );
        types
            .register(&owner, HostCustomTypeSchema::of::<Shared>())
            .unwrap();
        let duplicate = types
            .register(&owner, HostCustomTypeSchema::of::<Shared>())
            .unwrap_err();
        assert_eq!(
            duplicate,
            HostRegistrationError::DuplicateSharedCustomType {
                module: "handles".into(),
                type_: "Handle".into()
            }
        );
        assert_eq!(
            duplicate.to_string(),
            "custom type Handle was shared more than once in module handles"
        );
        assert_eq!(types.into_vec(), [HostCustomTypeSchema::of::<Shared>()]);
        let invalid = HostProviderModuleDeclaration::new("producer", "handles")
            .unwrap()
            .with_shared_custom_type::<Invalid>()
            .err()
            .unwrap();
        assert_eq!(
            invalid.to_string(),
            "shared custom type name bad in module handles is invalid"
        );
    }
}
