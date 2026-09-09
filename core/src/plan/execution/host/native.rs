use crate::plan::ValueType;
use crate::plan::execution::type_::{CustomConstructorId, ListTypeId};
use ecow::EcoString;

#[derive(Clone, Default)]
pub(crate) struct NativeConversions {
    roots: Box<[NativeConversionId]>,
    nodes: Box<[NativeConversion]>,
}

#[derive(Clone, Copy)]
pub(crate) struct NativeConversionId(usize);

#[derive(Clone)]
pub(crate) struct NativeConversion {
    type_: ValueType,
    kind: NativeConversionKind,
}

#[derive(Clone)]
pub(crate) enum NativeConversionKind {
    Exact,
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Bool,
    Nil,
    Tuple(Box<[NativeConversionId]>),
    List {
        storage: ListTypeId,
        item: NativeConversionId,
    },
    Custom(Box<[NativeConstructor]>),
    External {
        rule: usize,
    },
}

#[derive(Clone)]
pub(crate) struct NativeConstructor {
    constructor: CustomConstructorId,
    tag: EcoString,
    fields: Box<[NativeConversionId]>,
}

impl NativeConversions {
    pub(in crate::plan::execution) fn new(
        roots: Box<[NativeConversionId]>,
        nodes: Box<[NativeConversion]>,
    ) -> Self {
        Self { roots, nodes }
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
        Self { type_, kind }
    }

    pub(crate) fn type_(&self) -> &ValueType {
        &self.type_
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
            tag,
            fields,
        }
    }

    pub(crate) fn constructor(&self) -> CustomConstructorId {
        self.constructor
    }

    pub(crate) fn tag(&self) -> &EcoString {
        &self.tag
    }

    pub(crate) fn fields(&self) -> &[NativeConversionId] {
        &self.fields
    }
}
