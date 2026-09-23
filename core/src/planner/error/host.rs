use crate::host::{HostCustomTypeSchema, HostExternalTypeSchema};
use crate::plan::{CustomTypeName, ExternalTypeName, FunctionType, TypeScheme};
use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ExternalTypeProviderLinkReason {
    #[error("source module is not linked")]
    MissingModule,
    #[error("external type registration is missing")]
    MissingRegistration,
    #[error("type declaration is missing")]
    MissingDeclaration,
    #[error("external storage type has source constructors")]
    ConstructorBackedType,
    #[error("external type identity mismatch: expected {expected:?}, got {actual:?}")]
    IdentityMismatch {
        expected: ExternalTypeName,
        actual: ExternalTypeName,
    },
    #[error("external type expects {expected} type arguments, but host ABI declares {actual}")]
    ParameterCount { expected: usize, actual: usize },
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum SharedCustomTypeProviderLinkReason {
    #[error("source module is not linked")]
    MissingModule,
    #[error("custom type declaration is missing")]
    MissingDeclaration,
    #[error("shared custom schema mismatch: expected {expected:?}, got {actual:?}")]
    SchemaMismatch {
        expected: HostCustomTypeSchema,
        actual: HostCustomTypeSchema,
    },
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum HostProviderLinkReason {
    #[error(
        "native callable construction cycle through {package}::{module}.{function} expands type parameters without bound"
    )]
    ExpandingCallableCycle {
        package: ecow::EcoString,
        module: ecow::EcoString,
        function: ecow::EcoString,
    },

    #[error("native callable {package}::{module}.{function} is not registered")]
    MissingCallable {
        package: ecow::EcoString,
        module: ecow::EcoString,
        function: ecow::EcoString,
    },
    #[error(
        "native callable {package}::{module}.{function} has an incompatible argument, capture, return or completion contract"
    )]
    CallableContractMismatch {
        package: ecow::EcoString,
        module: ecow::EcoString,
        function: ecow::EcoString,
    },
    #[error("source module is not linked")]
    MissingModule,
    #[error("function declaration is missing")]
    MissingDeclaration,
    #[error("function is not external")]
    NonExternalFunction,
    #[error(
        "function scheme mismatch: expected {expected_scheme:?} {expected_type:?}, got {actual_scheme:?} {actual_type:?}"
    )]
    SchemeMismatch {
        expected_scheme: TypeScheme,
        expected_type: FunctionType,
        actual_scheme: TypeScheme,
        actual_type: FunctionType,
    },
    #[error("custom type {custom_type:?} requires a sharing registration from its source owner")]
    MissingSharedCustomType { custom_type: CustomTypeName },
    #[error("custom type {custom_type:?} is missing")]
    MissingCustomType { custom_type: CustomTypeName },
    #[error("custom type {custom_type:?} is not visible to the host function")]
    CustomTypeVisibility { custom_type: CustomTypeName },
    #[error(
        "custom type {custom_type:?} expects {expected} type arguments, but host ABI applies {actual}"
    )]
    CustomTypeArgumentCount {
        custom_type: CustomTypeName,
        expected: usize,
        actual: usize,
    },
    #[error("custom schema mismatch: expected {expected:?}, got {actual:?}")]
    CustomSchemaMismatch {
        expected: HostCustomTypeSchema,
        actual: HostCustomTypeSchema,
    },
    #[error("external type {external_type:?} is missing")]
    MissingExternalType { external_type: ExternalTypeName },
    #[error(
        "external type {external_type:?} expects {expected} type arguments, but host ABI applies {actual}"
    )]
    ExternalTypeArgumentCount {
        external_type: ExternalTypeName,
        expected: usize,
        actual: usize,
    },
    #[error("external schema mismatch: expected {expected:?}, got {actual:?}")]
    ExternalSchemaMismatch {
        expected: HostExternalTypeSchema,
        actual: HostExternalTypeSchema,
    },
}

#[cfg(test)]
mod tests {
    use super::HostProviderLinkReason;

    #[test]
    fn sharing_schema_diagnostic_reports_both_exact_contracts() {
        let reason = super::SharedCustomTypeProviderLinkReason::SchemaMismatch {
            expected: crate::HostCustomTypeSchema::new("app", "handles", "Handle", 0, [])
                .with_shared_access(true),
            actual: crate::HostCustomTypeSchema::new("app", "handles", "Handle", 1, [])
                .with_shared_access(true),
        };
        assert_eq!(
            reason.to_string(),
            concat!(
                "shared custom schema mismatch: expected HostCustomTypeSchema { package: \"app\", module: \"handles\", name: \"Handle\", parameter_count: 0, constructors: [], shared: true }, ",
                "got HostCustomTypeSchema { package: \"app\", module: \"handles\", name: \"Handle\", parameter_count: 1, constructors: [], shared: true }",
            )
        );
        assert_eq!(reason.clone(), reason);
    }

    #[test]
    fn missing_custom_sharing_names_the_source_owner() {
        let reason = HostProviderLinkReason::MissingSharedCustomType {
            custom_type: crate::plan::CustomTypeName::new(
                "producer".into(),
                "handles".into(),
                "Handle".into(),
            ),
        };
        assert_eq!(
            reason.to_string(),
            "custom type CustomTypeName { package: \"producer\", module: \"handles\", name: \"Handle\" } requires a sharing registration from its source owner"
        );
        assert_eq!(reason.clone(), reason);
    }

    #[test]
    fn native_callable_diagnostics_identify_the_exact_declaration_and_contract() {
        for (reason, expected) in [
            (
                HostProviderLinkReason::MissingCallable {
                    package: "application".into(),
                    module: "pricing".into(),
                    function: "adjust".into(),
                },
                "native callable application::pricing.adjust is not registered",
            ),
            (
                HostProviderLinkReason::CallableContractMismatch {
                    package: "application".into(),
                    module: "pricing".into(),
                    function: "adjust".into(),
                },
                "native callable application::pricing.adjust has an incompatible argument, capture, return or completion contract",
            ),
            (
                HostProviderLinkReason::ExpandingCallableCycle {
                    package: "application".into(),
                    module: "pricing".into(),
                    function: "adjust".into(),
                },
                "native callable construction cycle through application::pricing.adjust expands type parameters without bound",
            ),
        ] {
            assert_eq!(reason.to_string(), expected);
            assert_eq!(reason.clone(), reason);
        }
    }
}
