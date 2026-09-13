use super::super::local::Address;
use crate::plan::execution::function::ExecutionGraphProfile;
use crate::plan::execution::graph::{
    ExternalListInstructionView, ListInstruction, ParameterListInstruction,
    ProfiledInstructionKind, StringInstruction, TypedListInstruction,
};

#[derive(Debug, PartialEq, Eq)]
pub(super) enum Origin<'data> {
    Unknown,
    List { head: usize, tail: Option<Address> },
    ListDrop { source: Address, count: usize },
    Text(&'data str),
    TextDrop { source: Address, prefix: &'data str },
    Concatenate { left: Address, right: Address },
}

pub(super) fn instruction<Graph: ExecutionGraphProfile>(
    value: &ProfiledInstructionKind<Graph>,
) -> Origin<'_> {
    match value {
        ProfiledInstructionKind::List(value) => list(value),
        ProfiledInstructionKind::ExternalList(value) => typed_list(value.instruction()),
        ProfiledInstructionKind::String(value) => match value {
            StringInstruction::Value(text) => Origin::Text(text.as_str()),
            StringInstruction::DropPrefix { value, prefix } => Origin::TextDrop {
                source: (*value).into(),
                prefix: prefix.as_str(),
            },
            StringInstruction::Concatenate { left, right } => Origin::Concatenate {
                left: (*left).into(),
                right: (*right).into(),
            },
            StringInstruction::Constant(_)
            | StringInstruction::Call { .. }
            | StringInstruction::FunctionCall { .. }
            | StringInstruction::TupleIndex { .. }
            | StringInstruction::CustomField { .. }
            | StringInstruction::ListIndex { .. } => Origin::Unknown,
        },
        ProfiledInstructionKind::Int(_)
        | ProfiledInstructionKind::Float(_)
        | ProfiledInstructionKind::BitArray(_)
        | ProfiledInstructionKind::UtfCodepoint(_)
        | ProfiledInstructionKind::Custom(_)
        | ProfiledInstructionKind::Bool(_)
        | ProfiledInstructionKind::Nil(_)
        | ProfiledInstructionKind::Tuple(_)
        | ProfiledInstructionKind::Function(_)
        | ProfiledInstructionKind::External(_)
        | ProfiledInstructionKind::ExternalFunction(_) => Origin::Unknown,
    }
}

fn list(value: &ListInstruction) -> Origin<'_> {
    match value {
        ListInstruction::Parameter(_, value) => match value {
            ParameterListInstruction::Empty => Origin::List {
                head: 0,
                tail: None,
            },
            ParameterListInstruction::Constant(_)
            | ParameterListInstruction::Call { .. }
            | ParameterListInstruction::FunctionCall { .. }
            | ParameterListInstruction::TupleIndex { .. }
            | ParameterListInstruction::CustomField { .. }
            | ParameterListInstruction::ListIndex { .. } => Origin::Unknown,
        },
        ListInstruction::ParameterList(_, value) => typed_list(value),
        ListInstruction::Int(_, value) => typed_list(value),
        ListInstruction::String(_, value) => typed_list(value),
        ListInstruction::BitArray(_, value) => typed_list(value),
        ListInstruction::UtfCodepoint(_, value) => typed_list(value),
        ListInstruction::Custom(_, value) => typed_list(value),
        ListInstruction::Float(_, value) => typed_list(value),
        ListInstruction::Bool(_, value) => typed_list(value),
        ListInstruction::Nil(_, value) => typed_list(value),
        ListInstruction::Tuple(_, value) => typed_list(value),
        ListInstruction::List(_, value) => typed_list(value),
        ListInstruction::Function(_, value) => typed_list(value),
    }
}

fn typed_list<Element, Local: Copy + Into<Address>, Function, FunctionLocal>(
    value: &TypedListInstruction<Element, Local, Function, FunctionLocal>,
) -> Origin<'_> {
    match value {
        TypedListInstruction::Value(elements) => Origin::List {
            head: elements.len(),
            tail: None,
        },
        TypedListInstruction::Spread { elements, tail } => Origin::List {
            head: elements.len(),
            tail: Some((*tail).into()),
        },
        TypedListInstruction::DropFirst { list, count } => Origin::ListDrop {
            source: (*list).into(),
            count: *count,
        },
        TypedListInstruction::Constant(_)
        | TypedListInstruction::Call { .. }
        | TypedListInstruction::FunctionCall { .. }
        | TypedListInstruction::TupleIndex { .. }
        | TypedListInstruction::CustomField { .. }
        | TypedListInstruction::ListIndex { .. } => Origin::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::{Origin, instruction};
    use crate::plan::execution::function::HostedExecutionGraph;
    use crate::plan::execution::graph::{
        BitArrayListLocalId, BoolInstruction, BoolListLocalId, CustomListLocalId,
        ExternalListInstruction, ExternalListLocalId, FloatListLocalId, FunctionListLocalId,
        IntListLocalId, IntLocalId, ListInstruction, ListListLocalId, NilListLocalId,
        ParameterListInstruction, ParameterListListLocalId, ProfiledInstructionKind,
        StringInstruction, StringListLocalId, StringLocalId, TupleListLocalId, TupleLocalId,
        TypedListInstruction, UtfCodepointListLocalId,
    };
    use crate::plan::execution::storage::Table;
    use crate::plan::execution::type_::{
        BitArrayListTypeId, BoolListTypeId, CustomListTypeId, CustomTypeId, ExternalListTypeId,
        ExternalTypeId, FloatListTypeId, FunctionListTypeId, IntListTypeId, ListListTypeId,
        ListTypeId, NilListTypeId, ParameterListListTypeId, ParameterListTypeId, StringListTypeId,
        TupleListTypeId, UtfCodepointListTypeId,
    };
    use crate::plan::{Text, TypeParameterId};

    #[test]
    fn list_origins_preserve_construction_tail_and_drop_for_each_element_family() {
        type Kind = ProfiledInstructionKind<HostedExecutionGraph>;
        let parameter = ParameterListTypeId::new(ListTypeId(0), TypeParameterId(0));
        let cases = [
            (
                ListInstruction::Parameter(parameter, ParameterListInstruction::Empty),
                Origin::List {
                    head: 0,
                    tail: None,
                },
            ),
            (
                ListInstruction::Parameter(
                    parameter,
                    ParameterListInstruction::TupleIndex {
                        tuple: TupleLocalId(2),
                        index: 1,
                    },
                ),
                Origin::Unknown,
            ),
            (
                ListInstruction::ParameterList(
                    ParameterListListTypeId::new(ListTypeId(1), parameter),
                    TypedListInstruction::Value(Table::Static(&[])),
                ),
                Origin::List {
                    head: 0,
                    tail: None,
                },
            ),
            (
                ListInstruction::ParameterList(
                    ParameterListListTypeId::new(ListTypeId(1), parameter),
                    TypedListInstruction::Spread {
                        elements: Table::Static(&[]),
                        tail: ParameterListListLocalId(3),
                    },
                ),
                Origin::List {
                    head: 0,
                    tail: Some(ParameterListListLocalId(3).into()),
                },
            ),
            (
                ListInstruction::ParameterList(
                    ParameterListListTypeId::new(ListTypeId(1), parameter),
                    TypedListInstruction::DropFirst {
                        list: ParameterListListLocalId(3),
                        count: 2,
                    },
                ),
                Origin::ListDrop {
                    source: ParameterListListLocalId(3).into(),
                    count: 2,
                },
            ),
            (
                ListInstruction::ParameterList(
                    ParameterListListTypeId::new(ListTypeId(1), parameter),
                    TypedListInstruction::TupleIndex {
                        tuple: TupleLocalId(2),
                        index: 1,
                    },
                ),
                Origin::Unknown,
            ),
            (
                ListInstruction::Int(
                    IntListTypeId::new(ListTypeId(2)),
                    TypedListInstruction::Value(Table::Static(&[])),
                ),
                Origin::List {
                    head: 0,
                    tail: None,
                },
            ),
            (
                ListInstruction::Int(
                    IntListTypeId::new(ListTypeId(2)),
                    TypedListInstruction::Spread {
                        elements: Table::Static(&[]),
                        tail: IntListLocalId(3),
                    },
                ),
                Origin::List {
                    head: 0,
                    tail: Some(IntListLocalId(3).into()),
                },
            ),
            (
                ListInstruction::Int(
                    IntListTypeId::new(ListTypeId(2)),
                    TypedListInstruction::DropFirst {
                        list: IntListLocalId(3),
                        count: 2,
                    },
                ),
                Origin::ListDrop {
                    source: IntListLocalId(3).into(),
                    count: 2,
                },
            ),
            (
                ListInstruction::Int(
                    IntListTypeId::new(ListTypeId(2)),
                    TypedListInstruction::TupleIndex {
                        tuple: TupleLocalId(2),
                        index: 1,
                    },
                ),
                Origin::Unknown,
            ),
            (
                ListInstruction::String(
                    StringListTypeId::new(ListTypeId(3)),
                    TypedListInstruction::Value(Table::Static(&[])),
                ),
                Origin::List {
                    head: 0,
                    tail: None,
                },
            ),
            (
                ListInstruction::String(
                    StringListTypeId::new(ListTypeId(3)),
                    TypedListInstruction::Spread {
                        elements: Table::Static(&[]),
                        tail: StringListLocalId(3),
                    },
                ),
                Origin::List {
                    head: 0,
                    tail: Some(StringListLocalId(3).into()),
                },
            ),
            (
                ListInstruction::String(
                    StringListTypeId::new(ListTypeId(3)),
                    TypedListInstruction::DropFirst {
                        list: StringListLocalId(3),
                        count: 2,
                    },
                ),
                Origin::ListDrop {
                    source: StringListLocalId(3).into(),
                    count: 2,
                },
            ),
            (
                ListInstruction::String(
                    StringListTypeId::new(ListTypeId(3)),
                    TypedListInstruction::TupleIndex {
                        tuple: TupleLocalId(2),
                        index: 1,
                    },
                ),
                Origin::Unknown,
            ),
            (
                ListInstruction::BitArray(
                    BitArrayListTypeId::new(ListTypeId(4)),
                    TypedListInstruction::Value(Table::Static(&[])),
                ),
                Origin::List {
                    head: 0,
                    tail: None,
                },
            ),
            (
                ListInstruction::BitArray(
                    BitArrayListTypeId::new(ListTypeId(4)),
                    TypedListInstruction::Spread {
                        elements: Table::Static(&[]),
                        tail: BitArrayListLocalId(3),
                    },
                ),
                Origin::List {
                    head: 0,
                    tail: Some(BitArrayListLocalId(3).into()),
                },
            ),
            (
                ListInstruction::BitArray(
                    BitArrayListTypeId::new(ListTypeId(4)),
                    TypedListInstruction::DropFirst {
                        list: BitArrayListLocalId(3),
                        count: 2,
                    },
                ),
                Origin::ListDrop {
                    source: BitArrayListLocalId(3).into(),
                    count: 2,
                },
            ),
            (
                ListInstruction::BitArray(
                    BitArrayListTypeId::new(ListTypeId(4)),
                    TypedListInstruction::TupleIndex {
                        tuple: TupleLocalId(2),
                        index: 1,
                    },
                ),
                Origin::Unknown,
            ),
            (
                ListInstruction::UtfCodepoint(
                    UtfCodepointListTypeId::new(ListTypeId(5)),
                    TypedListInstruction::Value(Table::Static(&[])),
                ),
                Origin::List {
                    head: 0,
                    tail: None,
                },
            ),
            (
                ListInstruction::UtfCodepoint(
                    UtfCodepointListTypeId::new(ListTypeId(5)),
                    TypedListInstruction::Spread {
                        elements: Table::Static(&[]),
                        tail: UtfCodepointListLocalId(3),
                    },
                ),
                Origin::List {
                    head: 0,
                    tail: Some(UtfCodepointListLocalId(3).into()),
                },
            ),
            (
                ListInstruction::UtfCodepoint(
                    UtfCodepointListTypeId::new(ListTypeId(5)),
                    TypedListInstruction::DropFirst {
                        list: UtfCodepointListLocalId(3),
                        count: 2,
                    },
                ),
                Origin::ListDrop {
                    source: UtfCodepointListLocalId(3).into(),
                    count: 2,
                },
            ),
            (
                ListInstruction::UtfCodepoint(
                    UtfCodepointListTypeId::new(ListTypeId(5)),
                    TypedListInstruction::TupleIndex {
                        tuple: TupleLocalId(2),
                        index: 1,
                    },
                ),
                Origin::Unknown,
            ),
            (
                ListInstruction::Custom(
                    CustomListTypeId::new(ListTypeId(6), CustomTypeId(2)),
                    TypedListInstruction::Value(Table::Static(&[])),
                ),
                Origin::List {
                    head: 0,
                    tail: None,
                },
            ),
            (
                ListInstruction::Custom(
                    CustomListTypeId::new(ListTypeId(6), CustomTypeId(2)),
                    TypedListInstruction::Spread {
                        elements: Table::Static(&[]),
                        tail: CustomListLocalId(3),
                    },
                ),
                Origin::List {
                    head: 0,
                    tail: Some(CustomListLocalId(3).into()),
                },
            ),
            (
                ListInstruction::Custom(
                    CustomListTypeId::new(ListTypeId(6), CustomTypeId(2)),
                    TypedListInstruction::DropFirst {
                        list: CustomListLocalId(3),
                        count: 2,
                    },
                ),
                Origin::ListDrop {
                    source: CustomListLocalId(3).into(),
                    count: 2,
                },
            ),
            (
                ListInstruction::Custom(
                    CustomListTypeId::new(ListTypeId(6), CustomTypeId(2)),
                    TypedListInstruction::TupleIndex {
                        tuple: TupleLocalId(2),
                        index: 1,
                    },
                ),
                Origin::Unknown,
            ),
            (
                ListInstruction::Float(
                    FloatListTypeId::new(ListTypeId(7)),
                    TypedListInstruction::Value(Table::Static(&[])),
                ),
                Origin::List {
                    head: 0,
                    tail: None,
                },
            ),
            (
                ListInstruction::Float(
                    FloatListTypeId::new(ListTypeId(7)),
                    TypedListInstruction::Spread {
                        elements: Table::Static(&[]),
                        tail: FloatListLocalId(3),
                    },
                ),
                Origin::List {
                    head: 0,
                    tail: Some(FloatListLocalId(3).into()),
                },
            ),
            (
                ListInstruction::Float(
                    FloatListTypeId::new(ListTypeId(7)),
                    TypedListInstruction::DropFirst {
                        list: FloatListLocalId(3),
                        count: 2,
                    },
                ),
                Origin::ListDrop {
                    source: FloatListLocalId(3).into(),
                    count: 2,
                },
            ),
            (
                ListInstruction::Float(
                    FloatListTypeId::new(ListTypeId(7)),
                    TypedListInstruction::TupleIndex {
                        tuple: TupleLocalId(2),
                        index: 1,
                    },
                ),
                Origin::Unknown,
            ),
            (
                ListInstruction::Bool(
                    BoolListTypeId::new(ListTypeId(8)),
                    TypedListInstruction::Value(Table::Static(&[])),
                ),
                Origin::List {
                    head: 0,
                    tail: None,
                },
            ),
            (
                ListInstruction::Bool(
                    BoolListTypeId::new(ListTypeId(8)),
                    TypedListInstruction::Spread {
                        elements: Table::Static(&[]),
                        tail: BoolListLocalId(3),
                    },
                ),
                Origin::List {
                    head: 0,
                    tail: Some(BoolListLocalId(3).into()),
                },
            ),
            (
                ListInstruction::Bool(
                    BoolListTypeId::new(ListTypeId(8)),
                    TypedListInstruction::DropFirst {
                        list: BoolListLocalId(3),
                        count: 2,
                    },
                ),
                Origin::ListDrop {
                    source: BoolListLocalId(3).into(),
                    count: 2,
                },
            ),
            (
                ListInstruction::Bool(
                    BoolListTypeId::new(ListTypeId(8)),
                    TypedListInstruction::TupleIndex {
                        tuple: TupleLocalId(2),
                        index: 1,
                    },
                ),
                Origin::Unknown,
            ),
            (
                ListInstruction::Nil(
                    NilListTypeId::new(ListTypeId(9)),
                    TypedListInstruction::Value(Table::Static(&[])),
                ),
                Origin::List {
                    head: 0,
                    tail: None,
                },
            ),
            (
                ListInstruction::Nil(
                    NilListTypeId::new(ListTypeId(9)),
                    TypedListInstruction::Spread {
                        elements: Table::Static(&[]),
                        tail: NilListLocalId(3),
                    },
                ),
                Origin::List {
                    head: 0,
                    tail: Some(NilListLocalId(3).into()),
                },
            ),
            (
                ListInstruction::Nil(
                    NilListTypeId::new(ListTypeId(9)),
                    TypedListInstruction::DropFirst {
                        list: NilListLocalId(3),
                        count: 2,
                    },
                ),
                Origin::ListDrop {
                    source: NilListLocalId(3).into(),
                    count: 2,
                },
            ),
            (
                ListInstruction::Nil(
                    NilListTypeId::new(ListTypeId(9)),
                    TypedListInstruction::TupleIndex {
                        tuple: TupleLocalId(2),
                        index: 1,
                    },
                ),
                Origin::Unknown,
            ),
            (
                ListInstruction::Tuple(
                    TupleListTypeId::new(ListTypeId(10), 0),
                    TypedListInstruction::Value(Table::Static(&[])),
                ),
                Origin::List {
                    head: 0,
                    tail: None,
                },
            ),
            (
                ListInstruction::Tuple(
                    TupleListTypeId::new(ListTypeId(10), 0),
                    TypedListInstruction::Spread {
                        elements: Table::Static(&[]),
                        tail: TupleListLocalId(3),
                    },
                ),
                Origin::List {
                    head: 0,
                    tail: Some(TupleListLocalId(3).into()),
                },
            ),
            (
                ListInstruction::Tuple(
                    TupleListTypeId::new(ListTypeId(10), 0),
                    TypedListInstruction::DropFirst {
                        list: TupleListLocalId(3),
                        count: 2,
                    },
                ),
                Origin::ListDrop {
                    source: TupleListLocalId(3).into(),
                    count: 2,
                },
            ),
            (
                ListInstruction::Tuple(
                    TupleListTypeId::new(ListTypeId(10), 0),
                    TypedListInstruction::TupleIndex {
                        tuple: TupleLocalId(2),
                        index: 1,
                    },
                ),
                Origin::Unknown,
            ),
            (
                ListInstruction::List(
                    ListListTypeId::new(ListTypeId(11), ListTypeId(2)),
                    TypedListInstruction::Value(Table::Static(&[])),
                ),
                Origin::List {
                    head: 0,
                    tail: None,
                },
            ),
            (
                ListInstruction::List(
                    ListListTypeId::new(ListTypeId(11), ListTypeId(2)),
                    TypedListInstruction::Spread {
                        elements: Table::Static(&[]),
                        tail: ListListLocalId(3),
                    },
                ),
                Origin::List {
                    head: 0,
                    tail: Some(ListListLocalId(3).into()),
                },
            ),
            (
                ListInstruction::List(
                    ListListTypeId::new(ListTypeId(11), ListTypeId(2)),
                    TypedListInstruction::DropFirst {
                        list: ListListLocalId(3),
                        count: 2,
                    },
                ),
                Origin::ListDrop {
                    source: ListListLocalId(3).into(),
                    count: 2,
                },
            ),
            (
                ListInstruction::List(
                    ListListTypeId::new(ListTypeId(11), ListTypeId(2)),
                    TypedListInstruction::TupleIndex {
                        tuple: TupleLocalId(2),
                        index: 1,
                    },
                ),
                Origin::Unknown,
            ),
            (
                ListInstruction::Function(
                    FunctionListTypeId::new(ListTypeId(12), 0),
                    TypedListInstruction::Value(Table::Static(&[])),
                ),
                Origin::List {
                    head: 0,
                    tail: None,
                },
            ),
            (
                ListInstruction::Function(
                    FunctionListTypeId::new(ListTypeId(12), 0),
                    TypedListInstruction::Spread {
                        elements: Table::Static(&[]),
                        tail: FunctionListLocalId(3),
                    },
                ),
                Origin::List {
                    head: 0,
                    tail: Some(FunctionListLocalId(3).into()),
                },
            ),
            (
                ListInstruction::Function(
                    FunctionListTypeId::new(ListTypeId(12), 0),
                    TypedListInstruction::DropFirst {
                        list: FunctionListLocalId(3),
                        count: 2,
                    },
                ),
                Origin::ListDrop {
                    source: FunctionListLocalId(3).into(),
                    count: 2,
                },
            ),
            (
                ListInstruction::Function(
                    FunctionListTypeId::new(ListTypeId(12), 0),
                    TypedListInstruction::TupleIndex {
                        tuple: TupleLocalId(2),
                        index: 1,
                    },
                ),
                Origin::Unknown,
            ),
        ];
        for (index, (value, expected)) in cases.into_iter().enumerate() {
            assert_eq!(instruction(&Kind::List(value)), expected, "case {index}");
        }
        let type_id = ExternalListTypeId::new(ListTypeId(13), ExternalTypeId(0));
        for (value, expected) in [
            (
                TypedListInstruction::Value(Table::Static(&[])),
                Origin::List {
                    head: 0,
                    tail: None,
                },
            ),
            (
                TypedListInstruction::Spread {
                    elements: Table::Static(&[]),
                    tail: ExternalListLocalId(3),
                },
                Origin::List {
                    head: 0,
                    tail: Some(ExternalListLocalId(3).into()),
                },
            ),
            (
                TypedListInstruction::DropFirst {
                    list: ExternalListLocalId(3),
                    count: 2,
                },
                Origin::ListDrop {
                    source: ExternalListLocalId(3).into(),
                    count: 2,
                },
            ),
            (
                TypedListInstruction::TupleIndex {
                    tuple: TupleLocalId(2),
                    index: 1,
                },
                Origin::Unknown,
            ),
        ] {
            assert_eq!(
                instruction(&Kind::ExternalList(ExternalListInstruction {
                    type_id,
                    instruction: value
                })),
                expected
            );
        }
        for (value, expected) in [
            (
                TypedListInstruction::Value(Table::Static(&[IntLocalId(0), IntLocalId(1)])),
                Origin::List {
                    head: 2,
                    tail: None,
                },
            ),
            (
                TypedListInstruction::Spread {
                    elements: Table::Static(&[IntLocalId(0), IntLocalId(1)]),
                    tail: IntListLocalId(3),
                },
                Origin::List {
                    head: 2,
                    tail: Some(IntListLocalId(3).into()),
                },
            ),
        ] {
            assert_eq!(
                instruction(&Kind::List(ListInstruction::Int(
                    IntListTypeId::new(ListTypeId(2)),
                    value
                ))),
                expected
            );
        }
    }

    #[test]
    fn text_origins_borrow_literal_and_prefix_bytes_and_keep_concatenation_order() {
        type Kind = ProfiledInstructionKind<HostedExecutionGraph>;
        for (value, expected) in [
            (
                StringInstruction::Value(Text::Static("text")),
                Origin::Text("text"),
            ),
            (
                StringInstruction::DropPrefix {
                    value: StringLocalId(3),
                    prefix: Text::Static("pre-"),
                },
                Origin::TextDrop {
                    source: StringLocalId(3).into(),
                    prefix: "pre-",
                },
            ),
            (
                StringInstruction::Concatenate {
                    left: StringLocalId(2),
                    right: StringLocalId(5),
                },
                Origin::Concatenate {
                    left: StringLocalId(2).into(),
                    right: StringLocalId(5).into(),
                },
            ),
            (
                StringInstruction::TupleIndex {
                    tuple: TupleLocalId(4),
                    index: 2,
                },
                Origin::Unknown,
            ),
        ] {
            assert_eq!(instruction(&Kind::String(value)), expected);
        }
        assert_eq!(
            instruction(&Kind::Bool(BoolInstruction::Value(true))),
            Origin::Unknown
        );
    }
}
