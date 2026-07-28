mod parameter;

use crate::plan::{FunctionTemplateId, FunctionTemplateSignature, FunctionType, TypeScheme};
use ecow::EcoString;

pub(crate) use parameter::HostParameter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostFunctionTemplate {
    signature: FunctionTemplateSignature,
    package: EcoString,
    site: crate::plan::HostCallSite,
    parameters: Box<[HostParameter]>,
    type_: FunctionType,
}

impl HostFunctionTemplate {
    pub(crate) fn from_signature(
        signature: FunctionTemplateSignature,
        package: EcoString,
        site: crate::plan::HostCallSite,
        parameters: Vec<HostParameter>,
        type_: FunctionType,
    ) -> Self {
        Self {
            signature,
            package,
            site,
            parameters: parameters.into_boxed_slice(),
            type_,
        }
    }

    pub fn id(&self) -> FunctionTemplateId {
        self.signature.id()
    }

    pub fn package(&self) -> &EcoString {
        &self.package
    }

    pub fn module(&self) -> &EcoString {
        self.site.module()
    }

    pub fn name(&self) -> &EcoString {
        self.site.function()
    }

    pub(crate) fn site(&self) -> &crate::plan::HostCallSite {
        &self.site
    }

    pub fn scheme(&self) -> &TypeScheme {
        self.signature.scheme()
    }

    pub fn type_(&self) -> &FunctionType {
        &self.type_
    }

    pub(crate) fn parameters(&self) -> &[HostParameter] {
        &self.parameters
    }

    pub(crate) fn signature(&self) -> &FunctionTemplateSignature {
        &self.signature
    }
}
