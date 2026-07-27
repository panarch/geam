use crate::host::HostIntFunction;
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HostIntFunctionId(usize);

pub(crate) struct HostedIntFunction {
    package: EcoString,
    module: EcoString,
    name: EcoString,
    implementation: HostIntFunction,
}

impl HostIntFunctionId {
    pub(in crate::plan::execution) fn new(index: usize) -> Self {
        Self(index)
    }

    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl HostedIntFunction {
    pub(in crate::plan::execution) fn new(
        package: EcoString,
        module: EcoString,
        name: EcoString,
        implementation: HostIntFunction,
    ) -> Self {
        Self {
            package,
            module,
            name,
            implementation,
        }
    }

    pub(in crate::plan::execution) fn package(&self) -> &EcoString {
        &self.package
    }

    pub(in crate::plan::execution) fn module(&self) -> &EcoString {
        &self.module
    }

    pub(in crate::plan::execution) fn name(&self) -> &EcoString {
        &self.name
    }

    pub(crate) fn call(&self, left: BigInt, right: BigInt) -> BigInt {
        self.implementation.call(left, right)
    }
}
