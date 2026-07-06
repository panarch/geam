use ecow::EcoString;
use thiserror::Error;

mod invalid;
mod unsupported;

pub use invalid::{
    InvalidCallShapeReason, InvalidCaseShapeReason, InvalidExpressionShapeKind,
    InvalidExpressionType, InvalidFunctionShapeReason, InvalidPipelineShapeReason,
    InvalidTypedAstReason, InvalidUseShapeReason,
};
pub use unsupported::{
    UnsupportedArgumentReason, UnsupportedBinOpKind, UnsupportedCaseReason,
    UnsupportedExpressionKind, UnsupportedFunctionReason, UnsupportedPatternKind,
    UnsupportedPipelineReason, UnsupportedStatementKind, UnsupportedTopLevelKind,
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

    #[error("unsupported statement: {kind}")]
    UnsupportedStatement { kind: UnsupportedStatementKind },

    #[error("unsupported pattern: {kind}")]
    UnsupportedPattern { kind: UnsupportedPatternKind },

    #[error("unsupported expression: {kind}")]
    UnsupportedExpression { kind: UnsupportedExpressionKind },

    #[error("unsupported binary operator: {operator}")]
    UnsupportedBinOp { operator: UnsupportedBinOpKind },

    #[error("unsupported case: {reason}")]
    UnsupportedCase { reason: UnsupportedCaseReason },

    #[error("unsupported pipeline: {reason}")]
    UnsupportedPipeline { reason: UnsupportedPipelineReason },

    #[error("invalid Gleam typed AST: {reason}")]
    InvalidTypedAst { reason: InvalidTypedAstReason },
}
