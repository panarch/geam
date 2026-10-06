pub(in crate::plan::execution) mod views;

use crate::plan::Text;
use crate::plan::ValueType;
use crate::plan::execution::function::RuntimeFunctionId;
use crate::plan::execution::graph::ParamSlot;
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::Table;
use crate::plan::execution::type_::{
    CustomConstructorId, FunctionMetadata, FunctionType, ListTypeId, TypeMetadata,
};
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
    Function(Table<NativeFunctionView>),
    CustomView(Table<NativeCustomView>),
    External {
        rule: usize,
    },
}

#[derive(Clone)]
pub struct NativeFunctionView {
    pub source: FunctionMetadata,
    pub target: RuntimeFunctionId,
    pub type_: FunctionType,
    pub captures: Table<ParamSlot>,
    pub host: usize,
    pub host_value: bool,
}

#[derive(Clone)]
pub struct NativeCustomView {
    pub source: TypeMetadata,
    pub constructors: Table<NativeConstructor>,
}

#[derive(Clone)]
pub struct HostNativeView {
    pub parent: usize,
    pub parent_value: bool,
    pub source: FunctionMetadata,
    pub arguments: Table<NativeConversionId>,
    pub return_: NativeConversionId,
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
            Self::Function(field_0) => {
                output.call("host::NativeConversionKind::Function", &[field_0])
            }
            Self::CustomView(field_0) => {
                output.call("host::NativeConversionKind::CustomView", &[field_0])
            }
            Self::External { rule } => {
                output.structure("host::NativeConversionKind::External", &[("rule", rule)])
            }
        }
    }
}

impl Emit for NativeFunctionView {
    fn emit(&self, output: &mut Rust) {
        let Self {
            source,
            target,
            type_,
            captures,
            host,
            host_value,
        } = self;
        output.structure(
            "host::NativeFunctionView",
            &[
                ("source", source),
                ("target", target),
                ("type_", type_),
                ("captures", captures),
                ("host", host),
                ("host_value", host_value),
            ],
        );
    }
}

impl Emit for NativeCustomView {
    fn emit(&self, output: &mut Rust) {
        let Self {
            source,
            constructors,
        } = self;
        output.structure(
            "host::NativeCustomView",
            &[("source", source), ("constructors", constructors)],
        );
    }
}

impl Emit for HostNativeView {
    fn emit(&self, output: &mut Rust) {
        let Self {
            parent,
            parent_value,
            source,
            arguments,
            return_,
        } = self;
        output.structure(
            "host::HostNativeView",
            &[
                ("parent", parent),
                ("source", source),
                ("parent_value", parent_value),
                ("arguments", arguments),
                ("return_", return_),
            ],
        );
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
    use super::{
        HostNativeView, NativeConstructor, NativeConversionId, NativeConversionKind,
        NativeCustomView, NativeFunctionView,
    };
    use crate::plan::execution::function::{
        CoreRuntimeFunctionId, IntFunctionId, RuntimeFunctionId,
    };
    use crate::plan::execution::graph::{IntFunctionLocalId, ParamLocal, ParamSlot};
    use crate::plan::execution::prepared::rust::Rust;
    use crate::plan::execution::type_::{CustomConstructorId, CustomTypeId, ListTypeId};
    use crate::plan::execution::type_::{
        FunctionMetadata, FunctionType, NominalTypeMetadata, TypeMetadata, ValueShapeId, ValueType,
    };

    #[test]
    fn emitted_views_preserve_source_adapter_capture_codec_and_custom_fields() {
        let source = FunctionMetadata {
            arguments: vec![TypeMetadata::BitArray].into(),
            return_: Box::new(TypeMetadata::Int).into(),
        };
        let view = NativeFunctionView {
            source: source.clone(),
            target: RuntimeFunctionId::Core(CoreRuntimeFunctionId::Int(IntFunctionId(7))),
            type_: FunctionType::new(vec![ValueType::String], ValueType::Int),
            captures: vec![ParamSlot {
                local: ParamLocal::IntFunction {
                    local: IntFunctionLocalId(0),
                    type_: FunctionType::new(vec![ValueType::BitArray], ValueType::Int),
                },
                shape: ValueShapeId(2),
            }]
            .into(),
            host: 3,
            host_value: true,
        };
        assert_eq!(
            Rust::expression(&view),
            r#"data::host::NativeFunctionView {
    source: data::type_::FunctionMetadata {
        arguments: data::Storage::Static(&[
            data::type_::TypeMetadata::BitArray,
        ]),
        return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
    },
    target: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Int(data::function::IntFunctionId(7))),
    type_: data::type_::FunctionType {
        arguments: data::Storage::Static(&[
            data::type_::ValueType::String,
        ]),
        return_: data::Storage::Static(&data::type_::ValueType::Int),
    },
    captures: data::Storage::Static(&[
        data::graph::ParamSlot {
            local: data::graph::ParamLocal::IntFunction {
                local: data::graph::IntFunctionLocalId(0),
                type_: data::type_::FunctionType {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueType::BitArray,
                    ]),
                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                },
            },
            shape: data::type_::ValueShapeId(2),
        },
    ]),
    host: 3,
    host_value: true,
}"#
        );
        let codec = HostNativeView {
            parent: 2,
            parent_value: false,
            source,
            arguments: vec![NativeConversionId(1)].into(),
            return_: NativeConversionId(4),
        };
        assert_eq!(
            Rust::expression(&codec),
            r#"data::host::HostNativeView {
    parent: 2,
    source: data::type_::FunctionMetadata {
        arguments: data::Storage::Static(&[
            data::type_::TypeMetadata::BitArray,
        ]),
        return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
    },
    parent_value: false,
    arguments: data::Storage::Static(&[
        data::host::NativeConversionId(1),
    ]),
    return_: data::host::NativeConversionId(4),
}"#
        );
        let custom = NativeCustomView {
            source: TypeMetadata::Custom(NominalTypeMetadata {
                package: "app".into(),
                module: "main".into(),
                name: "Box".into(),
                arguments: vec![TypeMetadata::Int].into(),
            }),
            constructors: vec![NativeConstructor {
                constructor: CustomConstructorId {
                    type_id: CustomTypeId(2),
                    index: 0,
                },
                tag: "box".into(),
                fields: vec![NativeConversionId(3)].into(),
            }]
            .into(),
        };
        assert_eq!(
            Rust::expression(&custom),
            r#"data::host::NativeCustomView {
    source: data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
        package: data::Text::Static("app"),
        module: data::Text::Static("main"),
        name: data::Text::Static("Box"),
        arguments: data::Storage::Static(&[
            data::type_::TypeMetadata::Int,
        ]),
    }),
    constructors: data::Storage::Static(&[
        data::host::NativeConstructor {
            constructor: data::type_::CustomConstructorId {
                type_id: data::type_::CustomTypeId(2),
                index: 0,
            },
            tag: data::Text::Static("box"),
            fields: data::Storage::Static(&[
                data::host::NativeConversionId(3),
            ]),
        },
    ]),
}"#
        );
    }

    #[test]
    fn emitted_native_kinds_preserve_scalar_and_structural_conversion_rules() {
        let cases = [
            (
                NativeConversionKind::Function(Vec::new().into()),
                "data::host::NativeConversionKind::Function(data::Storage::Static(&[]))",
            ),
            (
                NativeConversionKind::CustomView(Vec::new().into()),
                "data::host::NativeConversionKind::CustomView(data::Storage::Static(&[]))",
            ),
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
                r#"
data::host::NativeConversionKind::Tuple(data::Storage::Static(&[
    data::host::NativeConversionId(2),
    data::host::NativeConversionId(3),
]))"#
                    .trim_start_matches('\n'),
            ),
            (
                NativeConversionKind::List {
                    storage: ListTypeId(4),
                    item: NativeConversionId(5),
                },
                r#"
data::host::NativeConversionKind::List {
    storage: data::type_::ListTypeId(4),
    item: data::host::NativeConversionId(5),
}"#
                .trim_start_matches('\n'),
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
                r#"
data::host::NativeConversionKind::Custom(data::Storage::Static(&[
    data::host::NativeConstructor {
        constructor: data::type_::CustomConstructorId {
            type_id: data::type_::CustomTypeId(6),
            index: 1,
        },
        tag: data::Text::Static("item"),
        fields: data::Storage::Static(&[
            data::host::NativeConversionId(7),
        ]),
    },
]))"#
                    .trim_start_matches('\n'),
            ),
            (
                NativeConversionKind::External { rule: 8 },
                r#"
data::host::NativeConversionKind::External {
    rule: 8,
}"#
                .trim_start_matches('\n'),
            ),
        ];
        for (kind, expected) in cases {
            assert_eq!(Rust::expression(&kind), expected);
        }
    }
}
