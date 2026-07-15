use super::{CustomTypeId, ValueType};
use crate::plan;
use ecow::EcoString;
use std::collections::BTreeMap;

pub(super) struct CustomTypeTable {
    types: Vec<CustomTypeDescriptor>,
}

pub(super) struct CustomTypeDescriptor {
    type_: plan::CustomType,
    constructors: BTreeMap<usize, CustomConstructorDescriptor>,
}

pub(crate) struct CustomConstructorDescriptor {
    id: super::CustomConstructorId,
    name: EcoString,
    fields: Vec<CustomFieldDescriptor>,
}

pub(crate) struct CustomFieldDescriptor {
    label: Option<EcoString>,
    type_: ValueType,
}

impl CustomTypeTable {
    pub(super) fn new(types: Vec<CustomTypeDescriptor>) -> Self {
        Self { types }
    }

    pub(crate) fn value_type(&self, id: CustomTypeId) -> plan::CustomType {
        self.types[id.index()].type_.clone()
    }

    pub(crate) fn constructor(
        &self,
        id: super::CustomConstructorId,
    ) -> &CustomConstructorDescriptor {
        &self.types[id.type_id().index()].constructors[&id.index()]
    }

    pub(crate) fn constructor_names(&self, id: CustomTypeId) -> Vec<EcoString> {
        self.types[id.index()]
            .constructors
            .values()
            .map(|constructor| constructor.name().clone())
            .collect()
    }

    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.types.len()
    }

    #[cfg(test)]
    pub(crate) fn constructor_id(
        &self,
        type_index: usize,
        constructor_index: usize,
    ) -> super::CustomConstructorId {
        self.types[type_index].constructors[&constructor_index].id()
    }
}

impl CustomTypeDescriptor {
    pub(super) fn new(type_: plan::CustomType) -> Self {
        Self {
            type_,
            constructors: BTreeMap::new(),
        }
    }

    pub(super) fn insert_constructor(&mut self, constructor: CustomConstructorDescriptor) {
        self.constructors
            .insert(constructor.id.index(), constructor);
    }

    pub(super) fn has_constructor(&self, index: usize) -> bool {
        self.constructors.contains_key(&index)
    }
}

impl CustomConstructorDescriptor {
    pub(super) fn new(
        id: super::CustomConstructorId,
        name: EcoString,
        fields: Vec<CustomFieldDescriptor>,
    ) -> Self {
        Self { id, name, fields }
    }

    pub(crate) fn id(&self) -> super::CustomConstructorId {
        self.id
    }

    pub(crate) fn name(&self) -> &EcoString {
        &self.name
    }

    pub(crate) fn fields(&self) -> &[CustomFieldDescriptor] {
        &self.fields
    }
}

impl CustomFieldDescriptor {
    pub(super) fn new(label: Option<EcoString>, type_: ValueType) -> Self {
        Self { label, type_ }
    }

    pub(crate) fn label(&self) -> Option<&EcoString> {
        self.label.as_ref()
    }

    pub(crate) fn type_(&self) -> &ValueType {
        &self.type_
    }
}
