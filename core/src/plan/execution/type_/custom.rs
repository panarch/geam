pub(in crate::plan::execution) mod definition;
mod refinement;
pub use definition::{ConstructorDefinition, CustomDefinition, FieldDefinition};
pub use refinement::FieldRefinement;

use super::{NominalTypeMetadata, ValueType};
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::Table;
use crate::plan::{self, Text};
use ecow::EcoString;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CustomTypeId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CustomConstructorId {
    pub type_id: CustomTypeId,
    pub index: usize,
}

#[derive(Clone)]
pub struct CustomTypeTable {
    pub types: Table<CustomTypeDescriptor>,
    pub definitions: Table<CustomDefinition>,
}

#[derive(Clone)]
pub struct CustomTypeDescriptor {
    pub type_: NominalTypeMetadata,
    pub constructor_count: usize,
    pub constructors: Table<CustomConstructorDescriptor>,
}

#[derive(Clone)]
pub struct CustomConstructorDescriptor {
    pub id: CustomConstructorId,
    pub name: Text,
    pub native_tag: Text,
    pub fields: Table<CustomFieldDescriptor>,
}

#[derive(Clone)]
pub struct CustomFieldDescriptor {
    pub label: Option<Text>,
    pub type_: ValueType,
    pub shape: super::ValueShapeId,
    pub refinement: FieldRefinement,
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
    pub(crate) fn native_constructor_tags(&self) -> impl Iterator<Item = &str> {
        self.types.iter().flat_map(|type_| {
            type_
                .constructors
                .iter()
                .map(|constructor| constructor.native_tag.as_str())
        })
    }

    pub(in crate::plan::execution) fn new(
        types: Vec<CustomTypeDescriptor>,
        mut definitions: Vec<CustomDefinition>,
    ) -> Self {
        definitions.sort_unstable_by(|left, right| left.identity().cmp(&right.identity()));
        Self {
            types: types.into(),
            definitions: definitions.into(),
        }
    }

    pub(crate) fn value_type(&self, id: CustomTypeId) -> plan::CustomType {
        self.types[id.index()].type_.custom_type()
    }

    pub(crate) fn constructor(&self, id: CustomConstructorId) -> &CustomConstructorDescriptor {
        self.types[id.type_id().index()].constructor(id.index())
    }

    pub(crate) fn constructor_id_for_type(
        &self,
        type_id: CustomTypeId,
        constructor_index: usize,
    ) -> CustomConstructorId {
        self.types[type_id.index()]
            .constructor(constructor_index)
            .id()
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
        self.types[type_index].constructor(constructor_index).id()
    }
}

impl CustomTypeDescriptor {
    pub(in crate::plan::execution) fn new(
        constructor_count: usize,
        type_: plan::CustomType,
        constructors: Vec<CustomConstructorDescriptor>,
    ) -> Self {
        Self {
            type_: NominalTypeMetadata::from_custom(&type_),
            constructor_count,
            constructors: constructors.into(),
        }
    }

    fn constructor(&self, index: usize) -> &CustomConstructorDescriptor {
        // Executable constructor uses refer to a checked row in this sparse table.
        let position = self
            .constructors
            .partition_point(|constructor| constructor.id.index() < index);
        &self.constructors[position]
    }
}

impl CustomConstructorDescriptor {
    pub(in crate::plan::execution) fn new(
        id: CustomConstructorId,
        name: EcoString,
        fields: Vec<CustomFieldDescriptor>,
    ) -> Self {
        Self {
            id,
            native_tag: gleam_compiler_core::strings::to_snake_case(&name).into(),
            name: name.into(),
            fields: fields.into(),
        }
    }

    pub(crate) fn id(&self) -> CustomConstructorId {
        self.id
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn native_tag(&self) -> &str {
        &self.native_tag
    }

    pub(crate) fn fields(&self) -> &[CustomFieldDescriptor] {
        &self.fields
    }
}

impl CustomFieldDescriptor {
    pub(in crate::plan::execution) fn new(
        label: Option<EcoString>,
        type_: ValueType,
        shape: super::ValueShapeId,
        refinement: FieldRefinement,
    ) -> Self {
        Self {
            label: label.map(Text::from),
            type_,
            shape,
            refinement,
        }
    }

    pub(crate) fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    pub(crate) fn type_(&self) -> &ValueType {
        &self.type_
    }
}

impl Emit for CustomTypeId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("type_::CustomTypeId", &[field_0]);
    }
}

impl Emit for CustomConstructorId {
    fn emit(&self, output: &mut Rust) {
        let Self { type_id, index } = self;
        output.structure(
            "type_::CustomConstructorId",
            &[("type_id", type_id), ("index", index)],
        );
    }
}

impl Emit for CustomTypeTable {
    fn emit(&self, output: &mut Rust) {
        let Self { types, definitions } = self;
        output.structure(
            "type_::CustomTypeTable",
            &[("types", types), ("definitions", definitions)],
        );
    }
}

impl Emit for CustomTypeDescriptor {
    fn emit(&self, output: &mut Rust) {
        let Self {
            type_,
            constructor_count,
            constructors,
        } = self;
        output.structure(
            "type_::CustomTypeDescriptor",
            &[
                ("type_", type_),
                ("constructor_count", constructor_count),
                ("constructors", constructors),
            ],
        );
    }
}

impl Emit for CustomConstructorDescriptor {
    fn emit(&self, output: &mut Rust) {
        let Self {
            id,
            name,
            native_tag,
            fields,
        } = self;
        output.structure(
            "type_::CustomConstructorDescriptor",
            &[
                ("id", id),
                ("name", name),
                ("native_tag", native_tag),
                ("fields", fields),
            ],
        );
    }
}

impl Emit for CustomFieldDescriptor {
    fn emit(&self, output: &mut Rust) {
        let Self {
            label,
            type_,
            shape,
            refinement,
        } = self;
        output.structure(
            "type_::CustomFieldDescriptor",
            &[
                ("label", label),
                ("type_", type_),
                ("shape", shape),
                ("refinement", refinement),
            ],
        );
    }
}
