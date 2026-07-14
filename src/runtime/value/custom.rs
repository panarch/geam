use ecow::EcoString;

use super::Value;
use crate::plan::CustomType;

#[derive(Debug, Clone, PartialEq)]
pub struct CustomValue {
    type_: CustomType,
    constructor_name: EcoString,
    constructor_index: usize,
    fields: Vec<CustomFieldValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CustomFieldValue {
    label: Option<EcoString>,
    value: Value,
}

impl CustomValue {
    pub(crate) fn from_evaluated(
        type_: CustomType,
        constructor_name: EcoString,
        constructor_index: usize,
        fields: Vec<CustomFieldValue>,
    ) -> Self {
        Self {
            type_,
            constructor_name,
            constructor_index,
            fields,
        }
    }

    pub fn type_(&self) -> &CustomType {
        &self.type_
    }

    pub fn constructor_name(&self) -> &EcoString {
        &self.constructor_name
    }

    pub fn constructor_index(&self) -> usize {
        self.constructor_index
    }

    pub fn fields(&self) -> &[CustomFieldValue] {
        &self.fields
    }
}

impl CustomFieldValue {
    pub(crate) fn from_evaluated(label: Option<EcoString>, value: Value) -> Self {
        Self { label, value }
    }

    pub fn label(&self) -> Option<&EcoString> {
        self.label.as_ref()
    }

    pub fn value(&self) -> &Value {
        &self.value
    }
}
