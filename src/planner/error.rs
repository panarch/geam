use ecow::EcoString;
use thiserror::Error;

mod invalid;
mod unsupported;

pub use invalid::{
    InvalidCallShapeReason, InvalidExpressionShapeKind, InvalidExpressionType,
    InvalidFunctionShapeReason, InvalidTypedAstReason,
};
pub use unsupported::{
    UnsupportedArgumentReason, UnsupportedAssignmentKind, UnsupportedBinOpKind,
    UnsupportedCallReason, UnsupportedExpressionKind, UnsupportedFunctionReason,
    UnsupportedPatternKind, UnsupportedStatementKind, UnsupportedTopLevelKind,
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

    #[error("unsupported assignment: {kind}")]
    UnsupportedAssignment { kind: UnsupportedAssignmentKind },

    #[error("unsupported pattern: {kind}")]
    UnsupportedPattern { kind: UnsupportedPatternKind },

    #[error("unsupported expression: {kind}")]
    UnsupportedExpression { kind: UnsupportedExpressionKind },

    #[error("unsupported binary operator: {operator}")]
    UnsupportedBinOp { operator: UnsupportedBinOpKind },

    #[error("unsupported call: {reason}")]
    UnsupportedCall { reason: UnsupportedCallReason },

    #[error("invalid Gleam typed AST: {reason}")]
    InvalidTypedAst { reason: InvalidTypedAstReason },
}
