use crate::plan::execution::prepared::rust::{Emit, Rust};
#[cfg(test)]
use crate::plan::execution::type_::ListTypeId;
use crate::plan::execution::type_::{
    BitArrayListTypeId, BoolListTypeId, CustomListTypeId, ExternalListTypeId, FloatListTypeId,
    FunctionListTypeId, IntListTypeId, ListListTypeId, NilListTypeId, ParameterListListTypeId,
    ParameterListTypeId, StringListTypeId, TupleListTypeId, UtfCodepointListTypeId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntListLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringListLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitArrayListLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UtfCodepointListLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParameterListLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CustomListLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExternalListLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FloatListLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoolListLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NilListLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TupleListLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ListListLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParameterListListLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionListLocalId(pub usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListLocal {
    Parameter {
        local: ParameterListLocalId,
        type_id: ParameterListTypeId,
    },
    ParameterList {
        local: ParameterListListLocalId,
        type_id: ParameterListListTypeId,
    },
    Int {
        local: IntListLocalId,
        type_id: IntListTypeId,
    },
    String {
        local: StringListLocalId,
        type_id: StringListTypeId,
    },
    BitArray {
        local: BitArrayListLocalId,
        type_id: BitArrayListTypeId,
    },
    UtfCodepoint {
        local: UtfCodepointListLocalId,
        type_id: UtfCodepointListTypeId,
    },
    Custom {
        local: CustomListLocalId,
        type_id: CustomListTypeId,
    },
    External {
        local: ExternalListLocalId,
        type_id: ExternalListTypeId,
    },
    Float {
        local: FloatListLocalId,
        type_id: FloatListTypeId,
    },
    Bool {
        local: BoolListLocalId,
        type_id: BoolListTypeId,
    },
    Nil {
        local: NilListLocalId,
        type_id: NilListTypeId,
    },
    Tuple {
        local: TupleListLocalId,
        type_id: TupleListTypeId,
    },
    List {
        local: ListListLocalId,
        type_id: ListListTypeId,
    },
    Function {
        local: FunctionListLocalId,
        type_id: FunctionListTypeId,
    },
}

#[derive(Clone)]
pub enum StoredListLocal {
    ParameterList(ParameterListListLocalId),
    Int(IntListLocalId),
    String(StringListLocalId),
    BitArray(BitArrayListLocalId),
    UtfCodepoint(UtfCodepointListLocalId),
    Custom(CustomListLocalId),
    External(ExternalListLocalId),
    Float(FloatListLocalId),
    Bool(BoolListLocalId),
    Nil(NilListLocalId),
    Tuple(TupleListLocalId),
    List(ListListLocalId),
    Function(FunctionListLocalId),
}

#[cfg(test)]
impl ListLocal {
    pub(crate) fn list_type(&self) -> ListTypeId {
        match self {
            Self::Parameter { type_id, .. } => type_id.list_type(),
            Self::ParameterList { type_id, .. } => type_id.list_type(),
            Self::Int { type_id, .. } => type_id.list_type(),
            Self::String { type_id, .. } => type_id.list_type(),
            Self::BitArray { type_id, .. } => type_id.list_type(),
            Self::UtfCodepoint { type_id, .. } => type_id.list_type(),
            Self::Custom { type_id, .. } => type_id.list_type(),
            Self::External { type_id, .. } => type_id.list_type(),
            Self::Float { type_id, .. } => type_id.list_type(),
            Self::Bool { type_id, .. } => type_id.list_type(),
            Self::Nil { type_id, .. } => type_id.list_type(),
            Self::Tuple { type_id, .. } => type_id.list_type(),
            Self::List { type_id, .. } => type_id.list_type(),
            Self::Function { type_id, .. } => type_id.list_type(),
        }
    }
}

impl Emit for IntListLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::IntListLocalId", &[field_0]);
    }
}

impl Emit for StringListLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::StringListLocalId", &[field_0]);
    }
}

impl Emit for BitArrayListLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::BitArrayListLocalId", &[field_0]);
    }
}

impl Emit for UtfCodepointListLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::UtfCodepointListLocalId", &[field_0]);
    }
}

impl Emit for ParameterListLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::ParameterListLocalId", &[field_0]);
    }
}

impl Emit for CustomListLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::CustomListLocalId", &[field_0]);
    }
}

impl Emit for ExternalListLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::ExternalListLocalId", &[field_0]);
    }
}

impl Emit for FloatListLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::FloatListLocalId", &[field_0]);
    }
}

impl Emit for BoolListLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::BoolListLocalId", &[field_0]);
    }
}

impl Emit for NilListLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::NilListLocalId", &[field_0]);
    }
}

impl Emit for TupleListLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::TupleListLocalId", &[field_0]);
    }
}

impl Emit for ListListLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::ListListLocalId", &[field_0]);
    }
}

impl Emit for ParameterListListLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::ParameterListListLocalId", &[field_0]);
    }
}

impl Emit for FunctionListLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::FunctionListLocalId", &[field_0]);
    }
}

impl Emit for ListLocal {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Parameter { local, type_id } => output.structure(
                "graph::ListLocal::Parameter",
                &[("local", local), ("type_id", type_id)],
            ),
            Self::ParameterList { local, type_id } => output.structure(
                "graph::ListLocal::ParameterList",
                &[("local", local), ("type_id", type_id)],
            ),
            Self::Int { local, type_id } => output.structure(
                "graph::ListLocal::Int",
                &[("local", local), ("type_id", type_id)],
            ),
            Self::String { local, type_id } => output.structure(
                "graph::ListLocal::String",
                &[("local", local), ("type_id", type_id)],
            ),
            Self::BitArray { local, type_id } => output.structure(
                "graph::ListLocal::BitArray",
                &[("local", local), ("type_id", type_id)],
            ),
            Self::UtfCodepoint { local, type_id } => output.structure(
                "graph::ListLocal::UtfCodepoint",
                &[("local", local), ("type_id", type_id)],
            ),
            Self::Custom { local, type_id } => output.structure(
                "graph::ListLocal::Custom",
                &[("local", local), ("type_id", type_id)],
            ),
            Self::External { local, type_id } => output.structure(
                "graph::ListLocal::External",
                &[("local", local), ("type_id", type_id)],
            ),
            Self::Float { local, type_id } => output.structure(
                "graph::ListLocal::Float",
                &[("local", local), ("type_id", type_id)],
            ),
            Self::Bool { local, type_id } => output.structure(
                "graph::ListLocal::Bool",
                &[("local", local), ("type_id", type_id)],
            ),
            Self::Nil { local, type_id } => output.structure(
                "graph::ListLocal::Nil",
                &[("local", local), ("type_id", type_id)],
            ),
            Self::Tuple { local, type_id } => output.structure(
                "graph::ListLocal::Tuple",
                &[("local", local), ("type_id", type_id)],
            ),
            Self::List { local, type_id } => output.structure(
                "graph::ListLocal::List",
                &[("local", local), ("type_id", type_id)],
            ),
            Self::Function { local, type_id } => output.structure(
                "graph::ListLocal::Function",
                &[("local", local), ("type_id", type_id)],
            ),
        }
    }
}

impl Emit for StoredListLocal {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::ParameterList(field_0) => {
                output.call("graph::StoredListLocal::ParameterList", &[field_0])
            }
            Self::Int(field_0) => output.call("graph::StoredListLocal::Int", &[field_0]),
            Self::String(field_0) => output.call("graph::StoredListLocal::String", &[field_0]),
            Self::BitArray(field_0) => output.call("graph::StoredListLocal::BitArray", &[field_0]),
            Self::UtfCodepoint(field_0) => {
                output.call("graph::StoredListLocal::UtfCodepoint", &[field_0])
            }
            Self::Custom(field_0) => output.call("graph::StoredListLocal::Custom", &[field_0]),
            Self::External(field_0) => output.call("graph::StoredListLocal::External", &[field_0]),
            Self::Float(field_0) => output.call("graph::StoredListLocal::Float", &[field_0]),
            Self::Bool(field_0) => output.call("graph::StoredListLocal::Bool", &[field_0]),
            Self::Nil(field_0) => output.call("graph::StoredListLocal::Nil", &[field_0]),
            Self::Tuple(field_0) => output.call("graph::StoredListLocal::Tuple", &[field_0]),
            Self::List(field_0) => output.call("graph::StoredListLocal::List", &[field_0]),
            Self::Function(field_0) => output.call("graph::StoredListLocal::Function", &[field_0]),
        }
    }
}

#[cfg(test)]
mod emission_tests {
    use super::{
        BitArrayListLocalId, BoolListLocalId, CustomListLocalId, ExternalListLocalId,
        FloatListLocalId, FunctionListLocalId, IntListLocalId, ListListLocalId, ListLocal,
        NilListLocalId, ParameterListListLocalId, ParameterListLocalId, StoredListLocal,
        StringListLocalId, TupleListLocalId, UtfCodepointListLocalId,
    };
    use crate::plan::execution::prepared::rust::Rust;
    use crate::plan::execution::type_::list::{FunctionItemTypeId, TupleItemTypeId};
    use crate::plan::execution::type_::{
        BitArrayListTypeId, BoolListTypeId, CustomListTypeId, CustomTypeId, ExternalListTypeId,
        ExternalTypeId, FloatListTypeId, FunctionListTypeId, IntListTypeId, ListListTypeId,
        ListTypeId, NilListTypeId, ParameterListListTypeId, ParameterListTypeId, StringListTypeId,
        TupleListTypeId, UtfCodepointListTypeId,
    };

    #[test]
    fn emits_list_locals_with_item_metadata_and_distinct_storage_families() {
        let cases = [
            (
                ListLocal::Int {
                    local: IntListLocalId(2),
                    type_id: IntListTypeId::new(ListTypeId(3)),
                },
                r#"
data::graph::ListLocal::Int {
    local: data::graph::IntListLocalId(2),
    type_id: data::type_::IntListTypeId {
        list_type: data::type_::ListTypeId(3),
    },
}"#
                .trim_start_matches('\n'),
            ),
            (
                ListLocal::String {
                    local: StringListLocalId(2),
                    type_id: StringListTypeId::new(ListTypeId(3)),
                },
                r#"
data::graph::ListLocal::String {
    local: data::graph::StringListLocalId(2),
    type_id: data::type_::StringListTypeId {
        list_type: data::type_::ListTypeId(3),
    },
}"#
                .trim_start_matches('\n'),
            ),
            (
                ListLocal::BitArray {
                    local: BitArrayListLocalId(2),
                    type_id: BitArrayListTypeId::new(ListTypeId(3)),
                },
                r#"
data::graph::ListLocal::BitArray {
    local: data::graph::BitArrayListLocalId(2),
    type_id: data::type_::BitArrayListTypeId {
        list_type: data::type_::ListTypeId(3),
    },
}"#
                .trim_start_matches('\n'),
            ),
            (
                ListLocal::UtfCodepoint {
                    local: UtfCodepointListLocalId(2),
                    type_id: UtfCodepointListTypeId::new(ListTypeId(3)),
                },
                r#"
data::graph::ListLocal::UtfCodepoint {
    local: data::graph::UtfCodepointListLocalId(2),
    type_id: data::type_::UtfCodepointListTypeId {
        list_type: data::type_::ListTypeId(3),
    },
}"#
                .trim_start_matches('\n'),
            ),
            (
                ListLocal::Float {
                    local: FloatListLocalId(2),
                    type_id: FloatListTypeId::new(ListTypeId(3)),
                },
                r#"
data::graph::ListLocal::Float {
    local: data::graph::FloatListLocalId(2),
    type_id: data::type_::FloatListTypeId {
        list_type: data::type_::ListTypeId(3),
    },
}"#
                .trim_start_matches('\n'),
            ),
            (
                ListLocal::Bool {
                    local: BoolListLocalId(2),
                    type_id: BoolListTypeId::new(ListTypeId(3)),
                },
                r#"
data::graph::ListLocal::Bool {
    local: data::graph::BoolListLocalId(2),
    type_id: data::type_::BoolListTypeId {
        list_type: data::type_::ListTypeId(3),
    },
}"#
                .trim_start_matches('\n'),
            ),
            (
                ListLocal::Nil {
                    local: NilListLocalId(2),
                    type_id: NilListTypeId::new(ListTypeId(3)),
                },
                r#"
data::graph::ListLocal::Nil {
    local: data::graph::NilListLocalId(2),
    type_id: data::type_::NilListTypeId {
        list_type: data::type_::ListTypeId(3),
    },
}"#
                .trim_start_matches('\n'),
            ),
            (
                ListLocal::Parameter {
                    local: ParameterListLocalId(2),
                    type_id: ParameterListTypeId::new(
                        ListTypeId(3),
                        crate::plan::TypeParameterId(4),
                    ),
                },
                r#"
data::graph::ListLocal::Parameter {
    local: data::graph::ParameterListLocalId(2),
    type_id: data::type_::ParameterListTypeId {
        list_type: data::type_::ListTypeId(3),
        item: data::type_::parameter_id(4),
    },
}"#
                .trim_start_matches('\n'),
            ),
            (
                ListLocal::ParameterList {
                    local: ParameterListListLocalId(2),
                    type_id: ParameterListListTypeId::new(
                        ListTypeId(3),
                        ParameterListTypeId::new(ListTypeId(4), crate::plan::TypeParameterId(5)),
                    ),
                },
                r#"
data::graph::ListLocal::ParameterList {
    local: data::graph::ParameterListListLocalId(2),
    type_id: data::type_::ParameterListListTypeId {
        list_type: data::type_::ListTypeId(3),
        item_type: data::type_::ParameterListTypeId {
            list_type: data::type_::ListTypeId(4),
            item: data::type_::parameter_id(5),
        },
    },
}"#
                .trim_start_matches('\n'),
            ),
            (
                ListLocal::Custom {
                    local: CustomListLocalId(2),
                    type_id: CustomListTypeId::new(ListTypeId(3), CustomTypeId(4)),
                },
                r#"
data::graph::ListLocal::Custom {
    local: data::graph::CustomListLocalId(2),
    type_id: data::type_::CustomListTypeId {
        list_type: data::type_::ListTypeId(3),
        item_type: data::type_::CustomTypeId(4),
    },
}"#
                .trim_start_matches('\n'),
            ),
            (
                ListLocal::External {
                    local: ExternalListLocalId(2),
                    type_id: ExternalListTypeId::new(ListTypeId(3), ExternalTypeId(4)),
                },
                r#"
data::graph::ListLocal::External {
    local: data::graph::ExternalListLocalId(2),
    type_id: data::type_::ExternalListTypeId {
        list_type: data::type_::ListTypeId(3),
        item_type: data::type_::ExternalTypeId(4),
    },
}"#
                .trim_start_matches('\n'),
            ),
            (
                ListLocal::Tuple {
                    local: TupleListLocalId(2),
                    type_id: TupleListTypeId {
                        list_type: ListTypeId(3),
                        item_type: TupleItemTypeId(4),
                    },
                },
                r#"
data::graph::ListLocal::Tuple {
    local: data::graph::TupleListLocalId(2),
    type_id: data::type_::TupleListTypeId {
        list_type: data::type_::ListTypeId(3),
        item_type: data::type_::TupleItemTypeId(4),
    },
}"#
                .trim_start_matches('\n'),
            ),
            (
                ListLocal::List {
                    local: ListListLocalId(2),
                    type_id: ListListTypeId::new(ListTypeId(3), ListTypeId(4)),
                },
                r#"
data::graph::ListLocal::List {
    local: data::graph::ListListLocalId(2),
    type_id: data::type_::ListListTypeId {
        list_type: data::type_::ListTypeId(3),
        item_type: data::type_::ListTypeId(4),
    },
}"#
                .trim_start_matches('\n'),
            ),
            (
                ListLocal::Function {
                    local: FunctionListLocalId(2),
                    type_id: FunctionListTypeId {
                        list_type: ListTypeId(3),
                        item_type: FunctionItemTypeId(4),
                    },
                },
                r#"
data::graph::ListLocal::Function {
    local: data::graph::FunctionListLocalId(2),
    type_id: data::type_::FunctionListTypeId {
        list_type: data::type_::ListTypeId(3),
        item_type: data::type_::FunctionItemTypeId(4),
    },
}"#
                .trim_start_matches('\n'),
            ),
        ];
        for (local, expected) in cases {
            assert_eq!(Rust::expression(&local), expected);
        }
        let stored = [
            (
                StoredListLocal::Int(IntListLocalId(2)),
                "data::graph::StoredListLocal::Int(data::graph::IntListLocalId(2))",
            ),
            (
                StoredListLocal::String(StringListLocalId(2)),
                "data::graph::StoredListLocal::String(data::graph::StringListLocalId(2))",
            ),
            (
                StoredListLocal::BitArray(BitArrayListLocalId(2)),
                "data::graph::StoredListLocal::BitArray(data::graph::BitArrayListLocalId(2))",
            ),
            (
                StoredListLocal::UtfCodepoint(UtfCodepointListLocalId(2)),
                "data::graph::StoredListLocal::UtfCodepoint(data::graph::UtfCodepointListLocalId(2))",
            ),
            (
                StoredListLocal::Float(FloatListLocalId(2)),
                "data::graph::StoredListLocal::Float(data::graph::FloatListLocalId(2))",
            ),
            (
                StoredListLocal::Bool(BoolListLocalId(2)),
                "data::graph::StoredListLocal::Bool(data::graph::BoolListLocalId(2))",
            ),
            (
                StoredListLocal::Nil(NilListLocalId(2)),
                "data::graph::StoredListLocal::Nil(data::graph::NilListLocalId(2))",
            ),
            (
                StoredListLocal::ParameterList(ParameterListListLocalId(2)),
                "data::graph::StoredListLocal::ParameterList(data::graph::ParameterListListLocalId(2))",
            ),
            (
                StoredListLocal::Custom(CustomListLocalId(2)),
                "data::graph::StoredListLocal::Custom(data::graph::CustomListLocalId(2))",
            ),
            (
                StoredListLocal::External(ExternalListLocalId(2)),
                "data::graph::StoredListLocal::External(data::graph::ExternalListLocalId(2))",
            ),
            (
                StoredListLocal::Tuple(TupleListLocalId(2)),
                "data::graph::StoredListLocal::Tuple(data::graph::TupleListLocalId(2))",
            ),
            (
                StoredListLocal::List(ListListLocalId(2)),
                "data::graph::StoredListLocal::List(data::graph::ListListLocalId(2))",
            ),
            (
                StoredListLocal::Function(FunctionListLocalId(2)),
                "data::graph::StoredListLocal::Function(data::graph::FunctionListLocalId(2))",
            ),
        ];
        for (local, expected) in stored {
            assert_eq!(Rust::expression(&local), expected);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::graph::{
        BitArrayListLocalId, BoolListLocalId, CustomListLocalId, ExternalListLocalId,
        FloatListLocalId, FunctionListLocalId, IntListLocalId, ListListLocalId, ListLocal,
        NilListLocalId, ParameterListListLocalId, ParameterListLocalId, StringListLocalId,
        TupleListLocalId, UtfCodepointListLocalId,
    };
    use crate::plan::execution::type_::{ExternalListTypeId, ExternalTypeId, ListTypeId};
    use crate::plan::{TypeParameterId, ValueType};

    #[test]
    fn parameter_list_locals_preserve_symbolic_and_nested_storage_types() {
        let parameter_plan = execution_plan("pub fn main() -> List(value) { [] }");
        let parameter = parameter_plan.parameter_list_function_id(0).type_id();
        let nested_plan = execution_plan("pub fn main() -> List(List(value)) { [] }");
        let nested = nested_plan.parameter_list_list_function_id(0).type_id();
        let locals = [
            ListLocal::Parameter {
                local: ParameterListLocalId(0),
                type_id: parameter,
            },
            ListLocal::ParameterList {
                local: ParameterListListLocalId(0),
                type_id: nested,
            },
        ];
        assert_eq!(
            parameter_plan.list_value_type(locals[0].list_type()),
            ValueType::List(Box::new(ValueType::Parameter(TypeParameterId(0)))),
        );
        assert_eq!(
            nested_plan.list_value_type(locals[1].list_type()),
            ValueType::List(Box::new(ValueType::List(Box::new(ValueType::Parameter(
                TypeParameterId(0)
            ),)))),
        );
    }

    #[test]
    fn concrete_list_locals_preserve_every_storage_type() {
        let plan = execution_plan(
            r#"
pub type Boxed { Boxed(Int) }

fn ints() -> List(Int) { [] }
fn strings() -> List(String) { [] }
fn bit_arrays() -> List(BitArray) { [] }
fn utf_codepoints() -> List(UtfCodepoint) { [] }
fn customs() -> List(Boxed) { [] }
fn floats() -> List(Float) { [] }
fn bools() -> List(Bool) { [] }
fn nils() -> List(Nil) { [] }
fn tuples() -> List(#(Int)) { [] }
fn lists() -> List(List(Int)) { [] }
fn functions() -> List(fn() -> Int) { [] }

pub fn main() {
  let _ = #(
    ints, strings, bit_arrays, utf_codepoints, customs, floats,
    bools, nils, tuples, lists, functions,
  )
  Nil
}
"#,
        );
        let locals = [
            ListLocal::Int {
                local: IntListLocalId(0),
                type_id: plan.int_list_function_id(0).type_id(),
            },
            ListLocal::String {
                local: StringListLocalId(0),
                type_id: plan.string_list_function_id(0).type_id(),
            },
            ListLocal::BitArray {
                local: BitArrayListLocalId(0),
                type_id: plan.bit_array_list_function_id(0).type_id(),
            },
            ListLocal::UtfCodepoint {
                local: UtfCodepointListLocalId(0),
                type_id: plan.utf_codepoint_list_function_id(0).type_id(),
            },
            ListLocal::Custom {
                local: CustomListLocalId(0),
                type_id: plan.custom_list_function_id(0).type_id(),
            },
            ListLocal::Float {
                local: FloatListLocalId(0),
                type_id: plan.float_list_function_id(0).type_id(),
            },
            ListLocal::Bool {
                local: BoolListLocalId(0),
                type_id: plan.bool_list_function_id(0).type_id(),
            },
            ListLocal::Nil {
                local: NilListLocalId(0),
                type_id: plan.nil_list_function_id(0).type_id(),
            },
            ListLocal::Tuple {
                local: TupleListLocalId(0),
                type_id: plan.tuple_list_function_id(0).type_id(),
            },
            ListLocal::List {
                local: ListListLocalId(0),
                type_id: plan.list_list_function_id(0).type_id(),
            },
            ListLocal::Function {
                local: FunctionListLocalId(0),
                type_id: plan.function_list_function_id(0).type_id(),
            },
        ];
        assert_eq!(
            locals.map(|local| local.list_type()),
            [
                plan.int_list_function_id(0).type_id().list_type(),
                plan.string_list_function_id(0).type_id().list_type(),
                plan.bit_array_list_function_id(0).type_id().list_type(),
                plan.utf_codepoint_list_function_id(0).type_id().list_type(),
                plan.custom_list_function_id(0).type_id().list_type(),
                plan.float_list_function_id(0).type_id().list_type(),
                plan.bool_list_function_id(0).type_id().list_type(),
                plan.nil_list_function_id(0).type_id().list_type(),
                plan.tuple_list_function_id(0).type_id().list_type(),
                plan.list_list_function_id(0).type_id().list_type(),
                plan.function_list_function_id(0).type_id().list_type(),
            ],
        );
    }

    #[test]
    fn external_list_locals_preserve_storage_type() {
        let local = ListLocal::External {
            local: ExternalListLocalId(3),
            type_id: ExternalListTypeId::new(ListTypeId::new(19), ExternalTypeId::new(5)),
        };

        assert_eq!(local.list_type(), ListTypeId::new(19));
    }

    fn execution_plan(source: &str) -> crate::ExecutionPlan {
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        crate::ExecutionPlan::from_module_plan(module_plan)
    }
}
