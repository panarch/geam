use crate::host::{HostBoolFunction, HostCallArguments, HostIntFunction};
use crate::plan::execution::graph::ParamLocal;
use crate::plan::execution::type_::FunctionType;
use ecow::EcoString;
use num_bigint::BigInt;

pub(crate) struct HostedExecutionHost;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HostIntFunctionId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HostBoolFunctionId(usize);

pub(crate) type HostedIntFunction = HostedFunction<HostIntFunction>;
pub(crate) type HostedBoolFunction = HostedFunction<HostBoolFunction>;

pub(crate) struct HostedFunction<Implementation> {
    package: EcoString,
    module: EcoString,
    name: EcoString,
    parameters: Box<[ParamLocal]>,
    type_: FunctionType,
    implementation: Implementation,
}

impl HostIntFunctionId {
    pub(in crate::plan::execution) fn new(index: usize) -> Self {
        Self(index)
    }

    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl HostBoolFunctionId {
    pub(in crate::plan::execution) fn new(index: usize) -> Self {
        Self(index)
    }

    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl<Implementation> HostedFunction<Implementation> {
    pub(in crate::plan::execution) fn new(
        package: EcoString,
        module: EcoString,
        name: EcoString,
        parameters: Box<[ParamLocal]>,
        type_: FunctionType,
        implementation: Implementation,
    ) -> Self {
        Self {
            package,
            module,
            name,
            parameters,
            type_,
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

    pub(crate) fn parameters(&self) -> &[ParamLocal] {
        &self.parameters
    }

    pub(in crate::plan::execution) fn type_(&self) -> &FunctionType {
        &self.type_
    }
}

impl HostedIntFunction {
    pub(crate) fn call(&self, arguments: &dyn HostCallArguments) -> BigInt {
        self.implementation.call(arguments)
    }
}

impl HostedBoolFunction {
    pub(crate) fn call(&self, arguments: &dyn HostCallArguments) -> bool {
        self.implementation.call(arguments)
    }
}
