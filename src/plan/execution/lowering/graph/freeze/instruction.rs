use super::super::draft::instruction::{
    DraftBitArrayBitsSize, DraftBitArrayEvaluatedSize, DraftBitArrayInstruction,
    DraftBitArraySegment, DraftBoolInstruction, DraftCustomInstruction, DraftExternalInstruction,
    DraftFloatInstruction, DraftFunctionInstruction, DraftFunctionTarget, DraftIntInstruction,
    DraftListInstruction, DraftNilInstruction, DraftParameterListInstruction,
    DraftStringInstruction, DraftTupleInstruction, DraftTypedListInstruction,
    DraftUtfCodepointInstruction,
};
use super::super::draft::{DraftFunction, DraftInstruction, DraftList};
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
        DraftInstruction::External { output, kind } => {
            (output.erase(), K::External(freeze_external(kind, values)))
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
        DraftInstruction::List { output, kind } => (output.erase(), freeze_list(kind, values)),
        DraftInstruction::Function {
            output,
            shape,
            kind,
        } => {
            let type_ = context.types.function_type(shape);
            let family = function_return_family(shape, &context.representations);
            (output.erase(), freeze_function(kind, values, type_, family))
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
            StoredValueShape::External(_) => F::External,
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

fn freeze_external(
    instruction: &DraftExternalInstruction,
    values: &BlockValues,
) -> execution::graph::ExternalInstruction {
    use execution::graph::ExternalInstruction as E;

    match instruction {
        DraftExternalInstruction::Call {
            function,
            args,
            site,
        } => E::Call {
            function: *function,
            args: values.any_slice(args),
            site: site.clone(),
        },
        DraftExternalInstruction::FunctionCall {
            function,
            args,
            site,
        } => E::FunctionCall {
            function: values.external_function(function),
            args: values.any_slice(args),
            site: site.clone(),
        },
        DraftExternalInstruction::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: values.tuple(tuple),
            index: *index,
        },
        DraftExternalInstruction::CustomField { source, index } => E::CustomField {
            source: values.custom(source),
            index: *index,
        },
        DraftExternalInstruction::ListIndex { list, index } => E::ListIndex {
            list: values.external_list(list),
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
) -> execution::graph::InstructionKind {
    use execution::graph::InstructionKind as K;
    use execution::graph::ListInstruction as E;

    match instruction {
        DraftListInstruction::Parameter(type_id, instruction) => K::List(E::Parameter(
            *type_id,
            freeze_parameter_list(instruction, values),
        )),
        DraftListInstruction::ParameterList(type_id, instruction) => K::List(E::ParameterList(
            *type_id,
            freeze_typed_list(
                instruction,
                values,
                BlockValues::parameter_list,
                BlockValues::parameter_list_list,
            ),
        )),
        DraftListInstruction::Int(type_id, instruction) => K::List(E::Int(
            *type_id,
            freeze_typed_list(instruction, values, BlockValues::int, BlockValues::int_list),
        )),
        DraftListInstruction::String(type_id, instruction) => K::List(E::String(
            *type_id,
            freeze_typed_list(
                instruction,
                values,
                BlockValues::string,
                BlockValues::string_list,
            ),
        )),
        DraftListInstruction::BitArray(type_id, instruction) => K::List(E::BitArray(
            *type_id,
            freeze_typed_list(
                instruction,
                values,
                BlockValues::bit_array,
                BlockValues::bit_array_list,
            ),
        )),
        DraftListInstruction::UtfCodepoint(type_id, instruction) => K::List(E::UtfCodepoint(
            *type_id,
            freeze_typed_list(
                instruction,
                values,
                BlockValues::utf_codepoint,
                BlockValues::utf_codepoint_list,
            ),
        )),
        DraftListInstruction::Custom(type_id, instruction) => K::List(E::Custom(
            *type_id,
            freeze_typed_list(
                instruction,
                values,
                BlockValues::custom,
                BlockValues::custom_list,
            ),
        )),
        DraftListInstruction::External(type_id, instruction) => {
            K::ExternalList(execution::graph::ExternalListInstruction::new(
                *type_id,
                freeze_typed_list_with_function_local(
                    instruction,
                    values,
                    BlockValues::external,
                    BlockValues::external_list,
                    BlockValues::external_list_function,
                ),
            ))
        }
        DraftListInstruction::Float(type_id, instruction) => K::List(E::Float(
            *type_id,
            freeze_typed_list(
                instruction,
                values,
                BlockValues::float,
                BlockValues::float_list,
            ),
        )),
        DraftListInstruction::Bool(type_id, instruction) => K::List(E::Bool(
            *type_id,
            freeze_typed_list(
                instruction,
                values,
                BlockValues::bool,
                BlockValues::bool_list,
            ),
        )),
        DraftListInstruction::Nil(type_id, instruction) => K::List(E::Nil(
            *type_id,
            freeze_typed_list(instruction, values, BlockValues::nil, BlockValues::nil_list),
        )),
        DraftListInstruction::Tuple(type_id, instruction) => K::List(E::Tuple(
            *type_id,
            freeze_typed_list(
                instruction,
                values,
                BlockValues::tuple,
                BlockValues::tuple_list,
            ),
        )),
        DraftListInstruction::List(type_id, instruction) => K::List(E::List(
            *type_id,
            freeze_typed_list(
                instruction,
                values,
                BlockValues::stored_list,
                BlockValues::list_list,
            ),
        )),
        DraftListInstruction::Function(type_id, instruction) => K::List(E::Function(
            *type_id,
            freeze_typed_list(
                instruction,
                values,
                BlockValues::function,
                BlockValues::function_list,
            ),
        )),
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
    freeze_typed_list_with_function_local(
        instruction,
        values,
        element,
        list,
        BlockValues::list_function,
    )
}

fn freeze_typed_list_with_function_local<Element, FinalElement, Local, Function, FunctionLocal>(
    instruction: &DraftTypedListInstruction<Element, Local, Function>,
    values: &BlockValues,
    element: fn(&BlockValues, &Element) -> FinalElement,
    list: fn(&BlockValues, &DraftList) -> Local,
    function_local: fn(&BlockValues, &DraftFunction) -> FunctionLocal,
) -> execution::graph::TypedListInstruction<FinalElement, Local, Function, FunctionLocal>
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
            function: function_local(values, function),
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
    type_: execution::type_::FunctionType,
    family: execution::function::FunctionReturnFamily,
) -> execution::graph::InstructionKind {
    use execution::graph::{ExternalFunctionInstructionKind as X, FunctionInstructionKind as F};

    match instruction {
        DraftFunctionInstruction::Constant(id) => {
            function_instruction(type_, family, F::Constant(*id))
        }
        DraftFunctionInstruction::Reference(target) => match freeze_function_target(target) {
            FrozenFunctionTarget::Function(target) => {
                function_instruction(type_, family, F::Reference(target))
            }
            FrozenFunctionTarget::External(target) => {
                external_function_instruction(type_, family, X::Reference(target))
            }
        },
        DraftFunctionInstruction::Closure { target, captures } => {
            let captures = captures
                .iter()
                .map(|capture| values.capture(&capture.target, &capture.source))
                .collect::<Vec<_>>()
                .into_boxed_slice();
            match freeze_function_target(target) {
                FrozenFunctionTarget::Function(target) => {
                    function_instruction(type_, family, F::Closure { target, captures })
                }
                FrozenFunctionTarget::External(target) => {
                    external_function_instruction(type_, family, X::Closure { target, captures })
                }
            }
        }
        DraftFunctionInstruction::Constructor(constructor) => {
            function_instruction(type_, family, F::Constructor(*constructor))
        }
        DraftFunctionInstruction::Call {
            function,
            args,
            site,
        } => match freeze_function_function_target(function) {
            FrozenFunctionFunctionTarget::Function(function) => function_instruction(
                type_,
                family,
                F::Call {
                    function,
                    args: values.any_slice(args),
                    site: site.clone(),
                },
            ),
            FrozenFunctionFunctionTarget::External(function) => external_function_instruction(
                type_,
                family,
                X::Call {
                    function,
                    args: values.any_slice(args),
                    site: site.clone(),
                },
            ),
        },
        DraftFunctionInstruction::FunctionCall {
            function,
            args,
            site,
        } => match values.function_function(function) {
            execution::graph::FunctionFunctionLocal::Core(function) => function_instruction(
                type_,
                family,
                F::FunctionCall {
                    function,
                    args: values.any_slice(args),
                    site: site.clone(),
                },
            ),
            execution::graph::FunctionFunctionLocal::External(function) => {
                external_function_instruction(
                    type_,
                    family,
                    X::FunctionCall {
                        function,
                        args: values.any_slice(args),
                        site: site.clone(),
                    },
                )
            }
        },
        DraftFunctionInstruction::TupleIndex { tuple, index } => function_instruction(
            type_,
            family,
            F::TupleIndex {
                tuple: values.tuple(tuple),
                index: *index,
            },
        ),
        DraftFunctionInstruction::CustomField { source, index } => function_instruction(
            type_,
            family,
            F::CustomField {
                source: values.custom(source),
                index: *index,
            },
        ),
        DraftFunctionInstruction::ListIndex { list, index } => function_instruction(
            type_,
            family,
            F::ListIndex {
                list: values.function_list(list),
                index: *index,
            },
        ),
    }
}

enum FrozenFunctionTarget {
    Function(execution::graph::FunctionTarget),
    External(execution::graph::ExternalFunctionTarget),
}

enum FrozenFunctionFunctionTarget {
    Function(execution::function::ProfiledFunctionFunctionId<std::convert::Infallible>),
    External(execution::graph::ExternalFunctionCallTarget),
}

fn function_instruction(
    type_: execution::type_::FunctionType,
    family: execution::function::FunctionReturnFamily,
    kind: execution::graph::FunctionInstructionKind,
) -> execution::graph::InstructionKind {
    execution::graph::InstructionKind::Function(execution::graph::FunctionInstruction::new(
        type_, family, kind,
    ))
}

fn external_function_instruction(
    type_: execution::type_::FunctionType,
    family: execution::function::FunctionReturnFamily,
    kind: execution::graph::ExternalFunctionInstructionKind,
) -> execution::graph::InstructionKind {
    execution::graph::InstructionKind::ExternalFunction(
        execution::graph::ExternalFunctionInstruction::new(type_, family, kind),
    )
}

fn freeze_function_target(target: &DraftFunctionTarget) -> FrozenFunctionTarget {
    use execution::graph::{ExternalFunctionTarget as X, FunctionTarget as F};

    match target {
        DraftFunctionTarget::Generic(function) => {
            FrozenFunctionTarget::Function(F::Generic(function.clone()))
        }
        DraftFunctionTarget::Never(function) => FrozenFunctionTarget::Function(F::Never(*function)),
        DraftFunctionTarget::Int(function) => FrozenFunctionTarget::Function(F::Int(*function)),
        DraftFunctionTarget::Float(function) => FrozenFunctionTarget::Function(F::Float(*function)),
        DraftFunctionTarget::String(function) => {
            FrozenFunctionTarget::Function(F::String(*function))
        }
        DraftFunctionTarget::BitArray(function) => {
            FrozenFunctionTarget::Function(F::BitArray(*function))
        }
        DraftFunctionTarget::UtfCodepoint(function) => {
            FrozenFunctionTarget::Function(F::UtfCodepoint(*function))
        }
        DraftFunctionTarget::Custom(function) => {
            FrozenFunctionTarget::Function(F::Custom(*function))
        }
        DraftFunctionTarget::External(function) => {
            FrozenFunctionTarget::External(X::Value(*function))
        }
        DraftFunctionTarget::Bool(function) => FrozenFunctionTarget::Function(F::Bool(*function)),
        DraftFunctionTarget::Nil(function) => FrozenFunctionTarget::Function(F::Nil(*function)),
        DraftFunctionTarget::Tuple(function) => FrozenFunctionTarget::Function(F::Tuple(*function)),
        DraftFunctionTarget::List(function) => match freeze_list_function_target(function) {
            Ok(function) => FrozenFunctionTarget::Function(F::List(function)),
            Err(function) => FrozenFunctionTarget::External(X::List(function)),
        },
        DraftFunctionTarget::Function(function) => {
            match freeze_function_function_target(function) {
                FrozenFunctionFunctionTarget::Function(function) => {
                    FrozenFunctionTarget::Function(F::Function(function))
                }
                FrozenFunctionFunctionTarget::External(
                    execution::graph::ExternalFunctionCallTarget::Function(function),
                ) => FrozenFunctionTarget::External(X::Function(function)),
                FrozenFunctionFunctionTarget::External(
                    execution::graph::ExternalFunctionCallTarget::ListFunction {
                        id,
                        type_,
                        list_type,
                    },
                ) => FrozenFunctionTarget::External(X::ListFunction {
                    id,
                    type_,
                    list_type,
                }),
            }
        }
    }
}

fn freeze_list_function_target(
    function: &execution::function::RuntimeListFunctionId,
) -> Result<execution::function::ListFunctionId, execution::function::ExternalListFunctionId> {
    use execution::function::ProfiledListFunctionId as F;

    match function {
        F::Core(function) => Ok(function.clone()),
        F::External(function) => Err(*function),
    }
}

fn freeze_function_function_target(
    function: &execution::function::FunctionFunctionId,
) -> FrozenFunctionFunctionTarget {
    use execution::function::ProfiledFunctionFunctionId as F;

    match function {
        F::Generic(function) => {
            FrozenFunctionFunctionTarget::Function(F::Generic(function.clone()))
        }
        F::Never(function) => FrozenFunctionFunctionTarget::Function(F::Never(function.clone())),
        F::Int(function) => FrozenFunctionFunctionTarget::Function(F::Int(*function)),
        F::Float(function) => FrozenFunctionFunctionTarget::Function(F::Float(*function)),
        F::String(function) => FrozenFunctionFunctionTarget::Function(F::String(*function)),
        F::BitArray(function) => FrozenFunctionFunctionTarget::Function(F::BitArray(*function)),
        F::UtfCodepoint(function) => {
            FrozenFunctionFunctionTarget::Function(F::UtfCodepoint(*function))
        }
        F::Custom(function) => FrozenFunctionFunctionTarget::Function(F::Custom(function.clone())),
        F::External(function) => FrozenFunctionFunctionTarget::External(
            execution::graph::ExternalFunctionCallTarget::Function(function.clone()),
        ),
        F::Bool(function) => FrozenFunctionFunctionTarget::Function(F::Bool(*function)),
        F::Nil(function) => FrozenFunctionFunctionTarget::Function(F::Nil(*function)),
        F::Tuple(function) => FrozenFunctionFunctionTarget::Function(F::Tuple(*function)),
        F::List(function) => match freeze_list_function_function_target(function) {
            Ok(function) => FrozenFunctionFunctionTarget::Function(F::List(function)),
            Err(FrozenExternalListFunctionFunction {
                id,
                type_,
                list_type,
            }) => FrozenFunctionFunctionTarget::External(
                execution::graph::ExternalFunctionCallTarget::ListFunction {
                    id,
                    type_,
                    list_type,
                },
            ),
        },
        F::Function(function) => {
            FrozenFunctionFunctionTarget::Function(F::Function(function.clone()))
        }
    }
}

struct FrozenExternalListFunctionFunction {
    id: execution::function::ExternalListFunctionFunctionId,
    type_: execution::type_::FunctionType,
    list_type: execution::type_::ExternalListTypeId,
}

fn freeze_list_function_function_target(
    function: &execution::function::ListFunctionFunctionId,
) -> Result<
    execution::function::ProfiledListFunctionFunctionId<std::convert::Infallible>,
    FrozenExternalListFunctionFunction,
> {
    use execution::function::ProfiledListFunctionFunctionId as F;

    match function {
        F::Parameter {
            id,
            type_,
            list_type,
        } => Ok(F::Parameter {
            id: *id,
            type_: type_.clone(),
            list_type: *list_type,
        }),
        F::ParameterList {
            id,
            type_,
            list_type,
        } => Ok(F::ParameterList {
            id: *id,
            type_: type_.clone(),
            list_type: *list_type,
        }),
        F::Int {
            id,
            type_,
            list_type,
        } => Ok(F::Int {
            id: *id,
            type_: type_.clone(),
            list_type: *list_type,
        }),
        F::String {
            id,
            type_,
            list_type,
        } => Ok(F::String {
            id: *id,
            type_: type_.clone(),
            list_type: *list_type,
        }),
        F::BitArray {
            id,
            type_,
            list_type,
        } => Ok(F::BitArray {
            id: *id,
            type_: type_.clone(),
            list_type: *list_type,
        }),
        F::UtfCodepoint {
            id,
            type_,
            list_type,
        } => Ok(F::UtfCodepoint {
            id: *id,
            type_: type_.clone(),
            list_type: *list_type,
        }),
        F::Custom {
            id,
            type_,
            list_type,
        } => Ok(F::Custom {
            id: *id,
            type_: type_.clone(),
            list_type: *list_type,
        }),
        F::External {
            id,
            type_,
            list_type,
        } => Err(FrozenExternalListFunctionFunction {
            id: *id,
            type_: type_.clone(),
            list_type: *list_type,
        }),
        F::Float {
            id,
            type_,
            list_type,
        } => Ok(F::Float {
            id: *id,
            type_: type_.clone(),
            list_type: *list_type,
        }),
        F::Bool {
            id,
            type_,
            list_type,
        } => Ok(F::Bool {
            id: *id,
            type_: type_.clone(),
            list_type: *list_type,
        }),
        F::Nil {
            id,
            type_,
            list_type,
        } => Ok(F::Nil {
            id: *id,
            type_: type_.clone(),
            list_type: *list_type,
        }),
        F::Tuple {
            id,
            type_,
            list_type,
        } => Ok(F::Tuple {
            id: *id,
            type_: type_.clone(),
            list_type: *list_type,
        }),
        F::List {
            id,
            type_,
            list_type,
        } => Ok(F::List {
            id: *id,
            type_: type_.clone(),
            list_type: *list_type,
        }),
        F::Function {
            id,
            type_,
            list_type,
        } => Ok(F::Function {
            id: *id,
            type_: type_.clone(),
            list_type: *list_type,
        }),
    }
}
