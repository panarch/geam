use super::super::body::FunctionBody;
use super::{
    BitArrayListFunctionId, BoolListFunctionId, CustomListFunctionId, FloatListFunctionId,
    FunctionListFunctionId, IntListFunctionId, ListListFunctionId, NilListFunctionId,
    ParameterListFunctionId, ParameterListListFunctionId, StringListFunctionId,
    TupleListFunctionId, UtfCodepointListFunctionId,
};
use crate::plan::execution::graph::{
    BitArrayListLocalId, BoolListLocalId, CustomListLocalId, FloatListLocalId, FunctionListLocalId,
    IntListLocalId, ListListLocalId, NilListLocalId, ParameterListListLocalId,
    ParameterListLocalId, StringListLocalId, TupleListLocalId, UtfCodepointListLocalId,
};

pub(crate) type ParameterListFunctionBody =
    FunctionBody<ParameterListLocalId, crate::plan::FunctionCallTarget<ParameterListFunctionId>>;
pub(crate) type IntListFunctionBody =
    FunctionBody<IntListLocalId, crate::plan::FunctionCallTarget<IntListFunctionId>>;
pub(crate) type FloatListFunctionBody =
    FunctionBody<FloatListLocalId, crate::plan::FunctionCallTarget<FloatListFunctionId>>;
pub(crate) type StringListFunctionBody =
    FunctionBody<StringListLocalId, crate::plan::FunctionCallTarget<StringListFunctionId>>;
pub(crate) type BitArrayListFunctionBody =
    FunctionBody<BitArrayListLocalId, crate::plan::FunctionCallTarget<BitArrayListFunctionId>>;
pub(crate) type UtfCodepointListFunctionBody = FunctionBody<
    UtfCodepointListLocalId,
    crate::plan::FunctionCallTarget<UtfCodepointListFunctionId>,
>;
pub(crate) type CustomListFunctionBody =
    FunctionBody<CustomListLocalId, crate::plan::FunctionCallTarget<CustomListFunctionId>>;
pub(crate) type BoolListFunctionBody =
    FunctionBody<BoolListLocalId, crate::plan::FunctionCallTarget<BoolListFunctionId>>;
pub(crate) type NilListFunctionBody =
    FunctionBody<NilListLocalId, crate::plan::FunctionCallTarget<NilListFunctionId>>;
pub(crate) type TupleListFunctionBody =
    FunctionBody<TupleListLocalId, crate::plan::FunctionCallTarget<TupleListFunctionId>>;
pub(crate) type ParameterListListFunctionBody = FunctionBody<
    ParameterListListLocalId,
    crate::plan::FunctionCallTarget<ParameterListListFunctionId>,
>;
pub(crate) type ListListFunctionBody =
    FunctionBody<ListListLocalId, crate::plan::FunctionCallTarget<ListListFunctionId>>;
pub(crate) type FunctionListFunctionBody =
    FunctionBody<FunctionListLocalId, crate::plan::FunctionCallTarget<FunctionListFunctionId>>;
