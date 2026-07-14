use ecow::EcoString;
use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum InvalidTypedAstReason {
    #[error("custom type {name}: {reason}")]
    CustomType {
        name: EcoString,
        reason: InvalidCustomTypeReason,
    },
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
    #[error("pipeline shape: {reason}")]
    PipelineShape { reason: InvalidPipelineShapeReason },
    #[error("use shape: {reason}")]
    UseShape { reason: InvalidUseShapeReason },
    #[error("unknown local variable: {name}")]
    UnknownLocal { name: EcoString },
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum InvalidCustomTypeReason {
    #[error("type argument count does not match definition")]
    TypeArgumentCount,
    #[error("constructor index is out of range")]
    ConstructorIndex,
    #[error("constructor name does not match definition")]
    ConstructorName,
    #[error("constructor module does not match custom type")]
    ConstructorModule,
    #[error("constructor field count does not match definition")]
    ConstructorArity,
    #[error("constructor type is not a custom type")]
    ConstructorType,
    #[error("constructor field type is invalid")]
    FieldType,
    #[error("type parameter is not generic")]
    ParameterType,
    #[error("custom type definition is missing")]
    UnknownDefinition,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum InvalidFunctionShapeReason {
    #[error("function argument types do not match signature")]
    ArgumentTypeMismatch,
    #[error("anonymous functions are not module functions")]
    Anonymous,
    #[error("empty function bodies are not supported")]
    EmptyBody,
    #[error("labelled function argument")]
    LabelledArgument,
    #[error("function return type does not match body")]
    ReturnTypeMismatch,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum InvalidExpressionShapeKind {
    #[error("bit array segment option")]
    BitArraySegmentOption,
    #[error("invalid")]
    Invalid,
    #[error("function capture literal")]
    FunctionCaptureLiteral,
    #[error("module select")]
    ModuleSelect,
    #[error("positional access")]
    PositionalAccess,
    #[error("prelude constructor")]
    PreludeConstructor,
    #[error("record constructor")]
    RecordConstructor,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum InvalidExpressionType {
    #[error("unsupported")]
    Unsupported,
    #[error("Int")]
    Int,
    #[error("String")]
    String,
    #[error("BitArray")]
    BitArray,
    #[error("UtfCodepoint")]
    UtfCodepoint,
    #[error("custom type")]
    Custom,
    #[error("Float")]
    Float,
    #[error("Bool")]
    Bool,
    #[error("Nil")]
    Nil,
    #[error("Tuple")]
    Tuple,
    #[error("List")]
    List,
    #[error("Function")]
    Function,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum InvalidCallShapeReason {
    #[error("function value call arity mismatch")]
    FunctionCallArityMismatch,
    #[error("function value call argument type mismatch")]
    FunctionCallArgumentTypeMismatch,
    #[error("function value call return type mismatch")]
    FunctionCallReturnTypeMismatch,
    #[error("function value call return type is not supported")]
    FunctionCallUnsupportedReturnType,
    #[error("implicit call arguments")]
    ImplicitArguments,
    #[error("labelled call arguments")]
    LabelledArguments,
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
    #[error("missing fallback pattern")]
    MissingFallbackPattern,
    #[error("missing true pattern")]
    MissingTruePattern,
    #[error("pattern type mismatch")]
    PatternTypeMismatch,
    #[error("pattern subject count mismatch")]
    PatternSubjectCountMismatch,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum InvalidPipelineShapeReason {
    #[error("invalid hole capture")]
    InvalidHoleCapture,
    #[error("missing pipe argument")]
    MissingPipeArgument,
    #[error("multiple pipe arguments")]
    MultiplePipeArguments,
    #[error("non-call pipeline step")]
    NonCallStep,
    #[error("unsupported implicit argument")]
    UnsupportedImplicitArgument,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum InvalidUseShapeReason {
    #[error("callback literal kind is not use")]
    CallbackLiteralKindNotUse,
    #[error("callback is not the last argument")]
    CallbackNotLast,
    #[error("callback argument is not a function literal")]
    CallbackNotFunctionLiteral,
    #[error("invalid generated assignment")]
    InvalidGeneratedAssignment,
    #[error("missing callback")]
    MissingCallback,
    #[error("multiple callbacks")]
    MultipleCallbacks,
    #[error("non-call use right hand side")]
    NonCallRhs,
    #[error("unexpected variable use assignment")]
    UnexpectedVariableAssignment,
    #[error("unsupported implicit argument")]
    UnsupportedImplicitArgument,
}
