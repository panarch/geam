use crate::host::HostCustomTypeSchema;
use crate::plan::{CustomTypeName, FunctionType, TypeScheme};
use thiserror::Error;

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
}
