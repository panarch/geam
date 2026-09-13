use super::{Address, Projection};
use crate::plan::execution::function::ExecutionGraphProfile;
use crate::plan::execution::graph::{
    BitArrayInstruction, BoolInstruction, CustomInstruction, ExternalInstructionRef,
    ExternalInstructionView, ExternalListInstructionView, FloatInstruction,
    FunctionInstructionKind, IntInstruction, ListInstruction, NilInstruction, ParamLocal,
    ParameterListInstruction, ProfiledInstructionKind, StringInstruction, TupleInstruction,
    TypedListInstruction, UtfCodepointInstruction,
};

pub(in crate::plan::execution::prepared::admission) fn instruction<Graph: ExecutionGraphProfile>(
    value: &ProfiledInstructionKind<Graph>,
) -> Option<(Address, Projection)> {
    match value {
        ProfiledInstructionKind::Int(value) => int(value),
        ProfiledInstructionKind::Float(value) => float(value),
        ProfiledInstructionKind::String(value) => string(value),
        ProfiledInstructionKind::BitArray(value) => bit_array(value),
        ProfiledInstructionKind::UtfCodepoint(value) => utf_codepoint(value),
        ProfiledInstructionKind::Custom(value) => custom(value),
        ProfiledInstructionKind::Bool(value) => bool(value),
        ProfiledInstructionKind::Nil(value) => nil(value),
        ProfiledInstructionKind::Tuple(value) => tuple(value),
        ProfiledInstructionKind::Function(value) => function(&value.kind),
        ProfiledInstructionKind::List(value) => list(value),
        ProfiledInstructionKind::External(value) => external(value.instruction_ref()),
        ProfiledInstructionKind::ExternalList(value) => typed_list(value.instruction()),
        ProfiledInstructionKind::ExternalFunction(_) => None,
    }
}

fn int(value: &IntInstruction) -> Option<(Address, Projection)> {
    match value {
        IntInstruction::TupleIndex { tuple, index } => {
            Some(((*tuple).into(), Projection::Tuple(*index)))
        }
        IntInstruction::CustomField { source, index } => Some((
            Address::of(&ParamLocal::Custom(*source)),
            Projection::Custom(*index),
        )),
        IntInstruction::ListIndex { list, index } => {
            Some(((*list).into(), Projection::List(*index)))
        }
        IntInstruction::Value(..)
        | IntInstruction::Constant(..)
        | IntInstruction::Call { .. }
        | IntInstruction::FunctionCall { .. }
        | IntInstruction::Add { .. }
        | IntInstruction::Sub { .. }
        | IntInstruction::Mult { .. }
        | IntInstruction::Div { .. }
        | IntInstruction::Remainder { .. }
        | IntInstruction::Negate(..) => None,
    }
}

fn float(value: &FloatInstruction) -> Option<(Address, Projection)> {
    match value {
        FloatInstruction::TupleIndex { tuple, index } => {
            Some(((*tuple).into(), Projection::Tuple(*index)))
        }
        FloatInstruction::CustomField { source, index } => Some((
            Address::of(&ParamLocal::Custom(*source)),
            Projection::Custom(*index),
        )),
        FloatInstruction::ListIndex { list, index } => {
            Some(((*list).into(), Projection::List(*index)))
        }
        FloatInstruction::Value(..)
        | FloatInstruction::Constant(..)
        | FloatInstruction::Call { .. }
        | FloatInstruction::FunctionCall { .. }
        | FloatInstruction::Add { .. }
        | FloatInstruction::Sub { .. }
        | FloatInstruction::Mult { .. }
        | FloatInstruction::Div { .. } => None,
    }
}

fn string(value: &StringInstruction) -> Option<(Address, Projection)> {
    match value {
        StringInstruction::TupleIndex { tuple, index } => {
            Some(((*tuple).into(), Projection::Tuple(*index)))
        }
        StringInstruction::CustomField { source, index } => Some((
            Address::of(&ParamLocal::Custom(*source)),
            Projection::Custom(*index),
        )),
        StringInstruction::ListIndex { list, index } => {
            Some(((*list).into(), Projection::List(*index)))
        }
        StringInstruction::Value(..)
        | StringInstruction::Constant(..)
        | StringInstruction::Call { .. }
        | StringInstruction::FunctionCall { .. }
        | StringInstruction::Concatenate { .. }
        | StringInstruction::DropPrefix { .. } => None,
    }
}

fn bit_array(value: &BitArrayInstruction) -> Option<(Address, Projection)> {
    match value {
        BitArrayInstruction::TupleIndex { tuple, index } => {
            Some(((*tuple).into(), Projection::Tuple(*index)))
        }
        BitArrayInstruction::CustomField { source, index } => Some((
            Address::of(&ParamLocal::Custom(*source)),
            Projection::Custom(*index),
        )),
        BitArrayInstruction::ListIndex { list, index } => {
            Some(((*list).into(), Projection::List(*index)))
        }
        BitArrayInstruction::Value(..)
        | BitArrayInstruction::Constant(..)
        | BitArrayInstruction::Call { .. }
        | BitArrayInstruction::FunctionCall { .. } => None,
    }
}

fn utf_codepoint(value: &UtfCodepointInstruction) -> Option<(Address, Projection)> {
    match value {
        UtfCodepointInstruction::TupleIndex { tuple, index } => {
            Some(((*tuple).into(), Projection::Tuple(*index)))
        }
        UtfCodepointInstruction::CustomField { source, index } => Some((
            Address::of(&ParamLocal::Custom(*source)),
            Projection::Custom(*index),
        )),
        UtfCodepointInstruction::ListIndex { list, index } => {
            Some(((*list).into(), Projection::List(*index)))
        }
        UtfCodepointInstruction::Call { .. } | UtfCodepointInstruction::FunctionCall { .. } => None,
    }
}

fn custom(value: &CustomInstruction) -> Option<(Address, Projection)> {
    match value {
        CustomInstruction::TupleIndex { tuple, index } => {
            Some(((*tuple).into(), Projection::Tuple(*index)))
        }
        CustomInstruction::CustomField { source, index } => Some((
            Address::of(&ParamLocal::Custom(*source)),
            Projection::Custom(*index),
        )),
        CustomInstruction::ListIndex { list, index } => {
            Some(((*list).into(), Projection::List(*index)))
        }
        CustomInstruction::Construct { .. }
        | CustomInstruction::Constant(..)
        | CustomInstruction::Call { .. }
        | CustomInstruction::FunctionCall { .. } => None,
    }
}

fn bool(value: &BoolInstruction) -> Option<(Address, Projection)> {
    match value {
        BoolInstruction::TupleIndex { tuple, index } => {
            Some(((*tuple).into(), Projection::Tuple(*index)))
        }
        BoolInstruction::CustomField { source, index } => Some((
            Address::of(&ParamLocal::Custom(*source)),
            Projection::Custom(*index),
        )),
        BoolInstruction::ListIndex { list, index } => {
            Some(((*list).into(), Projection::List(*index)))
        }
        BoolInstruction::Value(..)
        | BoolInstruction::Constant(..)
        | BoolInstruction::Call { .. }
        | BoolInstruction::FunctionCall { .. }
        | BoolInstruction::Not(..)
        | BoolInstruction::LtInt { .. }
        | BoolInstruction::LtEqInt { .. }
        | BoolInstruction::GtInt { .. }
        | BoolInstruction::GtEqInt { .. }
        | BoolInstruction::LtFloat { .. }
        | BoolInstruction::LtEqFloat { .. }
        | BoolInstruction::GtFloat { .. }
        | BoolInstruction::GtEqFloat { .. }
        | BoolInstruction::Equal { .. }
        | BoolInstruction::NotEqual { .. }
        | BoolInstruction::StringStartsWith { .. }
        | BoolInstruction::ListLengthEquals { .. }
        | BoolInstruction::ListLengthAtLeast { .. } => None,
    }
}

fn nil(value: &NilInstruction) -> Option<(Address, Projection)> {
    match value {
        NilInstruction::TupleIndex { tuple, index } => {
            Some(((*tuple).into(), Projection::Tuple(*index)))
        }
        NilInstruction::CustomField { source, index } => Some((
            Address::of(&ParamLocal::Custom(*source)),
            Projection::Custom(*index),
        )),
        NilInstruction::ListIndex { list, index } => {
            Some(((*list).into(), Projection::List(*index)))
        }
        NilInstruction::Value
        | NilInstruction::Constant(..)
        | NilInstruction::Call { .. }
        | NilInstruction::FunctionCall { .. } => None,
    }
}

fn tuple(value: &TupleInstruction) -> Option<(Address, Projection)> {
    match value {
        TupleInstruction::TupleIndex { tuple, index } => {
            Some(((*tuple).into(), Projection::Tuple(*index)))
        }
        TupleInstruction::CustomField { source, index } => Some((
            Address::of(&ParamLocal::Custom(*source)),
            Projection::Custom(*index),
        )),
        TupleInstruction::ListIndex { list, index } => {
            Some(((*list).into(), Projection::List(*index)))
        }
        TupleInstruction::Value(..)
        | TupleInstruction::Constant(..)
        | TupleInstruction::Call { .. }
        | TupleInstruction::FunctionCall { .. } => None,
    }
}

fn function(value: &FunctionInstructionKind) -> Option<(Address, Projection)> {
    match value {
        FunctionInstructionKind::TupleIndex { tuple, index } => {
            Some(((*tuple).into(), Projection::Tuple(*index)))
        }
        FunctionInstructionKind::CustomField { source, index } => Some((
            Address::of(&ParamLocal::Custom(*source)),
            Projection::Custom(*index),
        )),
        FunctionInstructionKind::ListIndex { list, index } => {
            Some(((*list).into(), Projection::List(*index)))
        }
        FunctionInstructionKind::Constant(..)
        | FunctionInstructionKind::Reference(..)
        | FunctionInstructionKind::Closure { .. }
        | FunctionInstructionKind::Constructor(..)
        | FunctionInstructionKind::Call { .. }
        | FunctionInstructionKind::FunctionCall { .. } => None,
    }
}

fn list(value: &ListInstruction) -> Option<(Address, Projection)> {
    match value {
        ListInstruction::Parameter(_, value) => parameter_list(value),
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

fn parameter_list(value: &ParameterListInstruction) -> Option<(Address, Projection)> {
    match value {
        ParameterListInstruction::TupleIndex { tuple, index } => {
            Some(((*tuple).into(), Projection::Tuple(*index)))
        }
        ParameterListInstruction::CustomField { source, index } => Some((
            Address::of(&ParamLocal::Custom(*source)),
            Projection::Custom(*index),
        )),
        ParameterListInstruction::ListIndex { list, index } => {
            Some(((*list).into(), Projection::List(*index)))
        }
        ParameterListInstruction::Empty
        | ParameterListInstruction::Constant(..)
        | ParameterListInstruction::Call { .. }
        | ParameterListInstruction::FunctionCall { .. } => None,
    }
}

fn typed_list<Element, Local, Function, FunctionLocal>(
    value: &TypedListInstruction<Element, Local, Function, FunctionLocal>,
) -> Option<(Address, Projection)> {
    match value {
        TypedListInstruction::TupleIndex { tuple, index } => {
            Some(((*tuple).into(), Projection::Tuple(*index)))
        }
        TypedListInstruction::CustomField { source, index } => Some((
            Address::of(&ParamLocal::Custom(*source)),
            Projection::Custom(*index),
        )),
        TypedListInstruction::ListIndex { list, index } => {
            Some(((*list).into(), Projection::List(*index)))
        }
        TypedListInstruction::Value(..)
        | TypedListInstruction::Constant(..)
        | TypedListInstruction::Spread { .. }
        | TypedListInstruction::Call { .. }
        | TypedListInstruction::FunctionCall { .. }
        | TypedListInstruction::DropFirst { .. } => None,
    }
}

fn external<Function>(
    value: ExternalInstructionRef<'_, Function>,
) -> Option<(Address, Projection)> {
    match value {
        ExternalInstructionRef::TupleIndex { tuple, index } => {
            Some((tuple.into(), Projection::Tuple(index)))
        }
        ExternalInstructionRef::CustomField { source, index } => Some((
            Address::of(&ParamLocal::Custom(*source)),
            Projection::Custom(index),
        )),
        ExternalInstructionRef::ListIndex { list, index } => {
            Some((list.into(), Projection::List(index)))
        }
        ExternalInstructionRef::Call { .. } | ExternalInstructionRef::FunctionCall { .. } => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{Projection, function, instruction};
    use crate::plan::TypeParameterId;
    use crate::plan::execution::function::HostedExecutionGraph;
    use crate::plan::execution::graph::{
        BitArrayInstruction, BitArrayListLocalId, BoolInstruction, BoolListLocalId,
        CustomInstruction, CustomListLocalId, CustomLocal, CustomLocalId, ExternalInstruction,
        ExternalListInstruction, ExternalListLocalId, FloatInstruction, FloatListLocalId,
        FunctionInstructionKind, FunctionListLocalId, IntInstruction, IntListLocalId,
        ListInstruction, ListListLocalId, NilInstruction, NilListLocalId, ParameterListInstruction,
        ParameterListListLocalId, ProfiledInstructionKind, StringInstruction, StringListLocalId,
        TupleInstruction, TupleListLocalId, TupleLocalId, TypedListInstruction,
        UtfCodepointInstruction, UtfCodepointListLocalId,
    };
    use crate::plan::execution::storage::Table;
    use crate::plan::execution::type_::{
        BitArrayListTypeId, BoolListTypeId, CustomConstructorId, CustomListTypeId, CustomTypeId,
        CustomValueShape, CustomValueShapeId, ExternalListTypeId, ExternalTypeId, FloatListTypeId,
        FunctionListTypeId, IntListTypeId, ListListTypeId, ListTypeId, NilListTypeId,
        ParameterListListTypeId, ParameterListTypeId, StringListTypeId, TupleListTypeId,
        UtfCodepointListTypeId,
    };

    #[test]
    fn scalar_tuple_and_external_reads_preserve_source_address_and_field_index() {
        type Kind = ProfiledInstructionKind<HostedExecutionGraph>;
        let source = CustomLocal {
            id: CustomLocalId(7),
            shape: CustomValueShape {
                type_id: CustomTypeId(2),
                shape_id: CustomValueShapeId(5),
            },
        };
        let cases = [
            (
                Kind::Int(IntInstruction::TupleIndex {
                    tuple: TupleLocalId(4),
                    index: 2,
                }),
                Some((TupleLocalId(4).into(), Projection::Tuple(2))),
            ),
            (
                Kind::Int(IntInstruction::CustomField { source, index: 2 }),
                Some((CustomLocalId(7).into(), Projection::Custom(2))),
            ),
            (
                Kind::Int(IntInstruction::ListIndex {
                    list: IntListLocalId(3),
                    index: 2,
                }),
                Some((IntListLocalId(3).into(), Projection::List(2))),
            ),
            (
                Kind::Float(FloatInstruction::TupleIndex {
                    tuple: TupleLocalId(4),
                    index: 2,
                }),
                Some((TupleLocalId(4).into(), Projection::Tuple(2))),
            ),
            (
                Kind::Float(FloatInstruction::CustomField { source, index: 2 }),
                Some((CustomLocalId(7).into(), Projection::Custom(2))),
            ),
            (
                Kind::Float(FloatInstruction::ListIndex {
                    list: FloatListLocalId(3),
                    index: 2,
                }),
                Some((FloatListLocalId(3).into(), Projection::List(2))),
            ),
            (
                Kind::String(StringInstruction::TupleIndex {
                    tuple: TupleLocalId(4),
                    index: 2,
                }),
                Some((TupleLocalId(4).into(), Projection::Tuple(2))),
            ),
            (
                Kind::String(StringInstruction::CustomField { source, index: 2 }),
                Some((CustomLocalId(7).into(), Projection::Custom(2))),
            ),
            (
                Kind::String(StringInstruction::ListIndex {
                    list: StringListLocalId(3),
                    index: 2,
                }),
                Some((StringListLocalId(3).into(), Projection::List(2))),
            ),
            (
                Kind::BitArray(BitArrayInstruction::TupleIndex {
                    tuple: TupleLocalId(4),
                    index: 2,
                }),
                Some((TupleLocalId(4).into(), Projection::Tuple(2))),
            ),
            (
                Kind::BitArray(BitArrayInstruction::CustomField { source, index: 2 }),
                Some((CustomLocalId(7).into(), Projection::Custom(2))),
            ),
            (
                Kind::BitArray(BitArrayInstruction::ListIndex {
                    list: BitArrayListLocalId(3),
                    index: 2,
                }),
                Some((BitArrayListLocalId(3).into(), Projection::List(2))),
            ),
            (
                Kind::UtfCodepoint(UtfCodepointInstruction::TupleIndex {
                    tuple: TupleLocalId(4),
                    index: 2,
                }),
                Some((TupleLocalId(4).into(), Projection::Tuple(2))),
            ),
            (
                Kind::UtfCodepoint(UtfCodepointInstruction::CustomField { source, index: 2 }),
                Some((CustomLocalId(7).into(), Projection::Custom(2))),
            ),
            (
                Kind::UtfCodepoint(UtfCodepointInstruction::ListIndex {
                    list: UtfCodepointListLocalId(3),
                    index: 2,
                }),
                Some((UtfCodepointListLocalId(3).into(), Projection::List(2))),
            ),
            (
                Kind::Custom(CustomInstruction::TupleIndex {
                    tuple: TupleLocalId(4),
                    index: 2,
                }),
                Some((TupleLocalId(4).into(), Projection::Tuple(2))),
            ),
            (
                Kind::Custom(CustomInstruction::CustomField { source, index: 2 }),
                Some((CustomLocalId(7).into(), Projection::Custom(2))),
            ),
            (
                Kind::Custom(CustomInstruction::ListIndex {
                    list: CustomListLocalId(3),
                    index: 2,
                }),
                Some((CustomListLocalId(3).into(), Projection::List(2))),
            ),
            (
                Kind::Bool(BoolInstruction::TupleIndex {
                    tuple: TupleLocalId(4),
                    index: 2,
                }),
                Some((TupleLocalId(4).into(), Projection::Tuple(2))),
            ),
            (
                Kind::Bool(BoolInstruction::CustomField { source, index: 2 }),
                Some((CustomLocalId(7).into(), Projection::Custom(2))),
            ),
            (
                Kind::Bool(BoolInstruction::ListIndex {
                    list: BoolListLocalId(3),
                    index: 2,
                }),
                Some((BoolListLocalId(3).into(), Projection::List(2))),
            ),
            (
                Kind::Nil(NilInstruction::TupleIndex {
                    tuple: TupleLocalId(4),
                    index: 2,
                }),
                Some((TupleLocalId(4).into(), Projection::Tuple(2))),
            ),
            (
                Kind::Nil(NilInstruction::CustomField { source, index: 2 }),
                Some((CustomLocalId(7).into(), Projection::Custom(2))),
            ),
            (
                Kind::Nil(NilInstruction::ListIndex {
                    list: NilListLocalId(3),
                    index: 2,
                }),
                Some((NilListLocalId(3).into(), Projection::List(2))),
            ),
            (
                Kind::Tuple(TupleInstruction::TupleIndex {
                    tuple: TupleLocalId(4),
                    index: 2,
                }),
                Some((TupleLocalId(4).into(), Projection::Tuple(2))),
            ),
            (
                Kind::Tuple(TupleInstruction::CustomField { source, index: 2 }),
                Some((CustomLocalId(7).into(), Projection::Custom(2))),
            ),
            (
                Kind::Tuple(TupleInstruction::ListIndex {
                    list: TupleListLocalId(3),
                    index: 2,
                }),
                Some((TupleListLocalId(3).into(), Projection::List(2))),
            ),
            (
                Kind::External(ExternalInstruction::TupleIndex {
                    tuple: TupleLocalId(4),
                    index: 2,
                }),
                Some((TupleLocalId(4).into(), Projection::Tuple(2))),
            ),
            (
                Kind::External(ExternalInstruction::CustomField { source, index: 2 }),
                Some((CustomLocalId(7).into(), Projection::Custom(2))),
            ),
            (
                Kind::External(ExternalInstruction::ListIndex {
                    list: ExternalListLocalId(3),
                    index: 2,
                }),
                Some((ExternalListLocalId(3).into(), Projection::List(2))),
            ),
        ];
        for (index, (value, expected)) in cases.iter().enumerate() {
            assert_eq!(instruction(value), *expected, "case {index}");
        }
        for value in [
            Kind::Int(IntInstruction::Value(num_bigint::BigInt::from(42).into())),
            Kind::Float(FloatInstruction::Value(1.5)),
            Kind::String(StringInstruction::Value("text".into())),
            Kind::Bool(BoolInstruction::Value(true)),
            Kind::Nil(NilInstruction::Value),
            Kind::Tuple(TupleInstruction::Value(Table::Static(&[]))),
        ] {
            assert_eq!(instruction(&value), None);
        }
    }

    #[test]
    fn list_projections_are_independent_of_the_list_element_storage_family() {
        type Kind = ProfiledInstructionKind<HostedExecutionGraph>;
        let source = CustomLocal {
            id: CustomLocalId(7),
            shape: CustomValueShape {
                type_id: CustomTypeId(2),
                shape_id: CustomValueShapeId(5),
            },
        };
        let parameter = ParameterListTypeId::new(ListTypeId(0), TypeParameterId(0));
        let cases = [
            (
                ListInstruction::Parameter(
                    parameter,
                    ParameterListInstruction::TupleIndex {
                        tuple: TupleLocalId(4),
                        index: 2,
                    },
                ),
                Some((TupleLocalId(4).into(), Projection::Tuple(2))),
            ),
            (
                ListInstruction::Parameter(
                    parameter,
                    ParameterListInstruction::CustomField { source, index: 2 },
                ),
                Some((CustomLocalId(7).into(), Projection::Custom(2))),
            ),
            (
                ListInstruction::Parameter(
                    parameter,
                    ParameterListInstruction::ListIndex {
                        list: ParameterListListLocalId(3),
                        index: 2,
                    },
                ),
                Some((ParameterListListLocalId(3).into(), Projection::List(2))),
            ),
            (
                ListInstruction::Parameter(parameter, ParameterListInstruction::Empty),
                None,
            ),
            (
                ListInstruction::ParameterList(
                    ParameterListListTypeId::new(ListTypeId(1), parameter),
                    TypedListInstruction::TupleIndex {
                        tuple: TupleLocalId(4),
                        index: 2,
                    },
                ),
                Some((TupleLocalId(4).into(), Projection::Tuple(2))),
            ),
            (
                ListInstruction::ParameterList(
                    ParameterListListTypeId::new(ListTypeId(1), parameter),
                    TypedListInstruction::CustomField { source, index: 2 },
                ),
                Some((CustomLocalId(7).into(), Projection::Custom(2))),
            ),
            (
                ListInstruction::ParameterList(
                    ParameterListListTypeId::new(ListTypeId(1), parameter),
                    TypedListInstruction::ListIndex {
                        list: ListListLocalId(3),
                        index: 2,
                    },
                ),
                Some((ListListLocalId(3).into(), Projection::List(2))),
            ),
            (
                ListInstruction::ParameterList(
                    ParameterListListTypeId::new(ListTypeId(1), parameter),
                    TypedListInstruction::Value(Table::Static(&[])),
                ),
                None,
            ),
            (
                ListInstruction::Int(
                    IntListTypeId::new(ListTypeId(2)),
                    TypedListInstruction::TupleIndex {
                        tuple: TupleLocalId(4),
                        index: 2,
                    },
                ),
                Some((TupleLocalId(4).into(), Projection::Tuple(2))),
            ),
            (
                ListInstruction::Int(
                    IntListTypeId::new(ListTypeId(2)),
                    TypedListInstruction::CustomField { source, index: 2 },
                ),
                Some((CustomLocalId(7).into(), Projection::Custom(2))),
            ),
            (
                ListInstruction::Int(
                    IntListTypeId::new(ListTypeId(2)),
                    TypedListInstruction::ListIndex {
                        list: ListListLocalId(3),
                        index: 2,
                    },
                ),
                Some((ListListLocalId(3).into(), Projection::List(2))),
            ),
            (
                ListInstruction::Int(
                    IntListTypeId::new(ListTypeId(2)),
                    TypedListInstruction::Value(Table::Static(&[])),
                ),
                None,
            ),
            (
                ListInstruction::String(
                    StringListTypeId::new(ListTypeId(3)),
                    TypedListInstruction::TupleIndex {
                        tuple: TupleLocalId(4),
                        index: 2,
                    },
                ),
                Some((TupleLocalId(4).into(), Projection::Tuple(2))),
            ),
            (
                ListInstruction::String(
                    StringListTypeId::new(ListTypeId(3)),
                    TypedListInstruction::CustomField { source, index: 2 },
                ),
                Some((CustomLocalId(7).into(), Projection::Custom(2))),
            ),
            (
                ListInstruction::String(
                    StringListTypeId::new(ListTypeId(3)),
                    TypedListInstruction::ListIndex {
                        list: ListListLocalId(3),
                        index: 2,
                    },
                ),
                Some((ListListLocalId(3).into(), Projection::List(2))),
            ),
            (
                ListInstruction::String(
                    StringListTypeId::new(ListTypeId(3)),
                    TypedListInstruction::Value(Table::Static(&[])),
                ),
                None,
            ),
            (
                ListInstruction::BitArray(
                    BitArrayListTypeId::new(ListTypeId(4)),
                    TypedListInstruction::TupleIndex {
                        tuple: TupleLocalId(4),
                        index: 2,
                    },
                ),
                Some((TupleLocalId(4).into(), Projection::Tuple(2))),
            ),
            (
                ListInstruction::BitArray(
                    BitArrayListTypeId::new(ListTypeId(4)),
                    TypedListInstruction::CustomField { source, index: 2 },
                ),
                Some((CustomLocalId(7).into(), Projection::Custom(2))),
            ),
            (
                ListInstruction::BitArray(
                    BitArrayListTypeId::new(ListTypeId(4)),
                    TypedListInstruction::ListIndex {
                        list: ListListLocalId(3),
                        index: 2,
                    },
                ),
                Some((ListListLocalId(3).into(), Projection::List(2))),
            ),
            (
                ListInstruction::BitArray(
                    BitArrayListTypeId::new(ListTypeId(4)),
                    TypedListInstruction::Value(Table::Static(&[])),
                ),
                None,
            ),
            (
                ListInstruction::UtfCodepoint(
                    UtfCodepointListTypeId::new(ListTypeId(5)),
                    TypedListInstruction::TupleIndex {
                        tuple: TupleLocalId(4),
                        index: 2,
                    },
                ),
                Some((TupleLocalId(4).into(), Projection::Tuple(2))),
            ),
            (
                ListInstruction::UtfCodepoint(
                    UtfCodepointListTypeId::new(ListTypeId(5)),
                    TypedListInstruction::CustomField { source, index: 2 },
                ),
                Some((CustomLocalId(7).into(), Projection::Custom(2))),
            ),
            (
                ListInstruction::UtfCodepoint(
                    UtfCodepointListTypeId::new(ListTypeId(5)),
                    TypedListInstruction::ListIndex {
                        list: ListListLocalId(3),
                        index: 2,
                    },
                ),
                Some((ListListLocalId(3).into(), Projection::List(2))),
            ),
            (
                ListInstruction::UtfCodepoint(
                    UtfCodepointListTypeId::new(ListTypeId(5)),
                    TypedListInstruction::Value(Table::Static(&[])),
                ),
                None,
            ),
            (
                ListInstruction::Custom(
                    CustomListTypeId::new(ListTypeId(6), CustomTypeId(2)),
                    TypedListInstruction::TupleIndex {
                        tuple: TupleLocalId(4),
                        index: 2,
                    },
                ),
                Some((TupleLocalId(4).into(), Projection::Tuple(2))),
            ),
            (
                ListInstruction::Custom(
                    CustomListTypeId::new(ListTypeId(6), CustomTypeId(2)),
                    TypedListInstruction::CustomField { source, index: 2 },
                ),
                Some((CustomLocalId(7).into(), Projection::Custom(2))),
            ),
            (
                ListInstruction::Custom(
                    CustomListTypeId::new(ListTypeId(6), CustomTypeId(2)),
                    TypedListInstruction::ListIndex {
                        list: ListListLocalId(3),
                        index: 2,
                    },
                ),
                Some((ListListLocalId(3).into(), Projection::List(2))),
            ),
            (
                ListInstruction::Custom(
                    CustomListTypeId::new(ListTypeId(6), CustomTypeId(2)),
                    TypedListInstruction::Value(Table::Static(&[])),
                ),
                None,
            ),
            (
                ListInstruction::Float(
                    FloatListTypeId::new(ListTypeId(7)),
                    TypedListInstruction::TupleIndex {
                        tuple: TupleLocalId(4),
                        index: 2,
                    },
                ),
                Some((TupleLocalId(4).into(), Projection::Tuple(2))),
            ),
            (
                ListInstruction::Float(
                    FloatListTypeId::new(ListTypeId(7)),
                    TypedListInstruction::CustomField { source, index: 2 },
                ),
                Some((CustomLocalId(7).into(), Projection::Custom(2))),
            ),
            (
                ListInstruction::Float(
                    FloatListTypeId::new(ListTypeId(7)),
                    TypedListInstruction::ListIndex {
                        list: ListListLocalId(3),
                        index: 2,
                    },
                ),
                Some((ListListLocalId(3).into(), Projection::List(2))),
            ),
            (
                ListInstruction::Float(
                    FloatListTypeId::new(ListTypeId(7)),
                    TypedListInstruction::Value(Table::Static(&[])),
                ),
                None,
            ),
            (
                ListInstruction::Bool(
                    BoolListTypeId::new(ListTypeId(8)),
                    TypedListInstruction::TupleIndex {
                        tuple: TupleLocalId(4),
                        index: 2,
                    },
                ),
                Some((TupleLocalId(4).into(), Projection::Tuple(2))),
            ),
            (
                ListInstruction::Bool(
                    BoolListTypeId::new(ListTypeId(8)),
                    TypedListInstruction::CustomField { source, index: 2 },
                ),
                Some((CustomLocalId(7).into(), Projection::Custom(2))),
            ),
            (
                ListInstruction::Bool(
                    BoolListTypeId::new(ListTypeId(8)),
                    TypedListInstruction::ListIndex {
                        list: ListListLocalId(3),
                        index: 2,
                    },
                ),
                Some((ListListLocalId(3).into(), Projection::List(2))),
            ),
            (
                ListInstruction::Bool(
                    BoolListTypeId::new(ListTypeId(8)),
                    TypedListInstruction::Value(Table::Static(&[])),
                ),
                None,
            ),
            (
                ListInstruction::Nil(
                    NilListTypeId::new(ListTypeId(9)),
                    TypedListInstruction::TupleIndex {
                        tuple: TupleLocalId(4),
                        index: 2,
                    },
                ),
                Some((TupleLocalId(4).into(), Projection::Tuple(2))),
            ),
            (
                ListInstruction::Nil(
                    NilListTypeId::new(ListTypeId(9)),
                    TypedListInstruction::CustomField { source, index: 2 },
                ),
                Some((CustomLocalId(7).into(), Projection::Custom(2))),
            ),
            (
                ListInstruction::Nil(
                    NilListTypeId::new(ListTypeId(9)),
                    TypedListInstruction::ListIndex {
                        list: ListListLocalId(3),
                        index: 2,
                    },
                ),
                Some((ListListLocalId(3).into(), Projection::List(2))),
            ),
            (
                ListInstruction::Nil(
                    NilListTypeId::new(ListTypeId(9)),
                    TypedListInstruction::Value(Table::Static(&[])),
                ),
                None,
            ),
            (
                ListInstruction::Tuple(
                    TupleListTypeId::new(ListTypeId(10), 0),
                    TypedListInstruction::TupleIndex {
                        tuple: TupleLocalId(4),
                        index: 2,
                    },
                ),
                Some((TupleLocalId(4).into(), Projection::Tuple(2))),
            ),
            (
                ListInstruction::Tuple(
                    TupleListTypeId::new(ListTypeId(10), 0),
                    TypedListInstruction::CustomField { source, index: 2 },
                ),
                Some((CustomLocalId(7).into(), Projection::Custom(2))),
            ),
            (
                ListInstruction::Tuple(
                    TupleListTypeId::new(ListTypeId(10), 0),
                    TypedListInstruction::ListIndex {
                        list: ListListLocalId(3),
                        index: 2,
                    },
                ),
                Some((ListListLocalId(3).into(), Projection::List(2))),
            ),
            (
                ListInstruction::Tuple(
                    TupleListTypeId::new(ListTypeId(10), 0),
                    TypedListInstruction::Value(Table::Static(&[])),
                ),
                None,
            ),
            (
                ListInstruction::List(
                    ListListTypeId::new(ListTypeId(11), ListTypeId(2)),
                    TypedListInstruction::TupleIndex {
                        tuple: TupleLocalId(4),
                        index: 2,
                    },
                ),
                Some((TupleLocalId(4).into(), Projection::Tuple(2))),
            ),
            (
                ListInstruction::List(
                    ListListTypeId::new(ListTypeId(11), ListTypeId(2)),
                    TypedListInstruction::CustomField { source, index: 2 },
                ),
                Some((CustomLocalId(7).into(), Projection::Custom(2))),
            ),
            (
                ListInstruction::List(
                    ListListTypeId::new(ListTypeId(11), ListTypeId(2)),
                    TypedListInstruction::ListIndex {
                        list: ListListLocalId(3),
                        index: 2,
                    },
                ),
                Some((ListListLocalId(3).into(), Projection::List(2))),
            ),
            (
                ListInstruction::List(
                    ListListTypeId::new(ListTypeId(11), ListTypeId(2)),
                    TypedListInstruction::Value(Table::Static(&[])),
                ),
                None,
            ),
            (
                ListInstruction::Function(
                    FunctionListTypeId::new(ListTypeId(12), 0),
                    TypedListInstruction::TupleIndex {
                        tuple: TupleLocalId(4),
                        index: 2,
                    },
                ),
                Some((TupleLocalId(4).into(), Projection::Tuple(2))),
            ),
            (
                ListInstruction::Function(
                    FunctionListTypeId::new(ListTypeId(12), 0),
                    TypedListInstruction::CustomField { source, index: 2 },
                ),
                Some((CustomLocalId(7).into(), Projection::Custom(2))),
            ),
            (
                ListInstruction::Function(
                    FunctionListTypeId::new(ListTypeId(12), 0),
                    TypedListInstruction::ListIndex {
                        list: ListListLocalId(3),
                        index: 2,
                    },
                ),
                Some((ListListLocalId(3).into(), Projection::List(2))),
            ),
            (
                ListInstruction::Function(
                    FunctionListTypeId::new(ListTypeId(12), 0),
                    TypedListInstruction::Value(Table::Static(&[])),
                ),
                None,
            ),
        ];
        for (index, (value, expected)) in cases.into_iter().enumerate() {
            assert_eq!(instruction(&Kind::List(value)), expected, "case {index}");
        }
        let type_id = ExternalListTypeId::new(ListTypeId(13), ExternalTypeId(0));
        for (value, expected) in [
            (
                TypedListInstruction::TupleIndex {
                    tuple: TupleLocalId(4),
                    index: 2,
                },
                Some((TupleLocalId(4).into(), Projection::Tuple(2))),
            ),
            (
                TypedListInstruction::CustomField { source, index: 2 },
                Some((CustomLocalId(7).into(), Projection::Custom(2))),
            ),
            (
                TypedListInstruction::ListIndex {
                    list: ListListLocalId(3),
                    index: 2,
                },
                Some((ListListLocalId(3).into(), Projection::List(2))),
            ),
            (TypedListInstruction::Value(Table::Static(&[])), None),
        ] {
            assert_eq!(
                instruction(&Kind::ExternalList(ExternalListInstruction {
                    type_id,
                    instruction: value
                })),
                expected
            );
        }
    }

    #[test]
    fn function_reads_are_projections_but_constructor_references_are_not() {
        let source = CustomLocal {
            id: CustomLocalId(7),
            shape: CustomValueShape {
                type_id: CustomTypeId(2),
                shape_id: CustomValueShapeId(5),
            },
        };
        for (value, expected) in [
            (
                FunctionInstructionKind::TupleIndex {
                    tuple: TupleLocalId(4),
                    index: 2,
                },
                Some((TupleLocalId(4).into(), Projection::Tuple(2))),
            ),
            (
                FunctionInstructionKind::CustomField { source, index: 2 },
                Some((CustomLocalId(7).into(), Projection::Custom(2))),
            ),
            (
                FunctionInstructionKind::ListIndex {
                    list: FunctionListLocalId(3),
                    index: 2,
                },
                Some((FunctionListLocalId(3).into(), Projection::List(2))),
            ),
            (
                FunctionInstructionKind::Constructor(CustomConstructorId {
                    type_id: CustomTypeId(2),
                    index: 1,
                }),
                None,
            ),
        ] {
            assert_eq!(function(&value), expected);
        }
    }
}
