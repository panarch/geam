mod parameter;
mod return_;

use crate::plan::{FunctionTemplateId, FunctionTemplateSignature, FunctionType, TypeScheme};
use ecow::EcoString;

pub(crate) use parameter::HostParameter;
pub(crate) use return_::HostReturnFamily;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostFunctionTemplate {
    signature: FunctionTemplateSignature,
    package: EcoString,
    site: crate::plan::HostCallSite,
    parameters: Box<[HostParameter]>,
    return_family: HostReturnFamily,
    type_: FunctionType,
}

impl HostFunctionTemplate {
    pub(crate) fn from_signature(
        signature: FunctionTemplateSignature,
        package: EcoString,
        site: crate::plan::HostCallSite,
        parameters: Vec<HostParameter>,
        return_family: HostReturnFamily,
        type_: FunctionType,
    ) -> Self {
        Self {
            signature,
            package,
            site,
            parameters: parameters.into_boxed_slice(),
            return_family,
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

    pub(crate) fn return_family(&self) -> HostReturnFamily {
        self.return_family
    }

    pub(crate) fn signature(&self) -> &FunctionTemplateSignature {
        &self.signature
    }
}
