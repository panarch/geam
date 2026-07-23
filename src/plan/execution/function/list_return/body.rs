use super::super::graph::FunctionGraph;
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

pub(crate) type ParameterListReturn = FunctionGraph<ParameterListLocalId, ParameterListFunctionId>;
pub(crate) type IntListReturn = FunctionGraph<IntListLocalId, IntListFunctionId>;
pub(crate) type FloatListReturn = FunctionGraph<FloatListLocalId, FloatListFunctionId>;
pub(crate) type StringListReturn = FunctionGraph<StringListLocalId, StringListFunctionId>;
pub(crate) type BitArrayListReturn = FunctionGraph<BitArrayListLocalId, BitArrayListFunctionId>;
pub(crate) type UtfCodepointListReturn =
    FunctionGraph<UtfCodepointListLocalId, UtfCodepointListFunctionId>;
pub(crate) type CustomListReturn = FunctionGraph<CustomListLocalId, CustomListFunctionId>;
pub(crate) type BoolListReturn = FunctionGraph<BoolListLocalId, BoolListFunctionId>;
pub(crate) type NilListReturn = FunctionGraph<NilListLocalId, NilListFunctionId>;
pub(crate) type TupleListReturn = FunctionGraph<TupleListLocalId, TupleListFunctionId>;
pub(crate) type ParameterListListReturn =
    FunctionGraph<ParameterListListLocalId, ParameterListListFunctionId>;
pub(crate) type ListListReturn = FunctionGraph<ListListLocalId, ListListFunctionId>;
pub(crate) type FunctionListReturn = FunctionGraph<FunctionListLocalId, FunctionListFunctionId>;
