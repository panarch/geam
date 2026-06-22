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
    #[error("expression type: expected {expected}, got {actual}")]
    ExpressionType {
        expected: InvalidExpressionType,
        actual: InvalidExpressionType,
    },
    #[error("call shape: {reason}")]
    CallShape { reason: InvalidCallShapeReason },
    #[error("case shape: {reason}")]
    CaseShape { reason: InvalidCaseShapeReason },
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
pub enum InvalidExpressionType {
    #[error("Int")]
    Int,
    #[error("String")]
    String,
    #[error("Bool")]
    Bool,
    #[error("Nil")]
    Nil,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum InvalidCallShapeReason {
    #[error("implicit call arguments")]
    ImplicitArguments,
    #[error("labelled call arguments")]
    LabelledArguments,
    #[error("local function value")]
    LocalFunctionValue,
    #[error("local function call arity mismatch")]
    LocalFunctionCallArityMismatch,
    #[error("local function call return type is not supported")]
    LocalFunctionCallUnsupportedReturnType,
    #[error("local function call return type does not match function table")]
    LocalFunctionCallReturnTypeMismatch,
    #[error("calling module constants is not supported")]
    ModuleConstant,
    #[error("non-current module function")]
    NonCurrentModuleFunction,
    #[error("current-module function is missing from function table")]
    MissingCurrentModuleFunction,
    #[error("calling record constructors is not supported")]
    RecordConstructor,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum InvalidCaseShapeReason {
    #[error("branch return type does not match case type")]
    BranchReturnTypeMismatch,
    #[error("empty clauses")]
    EmptyClauses,
    #[error("empty subjects")]
    EmptySubjects,
    #[error("invalid pattern")]
    InvalidPattern,
    #[error("missing false pattern")]
    MissingFalsePattern,
    #[error("missing true pattern")]
    MissingTruePattern,
    #[error("pattern type mismatch")]
    PatternTypeMismatch,
    #[error("pattern subject count mismatch")]
    PatternSubjectCountMismatch,
}
