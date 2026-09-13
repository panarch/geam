use crate::plan::Text;
use crate::plan::ValueType;
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::Table;
use crate::plan::execution::type_::{CustomConstructorId, ListTypeId, TypeMetadata};
use ecow::EcoString;

#[derive(Clone)]
pub struct NativeConversions {
    pub roots: Table<NativeConversionId>,
    pub nodes: Table<NativeConversion>,
}

#[derive(Clone, Copy)]
pub struct NativeConversionId(pub usize);

#[derive(Clone)]
pub struct NativeConversion {
    pub type_: TypeMetadata,
    pub kind: NativeConversionKind,
}

#[derive(Clone)]
pub enum NativeConversionKind {
    Exact,
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Bool,
    Nil,
    Tuple(Table<NativeConversionId>),
    List {
        storage: ListTypeId,
        item: NativeConversionId,
    },
    Custom(Table<NativeConstructor>),
    External {
        rule: usize,
    },
}

#[derive(Clone)]
pub struct NativeConstructor {
    pub constructor: CustomConstructorId,
    pub tag: Text,
    pub fields: Table<NativeConversionId>,
}

impl Default for NativeConversions {
    fn default() -> Self {
        Self::new(Box::new([]), Box::new([]))
    }
}

impl NativeConversions {
    pub(in crate::plan::execution) fn new(
        roots: Box<[NativeConversionId]>,
        nodes: Box<[NativeConversion]>,
    ) -> Self {
        Self {
            roots: roots.into(),
            nodes: nodes.into(),
        }
    }

    pub(crate) fn root(&self, index: usize) -> NativeConversionId {
        self.roots[index]
    }

    pub(crate) fn get(&self, id: NativeConversionId) -> &NativeConversion {
        &self.nodes[id.0]
    }
}

impl NativeConversionId {
    pub(in crate::plan::execution) fn new(index: usize) -> Self {
        Self(index)
    }
}

impl NativeConversion {
    pub(in crate::plan::execution) fn new(type_: ValueType, kind: NativeConversionKind) -> Self {
        Self {
            type_: TypeMetadata::from_public(&type_),
            kind,
        }
    }

    pub(crate) fn matches_type(&self, type_: &ValueType) -> bool {
        self.type_.compare(type_).is_eq()
    }

    pub(crate) fn kind(&self) -> &NativeConversionKind {
        &self.kind
    }
}

impl NativeConstructor {
    pub(in crate::plan::execution) fn new(
        constructor: CustomConstructorId,
        tag: EcoString,
        fields: Box<[NativeConversionId]>,
    ) -> Self {
        Self {
            constructor,
            tag: tag.into(),
            fields: fields.into(),
        }
    }

    pub(crate) fn constructor(&self) -> CustomConstructorId {
        self.constructor
    }

    pub(crate) fn tag(&self) -> &str {
        &self.tag
    }

    pub(crate) fn fields(&self) -> &[NativeConversionId] {
        &self.fields
    }
}

impl Emit for NativeConversions {
    fn emit(&self, output: &mut Rust) {
        let Self { roots, nodes } = self;
        output.structure(
            "host::NativeConversions",
            &[("roots", roots), ("nodes", nodes)],
        );
    }
}

impl Emit for NativeConversionId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("host::NativeConversionId", &[field_0]);
    }
}

impl Emit for NativeConversion {
    fn emit(&self, output: &mut Rust) {
        let Self { type_, kind } = self;
        output.structure(
            "host::NativeConversion",
            &[("type_", type_), ("kind", kind)],
        );
    }
}

impl Emit for NativeConversionKind {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Exact => output.path("host::NativeConversionKind::Exact"),
            Self::Int => output.path("host::NativeConversionKind::Int"),
            Self::Float => output.path("host::NativeConversionKind::Float"),
            Self::String => output.path("host::NativeConversionKind::String"),
            Self::BitArray => output.path("host::NativeConversionKind::BitArray"),
            Self::UtfCodepoint => output.path("host::NativeConversionKind::UtfCodepoint"),
            Self::Bool => output.path("host::NativeConversionKind::Bool"),
            Self::Nil => output.path("host::NativeConversionKind::Nil"),
            Self::Tuple(field_0) => output.call("host::NativeConversionKind::Tuple", &[field_0]),
            Self::List { storage, item } => output.structure(
                "host::NativeConversionKind::List",
                &[("storage", storage), ("item", item)],
            ),
            Self::Custom(field_0) => output.call("host::NativeConversionKind::Custom", &[field_0]),
            Self::External { rule } => {
                output.structure("host::NativeConversionKind::External", &[("rule", rule)])
            }
        }
    }
}

impl Emit for NativeConstructor {
    fn emit(&self, output: &mut Rust) {
        let Self {
            constructor,
            tag,
            fields,
        } = self;
        output.structure(
            "host::NativeConstructor",
            &[
                ("constructor", constructor),
                ("tag", tag),
                ("fields", fields),
            ],
        );
    }
}

#[cfg(test)]
mod tests {
    use super::{NativeConstructor, NativeConversionId, NativeConversionKind};
    use crate::plan::execution::prepared::rust::Rust;
    use crate::plan::execution::type_::{CustomConstructorId, CustomTypeId, ListTypeId};

    #[test]
    fn emitted_native_kinds_preserve_scalar_and_structural_conversion_rules() {
        let cases = [
            (
                NativeConversionKind::Exact,
                "data::host::NativeConversionKind::Exact",
            ),
            (
                NativeConversionKind::Int,
                "data::host::NativeConversionKind::Int",
            ),
            (
                NativeConversionKind::Float,
                "data::host::NativeConversionKind::Float",
            ),
            (
                NativeConversionKind::String,
                "data::host::NativeConversionKind::String",
            ),
            (
                NativeConversionKind::BitArray,
                "data::host::NativeConversionKind::BitArray",
            ),
            (
                NativeConversionKind::UtfCodepoint,
                "data::host::NativeConversionKind::UtfCodepoint",
            ),
            (
                NativeConversionKind::Bool,
                "data::host::NativeConversionKind::Bool",
            ),
            (
                NativeConversionKind::Nil,
                "data::host::NativeConversionKind::Nil",
            ),
            (
                NativeConversionKind::Tuple(
                    vec![NativeConversionId(2), NativeConversionId(3)].into(),
                ),
                "data::host::NativeConversionKind::Tuple(data::Storage::Static(&[data::host::NativeConversionId(2,),data::host::NativeConversionId(3,),]),)",
            ),
            (
                NativeConversionKind::List {
                    storage: ListTypeId(4),
                    item: NativeConversionId(5),
                },
                "data::host::NativeConversionKind::List {storage: data::type_::ListTypeId(4,),item: data::host::NativeConversionId(5,),}",
            ),
            (
                NativeConversionKind::Custom(
                    vec![NativeConstructor {
                        constructor: CustomConstructorId {
                            type_id: CustomTypeId(6),
                            index: 1,
                        },
                        tag: "item".into(),
                        fields: vec![NativeConversionId(7)].into(),
                    }]
                    .into(),
                ),
                "data::host::NativeConversionKind::Custom(data::Storage::Static(&[data::host::NativeConstructor {constructor: data::type_::CustomConstructorId {type_id: data::type_::CustomTypeId(6,),index: 1,},tag: data::Text::Static(\"item\",),fields: data::Storage::Static(&[data::host::NativeConversionId(7,),]),},]),)",
            ),
            (
                NativeConversionKind::External { rule: 8 },
                "data::host::NativeConversionKind::External {rule: 8,}",
            ),
        ];
        for (kind, expected) in cases {
            assert_eq!(Rust::expression(&kind), expected);
        }
    }
}
