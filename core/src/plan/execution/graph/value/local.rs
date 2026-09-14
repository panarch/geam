use crate::plan::execution::prepared::rust::{Emit, Rust};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FloatLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitArrayLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UtfCodepointLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CustomLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CustomLocal {
    pub id: CustomLocalId,
    pub shape: crate::plan::execution::type_::CustomValueShape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExternalLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExternalLocal {
    pub id: ExternalLocalId,
    pub type_id: crate::plan::execution::type_::ExternalTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoolLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NilLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TupleLocalId(pub usize);

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

impl Emit for IntLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::IntLocalId", &[field_0]);
    }
}

impl Emit for FloatLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::FloatLocalId", &[field_0]);
    }
}

impl Emit for StringLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::StringLocalId", &[field_0]);
    }
}

impl Emit for BitArrayLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::BitArrayLocalId", &[field_0]);
    }
}

impl Emit for UtfCodepointLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::UtfCodepointLocalId", &[field_0]);
    }
}

impl Emit for CustomLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::CustomLocalId", &[field_0]);
    }
}

impl Emit for CustomLocal {
    fn emit(&self, output: &mut Rust) {
        let Self { id, shape } = self;
        output.structure("graph::CustomLocal", &[("id", id), ("shape", shape)]);
    }
}

impl Emit for ExternalLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::ExternalLocalId", &[field_0]);
    }
}

impl Emit for ExternalLocal {
    fn emit(&self, output: &mut Rust) {
        let Self { id, type_id } = self;
        output.structure("graph::ExternalLocal", &[("id", id), ("type_id", type_id)]);
    }
}

impl Emit for BoolLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::BoolLocalId", &[field_0]);
    }
}

impl Emit for NilLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::NilLocalId", &[field_0]);
    }
}

impl Emit for TupleLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::TupleLocalId", &[field_0]);
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BitArrayLocalId, BoolLocalId, CustomLocal, CustomLocalId, ExternalLocal, ExternalLocalId,
        FloatLocalId, IntLocalId, NilLocalId, StringLocalId, TupleLocalId, UtfCodepointLocalId,
    };
    use crate::plan::execution::prepared::rust::Rust;
    use crate::plan::execution::type_::{
        CustomTypeId, CustomValueShape, CustomValueShapeId, ExternalTypeId,
    };

    #[test]
    fn emits_each_local_family_and_preserves_nominal_shapes() {
        assert_eq!(
            Rust::expression(&IntLocalId(2)),
            "data::graph::IntLocalId(2)"
        );
        assert_eq!(
            Rust::expression(&FloatLocalId(3)),
            "data::graph::FloatLocalId(3)"
        );
        assert_eq!(
            Rust::expression(&StringLocalId(4)),
            "data::graph::StringLocalId(4)"
        );
        assert_eq!(
            Rust::expression(&BitArrayLocalId(5)),
            "data::graph::BitArrayLocalId(5)"
        );
        assert_eq!(
            Rust::expression(&UtfCodepointLocalId(6)),
            "data::graph::UtfCodepointLocalId(6)"
        );
        assert_eq!(
            Rust::expression(&CustomLocalId(7)),
            "data::graph::CustomLocalId(7)"
        );
        assert_eq!(
            Rust::expression(&ExternalLocalId(8)),
            "data::graph::ExternalLocalId(8)"
        );
        assert_eq!(
            Rust::expression(&BoolLocalId(9)),
            "data::graph::BoolLocalId(9)"
        );
        assert_eq!(
            Rust::expression(&NilLocalId(10)),
            "data::graph::NilLocalId(10)"
        );
        assert_eq!(
            Rust::expression(&TupleLocalId(11)),
            "data::graph::TupleLocalId(11)"
        );
        assert_eq!(
            Rust::expression(&CustomLocal::new(
                CustomLocalId(3),
                CustomValueShape::new(CustomTypeId(5), CustomValueShapeId(7)),
            )),
            r#"
data::graph::CustomLocal {
    id: data::graph::CustomLocalId(3),
    shape: data::type_::CustomValueShape {
        type_id: data::type_::CustomTypeId(5),
        shape_id: data::type_::CustomValueShapeId(7),
    },
}"#
            .trim_start_matches('\n')
        );
        assert_eq!(
            Rust::expression(&ExternalLocal::new(ExternalLocalId(3), ExternalTypeId(5))),
            r#"
data::graph::ExternalLocal {
    id: data::graph::ExternalLocalId(3),
    type_id: data::type_::ExternalTypeId(5),
}"#
            .trim_start_matches('\n')
        );
    }
}
