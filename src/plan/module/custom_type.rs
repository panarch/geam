use crate::plan::{CustomTypeName, ValueType};
use ecow::EcoString;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CustomTypePublicity {
    Public,
    Private,
    Internal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CustomTypeParameterId(pub(crate) usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomTypeDefinition {
    name: CustomTypeName,
    publicity: CustomTypePublicity,
    opaque: bool,
    parameters: Vec<CustomTypeParameterId>,
    constructors: Vec<CustomConstructorDefinition>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomConstructorDefinition {
    name: EcoString,
    index: usize,
    fields: Vec<CustomFieldDefinition>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomFieldDefinition {
    label: Option<EcoString>,
    type_: CustomTypeTemplate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CustomTypeTemplate {
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Bool,
    Nil,
    Tuple(Vec<CustomTypeTemplate>),
    List(Box<CustomTypeTemplate>),
    Function {
        arguments: Vec<CustomTypeTemplate>,
        return_: Box<CustomTypeTemplate>,
    },
    Custom {
        name: CustomTypeName,
        arguments: Vec<CustomTypeTemplate>,
    },
    External {
        name: crate::plan::ExternalTypeName,
        arguments: Vec<CustomTypeTemplate>,
    },
    Parameter(CustomTypeParameterId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CustomConstructor {
    type_: crate::plan::CustomType,
    name: EcoString,
    index: usize,
    fields: Vec<CustomConstructorField>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CustomConstructorField {
    label: Option<EcoString>,
    type_: ValueType,
}

impl CustomTypeDefinition {
    pub(crate) fn new(
        name: CustomTypeName,
        publicity: CustomTypePublicity,
        opaque: bool,
        parameters: Vec<CustomTypeParameterId>,
        constructors: Vec<CustomConstructorDefinition>,
    ) -> Self {
        Self {
            name,
            publicity,
            opaque,
            parameters,
            constructors,
        }
    }

    pub fn name(&self) -> &CustomTypeName {
        &self.name
    }

    pub fn publicity(&self) -> CustomTypePublicity {
        self.publicity
    }

    pub fn is_opaque(&self) -> bool {
        self.opaque
    }

    pub fn parameters(&self) -> &[CustomTypeParameterId] {
        &self.parameters
    }

    pub fn constructors(&self) -> &[CustomConstructorDefinition] {
        &self.constructors
    }

    pub(crate) fn constructor(&self, index: usize) -> Option<&CustomConstructorDefinition> {
        self.constructors.get(index)
    }
}

impl CustomConstructorDefinition {
    pub(crate) fn new(name: EcoString, index: usize, fields: Vec<CustomFieldDefinition>) -> Self {
        Self {
            name,
            index,
            fields,
        }
    }

    pub fn name(&self) -> &EcoString {
        &self.name
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn fields(&self) -> &[CustomFieldDefinition] {
        &self.fields
    }
}

impl CustomFieldDefinition {
    pub(crate) fn new(label: Option<EcoString>, type_: CustomTypeTemplate) -> Self {
        Self { label, type_ }
    }

    pub fn label(&self) -> Option<&EcoString> {
        self.label.as_ref()
    }

    pub fn type_(&self) -> &CustomTypeTemplate {
        &self.type_
    }
}

impl CustomConstructor {
    pub(crate) fn new(
        type_: crate::plan::CustomType,
        name: EcoString,
        index: usize,
        fields: Vec<CustomConstructorField>,
    ) -> Self {
        Self {
            type_,
            name,
            index,
            fields,
        }
    }

    pub(crate) fn type_(&self) -> &crate::plan::CustomType {
        &self.type_
    }

    pub(crate) fn index(&self) -> usize {
        self.index
    }

    pub(crate) fn fields(&self) -> &[CustomConstructorField] {
        &self.fields
    }

    pub(crate) fn substitute(&self, substitution: &crate::plan::TypeSubstitution) -> Self {
        Self {
            type_: crate::plan::CustomType::new(
                self.type_.type_name().clone(),
                self.type_
                    .arguments()
                    .iter()
                    .cloned()
                    .map(crate::plan::ValueShape::from_value_type)
                    .map(|shape| shape.substitute(substitution).value_type())
                    .collect(),
            ),
            name: self.name.clone(),
            index: self.index,
            fields: self
                .fields
                .iter()
                .map(|field| CustomConstructorField {
                    label: field.label.clone(),
                    type_: crate::plan::ValueShape::from_value_type(field.type_.clone())
                        .substitute(substitution)
                        .value_type(),
                })
                .collect(),
        }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        crate::plan::CustomType,
        EcoString,
        usize,
        Vec<CustomConstructorField>,
    ) {
        (self.type_, self.name, self.index, self.fields)
    }
}

impl CustomConstructorField {
    pub(crate) fn new(label: Option<EcoString>, type_: ValueType) -> Self {
        Self { label, type_ }
    }

    pub(crate) fn label(&self) -> Option<&EcoString> {
        self.label.as_ref()
    }

    pub(crate) fn type_(&self) -> &ValueType {
        &self.type_
    }

    pub(crate) fn into_parts(self) -> (Option<EcoString>, ValueType) {
        (self.label, self.type_)
    }
}
