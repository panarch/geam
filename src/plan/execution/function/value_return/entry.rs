use crate::plan::execution::function::ExecutableFunction;

pub(crate) enum ValueFunctionEntry<Body, HostTarget> {
    Graph(ExecutableFunction<Body>),
    Host(HostTarget),
}

impl<Body, HostTarget> ValueFunctionEntry<Body, HostTarget> {
    pub(in crate::plan::execution) fn graph(function: ExecutableFunction<Body>) -> Self {
        Self::Graph(function)
    }

    pub(in crate::plan::execution) fn host(target: HostTarget) -> Self {
        Self::Host(target)
    }
}
