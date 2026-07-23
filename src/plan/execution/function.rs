mod entry;
mod function_return;
mod graph;
mod list_return;
mod runtime;
mod table;
mod value_return;

pub(crate) use entry::FunctionEntry;
pub(in crate::plan::execution) use function_return::FunctionFunctionTables;
pub(crate) use function_return::{
    BitArrayFunctionFunctionId, BitArrayFunctionReturn, BitArrayListFunctionFunctionId,
    BoolFunctionFunctionId, BoolFunctionReturn, BoolListFunctionFunctionId,
    CustomFunctionFunctionId, CustomFunctionReturn, CustomListFunctionFunctionId,
    FloatFunctionFunctionId, FloatFunctionReturn, FloatListFunctionFunctionId,
    FunctionFunctionFunctionId, FunctionFunctionId, FunctionFunctionReturn,
    FunctionListFunctionFunctionId, GenericFunctionFunctionId, GenericFunctionReturn,
    IntFunctionFunctionId, IntFunctionReturn, IntListFunctionFunctionId, ListFunctionFunctionId,
    ListFunctionReturn, ListListFunctionFunctionId, NeverFunctionFunctionId, NeverFunctionReturn,
    NilFunctionFunctionId, NilFunctionReturn, NilListFunctionFunctionId,
    ParameterListFunctionFunctionId, ParameterListListFunctionFunctionId, StringFunctionFunctionId,
    StringFunctionReturn, StringListFunctionFunctionId, TupleFunctionFunctionId,
    TupleFunctionReturn, TupleListFunctionFunctionId, TypedFunctionReturn,
    UtfCodepointFunctionFunctionId, UtfCodepointFunctionReturn, UtfCodepointListFunctionFunctionId,
};
pub(crate) use graph::{FunctionGraph, FunctionGraphExit};
pub(in crate::plan::execution) use list_return::ListFunctionTables;
pub(crate) use list_return::{
    BitArrayListFunctionId, BitArrayListReturn, BoolListFunctionId, BoolListReturn,
    CustomListFunctionId, CustomListReturn, FloatListFunctionId, FloatListReturn,
    FunctionListFunctionId, FunctionListReturn, IntListFunctionId, IntListReturn, ListFunctionId,
    ListListFunctionId, ListListReturn, NilListFunctionId, NilListReturn, ParameterListFunctionId,
    ParameterListListFunctionId, ParameterListListReturn, ParameterListReturn,
    StringListFunctionId, StringListReturn, TupleListFunctionId, TupleListReturn,
    UtfCodepointListFunctionId, UtfCodepointListReturn,
};
pub(crate) use runtime::{FunctionReturnFamily, GenericCallableId, RuntimeFunctionId};
pub(super) use table::FunctionTables;
pub(in crate::plan::execution) use value_return::ValueFunctionTables;
pub(crate) use value_return::{
    BitArrayFunctionId, BitArrayReturn, BoolFunctionId, BoolReturn, CustomFunctionId, CustomReturn,
    FloatFunctionId, FloatReturn, IntFunctionId, IntReturn, NeverFunctionId, NeverReturn,
    NilFunctionId, NilReturn, StringFunctionId, StringReturn, TupleFunctionId, TupleReturn,
    UtfCodepointFunctionId, UtfCodepointReturn,
};

pub(crate) struct ExecutableFunction<Graph> {
    entry: FunctionEntry,
    graph: Graph,
}

impl<Graph> ExecutableFunction<Graph> {
    pub(super) fn new(parameter_count: usize, graph: Graph) -> Self {
        Self {
            entry: FunctionEntry::new(parameter_count),
            graph,
        }
    }

    pub(crate) fn graph(&self) -> &Graph {
        &self.graph
    }

    pub(crate) fn entry(&self) -> &FunctionEntry {
        &self.entry
    }
}
