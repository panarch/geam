use crate::plan::ValueType;
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
    #[error("record update shape: {reason}")]
    RecordUpdateShape {
        reason: InvalidRecordUpdateShapeReason,
    },
    #[error("module reference {module}.{name}: {reason}")]
    ModuleReference {
        module: EcoString,
        name: EcoString,
        reason: InvalidModuleReferenceReason,
    },
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
    #[error("constructor field index is invalid")]
    FieldIndex,
    #[error("constructor field label is invalid")]
    FieldLabel,
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
    #[error("positional access")]
    PositionalAccess,
    #[error("prelude constructor")]
    PreludeConstructor,
    #[error("record constructor")]
    RecordConstructor,
    #[error("record access")]
    RecordAccess,
    #[error("record update")]
    RecordUpdate,
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum InvalidRecordUpdateShapeReason {
    #[error("record constructor expression")]
    ConstructorExpression,
    #[error("record constructor kind")]
    ConstructorKind,
    #[error("record constructor name: expected {expected}, got {actual}")]
    ConstructorName {
        expected: EcoString,
        actual: EcoString,
    },
    #[error("record constructor argument count")]
    ArgumentCount,
    #[error("record constructor argument label")]
    ArgumentLabel,
    #[error("record update base assignment")]
    BaseAssignment,
    #[error("record update implicit argument origin")]
    ImplicitArgumentOrigin,
    #[error("record update implicit field access")]
    ImplicitFieldAccess,
    #[error("record update implicit field target")]
    ImplicitFieldTarget,
    #[error("record update type")]
    Type,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum InvalidExpressionType {
    #[error("unsupported")]
    Unsupported,
    #[error("type parameter")]
    TypeParameter,
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
    #[error("external type")]
    External,
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

impl InvalidExpressionType {
    pub(crate) fn from_value_type(type_: ValueType) -> Self {
        match type_ {
            ValueType::Parameter(_) => Self::TypeParameter,
            ValueType::Int => Self::Int,
            ValueType::String => Self::String,
            ValueType::BitArray => Self::BitArray,
            ValueType::UtfCodepoint => Self::UtfCodepoint,
            ValueType::Custom(_) => Self::Custom,
            ValueType::External(_) => Self::External,
            ValueType::Float => Self::Float,
            ValueType::Bool => Self::Bool,
            ValueType::Nil => Self::Nil,
            ValueType::Tuple(_) => Self::Tuple,
            ValueType::List(_) => Self::List,
            ValueType::Function(_) => Self::Function,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::InvalidExpressionType;
    use crate::plan::{ExternalType, ExternalTypeName, ValueType};

    #[test]
    fn classifies_external_value_types() {
        let type_ = ExternalType::new(
            ExternalTypeName::new("application".into(), "main".into(), "Token".into()),
            Vec::new(),
        );

        assert_eq!(
            InvalidExpressionType::from_value_type(ValueType::External(type_)),
            InvalidExpressionType::External,
        );
    }
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum InvalidCallShapeReason {
    #[error("function value call arity mismatch")]
    FunctionCallArityMismatch,
    #[error("function value call argument type mismatch")]
    FunctionCallArgumentTypeMismatch,
    #[error("function value call return type mismatch")]
    FunctionCallReturnTypeMismatch,
    #[error("implicit call arguments")]
    ImplicitArguments,
    #[error("labelled call arguments")]
    LabelledArguments,
    #[error("local function call arity mismatch")]
    LocalFunctionCallArityMismatch,
    #[error("local function call return type does not match function table")]
    LocalFunctionCallReturnTypeMismatch,
    #[error("record constructor has extra arguments: expected {expected}, got {actual}")]
    RecordConstructorExtraArguments { expected: usize, actual: usize },
    #[error("record constructor has missing arguments: expected {expected}, got {actual}")]
    RecordConstructorMissingArguments { expected: usize, actual: usize },
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum InvalidModuleReferenceReason {
    #[error("module is not linked")]
    UnlinkedModule,
    #[error("function is missing from the linked registry")]
    MissingFunction,
    #[error("constant is missing from the linked registry")]
    MissingConstant,
    #[error("constant is not callable")]
    NonCallableConstant,
    #[error("function constructor module is {actual}")]
    FunctionModule { actual: EcoString },
    #[error("function constructor name is {actual}")]
    FunctionName { actual: EcoString },
    #[error("external function")]
    ExternalFunction,
    #[error("record constructor name is {actual}")]
    RecordConstructorName { actual: EcoString },
    #[error("record constructor result shape")]
    RecordConstructorResultShape,
    #[error("function value type")]
    FunctionType,
    #[error("function signature instantiation")]
    FunctionInstantiation,
    #[error("function reference result shape")]
    FunctionReferenceShape,
    #[error("constant signature instantiation")]
    ConstantInstantiation,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum InvalidCaseShapeReason {
    #[error("branch return type does not match case type")]
    BranchReturnTypeMismatch,
    #[error("empty clauses")]
    EmptyClauses,
    #[error("empty subjects")]
    EmptySubjects,
    #[error("compiled case clause index is out of bounds")]
    CompiledCaseClauseIndex,
    #[error("compiled case contains a reachable failure")]
    CompiledCaseFailure,
    #[error("compiled case guard index is out of bounds")]
    CompiledCaseGuardIndex,
    #[error("compiled case guard does not match its clause")]
    CompiledCaseGuard,
    #[error("compiled case subject count does not match case subjects")]
    CompiledCaseSubjectCountMismatch,
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
    #[error("invalid echo step")]
    EchoStep,
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
