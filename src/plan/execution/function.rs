use super::ParamSlot;
use super::graph::FunctionGraph;

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
