use thiserror::Error;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum UnsupportedTopLevelKind {
    #[error("import")]
    Import,
    #[error("custom type")]
    CustomType,
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
    #[error("argument type is not supported")]
    UnsupportedType,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum UnsupportedPatternKind {
    #[error("list")]
    List,
    #[error("literal")]
    Literal,
    #[error("bit array")]
    BitArray,
    #[error("constructor")]
    Constructor,
    #[error("string prefix")]
    StringPrefix,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum UnsupportedExpressionKind {
    #[error("bit array")]
    BitArray,
    #[error("echo")]
    Echo,
    #[error("list element type is not supported")]
    UnsupportedListElementType,
    #[error("record constructor")]
    RecordConstructor,
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
    #[error("case subject type is not supported")]
    UnsupportedSubjectType,
    #[error("list patterns are not supported")]
    ListPattern,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum UnsupportedPipelineReason {
    #[error("echo")]
    Echo,
}
