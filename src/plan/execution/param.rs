use super::{
    BitArrayFunctionLocalId, BitArrayLocalId, BoolFunctionLocalId, BoolLocalId,
    CustomFunctionLocal, CustomLocal, FloatFunctionLocalId, FloatLocalId, FunctionFunctionLocal,
    FunctionType, IntFunctionLocalId, IntLocalId, ListFunctionLocal, ListLocal, NilFunctionLocalId,
    NilLocalId, StringFunctionLocalId, StringLocalId, TupleFunctionLocalId, TupleLocalId,
    UtfCodepointFunctionLocalId, UtfCodepointLocalId, ValueShapeId, ValueType,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ParamSlot {
    local: ParamLocal,
    shape: ValueShapeId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ParamLocal {
    Int(IntLocalId),
    Float(FloatLocalId),
    String(StringLocalId),
    BitArray(BitArrayLocalId),
    UtfCodepoint(UtfCodepointLocalId),
    Custom(CustomLocal),
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
    CustomFunction(CustomFunctionLocal),
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
    FunctionFunction(FunctionFunctionLocal),
}

impl ParamSlot {
    pub(super) fn new(local: ParamLocal, shape: ValueShapeId) -> Self {
        Self { local, shape }
    }

    pub(crate) fn local(&self) -> &ParamLocal {
        &self.local
    }

    pub(crate) fn shape(&self) -> ValueShapeId {
        self.shape
    }
}
