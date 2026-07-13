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
pub enum UnsupportedBitArraySegmentReason {
    #[error("UtfCodepoint segments are not supported")]
    UtfCodepoint,
    #[error("native-endian segments are not supported")]
    NativeEndianness,
    #[error("dynamic segment sizes are not supported")]
    DynamicSize,
    #[error("sized bits segments are not supported")]
    SizedBits,
    #[error("16-bit float segments are not supported")]
    Float16,
    #[error("bit array segment size exceeds the supported host range")]
    SizeOutOfRange,
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
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum UnsupportedPipelineReason {
    #[error("echo")]
    Echo,
}
