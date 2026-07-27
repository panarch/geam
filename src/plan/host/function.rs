mod parameter;
mod return_;

use crate::plan::{
    FunctionShape, FunctionTemplateId, FunctionTemplateSignature, FunctionType, TypeScheme,
};
use ecow::EcoString;

pub(crate) use parameter::HostParameter;
pub(crate) use return_::HostReturnFamily;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostFunctionTemplate {
    signature: FunctionTemplateSignature,
    package: EcoString,
    module: EcoString,
    name: EcoString,
    parameters: Box<[HostParameter]>,
    return_family: HostReturnFamily,
    type_: FunctionType,
}

impl HostFunctionTemplate {
    pub(crate) fn new(
        id: FunctionTemplateId,
        package: EcoString,
        module: EcoString,
        name: EcoString,
        parameters: Vec<HostParameter>,
        return_family: HostReturnFamily,
        type_: FunctionType,
    ) -> Self {
        Self {
            signature: FunctionTemplateSignature::new(
                id,
                TypeScheme::new(0),
                FunctionShape::new(
                    parameters
                        .iter()
                        .map(|parameter| parameter.shape())
                        .collect(),
                    return_family.shape(),
                ),
            ),
            package,
            module,
            name,
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
        &self.module
    }

    pub fn name(&self) -> &EcoString {
        &self.name
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
