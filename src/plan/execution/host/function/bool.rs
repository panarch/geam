use super::HostedFunction;
use crate::host::{HostBoolFunction, HostCallArguments};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HostBoolFunctionId(usize);

pub(crate) type HostedBoolFunction = HostedFunction<HostBoolFunction>;

impl HostBoolFunctionId {
    pub(in crate::plan::execution) fn new(index: usize) -> Self {
        Self(index)
    }

    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl HostedBoolFunction {
    pub(crate) fn call(&self, arguments: &dyn HostCallArguments) -> bool {
        self.implementation.call(arguments)
    }
}
