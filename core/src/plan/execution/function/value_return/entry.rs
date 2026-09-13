use crate::plan::execution::function::ExecutableFunction;
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::Node;

pub enum ValueFunctionEntry<Body: 'static, HostTarget> {
    Graph(Node<ExecutableFunction<Body>>),
    Host(HostTarget),
}

impl<Body, HostTarget> ValueFunctionEntry<Body, HostTarget> {
    pub(in crate::plan::execution) fn graph(function: ExecutableFunction<Body>) -> Self {
        Self::Graph(Box::new(function).into())
    }

    pub(in crate::plan::execution) fn host(target: HostTarget) -> Self {
        Self::Host(target)
    }
}

impl<Body: 'static, HostTarget> Emit for ValueFunctionEntry<Body, HostTarget>
where
    Node<ExecutableFunction<Body>>: Emit,
    HostTarget: Emit,
{
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Graph(field_0) => output.call("function::ValueFunctionEntry::Graph", &[field_0]),
            Self::Host(field_0) => output.call("function::ValueFunctionEntry::Host", &[field_0]),
        }
    }
}
