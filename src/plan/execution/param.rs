use super::{
    BitArrayFunctionLocalId, BitArrayLocalId, BoolFunctionLocalId, BoolLocalId,
    CustomFunctionLocalId, CustomLocalId, CustomTypeId, FloatFunctionLocalId, FloatLocalId,
    FunctionFunctionLocalId, FunctionType, IntFunctionLocalId, IntLocalId, ListFunctionLocal,
    ListLocal, NilFunctionLocalId, NilLocalId, StringFunctionLocalId, StringLocalId,
    TupleFunctionLocalId, TupleLocalId, UtfCodepointFunctionLocalId, UtfCodepointLocalId,
    ValueType,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ParamLocal {
    Int(IntLocalId),
    Float(FloatLocalId),
    String(StringLocalId),
    BitArray(BitArrayLocalId),
    UtfCodepoint(UtfCodepointLocalId),
    Custom {
        local: CustomLocalId,
        type_id: CustomTypeId,
    },
    Bool(BoolLocalId),
    Nil(NilLocalId),
    Tuple {
        local: TupleLocalId,
        type_: Vec<ValueType>,
    },
    List(ListLocal),
    IntFunction {
        local: IntFunctionLocalId,
        type_: FunctionType,
    },
    FloatFunction {
        local: FloatFunctionLocalId,
        type_: FunctionType,
    },
    StringFunction {
        local: StringFunctionLocalId,
        type_: FunctionType,
    },
    BitArrayFunction {
        local: BitArrayFunctionLocalId,
        type_: FunctionType,
    },
    UtfCodepointFunction {
        local: UtfCodepointFunctionLocalId,
        type_: FunctionType,
    },
    CustomFunction {
        local: CustomFunctionLocalId,
        type_: FunctionType,
    },
    BoolFunction {
        local: BoolFunctionLocalId,
        type_: FunctionType,
    },
    NilFunction {
        local: NilFunctionLocalId,
        type_: FunctionType,
    },
    TupleFunction {
        local: TupleFunctionLocalId,
        type_: FunctionType,
    },
    ListFunction(ListFunctionLocal),
    FunctionFunction {
        local: FunctionFunctionLocalId,
        type_: FunctionType,
    },
}

impl ParamLocal {
    pub(crate) fn value_type(&self) -> ValueType {
        match self {
            Self::Int(_) => ValueType::Int,
            Self::Float(_) => ValueType::Float,
            Self::String(_) => ValueType::String,
            Self::BitArray(_) => ValueType::BitArray,
            Self::UtfCodepoint(_) => ValueType::UtfCodepoint,
            Self::Custom { type_id, .. } => ValueType::Custom(*type_id),
            Self::Bool(_) => ValueType::Bool,
            Self::Nil(_) => ValueType::Nil,
            Self::Tuple { type_, .. } => ValueType::Tuple(type_.clone()),
            Self::List(local) => ValueType::List(local.list_type()),
            Self::IntFunction { type_, .. }
            | Self::FloatFunction { type_, .. }
            | Self::StringFunction { type_, .. }
            | Self::BitArrayFunction { type_, .. }
            | Self::UtfCodepointFunction { type_, .. }
            | Self::CustomFunction { type_, .. }
            | Self::BoolFunction { type_, .. }
            | Self::NilFunction { type_, .. }
            | Self::TupleFunction { type_, .. }
            | Self::FunctionFunction { type_, .. } => ValueType::Function(Box::new(type_.clone())),
            Self::ListFunction(local) => ValueType::Function(Box::new(local.type_().clone())),
        }
    }
}
