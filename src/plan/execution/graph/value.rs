mod id;
mod param;

pub(crate) use id::{
    BitArrayFunctionLocalId, BitArrayListFunctionLocalId, BitArrayListLocalId, BitArrayLocalId,
    BoolFunctionLocalId, BoolListFunctionLocalId, BoolListLocalId, BoolLocalId,
    CustomFunctionLocal, CustomFunctionLocalId, CustomListFunctionLocalId, CustomListLocalId,
    CustomLocal, CustomLocalId, FloatFunctionLocalId, FloatListFunctionLocalId, FloatListLocalId,
    FloatLocalId, FunctionFunctionLocal, FunctionFunctionLocalId, FunctionListFunctionLocalId,
    FunctionListLocalId, GenericFunctionLocal, GenericFunctionLocalId, IntFunctionLocalId,
    IntListFunctionLocalId, IntListLocalId, IntLocalId, ListFunctionLocal, ListListFunctionLocalId,
    ListListLocalId, ListLocal, NeverFunctionLocal, NeverFunctionLocalId, NilFunctionLocalId,
    NilListFunctionLocalId, NilListLocalId, NilLocalId, ParameterListFunctionLocalId,
    ParameterListListFunctionLocalId, ParameterListListLocalId, ParameterListLocalId,
    StringFunctionLocalId, StringListFunctionLocalId, StringListLocalId, StringLocalId,
    TupleFunctionLocalId, TupleListFunctionLocalId, TupleListLocalId, TupleLocalId,
    UtfCodepointFunctionLocalId, UtfCodepointListFunctionLocalId, UtfCodepointListLocalId,
    UtfCodepointLocalId,
};
pub(crate) use param::{ParamLocal, ParamSlot};

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
