use thiserror::Error;

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
    #[error("function return type is not supported")]
    UnsupportedReturnType,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum UnsupportedArgumentReason {
    #[error("labelled arguments are not supported")]
    Labelled,
    #[error("argument type is not supported")]
    UnsupportedType,
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
    #[error("tuple")]
    Tuple,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum UnsupportedExpressionKind {
    #[error("bit array")]
    BitArray,
    #[error("echo")]
    Echo,
    #[error("function capture literal")]
    FunctionCaptureLiteral,
    #[error("list")]
    List,
    #[error("panic")]
    Panic,
    #[error("todo")]
    Todo,
    #[error("tuple")]
    Tuple,
    #[error("tuple index")]
    TupleIndex,
    #[error("function literal type is not supported")]
    UnsupportedFunctionLiteralType,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum UnsupportedBinOpKind {
    #[error("equal function")]
    EqFunction,
    #[error("not equal function")]
    NotEqFunction,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum UnsupportedCaseReason {
    #[error("alternative patterns are not supported")]
    AlternativePatterns,
    #[error("assign patterns are not supported")]
    AssignPattern,
    #[error("guards are not supported")]
    Guard,
    #[error("multiple subjects are not supported")]
    MultipleSubjects,
    #[error("case subject type is not supported")]
    UnsupportedSubjectType,
    #[error("string prefix patterns are not supported")]
    StringPrefixPattern,
    #[error("variable patterns are not supported")]
    VariablePattern,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum UnsupportedPipelineReason {
    #[error("echo")]
    Echo,
    #[error("function value pipeline calls are not supported")]
    FunctionValueCall,
}
