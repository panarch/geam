use crate::plan::{FunctionType, TypeScheme};
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
}
