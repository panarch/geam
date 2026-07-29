use crate::host::HostParameter;
use crate::plan::{FunctionTemplateId, FunctionTemplateSignature, FunctionType, TypeScheme};
use ecow::EcoString;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostFunctionTemplate {
    signature: FunctionTemplateSignature,
    package: EcoString,
    site: crate::plan::HostCallSite,
    layout: Box<[HostParameter]>,
    parameters: Box<[crate::host::HostTypeDescriptor]>,
    return_: crate::host::HostTypeDescriptor,
    custom_schemas: Box<[crate::host::HostCustomTypeSchema]>,
    type_: FunctionType,
}

impl HostFunctionTemplate {
    pub(crate) fn from_schema(
        signature: FunctionTemplateSignature,
        package: EcoString,
        site: crate::plan::HostCallSite,
        schema: crate::host::HostFunctionSchema,
    ) -> Self {
        Self {
            signature,
            package,
            site,
            layout: schema.layout().to_vec().into_boxed_slice(),
            parameters: schema.parameters().to_vec().into_boxed_slice(),
            return_: schema.return_type().clone(),
            custom_schemas: schema.custom_schemas().to_vec().into_boxed_slice(),
            type_: schema.type_().clone(),
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

    pub(crate) fn parameters(&self) -> &[crate::host::HostTypeDescriptor] {
        &self.parameters
    }

    pub(crate) fn return_type(&self) -> &crate::host::HostTypeDescriptor {
        &self.return_
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
