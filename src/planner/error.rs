use ecow::EcoString;
use thiserror::Error;

mod invalid;
mod unsupported;

pub use invalid::{
    InvalidCallShapeReason, InvalidCaseShapeReason, InvalidCustomTypeReason,
    InvalidExpressionShapeKind, InvalidExpressionType, InvalidFunctionShapeReason,
    InvalidPipelineShapeReason, InvalidRecordUpdateShapeReason, InvalidTypedAstReason,
    InvalidUseShapeReason,
};
pub use unsupported::{
    UnsupportedArgumentReason, UnsupportedBitArraySegmentReason, UnsupportedCaseReason,
    UnsupportedExpressionKind, UnsupportedFunctionReason, UnsupportedPatternKind,
    UnsupportedPipelineReason, UnsupportedTopLevelKind,
};

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum PlanError {
    #[error("unsupported top-level definition: {kind}")]
    UnsupportedTopLevel { kind: UnsupportedTopLevelKind },

    #[error("unsupported function {name}: {reason}")]
    UnsupportedFunction {
        name: EcoString,
        reason: UnsupportedFunctionReason,
    },

    #[error("unsupported argument in function {function}: {reason}")]
    UnsupportedArgument {
        function: EcoString,
        reason: UnsupportedArgumentReason,
    },

    #[error("unsupported pattern: {kind}")]
    UnsupportedPattern { kind: UnsupportedPatternKind },

    #[error("unsupported expression: {kind}")]
    UnsupportedExpression { kind: UnsupportedExpressionKind },

    #[error("unsupported bit array segment: {reason}")]
    UnsupportedBitArraySegment {
        reason: UnsupportedBitArraySegmentReason,
    },

    #[error("unsupported case: {reason}")]
    UnsupportedCase { reason: UnsupportedCaseReason },

    #[error("unsupported pipeline: {reason}")]
    UnsupportedPipeline { reason: UnsupportedPipelineReason },

    #[error("invalid Gleam typed AST: {reason}")]
    InvalidTypedAst { reason: InvalidTypedAstReason },
}
