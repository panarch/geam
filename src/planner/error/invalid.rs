use ecow::EcoString;
use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum InvalidTypedAstReason {
    #[error("function shape {name}: {reason}")]
    FunctionShape {
        name: EcoString,
        reason: InvalidFunctionShapeReason,
    },
    #[error("generated assignment")]
    GeneratedAssignment,
    #[error("use statement")]
    UseStatement,
    #[error("invalid pattern")]
    InvalidPattern,
    #[error("expression shape: {kind}")]
    ExpressionShape { kind: InvalidExpressionShapeKind },
    #[error("call shape: {reason}")]
    CallShape { reason: InvalidCallShapeReason },
    #[error("unknown local variable: {name}")]
    UnknownLocal { name: EcoString },
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum InvalidFunctionShapeReason {
    #[error("anonymous functions are not module functions")]
    Anonymous,
    #[error("empty function bodies are not supported")]
    EmptyBody,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum InvalidExpressionShapeKind {
    #[error("invalid")]
    Invalid,
    #[error("module select")]
    ModuleSelect,
    #[error("module constant")]
    ModuleConstant,
    #[error("positional access")]
    PositionalAccess,
    #[error("prelude constructor")]
    PreludeConstructor,
    #[error("record access")]
    RecordAccess,
    #[error("record constructor")]
    RecordConstructor,
    #[error("record update")]
    RecordUpdate,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum InvalidCallShapeReason {
    #[error("implicit call arguments")]
    ImplicitArguments,
    #[error("labelled call arguments")]
    LabelledArguments,
    #[error("local function call arity mismatch")]
    LocalFunctionCallArityMismatch,
    #[error("calling module constants is not supported")]
    ModuleConstant,
    #[error("non-current module function")]
    NonCurrentModuleFunction,
    #[error("current-module function is missing from function table")]
    MissingCurrentModuleFunction,
    #[error("calling record constructors is not supported")]
    RecordConstructor,
}
