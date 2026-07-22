use super::super::super::{FunctionLocal, StoredListLocal};
use crate::plan::execution::{
    BitArrayListFunctionId, BitArrayListLocalId, BitArrayListTypeId, BoolListFunctionId,
    BoolListLocalId, BoolListTypeId, ConstantId, CustomListFunctionId, CustomListLocalId,
    CustomListTypeId, CustomLocal, FloatListFunctionId, FloatListLocalId, FloatListTypeId,
    FloatLocalId, FunctionListFunctionId, FunctionListLocalId, FunctionListTypeId,
    IntListFunctionId, IntListLocalId, IntListTypeId, IntLocalId, ListFunctionLocal,
    ListListFunctionId, ListListLocalId, ListListTypeId, NilListFunctionId, NilListLocalId,
    NilListTypeId, ParamLocal, ParameterListFunctionId, ParameterListListFunctionId,
    ParameterListListLocalId, ParameterListListTypeId, ParameterListLocalId, ParameterListTypeId,
    StringListFunctionId, StringListLocalId, StringListTypeId, StringLocalId, TupleListFunctionId,
    TupleListLocalId, TupleListTypeId, TupleLocalId, UtfCodepointListFunctionId,
    UtfCodepointListLocalId, UtfCodepointListTypeId, UtfCodepointLocalId,
};

pub(crate) enum ParameterListInstruction {
    Empty,
    Constant(ConstantId<ParameterListLocalId>),
    Call {
        function: ParameterListFunctionId,
        args: Box<[ParamLocal]>,
    },
    FunctionCall {
        function: ListFunctionLocal,
        args: Box<[ParamLocal]>,
    },
    TupleIndex {
        tuple: TupleLocalId,
        index: usize,
    },
    CustomField {
        source: CustomLocal,
        index: usize,
    },
    ListIndex {
        list: ParameterListListLocalId,
        index: usize,
    },
}

pub(crate) enum TypedListInstruction<Element, Local, Function> {
    Value(Box<[Element]>),
    Constant(ConstantId<Local>),
    Spread {
        elements: Box<[Element]>,
        tail: Local,
    },
    Call {
        function: Function,
        args: Box<[ParamLocal]>,
    },
    FunctionCall {
        function: ListFunctionLocal,
        args: Box<[ParamLocal]>,
    },
    TupleIndex {
        tuple: TupleLocalId,
        index: usize,
    },
    CustomField {
        source: CustomLocal,
        index: usize,
    },
    ListIndex {
        list: ListListLocalId,
        index: usize,
    },
    DropFirst {
        list: Local,
        count: usize,
    },
}

pub(crate) enum ListInstruction {
    Parameter(ParameterListTypeId, ParameterListInstruction),
    ParameterList(
        ParameterListListTypeId,
        TypedListInstruction<
            ParameterListLocalId,
            ParameterListListLocalId,
            ParameterListListFunctionId,
        >,
    ),
    Int(
        IntListTypeId,
        TypedListInstruction<IntLocalId, IntListLocalId, IntListFunctionId>,
    ),
    String(
        StringListTypeId,
        TypedListInstruction<StringLocalId, StringListLocalId, StringListFunctionId>,
    ),
    BitArray(
        BitArrayListTypeId,
        TypedListInstruction<
            crate::plan::execution::BitArrayLocalId,
            BitArrayListLocalId,
            BitArrayListFunctionId,
        >,
    ),
    UtfCodepoint(
        UtfCodepointListTypeId,
        TypedListInstruction<
            UtfCodepointLocalId,
            UtfCodepointListLocalId,
            UtfCodepointListFunctionId,
        >,
    ),
    Custom(
        CustomListTypeId,
        TypedListInstruction<CustomLocal, CustomListLocalId, CustomListFunctionId>,
    ),
    Float(
        FloatListTypeId,
        TypedListInstruction<FloatLocalId, FloatListLocalId, FloatListFunctionId>,
    ),
    Bool(
        BoolListTypeId,
        TypedListInstruction<
            crate::plan::execution::BoolLocalId,
            BoolListLocalId,
            BoolListFunctionId,
        >,
    ),
    Nil(
        NilListTypeId,
        TypedListInstruction<crate::plan::execution::NilLocalId, NilListLocalId, NilListFunctionId>,
    ),
    Tuple(
        TupleListTypeId,
        TypedListInstruction<TupleLocalId, TupleListLocalId, TupleListFunctionId>,
    ),
    List(
        ListListTypeId,
        TypedListInstruction<StoredListLocal, ListListLocalId, ListListFunctionId>,
    ),
    Function(
        FunctionListTypeId,
        TypedListInstruction<FunctionLocal, FunctionListLocalId, FunctionListFunctionId>,
    ),
}
