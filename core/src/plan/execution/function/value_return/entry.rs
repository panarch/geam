use crate::plan::execution::function::ExecutableFunction;
use std::sync::Arc;

pub(crate) enum ValueFunctionEntry<Body, HostTarget> {
    Graph(Arc<ExecutableFunction<Body>>),
    Host(HostTarget),
}

impl<Body, HostTarget> ValueFunctionEntry<Body, HostTarget> {
    pub(in crate::plan::execution) fn graph(function: ExecutableFunction<Body>) -> Self {
        Self::Graph(Arc::new(function))
    }

    pub(in crate::plan::execution) fn host(target: HostTarget) -> Self {
        Self::Host(target)
    }
}
