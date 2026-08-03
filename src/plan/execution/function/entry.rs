use super::{ExecutionGraphProfile, ProfiledFunctionBody};
use crate::plan::execution::graph::ParamSlot;

pub(crate) struct FunctionEntry {
    parameter_count: usize,
}

impl FunctionEntry {
    pub(in crate::plan::execution) fn new(parameter_count: usize) -> Self {
        Self { parameter_count }
    }

    pub(crate) fn params<'a, Return, TailCall, Graph: ExecutionGraphProfile>(
        &self,
        body: &'a ProfiledFunctionBody<Return, TailCall, Graph>,
    ) -> &'a [ParamSlot] {
        let block_graph = body.block_graph();
        &block_graph.block(block_graph.entry()).params()[..self.parameter_count]
    }

    pub(in crate::plan::execution) fn captures<
        'a,
        Return,
        TailCall,
        Graph: ExecutionGraphProfile,
    >(
        &self,
        body: &'a ProfiledFunctionBody<Return, TailCall, Graph>,
    ) -> &'a [ParamSlot] {
        let block_graph = body.block_graph();
        &block_graph.block(block_graph.entry()).params()[self.parameter_count..]
    }
}
