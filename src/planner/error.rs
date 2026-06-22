use ecow::EcoString;
use thiserror::Error;

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

    #[error("unknown local variable: {name}")]
    UnknownLocal { name: EcoString },
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
    #[error("anonymous functions are not module functions")]
    Anonymous,
    #[error("empty function bodies are not supported")]
    EmptyBody,
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
    #[error("use")]
    Use,
    #[error("use as final statement")]
    UseAsFinalStatement,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum UnsupportedAssignmentKind {
    #[error("generated")]
    Generated,
    #[error("let assert")]
    LetAssert,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum UnsupportedPatternKind {
    #[error("assign")]
    Assign,
    #[error("bit array")]
    BitArray,
    #[error("bit array size")]
    BitArraySize,
    #[error("constructor")]
    Constructor,
    #[error("discard")]
    Discard,
    #[error("float")]
    Float,
    #[error("int")]
    Int,
    #[error("invalid")]
    Invalid,
    #[error("list")]
    List,
    #[error("string")]
    String,
    #[error("string prefix")]
    StringPrefix,
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
    #[error("invalid")]
    Invalid,
    #[error("list")]
    List,
    #[error("module constant")]
    ModuleConstant,
    #[error("module select")]
    ModuleSelect,
    #[error("panic")]
    Panic,
    #[error("pipeline")]
    Pipeline,
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
    #[error("calling module constants is not supported")]
    ModuleConstant,
    #[error("calling record constructors is not supported")]
    RecordConstructor,
    #[error("implicit call arguments are not supported")]
    ImplicitArguments,
    #[error("labelled call arguments are not supported")]
    LabelledArguments,
    #[error("local function call arity mismatch")]
    LocalFunctionCallArityMismatch,
    #[error("only current-module functions are supported")]
    NonCurrentModuleFunction,
    #[error("only direct local function calls are supported")]
    NonDirectLocalFunction,
}
