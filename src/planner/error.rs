use ecow::EcoString;
use thiserror::Error;

mod invalid;

pub use invalid::{
    InvalidCallShapeReason, InvalidExpressionShapeKind, InvalidFunctionShapeReason,
    InvalidTypedAstReason,
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

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum UnsupportedTopLevelKind {
    #[error("import")]
    Import,
    #[error("constant")]
    Constant,
    #[error("custom type")]
    CustomType,
    #[error("type alias")]
    TypeAlias,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum UnsupportedFunctionReason {
    #[error("external functions are not executable by the Geam runtime")]
    External,
    #[error("main function is required")]
    MissingMain,
    #[error("main must not take arguments")]
    MainWithArguments,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum UnsupportedArgumentReason {
    #[error("discard arguments are not supported")]
    Discard,
    #[error("labelled arguments are not supported")]
    Labelled,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum UnsupportedStatementKind {
    #[error("assert")]
    Assert,
    #[error("assert as final statement")]
    AssertAsFinalStatement,
    #[error("assignment as final statement")]
    AssignmentAsFinalStatement,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum UnsupportedAssignmentKind {
    #[error("let assert")]
    LetAssert,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum UnsupportedPatternKind {
    #[error("assign")]
    Assign,
    #[error("discard")]
    Discard,
    #[error("tuple")]
    Tuple,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum UnsupportedExpressionKind {
    #[error("anonymous function")]
    AnonymousFunction,
    #[error("bit array")]
    BitArray,
    #[error("block")]
    Block,
    #[error("case")]
    Case,
    #[error("echo")]
    Echo,
    #[error("float")]
    Float,
    #[error("function reference")]
    FunctionReference,
    #[error("list")]
    List,
    #[error("panic")]
    Panic,
    #[error("pipeline")]
    Pipeline,
    #[error("todo")]
    Todo,
    #[error("tuple")]
    Tuple,
    #[error("tuple index")]
    TupleIndex,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum UnsupportedBinOpKind {
    #[error("add float")]
    AddFloat,
    #[error("and")]
    And,
    #[error("div float")]
    DivFloat,
    #[error("gt float")]
    GtFloat,
    #[error("gte float")]
    GtEqFloat,
    #[error("lt float")]
    LtFloat,
    #[error("lte float")]
    LtEqFloat,
    #[error("mult float")]
    MultFloat,
    #[error("or")]
    Or,
    #[error("sub float")]
    SubFloat,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum UnsupportedCallReason {
    #[error("calling local function values is not supported")]
    LocalFunctionValue,
    #[error("only direct local function calls are supported")]
    NonDirectLocalFunction,
}
