use super::ValueType;
use crate::plan;
use ecow::EcoString;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct CustomTypeId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct CustomConstructorId {
    type_id: CustomTypeId,
    index: usize,
}

pub(crate) struct CustomTypeTable {
    types: Vec<CustomTypeDescriptor>,
}

pub(crate) struct CustomTypeDescriptor {
    type_: plan::CustomType,
    constructors: BTreeMap<usize, CustomConstructorDescriptor>,
}

pub(crate) struct CustomConstructorDescriptor {
    id: CustomConstructorId,
    name: EcoString,
    fields: Vec<CustomFieldDescriptor>,
}

pub(crate) struct CustomFieldDescriptor {
    label: Option<EcoString>,
    type_: ValueType,
}

impl CustomTypeId {
    pub(in crate::plan::execution) fn new(index: usize) -> Self {
        Self(index)
    }

    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl CustomConstructorId {
    pub(in crate::plan::execution) fn new(type_id: CustomTypeId, index: usize) -> Self {
        Self { type_id, index }
    }

    pub(crate) fn type_id(self) -> CustomTypeId {
        self.type_id
    }

    pub(crate) fn index(self) -> usize {
        self.index
    }
}

impl CustomTypeTable {
    pub(in crate::plan::execution) fn new(types: Vec<CustomTypeDescriptor>) -> Self {
        Self { types }
    }

    pub(crate) fn value_type(&self, id: CustomTypeId) -> plan::CustomType {
        self.types[id.index()].type_.clone()
    }

    pub(crate) fn constructor(&self, id: CustomConstructorId) -> &CustomConstructorDescriptor {
        &self.types[id.type_id().index()].constructors[&id.index()]
    }

    pub(crate) fn constructor_id_for_type(
        &self,
        type_id: CustomTypeId,
        constructor_index: usize,
    ) -> CustomConstructorId {
        self.types[type_id.index()].constructors[&constructor_index].id()
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
    ) -> CustomConstructorId {
        self.types[type_index].constructors[&constructor_index].id()
    }
}

impl CustomTypeDescriptor {
    pub(in crate::plan::execution) fn new(type_: plan::CustomType) -> Self {
        Self {
            type_,
            constructors: BTreeMap::new(),
        }
    }

    pub(in crate::plan::execution) fn insert_constructor(
        &mut self,
        constructor: CustomConstructorDescriptor,
    ) {
        self.constructors
            .insert(constructor.id.index(), constructor);
    }

    pub(in crate::plan::execution) fn has_constructor(&self, index: usize) -> bool {
        self.constructors.contains_key(&index)
    }
}

impl CustomConstructorDescriptor {
    pub(in crate::plan::execution) fn new(
        id: CustomConstructorId,
        name: EcoString,
        fields: Vec<CustomFieldDescriptor>,
    ) -> Self {
        Self { id, name, fields }
    }

    pub(crate) fn id(&self) -> CustomConstructorId {
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
    pub(in crate::plan::execution) fn new(label: Option<EcoString>, type_: ValueType) -> Self {
        Self { label, type_ }
    }

    pub(crate) fn label(&self) -> Option<&EcoString> {
        self.label.as_ref()
    }

    pub(crate) fn type_(&self) -> &ValueType {
        &self.type_
    }
}
