mod bool;
mod int;

use crate::plan::execution::graph::ParamLocal;
use crate::plan::execution::type_::FunctionType;
use ecow::EcoString;

pub(crate) use bool::{HostBoolFunctionId, HostedBoolFunction};
pub(crate) use int::{HostIntFunctionId, HostedIntFunction};

pub(crate) struct HostedFunction<Implementation> {
    package: EcoString,
    module: EcoString,
    name: EcoString,
    parameters: Box<[ParamLocal]>,
    type_: FunctionType,
    implementation: Implementation,
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
