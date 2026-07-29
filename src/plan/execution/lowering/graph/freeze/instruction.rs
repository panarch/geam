use super::super::draft::instruction::{
    DraftBitArrayBitsSize, DraftBitArrayEvaluatedSize, DraftBitArrayInstruction,
    DraftBitArraySegment, DraftBoolInstruction, DraftCustomInstruction, DraftFloatInstruction,
    DraftFunctionInstruction, DraftIntInstruction, DraftListInstruction, DraftNilInstruction,
    DraftParameterListInstruction, DraftStringInstruction, DraftTupleInstruction,
    DraftTypedListInstruction, DraftUtfCodepointInstruction,
};
use super::super::draft::{DraftInstruction, DraftList};
use super::value::BlockValues;
use crate::plan::execution;

pub(super) fn freeze(
    instruction: &DraftInstruction,
    values: &BlockValues,
    context: &mut super::super::super::LoweringContext,
) -> execution::graph::Instruction {
    use execution::graph::InstructionKind as K;

    let (output, kind) = match instruction {
        DraftInstruction::Int { output, kind } => {
            (output.erase(), K::Int(freeze_int(kind, values)))
        }
        DraftInstruction::Float { output, kind } => {
            (output.erase(), K::Float(freeze_float(kind, values)))
        }
        DraftInstruction::String { output, kind } => {
            (output.erase(), K::String(freeze_string(kind, values)))
        }
        DraftInstruction::BitArray { output, kind } => {
            (output.erase(), K::BitArray(freeze_bit_array(kind, values)))
        }
        DraftInstruction::UtfCodepoint { output, kind } => (
            output.erase(),
            K::UtfCodepoint(freeze_utf_codepoint(kind, values)),
        ),
        DraftInstruction::Custom { output, kind } => {
            (output.erase(), K::Custom(freeze_custom(kind, values)))
        }
        DraftInstruction::Bool { output, kind } => {
            (output.erase(), K::Bool(freeze_bool(kind, values)))
        }
        DraftInstruction::Nil { output, kind } => {
            (output.erase(), K::Nil(freeze_nil(kind, values)))
        }
        DraftInstruction::Tuple { output, kind } => {
            (output.erase(), K::Tuple(freeze_tuple(kind, values)))
        }
        DraftInstruction::List { output, kind } => {
            (output.erase(), K::List(freeze_list(kind, values)))
        }
        DraftInstruction::Function {
            output,
            shape,
            kind,
        } => {
            let type_ = context.types.function_type(shape);
            let family = function_return_family(shape, &context.representations);
            (
                output.erase(),
                K::Function(execution::graph::FunctionInstruction::new(
                    type_,
                    family,
                    freeze_function(kind, values),
                )),
            )
        }
    };
    execution::graph::Instruction::new(values.slot(&output), kind)
}

fn function_return_family(
    shape: &super::super::super::specialization::SpecializedFunctionShape,
    representations: &super::super::super::specialization::RepresentationContext,
) -> execution::function::FunctionReturnFamily {
    use super::super::super::specialization::{FunctionRepresentation, StoredValueShape};
    use execution::function::FunctionReturnFamily as F;

    match shape.representation(representations) {
        FunctionRepresentation::Symbolic => F::Generic,
        FunctionRepresentation::Never(_) => F::Never,
        FunctionRepresentation::Executable(return_) => match return_ {
            StoredValueShape::Int => F::Int,
            StoredValueShape::Float => F::Float,
            StoredValueShape::String => F::String,
            StoredValueShape::BitArray => F::BitArray,
            StoredValueShape::UtfCodepoint => F::UtfCodepoint,
            StoredValueShape::Custom(_) => F::Custom,
            StoredValueShape::Bool => F::Bool,
            StoredValueShape::Nil => F::Nil,
            StoredValueShape::Tuple(_) => F::Tuple,
            StoredValueShape::List(_) => F::List,
            StoredValueShape::Function(_) => F::Function,
        },
    }
}

fn freeze_int(
    instruction: &DraftIntInstruction,
    values: &BlockValues,
) -> execution::graph::IntInstruction {
    use execution::graph::IntInstruction as E;

    match instruction {
        DraftIntInstruction::Value(value) => E::Value(value.clone()),
        DraftIntInstruction::Constant(id) => E::Constant(*id),
        DraftIntInstruction::Call {
            function,
            args,
            site,
        } => E::Call {
            function: *function,
            args: values.any_slice(args),
            site: site.clone(),
        },
        DraftIntInstruction::FunctionCall {
            function,
            args,
            site,
        } => E::FunctionCall {
            function: values.int_function(function),
            args: values.any_slice(args),
            site: site.clone(),
        },
        DraftIntInstruction::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: values.tuple(tuple),
            index: *index,
        },
        DraftIntInstruction::CustomField { source, index } => E::CustomField {
            source: values.custom(source),
            index: *index,
        },
        DraftIntInstruction::ListIndex { list, index } => E::ListIndex {
            list: values.int_list(list),
            index: *index,
        },
        DraftIntInstruction::Add { left, right } => E::Add {
            left: values.int(left),
            right: values.int(right),
        },
        DraftIntInstruction::Sub { left, right } => E::Sub {
            left: values.int(left),
            right: values.int(right),
        },
        DraftIntInstruction::Mult { left, right } => E::Mult {
            left: values.int(left),
            right: values.int(right),
        },
        DraftIntInstruction::Div { left, right } => E::Div {
            left: values.int(left),
            right: values.int(right),
        },
        DraftIntInstruction::Remainder { left, right } => E::Remainder {
            left: values.int(left),
            right: values.int(right),
        },
        DraftIntInstruction::Negate(value) => E::Negate(values.int(value)),
    }
}

fn freeze_float(
    instruction: &DraftFloatInstruction,
    values: &BlockValues,
) -> execution::graph::FloatInstruction {
    use execution::graph::FloatInstruction as E;

    match instruction {
        DraftFloatInstruction::Value(value) => E::Value(*value),
        DraftFloatInstruction::Constant(id) => E::Constant(*id),
        DraftFloatInstruction::Call {
            function,
            args,
            site,
        } => E::Call {
            function: *function,
            args: values.any_slice(args),
            site: site.clone(),
        },
        DraftFloatInstruction::FunctionCall {
            function,
            args,
            site,
        } => E::FunctionCall {
            function: values.float_function(function),
            args: values.any_slice(args),
            site: site.clone(),
        },
        DraftFloatInstruction::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: values.tuple(tuple),
            index: *index,
        },
        DraftFloatInstruction::CustomField { source, index } => E::CustomField {
            source: values.custom(source),
            index: *index,
        },
        DraftFloatInstruction::ListIndex { list, index } => E::ListIndex {
            list: values.float_list(list),
            index: *index,
        },
        DraftFloatInstruction::Add { left, right } => E::Add {
            left: values.float(left),
            right: values.float(right),
        },
        DraftFloatInstruction::Sub { left, right } => E::Sub {
            left: values.float(left),
            right: values.float(right),
        },
        DraftFloatInstruction::Mult { left, right } => E::Mult {
            left: values.float(left),
            right: values.float(right),
        },
        DraftFloatInstruction::Div { left, right } => E::Div {
            left: values.float(left),
            right: values.float(right),
        },
    }
}

fn freeze_string(
    instruction: &DraftStringInstruction,
    values: &BlockValues,
) -> execution::graph::StringInstruction {
    use execution::graph::StringInstruction as E;

    match instruction {
        DraftStringInstruction::Value(value) => E::Value(value.clone()),
        DraftStringInstruction::Constant(id) => E::Constant(*id),
        DraftStringInstruction::Call {
            function,
            args,
            site,
        } => E::Call {
            function: *function,
            args: values.any_slice(args),
            site: site.clone(),
        },
        DraftStringInstruction::FunctionCall {
            function,
            args,
            site,
        } => E::FunctionCall {
            function: values.string_function(function),
            args: values.any_slice(args),
            site: site.clone(),
        },
        DraftStringInstruction::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: values.tuple(tuple),
            index: *index,
        },
        DraftStringInstruction::CustomField { source, index } => E::CustomField {
            source: values.custom(source),
            index: *index,
        },
        DraftStringInstruction::ListIndex { list, index } => E::ListIndex {
            list: values.string_list(list),
            index: *index,
        },
        DraftStringInstruction::Concatenate { left, right } => E::Concatenate {
            left: values.string(left),
            right: values.string(right),
        },
        DraftStringInstruction::DropPrefix { value, prefix } => E::DropPrefix {
            value: values.string(value),
            prefix: prefix.clone(),
        },
    }
}

fn freeze_bit_array(
    instruction: &DraftBitArrayInstruction,
    values: &BlockValues,
) -> execution::graph::BitArrayInstruction {
    use execution::graph::BitArrayInstruction as E;

    match instruction {
        DraftBitArrayInstruction::Value(segments) => E::Value(
            segments
                .iter()
                .map(|segment| freeze_bit_array_segment(segment, values))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        ),
        DraftBitArrayInstruction::Constant(id) => E::Constant(*id),
        DraftBitArrayInstruction::Call {
            function,
            args,
            site,
        } => E::Call {
            function: *function,
            args: values.any_slice(args),
            site: site.clone(),
        },
        DraftBitArrayInstruction::FunctionCall {
            function,
            args,
            site,
        } => E::FunctionCall {
            function: values.bit_array_function(function),
            args: values.any_slice(args),
            site: site.clone(),
        },
        DraftBitArrayInstruction::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: values.tuple(tuple),
            index: *index,
        },
        DraftBitArrayInstruction::CustomField { source, index } => E::CustomField {
            source: values.custom(source),
            index: *index,
        },
        DraftBitArrayInstruction::ListIndex { list, index } => E::ListIndex {
            list: values.bit_array_list(list),
            index: *index,
        },
    }
}

fn freeze_bit_array_segment(
    segment: &DraftBitArraySegment,
    values: &BlockValues,
) -> execution::graph::BitArraySegment {
    use execution::graph::BitArraySegment as E;

    match segment {
        DraftBitArraySegment::Int {
            value,
            bit_size,
            endianness,
        } => E::Int {
            value: values.int(value),
            bit_size: *bit_size,
            endianness: *endianness,
        },
        DraftBitArraySegment::EvaluatedInt {
            value,
            size,
            endianness,
            site,
        } => E::EvaluatedInt {
            value: values.int(value),
            size: freeze_bit_array_size(size, values),
            endianness: *endianness,
            site: site.clone(),
        },
        DraftBitArraySegment::Float {
            value,
            bit_size,
            endianness,
        } => E::Float {
            value: values.float(value),
            bit_size: *bit_size,
            endianness: *endianness,
        },
        DraftBitArraySegment::EvaluatedFloat {
            value,
            size,
            endianness,
            site,
        } => E::EvaluatedFloat {
            value: values.float(value),
            size: freeze_bit_array_size(size, values),
            endianness: *endianness,
            site: site.clone(),
        },
        DraftBitArraySegment::String { value, encoding } => E::String {
            value: values.string(value),
            encoding: *encoding,
        },
        DraftBitArraySegment::UtfCodepoint { value, encoding } => E::UtfCodepoint {
            value: values.utf_codepoint(value),
            encoding: *encoding,
        },
        DraftBitArraySegment::Bits(value) => E::Bits(values.bit_array(value)),
        DraftBitArraySegment::SizedBits { value, size, site } => E::SizedBits {
            value: values.bit_array(value),
            size: match size {
                DraftBitArrayBitsSize::Fixed(size) => {
                    execution::graph::BitArrayBitsSize::Fixed(*size)
                }
                DraftBitArrayBitsSize::Evaluated(size) => {
                    execution::graph::BitArrayBitsSize::Evaluated(freeze_bit_array_size(
                        size, values,
                    ))
                }
            },
            site: site.clone(),
        },
    }
}

fn freeze_bit_array_size(
    size: &DraftBitArrayEvaluatedSize,
    values: &BlockValues,
) -> execution::graph::BitArrayEvaluatedSize {
    execution::graph::BitArrayEvaluatedSize::new(values.int(&size.value), size.unit)
}

fn freeze_utf_codepoint(
    instruction: &DraftUtfCodepointInstruction,
    values: &BlockValues,
) -> execution::graph::UtfCodepointInstruction {
    use execution::graph::UtfCodepointInstruction as E;

    match instruction {
        DraftUtfCodepointInstruction::Call {
            function,
            args,
            site,
        } => E::Call {
            function: *function,
            args: values.any_slice(args),
            site: site.clone(),
        },
        DraftUtfCodepointInstruction::FunctionCall {
            function,
            args,
            site,
        } => E::FunctionCall {
            function: values.utf_codepoint_function(function),
            args: values.any_slice(args),
            site: site.clone(),
        },
        DraftUtfCodepointInstruction::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: values.tuple(tuple),
            index: *index,
        },
        DraftUtfCodepointInstruction::CustomField { source, index } => E::CustomField {
            source: values.custom(source),
            index: *index,
        },
        DraftUtfCodepointInstruction::ListIndex { list, index } => E::ListIndex {
            list: values.utf_codepoint_list(list),
            index: *index,
        },
    }
}

fn freeze_custom(
    instruction: &DraftCustomInstruction,
    values: &BlockValues,
) -> execution::graph::CustomInstruction {
    use execution::graph::CustomInstruction as E;

    match instruction {
        DraftCustomInstruction::Construct {
            constructor,
            fields,
        } => E::Construct {
            constructor: *constructor,
            fields: values.any_slice(fields),
        },
        DraftCustomInstruction::Constant(id) => E::Constant(*id),
        DraftCustomInstruction::Call {
            function,
            args,
            site,
        } => E::Call {
            function: *function,
            args: values.any_slice(args),
            site: site.clone(),
        },
        DraftCustomInstruction::FunctionCall {
            function,
            args,
            site,
        } => E::FunctionCall {
            function: values.custom_function(function),
            args: values.any_slice(args),
            site: site.clone(),
        },
        DraftCustomInstruction::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: values.tuple(tuple),
            index: *index,
        },
        DraftCustomInstruction::CustomField { source, index } => E::CustomField {
            source: values.custom(source),
            index: *index,
        },
        DraftCustomInstruction::ListIndex { list, index } => E::ListIndex {
            list: values.custom_list(list),
            index: *index,
        },
    }
}

fn freeze_bool(
    instruction: &DraftBoolInstruction,
    values: &BlockValues,
) -> execution::graph::BoolInstruction {
    use execution::graph::BoolInstruction as E;

    match instruction {
        DraftBoolInstruction::Value(value) => E::Value(*value),
        DraftBoolInstruction::Constant(id) => E::Constant(*id),
        DraftBoolInstruction::Call {
            function,
            args,
            site,
        } => E::Call {
            function: *function,
            args: values.any_slice(args),
            site: site.clone(),
        },
        DraftBoolInstruction::FunctionCall {
            function,
            args,
            site,
        } => E::FunctionCall {
            function: values.bool_function(function),
            args: values.any_slice(args),
            site: site.clone(),
        },
        DraftBoolInstruction::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: values.tuple(tuple),
            index: *index,
        },
        DraftBoolInstruction::CustomField { source, index } => E::CustomField {
            source: values.custom(source),
            index: *index,
        },
        DraftBoolInstruction::ListIndex { list, index } => E::ListIndex {
            list: values.bool_list(list),
            index: *index,
        },
        DraftBoolInstruction::Not(value) => E::Not(values.bool(value)),
        DraftBoolInstruction::LtInt { left, right } => E::LtInt {
            left: values.int(left),
            right: values.int(right),
        },
        DraftBoolInstruction::LtEqInt { left, right } => E::LtEqInt {
            left: values.int(left),
            right: values.int(right),
        },
        DraftBoolInstruction::GtInt { left, right } => E::GtInt {
            left: values.int(left),
            right: values.int(right),
        },
        DraftBoolInstruction::GtEqInt { left, right } => E::GtEqInt {
            left: values.int(left),
            right: values.int(right),
        },
        DraftBoolInstruction::LtFloat { left, right } => E::LtFloat {
            left: values.float(left),
            right: values.float(right),
        },
        DraftBoolInstruction::LtEqFloat { left, right } => E::LtEqFloat {
            left: values.float(left),
            right: values.float(right),
        },
        DraftBoolInstruction::GtFloat { left, right } => E::GtFloat {
            left: values.float(left),
            right: values.float(right),
        },
        DraftBoolInstruction::GtEqFloat { left, right } => E::GtEqFloat {
            left: values.float(left),
            right: values.float(right),
        },
        DraftBoolInstruction::Equal { left, right } => E::Equal {
            left: values.any(left),
            right: values.any(right),
        },
        DraftBoolInstruction::NotEqual { left, right } => E::NotEqual {
            left: values.any(left),
            right: values.any(right),
        },
        DraftBoolInstruction::StringStartsWith { value, prefix } => E::StringStartsWith {
            value: values.string(value),
            prefix: prefix.clone(),
        },
        DraftBoolInstruction::ListLengthEquals { value, length } => E::ListLengthEquals {
            value: values.list(value),
            length: *length,
        },
        DraftBoolInstruction::ListLengthAtLeast { value, length } => E::ListLengthAtLeast {
            value: values.list(value),
            length: *length,
        },
    }
}

fn freeze_nil(
    instruction: &DraftNilInstruction,
    values: &BlockValues,
) -> execution::graph::NilInstruction {
    use execution::graph::NilInstruction as E;

    match instruction {
        DraftNilInstruction::Value => E::Value,
        DraftNilInstruction::Constant(id) => E::Constant(*id),
        DraftNilInstruction::Call {
            function,
            args,
            site,
        } => E::Call {
            function: *function,
            args: values.any_slice(args),
            site: site.clone(),
        },
        DraftNilInstruction::FunctionCall {
            function,
            args,
            site,
        } => E::FunctionCall {
            function: values.nil_function(function),
            args: values.any_slice(args),
            site: site.clone(),
        },
        DraftNilInstruction::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: values.tuple(tuple),
            index: *index,
        },
        DraftNilInstruction::CustomField { source, index } => E::CustomField {
            source: values.custom(source),
            index: *index,
        },
        DraftNilInstruction::ListIndex { list, index } => E::ListIndex {
            list: values.nil_list(list),
            index: *index,
        },
    }
}

fn freeze_tuple(
    instruction: &DraftTupleInstruction,
    values: &BlockValues,
) -> execution::graph::TupleInstruction {
    use execution::graph::TupleInstruction as E;

    match instruction {
        DraftTupleInstruction::Value(elements) => E::Value(values.any_slice(elements)),
        DraftTupleInstruction::Constant(id) => E::Constant(*id),
        DraftTupleInstruction::Call {
            function,
            args,
            site,
        } => E::Call {
            function: *function,
            args: values.any_slice(args),
            site: site.clone(),
        },
        DraftTupleInstruction::FunctionCall {
            function,
            args,
            site,
        } => E::FunctionCall {
            function: values.tuple_function(function),
            args: values.any_slice(args),
            site: site.clone(),
        },
        DraftTupleInstruction::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: values.tuple(tuple),
            index: *index,
        },
        DraftTupleInstruction::CustomField { source, index } => E::CustomField {
            source: values.custom(source),
            index: *index,
        },
        DraftTupleInstruction::ListIndex { list, index } => E::ListIndex {
            list: values.tuple_list(list),
            index: *index,
        },
    }
}

fn freeze_parameter_list(
    instruction: &DraftParameterListInstruction,
    values: &BlockValues,
) -> execution::graph::ParameterListInstruction {
    use execution::graph::ParameterListInstruction as E;

    match instruction {
        DraftParameterListInstruction::Empty => E::Empty,
        DraftParameterListInstruction::Constant(id) => E::Constant(*id),
        DraftParameterListInstruction::Call {
            function,
            args,
            site,
        } => E::Call {
            function: *function,
            args: values.any_slice(args),
            site: site.clone(),
        },
        DraftParameterListInstruction::FunctionCall {
            function,
            args,
            site,
        } => E::FunctionCall {
            function: values.list_function(function),
            args: values.any_slice(args),
            site: site.clone(),
        },
        DraftParameterListInstruction::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: values.tuple(tuple),
            index: *index,
        },
        DraftParameterListInstruction::CustomField { source, index } => E::CustomField {
            source: values.custom(source),
            index: *index,
        },
        DraftParameterListInstruction::ListIndex { list, index } => E::ListIndex {
            list: values.parameter_list_list(list),
            index: *index,
        },
    }
}

fn freeze_list(
    instruction: &DraftListInstruction,
    values: &BlockValues,
) -> execution::graph::ListInstruction {
    use execution::graph::ListInstruction as E;

    match instruction {
        DraftListInstruction::Parameter(type_id, instruction) => {
            E::Parameter(*type_id, freeze_parameter_list(instruction, values))
        }
        DraftListInstruction::ParameterList(type_id, instruction) => E::ParameterList(
            *type_id,
            freeze_typed_list(
                instruction,
                values,
                BlockValues::parameter_list,
                BlockValues::parameter_list_list,
            ),
        ),
        DraftListInstruction::Int(type_id, instruction) => E::Int(
            *type_id,
            freeze_typed_list(instruction, values, BlockValues::int, BlockValues::int_list),
        ),
        DraftListInstruction::String(type_id, instruction) => E::String(
            *type_id,
            freeze_typed_list(
                instruction,
                values,
                BlockValues::string,
                BlockValues::string_list,
            ),
        ),
        DraftListInstruction::BitArray(type_id, instruction) => E::BitArray(
            *type_id,
            freeze_typed_list(
                instruction,
                values,
                BlockValues::bit_array,
                BlockValues::bit_array_list,
            ),
        ),
        DraftListInstruction::UtfCodepoint(type_id, instruction) => E::UtfCodepoint(
            *type_id,
            freeze_typed_list(
                instruction,
                values,
                BlockValues::utf_codepoint,
                BlockValues::utf_codepoint_list,
            ),
        ),
        DraftListInstruction::Custom(type_id, instruction) => E::Custom(
            *type_id,
            freeze_typed_list(
                instruction,
                values,
                BlockValues::custom,
                BlockValues::custom_list,
            ),
        ),
        DraftListInstruction::Float(type_id, instruction) => E::Float(
            *type_id,
            freeze_typed_list(
                instruction,
                values,
                BlockValues::float,
                BlockValues::float_list,
            ),
        ),
        DraftListInstruction::Bool(type_id, instruction) => E::Bool(
            *type_id,
            freeze_typed_list(
                instruction,
                values,
                BlockValues::bool,
                BlockValues::bool_list,
            ),
        ),
        DraftListInstruction::Nil(type_id, instruction) => E::Nil(
            *type_id,
            freeze_typed_list(instruction, values, BlockValues::nil, BlockValues::nil_list),
        ),
        DraftListInstruction::Tuple(type_id, instruction) => E::Tuple(
            *type_id,
            freeze_typed_list(
                instruction,
                values,
                BlockValues::tuple,
                BlockValues::tuple_list,
            ),
        ),
        DraftListInstruction::List(type_id, instruction) => E::List(
            *type_id,
            freeze_typed_list(
                instruction,
                values,
                BlockValues::stored_list,
                BlockValues::list_list,
            ),
        ),
        DraftListInstruction::Function(type_id, instruction) => E::Function(
            *type_id,
            freeze_typed_list(
                instruction,
                values,
                BlockValues::function,
                BlockValues::function_list,
            ),
        ),
    }
}

fn freeze_typed_list<Element, FinalElement, Local, Function>(
    instruction: &DraftTypedListInstruction<Element, Local, Function>,
    values: &BlockValues,
    element: fn(&BlockValues, &Element) -> FinalElement,
    list: fn(&BlockValues, &DraftList) -> Local,
) -> execution::graph::TypedListInstruction<FinalElement, Local, Function>
where
    Local: Copy,
    Function: Clone,
{
    use execution::graph::TypedListInstruction as E;

    match instruction {
        DraftTypedListInstruction::Value(elements) => E::Value(
            elements
                .iter()
                .map(|value| element(values, value))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        ),
        DraftTypedListInstruction::Constant(id) => E::Constant(*id),
        DraftTypedListInstruction::Spread { elements, tail } => E::Spread {
            elements: elements
                .iter()
                .map(|value| element(values, value))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            tail: list(values, tail),
        },
        DraftTypedListInstruction::Call {
            function,
            args,
            site,
        } => E::Call {
            function: function.clone(),
            args: values.any_slice(args),
            site: site.clone(),
        },
        DraftTypedListInstruction::FunctionCall {
            function,
            args,
            site,
        } => E::FunctionCall {
            function: values.list_function(function),
            args: values.any_slice(args),
            site: site.clone(),
        },
        DraftTypedListInstruction::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: values.tuple(tuple),
            index: *index,
        },
        DraftTypedListInstruction::CustomField { source, index } => E::CustomField {
            source: values.custom(source),
            index: *index,
        },
        DraftTypedListInstruction::ListIndex {
            list: source,
            index,
        } => E::ListIndex {
            list: values.list_list(source),
            index: *index,
        },
        DraftTypedListInstruction::DropFirst {
            list: source,
            count,
        } => E::DropFirst {
            list: list(values, source),
            count: *count,
        },
    }
}

fn freeze_function(
    instruction: &DraftFunctionInstruction,
    values: &BlockValues,
) -> execution::graph::FunctionInstructionKind {
    use execution::graph::FunctionInstructionKind as E;

    match instruction {
        DraftFunctionInstruction::Constant(id) => E::Constant(*id),
        DraftFunctionInstruction::Reference(target) => E::Reference(target.clone()),
        DraftFunctionInstruction::Closure { target, captures } => E::Closure {
            target: target.clone(),
            captures: captures
                .iter()
                .map(|capture| values.capture(&capture.target, &capture.source))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        },
        DraftFunctionInstruction::Constructor(constructor) => E::Constructor(*constructor),
        DraftFunctionInstruction::Call {
            function,
            args,
            site,
        } => E::Call {
            function: function.clone(),
            args: values.any_slice(args),
            site: site.clone(),
        },
        DraftFunctionInstruction::FunctionCall {
            function,
            args,
            site,
        } => E::FunctionCall {
            function: values.function_function(function),
            args: values.any_slice(args),
            site: site.clone(),
        },
        DraftFunctionInstruction::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: values.tuple(tuple),
            index: *index,
        },
        DraftFunctionInstruction::CustomField { source, index } => E::CustomField {
            source: values.custom(source),
            index: *index,
        },
        DraftFunctionInstruction::ListIndex { list, index } => E::ListIndex {
            list: values.function_list(list),
            index: *index,
        },
    }
}
