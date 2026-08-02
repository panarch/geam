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
pub enum HostProviderLinkReason {
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
