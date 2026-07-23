use super::FunctionGraph;
use crate::plan::execution::graph::ParamSlot;

pub(crate) struct FunctionEntry {
    parameter_count: usize,
}

impl FunctionEntry {
    pub(in crate::plan::execution) fn new(parameter_count: usize) -> Self {
        Self { parameter_count }
    }

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
