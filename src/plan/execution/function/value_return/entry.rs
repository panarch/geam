use super::IntFunctionBody;
use crate::plan::execution::function::ExecutableFunction;

pub(crate) enum IntFunctionEntry<Host> {
    Graph(ExecutableFunction<IntFunctionBody>),
    Host(Host),
}

impl<Host> IntFunctionEntry<Host> {
    pub(in crate::plan::execution) fn graph(function: ExecutableFunction<IntFunctionBody>) -> Self {
        Self::Graph(function)
    }

    pub(in crate::plan::execution) fn host(target: Host) -> Self {
        Self::Host(target)
    }
}
