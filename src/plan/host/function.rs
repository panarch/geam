use crate::host::HostParameter;
use crate::plan::{FunctionTemplateId, FunctionTemplateSignature, FunctionType, TypeScheme};
use ecow::EcoString;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostFunctionTemplate {
    signature: FunctionTemplateSignature,
    package: EcoString,
    site: crate::plan::HostCallSite,
    layout: Box<[HostParameter]>,
    custom_schemas: Box<[crate::host::HostCustomTypeSchema]>,
    type_: FunctionType,
}

impl HostFunctionTemplate {
    pub(crate) fn from_signature(
        signature: FunctionTemplateSignature,
        package: EcoString,
        site: crate::plan::HostCallSite,
        layout: Box<[HostParameter]>,
        custom_schemas: Box<[crate::host::HostCustomTypeSchema]>,
        type_: FunctionType,
    ) -> Self {
        Self {
            signature,
            package,
            site,
            layout,
            custom_schemas,
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

    pub(crate) fn layout(&self) -> &[HostParameter] {
        &self.layout
    }

    pub(crate) fn custom_schemas(&self) -> &[crate::host::HostCustomTypeSchema] {
        &self.custom_schemas
    }

    pub fn scheme(&self) -> &TypeScheme {
        self.signature.scheme()
    }

    pub fn type_(&self) -> &FunctionType {
        &self.type_
    }

    pub(crate) fn signature(&self) -> &FunctionTemplateSignature {
        &self.signature
    }
}
