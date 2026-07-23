pub(super) mod body;
pub(super) mod id;
mod table;

pub(crate) use body::{
    BitArrayFunctionReturn, BoolFunctionReturn, CustomFunctionReturn, FloatFunctionReturn,
    FunctionFunctionReturn, GenericFunctionReturn, IntFunctionReturn, ListFunctionReturn,
    NeverFunctionReturn, NilFunctionReturn, StringFunctionReturn, TupleFunctionReturn,
    TypedFunctionReturn, UtfCodepointFunctionReturn,
};
pub(crate) use id::{
    BitArrayFunctionFunctionId, BitArrayListFunctionFunctionId, BoolFunctionFunctionId,
    BoolListFunctionFunctionId, CustomFunctionFunctionId, CustomListFunctionFunctionId,
    FloatFunctionFunctionId, FloatListFunctionFunctionId, FunctionFunctionFunctionId,
    FunctionFunctionId, FunctionListFunctionFunctionId, GenericFunctionFunctionId,
    IntFunctionFunctionId, IntListFunctionFunctionId, ListFunctionFunctionId,
    ListListFunctionFunctionId, NeverFunctionFunctionId, NilFunctionFunctionId,
    NilListFunctionFunctionId, ParameterListFunctionFunctionId,
    ParameterListListFunctionFunctionId, StringFunctionFunctionId, StringListFunctionFunctionId,
    TupleFunctionFunctionId, TupleListFunctionFunctionId, UtfCodepointFunctionFunctionId,
    UtfCodepointListFunctionFunctionId,
};
pub(in crate::plan::execution) use table::FunctionFunctionTables;
