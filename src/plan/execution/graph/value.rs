use crate::plan::execution::{
    BitArrayFunctionLocalId, BitArrayListLocalId, BoolFunctionLocalId, BoolListLocalId,
    CustomFunctionLocal, CustomListLocalId, FloatFunctionLocalId, FloatListLocalId,
    FunctionFunctionLocal, FunctionListLocalId, GenericFunctionLocal, IntFunctionLocalId,
    IntListLocalId, ListFunctionLocal, ListListLocalId, NeverFunctionLocal, NilFunctionLocalId,
    NilListLocalId, ParameterListListLocalId, StringFunctionLocalId, StringListLocalId,
    TupleFunctionLocalId, TupleListLocalId, UtfCodepointFunctionLocalId, UtfCodepointListLocalId,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum FunctionLocal {
    Generic(GenericFunctionLocal),
    Never(NeverFunctionLocal),
    Int(IntFunctionLocalId),
    Float(FloatFunctionLocalId),
    String(StringFunctionLocalId),
    BitArray(BitArrayFunctionLocalId),
    UtfCodepoint(UtfCodepointFunctionLocalId),
    Custom(CustomFunctionLocal),
    Bool(BoolFunctionLocalId),
    Nil(NilFunctionLocalId),
    Tuple(TupleFunctionLocalId),
    List(ListFunctionLocal),
    Function(FunctionFunctionLocal),
}

pub(crate) enum NeverReturn {}

pub(crate) enum StoredListLocal {
    ParameterList(ParameterListListLocalId),
    Int(IntListLocalId),
    String(StringListLocalId),
    BitArray(BitArrayListLocalId),
    UtfCodepoint(UtfCodepointListLocalId),
    Custom(CustomListLocalId),
    Float(FloatListLocalId),
    Bool(BoolListLocalId),
    Nil(NilListLocalId),
    Tuple(TupleListLocalId),
    List(ListListLocalId),
    Function(FunctionListLocalId),
}
