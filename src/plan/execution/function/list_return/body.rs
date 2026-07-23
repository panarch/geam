use super::super::body::FunctionBody;
use super::{
    BitArrayListFunctionId, BoolListFunctionId, CustomListFunctionId, FloatListFunctionId,
    FunctionListFunctionId, IntListFunctionId, ListListFunctionId, NilListFunctionId,
    ParameterListFunctionId, ParameterListListFunctionId, StringListFunctionId,
    TupleListFunctionId, UtfCodepointListFunctionId,
};
use crate::plan::execution::{
    BitArrayListLocalId, BoolListLocalId, CustomListLocalId, FloatListLocalId, FunctionListLocalId,
    IntListLocalId, ListListLocalId, NilListLocalId, ParameterListListLocalId,
    ParameterListLocalId, StringListLocalId, TupleListLocalId, UtfCodepointListLocalId,
};

pub(crate) type ParameterListFunctionBody =
    FunctionBody<ParameterListLocalId, ParameterListFunctionId>;
pub(crate) type IntListFunctionBody = FunctionBody<IntListLocalId, IntListFunctionId>;
pub(crate) type FloatListFunctionBody = FunctionBody<FloatListLocalId, FloatListFunctionId>;
pub(crate) type StringListFunctionBody = FunctionBody<StringListLocalId, StringListFunctionId>;
pub(crate) type BitArrayListFunctionBody =
    FunctionBody<BitArrayListLocalId, BitArrayListFunctionId>;
pub(crate) type UtfCodepointListFunctionBody =
    FunctionBody<UtfCodepointListLocalId, UtfCodepointListFunctionId>;
pub(crate) type CustomListFunctionBody = FunctionBody<CustomListLocalId, CustomListFunctionId>;
pub(crate) type BoolListFunctionBody = FunctionBody<BoolListLocalId, BoolListFunctionId>;
pub(crate) type NilListFunctionBody = FunctionBody<NilListLocalId, NilListFunctionId>;
pub(crate) type TupleListFunctionBody = FunctionBody<TupleListLocalId, TupleListFunctionId>;
pub(crate) type ParameterListListFunctionBody =
    FunctionBody<ParameterListListLocalId, ParameterListListFunctionId>;
pub(crate) type ListListFunctionBody = FunctionBody<ListListLocalId, ListListFunctionId>;
pub(crate) type FunctionListFunctionBody =
    FunctionBody<FunctionListLocalId, FunctionListFunctionId>;
