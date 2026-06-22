use ecow::EcoString;
use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum PlanError {
    #[error("unsupported top-level definition: {kind}")]
    UnsupportedTopLevel { kind: &'static str },

    #[error("unsupported function {name}: {reason}")]
    UnsupportedFunction {
        name: EcoString,
        reason: &'static str,
    },

    #[error("unsupported argument in function {function}: {reason}")]
    UnsupportedArgument {
        function: EcoString,
        reason: &'static str,
    },

    #[error("unsupported statement: {kind}")]
    UnsupportedStatement { kind: &'static str },

    #[error("unsupported assignment: {kind}")]
    UnsupportedAssignment { kind: &'static str },

    #[error("unsupported pattern: {kind}")]
    UnsupportedPattern { kind: &'static str },

    #[error("unsupported expression: {kind}")]
    UnsupportedExpression { kind: &'static str },

    #[error("unsupported binary operator: {operator}")]
    UnsupportedBinOp { operator: &'static str },

    #[error("unsupported call: {reason}")]
    UnsupportedCall { reason: &'static str },

    #[error("unknown local variable: {name}")]
    UnknownLocal { name: EcoString },
}
