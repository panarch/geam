mod graph;
mod id;
mod return_;
mod table;

pub(crate) use graph::{FunctionGraph, FunctionGraphExit};
pub(crate) use id::{
    BitArrayFunctionFunctionId, BitArrayFunctionId, BitArrayListFunctionFunctionId,
    BitArrayListFunctionId, BoolFunctionFunctionId, BoolFunctionId, BoolListFunctionFunctionId,
    BoolListFunctionId, CustomFunctionFunctionId, CustomFunctionId, CustomListFunctionFunctionId,
    CustomListFunctionId, FloatFunctionFunctionId, FloatFunctionId, FloatListFunctionFunctionId,
    FloatListFunctionId, FunctionFunctionFunctionId, FunctionFunctionId,
    FunctionListFunctionFunctionId, FunctionListFunctionId, FunctionReturnFamily,
    GenericCallableId, GenericFunctionFunctionId, IntFunctionFunctionId, IntFunctionId,
    IntListFunctionFunctionId, IntListFunctionId, ListFunctionFunctionId, ListFunctionId,
    ListListFunctionFunctionId, ListListFunctionId, NeverFunctionFunctionId, NeverFunctionId,
    NilFunctionFunctionId, NilFunctionId, NilListFunctionFunctionId, NilListFunctionId,
    ParameterListFunctionFunctionId, ParameterListFunctionId, ParameterListListFunctionFunctionId,
    ParameterListListFunctionId, RuntimeFunctionId, StringFunctionFunctionId, StringFunctionId,
    StringListFunctionFunctionId, StringListFunctionId, TupleFunctionFunctionId, TupleFunctionId,
    TupleListFunctionFunctionId, TupleListFunctionId, UtfCodepointFunctionFunctionId,
    UtfCodepointFunctionId, UtfCodepointListFunctionFunctionId, UtfCodepointListFunctionId,
};
pub(crate) use return_::{
    BitArrayFunctionReturn, BitArrayListReturn, BitArrayReturn, BoolFunctionReturn, BoolListReturn,
    BoolReturn, CustomFunctionReturn, CustomListReturn, CustomReturn, FloatFunctionReturn,
    FloatListReturn, FloatReturn, FunctionFunctionReturn, FunctionListReturn,
    GenericFunctionReturn, IntFunctionReturn, IntListReturn, IntReturn, ListFunctionReturn,
    ListListReturn, NeverFunctionReturn, NeverReturn, NilFunctionReturn, NilListReturn, NilReturn,
    ParameterListListReturn, ParameterListReturn, StringFunctionReturn, StringListReturn,
    StringReturn, TupleFunctionReturn, TupleListReturn, TupleReturn, TypedFunctionReturn,
    UtfCodepointFunctionReturn, UtfCodepointListReturn, UtfCodepointReturn,
};
pub(super) use table::FunctionTables;

use super::graph::ParamSlot;

pub(crate) struct ExecutableFunction<Graph> {
    entry: FunctionEntry,
    graph: Graph,
}

pub(crate) struct FunctionEntry {
    parameter_count: usize,
}

impl<Graph> ExecutableFunction<Graph> {
    pub(super) fn new(parameter_count: usize, graph: Graph) -> Self {
        Self {
            entry: FunctionEntry { parameter_count },
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

impl FunctionEntry {
    pub(crate) fn params<'a, Return, TailCall>(
        &self,
        graph: &'a FunctionGraph<Return, TailCall>,
    ) -> &'a [ParamSlot] {
        &graph.block(graph.entry()).params()[..self.parameter_count]
    }

    pub(in crate::plan::execution) fn captures<'a, Return, TailCall>(
        &self,
        graph: &'a FunctionGraph<Return, TailCall>,
    ) -> &'a [ParamSlot] {
        &graph.block(graph.entry()).params()[self.parameter_count..]
    }
}
