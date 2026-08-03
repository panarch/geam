#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FloatLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitArrayLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UtfCodepointLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CustomLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct CustomLocal {
    id: CustomLocalId,
    shape: crate::plan::execution::type_::CustomValueShape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExternalLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ExternalLocal {
    id: ExternalLocalId,
    type_id: crate::plan::execution::type_::ExternalTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoolLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NilLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TupleLocalId(pub(crate) usize);

impl CustomLocal {
    pub(in crate::plan::execution) fn new(
        id: CustomLocalId,
        shape: crate::plan::execution::type_::CustomValueShape,
    ) -> Self {
        Self { id, shape }
    }

    pub(crate) fn id(self) -> CustomLocalId {
        self.id
    }
}

impl ExternalLocal {
    pub(in crate::plan::execution) fn new(
        id: ExternalLocalId,
        type_id: crate::plan::execution::type_::ExternalTypeId,
    ) -> Self {
        Self { id, type_id }
    }

    pub(crate) fn id(self) -> ExternalLocalId {
        self.id
    }
}
