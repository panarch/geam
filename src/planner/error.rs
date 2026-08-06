use ecow::EcoString;
use thiserror::Error;

mod host;
mod invalid;
mod unsupported;

pub use host::{ExternalTypeProviderLinkReason, HostProviderLinkReason};
pub use invalid::{
    InvalidCallShapeReason, InvalidCaseShapeReason, InvalidCustomTypeReason,
    InvalidExpressionShapeKind, InvalidExpressionType, InvalidFunctionShapeReason,
    InvalidModuleReferenceReason, InvalidPipelineShapeReason, InvalidRecordUpdateShapeReason,
    InvalidTypedAstReason, InvalidUseShapeReason,
};
pub use unsupported::{
    UnsupportedBitArraySegmentReason, UnsupportedFunctionReason, UnsupportedPatternKind,
    UnsupportedTopLevelKind,
};

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum PlanError {
    #[error("external type provider {package}::{module}.{type_}: {reason}")]
    ExternalTypeProviderLink {
        package: EcoString,
        module: EcoString,
        type_: EcoString,
        reason: Box<ExternalTypeProviderLinkReason>,
    },

    #[error("host provider {package}::{module}.{function}: {reason}")]
    HostProviderLink {
        package: EcoString,
        module: EcoString,
        function: EcoString,
        reason: Box<HostProviderLinkReason>,
    },

    #[error("external function {package}::{module}.{function} has no host provider or Gleam body")]
    MissingHostProvider {
        package: EcoString,
        module: EcoString,
        function: EcoString,
    },

    #[error("unsupported top-level definition: {kind}")]
    UnsupportedTopLevel { kind: UnsupportedTopLevelKind },

    #[error("unsupported function {name}: {reason}")]
    UnsupportedFunction {
        name: EcoString,
        reason: UnsupportedFunctionReason,
    },

    #[error("unsupported pattern: {kind}")]
    UnsupportedPattern { kind: UnsupportedPatternKind },

    #[error("unsupported bit array segment: {reason}")]
    UnsupportedBitArraySegment {
        reason: UnsupportedBitArraySegmentReason,
    },

    #[error("invalid Gleam typed AST: {reason}")]
    InvalidTypedAst { reason: InvalidTypedAstReason },
}
