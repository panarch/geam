mod bit_array;
mod bool;
mod float;
mod int;
mod nil;
mod string;
mod utf_codepoint;

use crate::plan::execution::graph::ParamLocal;
use crate::plan::execution::type_::FunctionType;
use ecow::EcoString;

pub(crate) use bit_array::{HostBitArrayFunctionId, HostedBitArrayFunction};
pub(crate) use bool::{HostBoolFunctionId, HostedBoolFunction};
pub(crate) use float::{HostFloatFunctionId, HostedFloatFunction};
pub(crate) use int::{HostIntFunctionId, HostedIntFunction};
pub(crate) use nil::{HostNilFunctionId, HostedNilFunction};
pub(crate) use string::{HostStringFunctionId, HostedStringFunction};
pub(crate) use utf_codepoint::{HostUtfCodepointFunctionId, HostedUtfCodepointFunction};

pub(crate) struct HostedFunction<Implementation> {
    metadata: HostedFunctionMetadata,
    implementation: Implementation,
}

pub(crate) struct HostedFunctionMetadata {
    package: EcoString,
    site: crate::plan::HostCallSite,
    signature: crate::plan::FunctionType,
    parameters: Box<[ParamLocal]>,
    type_: FunctionType,
}

impl<Implementation> HostedFunction<Implementation> {
    pub(in crate::plan::execution) fn new(
        package: EcoString,
        site: crate::plan::HostCallSite,
        signature: crate::plan::FunctionType,
        parameters: Box<[ParamLocal]>,
        type_: FunctionType,
        implementation: Implementation,
    ) -> Self {
        Self {
            metadata: HostedFunctionMetadata {
                package,
                site,
                signature,
                parameters,
                type_,
            },
            implementation,
        }
    }

    pub(crate) fn package(&self) -> &EcoString {
        self.metadata.package()
    }

    pub(crate) fn module(&self) -> &EcoString {
        self.metadata.module()
    }

    pub(crate) fn name(&self) -> &EcoString {
        self.metadata.name()
    }

    pub(crate) fn site(&self) -> &crate::plan::HostCallSite {
        self.metadata.site()
    }

    pub(crate) fn parameters(&self) -> &[ParamLocal] {
        self.metadata.parameters()
    }

    pub(in crate::plan::execution) fn type_(&self) -> &FunctionType {
        self.metadata.type_()
    }

    pub(crate) fn metadata(&self) -> &HostedFunctionMetadata {
        &self.metadata
    }
}

impl HostedFunctionMetadata {
    pub(crate) fn package(&self) -> &EcoString {
        &self.package
    }

    pub(crate) fn module(&self) -> &EcoString {
        self.site.module()
    }

    pub(crate) fn name(&self) -> &EcoString {
        self.site.function()
    }

    pub(crate) fn site(&self) -> &crate::plan::HostCallSite {
        &self.site
    }

    pub(crate) fn signature(&self) -> &crate::plan::FunctionType {
        &self.signature
    }

    fn parameters(&self) -> &[ParamLocal] {
        &self.parameters
    }

    fn type_(&self) -> &FunctionType {
        &self.type_
    }
}
