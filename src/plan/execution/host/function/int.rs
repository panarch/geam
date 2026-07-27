use super::HostedFunction;
use crate::host::{HostCallArguments, HostIntFunction};
use num_bigint::BigInt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HostIntFunctionId(usize);

pub(crate) type HostedIntFunction = HostedFunction<HostIntFunction>;

impl HostIntFunctionId {
    pub(in crate::plan::execution) fn new(index: usize) -> Self {
        Self(index)
    }

    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl HostedIntFunction {
    pub(crate) fn call(&self, arguments: &dyn HostCallArguments) -> BigInt {
        self.implementation.call(arguments)
    }
}
