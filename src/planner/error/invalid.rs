use crate::plan::ValueType;
use ecow::EcoString;
use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum InvalidTypedAstReason {
    #[error("custom type {package}:{module}.{name}: {reason}")]
    CustomType {
        package: EcoString,
        module: EcoString,
        name: EcoString,
        reason: Box<InvalidCustomTypeReason>,
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
    #[error("invalid expression node")]
    InvalidExpressionNode,
    #[error("expression shape refinement: expected {expected:?}, got {actual:?}")]
    ExpressionShapeRefinement {
        expected: ValueType,
        actual: ValueType,
    },
    #[error("pattern shape: {reason}")]
    PatternShape { reason: InvalidPatternShapeReason },
    #[error("expression shape: {kind}")]
    ExpressionShape { kind: InvalidExpressionShapeKind },
    #[error("expression type: expected {expected}, got {actual}")]
    ExpressionType {
        expected: InvalidExpressionType,
        actual: InvalidExpressionType,
    },
    #[error("expression value type: expected {expected:?}, got {actual:?}")]
    ExpressionValueTypeMismatch {
        expected: ValueType,
        actual: ValueType,
    },
    #[error("expression type has no supported runtime family; expected {expected}")]
    UnsupportedExpressionType { expected: InvalidExpressionType },
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

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum InvalidPatternShapeReason {
    #[error("pattern type: expected {expected:?}, got {actual:?}")]
    TypeMismatch {
        expected: ValueType,
        actual: ValueType,
    },
    #[error("pattern annotation has no supported runtime type")]
    UnsupportedType,
    #[error("{actual} pattern cannot represent {expected:?}")]
    KindMismatch {
        expected: ValueType,
        actual: PatternKind,
    },
    #[error("tuple pattern arity: expected {expected}, got {actual}")]
    TupleArity { expected: usize, actual: usize },
    #[error("list tail must bind or discard, got {actual}")]
    ListTailKind { actual: PatternKind },
    #[error("{actual} pattern cannot be used as a total binding")]
    BindingKind { actual: PatternKind },
    #[error("binding aliases cannot contain another alias")]
    NestedBindingAlias,
    #[error("{actual} binding cannot represent {expected:?}")]
    BindingShape {
        expected: ValueType,
        actual: PatternKind,
    },
    #[error("binding shapes for {type_:?} are incompatible")]
    BindingShapeConflict { type_: ValueType },
    #[error("binding constructor {expected} does not match {actual:?}")]
    BindingConstructorRefinement {
        expected: usize,
        actual: Option<usize>,
    },
    #[error("list binding must not contain elements, got {actual}")]
    ListBindingElements { actual: usize },
    #[error("list binding must contain a tail")]
    ListBindingTailMissing,
    #[error("constructor binding is refutable across {constructors} constructors")]
    RefutableBindingConstructor { constructors: usize },
    #[error("bit-array binding segment count: expected 1, got {actual}")]
    BitArrayBindingSegmentCount { actual: usize },
    #[error("bit-array binding segment must not have a size")]
    BitArrayBindingSegmentSize,
    #[error("bit-array binding segment must use the bits option")]
    BitArrayBindingSegmentOptions,
    #[error("unsized bit-array segment {index} is not the final segment of {count}")]
    BitArrayUnsizedSegment { index: usize, count: usize },
    #[error("bit-array segment options: {reason}")]
    BitArraySegmentOptions {
        reason: InvalidBitArraySegmentOptionsReason,
    },
    #[error("bit-array size must use a size node, got {actual}")]
    BitArraySizePattern { actual: PatternKind },
    #[error("bit-array size variable {name} has no constructor metadata")]
    BitArraySizeUnresolved { name: EcoString },
    #[error("bit-array size variable {name} has an unsupported source")]
    BitArraySizeSource { name: EcoString },
    #[error("bit-array size constant is not an integer expression")]
    BitArraySizeConstant,
    #[error("{actual:?} cannot be used as a refutable assertion subject")]
    AssertSubject { actual: ValueType },
    #[error("invalid pattern node")]
    InvalidNode,
    #[error("bit-array size node used as a pattern")]
    BitArraySizeNode,
    #[error("constructor metadata is unresolved")]
    UnresolvedConstructor,
    #[error("constructor module: expected {expected}, got {actual}")]
    ConstructorModule {
        expected: EcoString,
        actual: EcoString,
    },
    #[error("constructor name: expected {expected}, got {actual}")]
    ConstructorName {
        expected: EcoString,
        actual: EcoString,
    },
    #[error("constructor index: expected {expected}, got {actual}")]
    ConstructorIndex { expected: usize, actual: usize },
    #[error("constructor arity: expected {expected}, got {actual}")]
    ConstructorArity { expected: usize, actual: usize },
    #[error("constructor pattern cannot represent {type_:?}")]
    ConstructorType { type_: ValueType },
    #[error("constructor spread is invalid for {type_:?}")]
    ConstructorSpread { type_: ValueType },
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum InvalidBitArraySegmentOptionsReason {
    #[error("multiple segment kinds")]
    MultipleKinds,
    #[error("multiple signedness options")]
    MultipleSignedness,
    #[error("multiple endianness options")]
    MultipleEndianness,
    #[error("multiple size options")]
    MultipleSizes,
    #[error("multiple unit options")]
    MultipleUnits,
    #[error("unit option without a size")]
    UnitWithoutSize,
    #[error("zero unit")]
    ZeroUnit,
    #[error("options incompatible with the segment kind")]
    Incompatible,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum PatternKind {
    #[error("integer")]
    Int,
    #[error("float")]
    Float,
    #[error("string")]
    String,
    #[error("variable")]
    Variable,
    #[error("bit-array size")]
    BitArraySize,
    #[error("alias")]
    Assign,
    #[error("discard")]
    Discard,
    #[error("list")]
    List,
    #[error("constructor")]
    Constructor,
    #[error("tuple")]
    Tuple,
    #[error("bit array")]
    BitArray,
    #[error("string prefix")]
    StringPrefix,
    #[error("invalid")]
    Invalid,
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum InvalidCustomTypeReason {
    #[error("type parameter {index} is not a generic parameter")]
    DefinitionParameter { index: usize },
    #[error("field {field} of constructor {constructor} has no supported type template")]
    DefinitionField {
        constructor: EcoString,
        field: usize,
    },
    #[error("template parameter {index} is outside {available} type arguments")]
    TemplateParameterIndex { index: usize, available: usize },
    #[error("template shape expected {expected:?}, got {actual:?}")]
    TemplateShapeMismatch {
        expected: ValueType,
        actual: ValueType,
    },
    #[error("parameter {parameter} inferred incompatible shapes {previous:?} and {actual:?}")]
    ConflictingParameterShape {
        parameter: usize,
        previous: ValueType,
        actual: ValueType,
    },
    #[error("custom type definition is missing")]
    MissingDefinition,
    #[error("type argument count: expected {expected}, got {actual}")]
    TypeArgumentCount { expected: usize, actual: usize },
    #[error("constructor index {index} is outside {available} constructors")]
    ConstructorIndex { index: usize, available: usize },
    #[error("constructor {index} name: expected {expected}, got {actual}")]
    ConstructorName {
        index: usize,
        expected: EcoString,
        actual: EcoString,
    },
    #[error("constructor module: expected {expected}, got {actual}")]
    ConstructorModule {
        expected: EcoString,
        actual: EcoString,
    },
    #[error("constructor arity: expected {expected}, got {actual}")]
    ConstructorArity { expected: usize, actual: usize },
    #[error("constructor result is {actual:?}, not a custom type")]
    ConstructorType { actual: ValueType },
    #[error("field index {index} is outside {available} fields")]
    FieldIndex { index: usize, available: usize },
    #[error("field {index} label: expected {expected:?}, got {actual:?}")]
    FieldLabel {
        index: usize,
        expected: Option<EcoString>,
        actual: Option<EcoString>,
    },
    #[error("field {index} type: expected {expected:?}, got {actual:?}")]
    FieldType {
        index: usize,
        expected: ValueType,
        actual: ValueType,
    },
    #[error("field {index} has incompatible result shape for {type_:?}")]
    FieldShapeConflict { index: usize, type_: ValueType },
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum InvalidFunctionShapeReason {
    #[error("function argument types do not match signature")]
    ArgumentTypeMismatch,
    #[error("anonymous functions are not module functions")]
    Anonymous,
    #[error("empty function bodies are not supported")]
    EmptyBody,
    #[error("labelled function argument")]
    LabelledArgument,
    #[error("function expression has type {actual:?}")]
    ExpressionType { actual: ValueType },
    #[error("function return type does not match body")]
    ReturnTypeMismatch,
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum InvalidExpressionShapeKind {
    #[error("bit array segment option")]
    BitArraySegmentOption,
    #[error("constant local variable")]
    ConstantLocalVariable,
    #[error("constant variable has no constructor metadata")]
    ConstantMissingConstructor,
    #[error("invalid constant node")]
    ConstantNode,
    #[error("constant record constructor kind")]
    ConstantRecordConstructorKind,
    #[error("constant record kind")]
    ConstantRecordKind,
    #[error("constant list element type")]
    ConstantListElementType,
    #[error("constant list type: got {actual:?}")]
    ConstantListType { actual: Option<ValueType> },
    #[error("constant tuple elements: expected {expected:?}, got {actual:?}")]
    ConstantTupleElements {
        expected: Vec<ValueType>,
        actual: Vec<ValueType>,
    },
    #[error("constant tuple type: got {actual:?}")]
    ConstantTupleType { actual: Option<ValueType> },
    #[error("echo expression is missing")]
    EchoExpressionMissing,
    #[error("function capture literal")]
    FunctionCaptureLiteral,
    #[error("function literal kind")]
    FunctionLiteralKind,
    #[error("generated todo message")]
    GeneratedTodoMessage,
    #[error("invalid guard node")]
    GuardNode,
    #[error("guard local shape")]
    GuardLocalShape,
    #[error("guard function local shape")]
    GuardFunctionLocalShape,
    #[error("list spread has no prefix elements")]
    ListSpreadEmptyPrefix,
    #[error("list expression type: got {actual:?}")]
    ListType { actual: ValueType },
    #[error("list index expression shape for {type_:?}")]
    ListIndexShape { type_: ValueType },
    #[error("local binding shape")]
    LocalBindingShape,
    #[error("invalid module constant node")]
    ModuleConstantNode,
    #[error("module constant list spread has no prefix elements")]
    ModuleConstantListSpreadEmptyPrefix,
    #[error("module constant list type: got {actual:?}")]
    ModuleConstantListType { actual: ValueType },
    #[error("module constant local variable")]
    ModuleConstantLocalVariable,
    #[error("module constant variable has no constructor metadata")]
    ModuleConstantMissingConstructor,
    #[error("module constant record kind")]
    ModuleConstantRecordKind,
    #[error("module constant storage shape")]
    ModuleConstantStorageShape,
    #[error("module constant tuple arity: expected {expected}, got {actual}")]
    ModuleConstantTupleArity { expected: usize, actual: usize },
    #[error("module constant tuple type: got {actual:?}")]
    ModuleConstantTupleType { actual: ValueType },
    #[error("positional access")]
    PositionalAccess,
    #[error("prelude constructor")]
    PreludeConstructor,
    #[error("record constructor")]
    RecordConstructor,
    #[error("record access")]
    RecordAccess,
    #[error("record access index cannot be represented on this target")]
    RecordAccessIndexOverflow,
    #[error("record update")]
    RecordUpdate,
    #[error("tuple expression arity: expected {expected}, got {actual}")]
    TupleArity { expected: usize, actual: usize },
    #[error("tuple index {index} is outside {available} elements")]
    TupleIndex { index: usize, available: usize },
    #[error("tuple index cannot be represented on this target")]
    TupleIndexOverflow,
    #[error("tuple expression type: got {actual:?}")]
    TupleType { actual: ValueType },
    #[error("variable function local shape")]
    VariableFunctionLocalShape,
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

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum InvalidCaseShapeReason {
    #[error("branch annotation type: expected {expected:?}, got {actual:?}")]
    BranchAnnotatedTypeMismatch {
        expected: ValueType,
        actual: ValueType,
    },
    #[error("branch shapes are incompatible: expected {expected:?}, got {actual:?}")]
    BranchShapeIncompatibility {
        expected: ValueType,
        actual: ValueType,
    },
    #[error("branch family assembly: expected {expected:?}, got {actual:?}")]
    BranchFamilyAssemblyMismatch {
        expected: ValueType,
        actual: ValueType,
    },
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
    #[error("missing false pattern")]
    MissingFalsePattern,
    #[error("missing fallback pattern")]
    MissingFallbackPattern,
    #[error("missing true pattern")]
    MissingTruePattern,
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
