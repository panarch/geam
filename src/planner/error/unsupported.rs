use thiserror::Error;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum UnsupportedTopLevelKind {
    #[error("import")]
    Import,
    #[error("backend external custom type")]
    ExternalCustomType,
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
    #[error("constructor")]
    Constructor,
    #[error("string prefix")]
    StringPrefix,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum UnsupportedExpressionKind {
    #[error("echo")]
    Echo,
    #[error("list element type is not supported")]
    UnsupportedListElementType,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum UnsupportedBitArraySegmentReason {
    #[error("native-endian segments are not supported")]
    NativeEndianness,
    #[error("bit array segment size exceeds the supported host range")]
    SizeOutOfRange,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum UnsupportedCaseReason {
    #[error("case subject type is not supported")]
    UnsupportedSubjectType,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum UnsupportedPipelineReason {
    #[error("echo")]
    Echo,
}
