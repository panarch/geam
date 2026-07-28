mod bit_array;
mod bool;
mod float;
mod int;
mod nil;
mod string;
mod utf_codepoint;

use crate::host::{HostCallArguments, HostCallError, HostNeverFunction, HostProfile};
use crate::plan::execution::graph::ParamLocal;
use crate::plan::execution::type_::FunctionType;
use ecow::EcoString;
use std::convert::Infallible;

pub(crate) use bit_array::{HostBitArrayFunctionId, HostedBitArrayFunction};
pub(crate) use bool::{HostBoolFunctionId, HostedBoolFunction};
pub(crate) use float::{HostFloatFunctionId, HostedFloatFunction};
pub(crate) use int::{HostIntFunctionId, HostedIntFunction};
pub(crate) use nil::{HostNilFunctionId, HostedNilFunction};
pub(crate) use string::{HostStringFunctionId, HostedStringFunction};
pub(crate) use utf_codepoint::{HostUtfCodepointFunctionId, HostedUtfCodepointFunction};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HostNeverFunctionId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HostedFunctionTarget<ValueTarget> {
    Value(ValueTarget),
    Never(HostNeverFunctionId),
}

pub(crate) struct HostedFunction<Implementation> {
    metadata: HostedFunctionMetadata,
    implementation: Implementation,
}

pub(crate) type HostedNeverFunction<Profile> = HostedFunction<HostNeverFunction<Profile>>;

pub(crate) struct HostedFunctionMetadata {
    package: EcoString,
    site: crate::plan::HostCallSite,
    signature: crate::plan::FunctionType,
    parameters: Box<[ParamLocal]>,
    type_: FunctionType,
}

impl HostNeverFunctionId {
    pub(in crate::plan::execution) fn new(index: usize) -> Self {
        Self(index)
    }

    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl<ValueTarget> HostedFunctionTarget<ValueTarget> {
    pub(in crate::plan::execution) fn value(target: ValueTarget) -> Self {
        Self::Value(target)
    }

    pub(in crate::plan::execution) fn never(target: HostNeverFunctionId) -> Self {
        Self::Never(target)
    }
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

    pub(crate) fn parameters(&self) -> &[ParamLocal] {
        self.metadata.parameters()
    }

    pub(crate) fn metadata(&self) -> &HostedFunctionMetadata {
        &self.metadata
    }
}

impl<Profile: HostProfile> HostedNeverFunction<Profile> {
    pub(crate) fn call(
        &self,
        state: &mut Profile::RunState,
        arguments: &dyn HostCallArguments,
    ) -> Result<Infallible, HostCallError> {
        self.implementation.call(state, arguments)
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

    pub(crate) fn parameters(&self) -> &[ParamLocal] {
        &self.parameters
    }

    pub(crate) fn type_(&self) -> &FunctionType {
        &self.type_
    }
}
