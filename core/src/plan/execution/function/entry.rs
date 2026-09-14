use super::{ExecutionGraphProfile, ProfiledFunctionBody};
use crate::plan::execution::graph::ParamSlot;
use crate::plan::execution::prepared::rust::{Emit, Rust};

pub struct FunctionEntry {
    pub parameter_count: usize,
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

impl Emit for FunctionEntry {
    fn emit(&self, output: &mut Rust) {
        let Self { parameter_count } = self;
        output.structure(
            "function::FunctionEntry",
            &[("parameter_count", parameter_count)],
        );
    }
}
