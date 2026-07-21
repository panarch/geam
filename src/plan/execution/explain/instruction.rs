use super::super::graph::{
    BitArrayBitsSize, BitArrayEvaluatedSize, BitArrayInstruction, BitArraySegment, BoolInstruction,
    CustomInstruction, FloatInstruction, FunctionCapture, FunctionInstruction,
    FunctionInstructionKind, FunctionTarget, Instruction, InstructionKind, IntInstruction,
    ListInstruction, NilInstruction, ParameterListInstruction, StringInstruction, TupleInstruction,
    TypedListInstruction, UtfCodepointInstruction,
};
use super::super::{
    BitArrayFunctionId, BitArrayListFunctionId, BoolFunctionId, BoolListFunctionId, ConstantId,
    CustomFunctionId, CustomListFunctionId, ExecutionPlan, FloatFunctionId, FloatListFunctionId,
    FunctionListFunctionId, GenericCallableId, IntFunctionId, IntListFunctionId,
    ListListFunctionId, NeverFunctionId, NilFunctionId, NilListFunctionId, ParamLocal,
    ParameterListFunctionId, ParameterListListFunctionId, StringFunctionId, StringListFunctionId,
    TupleFunctionId, TupleListFunctionId, UtfCodepointFunctionId, UtfCodepointListFunctionId,
};
use super::bit_array::{endianness, float_size, string_encoding};
use super::label::{FunctionLabel, function_function_label, list_function_label};
use super::value::{ExplainLocal, write_list, write_locals, write_slot};

pub(super) fn write_instruction(
    output: &mut String,
    plan: &ExecutionPlan,
    instruction: &Instruction,
) {
    output.push_str("    ");
    write_slot(output, plan, instruction.output());
    output.push_str(" = ");
    match instruction.kind() {
        InstructionKind::Int(kind) => write_int(output, kind),
        InstructionKind::Float(kind) => write_float(output, kind),
        InstructionKind::String(kind) => write_string(output, kind),
        InstructionKind::BitArray(kind) => write_bit_array(output, kind),
        InstructionKind::UtfCodepoint(kind) => write_utf_codepoint(output, kind),
        InstructionKind::Custom(kind) => write_custom(output, kind),
        InstructionKind::Bool(kind) => write_bool(output, kind),
        InstructionKind::Nil(kind) => write_nil(output, kind),
        InstructionKind::Tuple(kind) => write_tuple(output, kind),
        InstructionKind::List(kind) => write_list_instruction(output, kind),
        InstructionKind::Function(kind) => write_function(output, kind),
    }
    output.push('\n');
}

trait ExplainFunctionId {
    fn label(&self) -> FunctionLabel;
}

macro_rules! tuple_function_id {
    ($type_:ty, $family:literal) => {
        impl ExplainFunctionId for $type_ {
            fn label(&self) -> FunctionLabel {
                FunctionLabel::new($family, self.0)
            }
        }
    };
}

tuple_function_id!(NeverFunctionId, "never");
tuple_function_id!(IntFunctionId, "int");
tuple_function_id!(FloatFunctionId, "float");
tuple_function_id!(StringFunctionId, "string");
tuple_function_id!(BitArrayFunctionId, "bit_array");
tuple_function_id!(UtfCodepointFunctionId, "utf_codepoint");
tuple_function_id!(BoolFunctionId, "bool");
tuple_function_id!(NilFunctionId, "nil");
tuple_function_id!(TupleFunctionId, "tuple");

macro_rules! indexed_function_id {
    ($type_:ty, $family:literal) => {
        impl ExplainFunctionId for $type_ {
            fn label(&self) -> FunctionLabel {
                FunctionLabel::new($family, self.index())
            }
        }
    };
}

indexed_function_id!(ParameterListFunctionId, "list.parameter");
indexed_function_id!(ParameterListListFunctionId, "list.parameter_list");
indexed_function_id!(IntListFunctionId, "list.int");
indexed_function_id!(StringListFunctionId, "list.string");
indexed_function_id!(BitArrayListFunctionId, "list.bit_array");
indexed_function_id!(UtfCodepointListFunctionId, "list.utf_codepoint");
indexed_function_id!(CustomListFunctionId, "list.custom");
indexed_function_id!(FloatListFunctionId, "list.float");
indexed_function_id!(BoolListFunctionId, "list.bool");
indexed_function_id!(NilListFunctionId, "list.nil");
indexed_function_id!(TupleListFunctionId, "list.tuple");
indexed_function_id!(ListListFunctionId, "list.list");
indexed_function_id!(FunctionListFunctionId, "list.function");

impl ExplainFunctionId for CustomFunctionId {
    fn label(&self) -> FunctionLabel {
        FunctionLabel::new("custom", self.index())
    }
}

fn write_int(output: &mut String, instruction: &IntInstruction) {
    match instruction {
        IntInstruction::Value(value) => write_literal(output, "int.value", &value.to_string()),
        IntInstruction::Constant(id) => write_constant(output, "int", *id),
        IntInstruction::Call { function, args } => write_call(output, "int.call", function, args),
        IntInstruction::FunctionCall { function, args } => {
            write_function_call(output, "int.function_call", function, args);
        }
        IntInstruction::TupleIndex { tuple, index } => {
            write_projection(output, "int.tuple_index", tuple, *index);
        }
        IntInstruction::CustomField { source, index } => {
            write_projection(output, "int.custom_field", source, *index);
        }
        IntInstruction::ListIndex { list, index } => {
            write_projection(output, "int.list_index", list, *index);
        }
        IntInstruction::Add { left, right } => write_binary(output, "int.add", left, right),
        IntInstruction::Sub { left, right } => write_binary(output, "int.sub", left, right),
        IntInstruction::Mult { left, right } => write_binary(output, "int.mult", left, right),
        IntInstruction::Div { left, right } => write_binary(output, "int.div", left, right),
        IntInstruction::Remainder { left, right } => {
            write_binary(output, "int.remainder", left, right);
        }
        IntInstruction::Negate(value) => write_unary(output, "int.negate", value),
    }
}

fn write_float(output: &mut String, instruction: &FloatInstruction) {
    match instruction {
        FloatInstruction::Value(value) => {
            write_literal(output, "float.value", &format!("{value:?}"));
        }
        FloatInstruction::Constant(id) => write_constant(output, "float", *id),
        FloatInstruction::Call { function, args } => {
            write_call(output, "float.call", function, args);
        }
        FloatInstruction::FunctionCall { function, args } => {
            write_function_call(output, "float.function_call", function, args);
        }
        FloatInstruction::TupleIndex { tuple, index } => {
            write_projection(output, "float.tuple_index", tuple, *index);
        }
        FloatInstruction::CustomField { source, index } => {
            write_projection(output, "float.custom_field", source, *index);
        }
        FloatInstruction::ListIndex { list, index } => {
            write_projection(output, "float.list_index", list, *index);
        }
        FloatInstruction::Add { left, right } => write_binary(output, "float.add", left, right),
        FloatInstruction::Sub { left, right } => write_binary(output, "float.sub", left, right),
        FloatInstruction::Mult { left, right } => {
            write_binary(output, "float.mult", left, right);
        }
        FloatInstruction::Div { left, right } => write_binary(output, "float.div", left, right),
    }
}

fn write_string(output: &mut String, instruction: &StringInstruction) {
    match instruction {
        StringInstruction::Value(value) => {
            write_literal(output, "string.value", &format!("{value:?}"));
        }
        StringInstruction::Constant(id) => write_constant(output, "string", *id),
        StringInstruction::Call { function, args } => {
            write_call(output, "string.call", function, args);
        }
        StringInstruction::FunctionCall { function, args } => {
            write_function_call(output, "string.function_call", function, args);
        }
        StringInstruction::TupleIndex { tuple, index } => {
            write_projection(output, "string.tuple_index", tuple, *index);
        }
        StringInstruction::CustomField { source, index } => {
            write_projection(output, "string.custom_field", source, *index);
        }
        StringInstruction::ListIndex { list, index } => {
            write_projection(output, "string.list_index", list, *index);
        }
        StringInstruction::Concatenate { left, right } => {
            write_binary(output, "string.concatenate", left, right);
        }
        StringInstruction::DropPrefix { value, prefix } => {
            output.push_str("string.drop_prefix ");
            value.write_local(output);
            output.push_str(" prefix=");
            output.push_str(&format!("{prefix:?}"));
        }
    }
}

fn write_bit_array(output: &mut String, instruction: &BitArrayInstruction) {
    match instruction {
        BitArrayInstruction::Value(segments) => {
            output.push_str("bit_array.value ");
            write_list(output, segments, write_bit_array_segment);
        }
        BitArrayInstruction::Constant(id) => write_constant(output, "bit_array", *id),
        BitArrayInstruction::Call { function, args } => {
            write_call(output, "bit_array.call", function, args);
        }
        BitArrayInstruction::FunctionCall { function, args } => {
            write_function_call(output, "bit_array.function_call", function, args);
        }
        BitArrayInstruction::TupleIndex { tuple, index } => {
            write_projection(output, "bit_array.tuple_index", tuple, *index);
        }
        BitArrayInstruction::CustomField { source, index } => {
            write_projection(output, "bit_array.custom_field", source, *index);
        }
        BitArrayInstruction::ListIndex { list, index } => {
            write_projection(output, "bit_array.list_index", list, *index);
        }
    }
}

fn write_bit_array_segment(output: &mut String, segment: &BitArraySegment) {
    match segment {
        BitArraySegment::Int {
            value,
            bit_size,
            endianness: order,
        } => {
            output.push_str("int(");
            value.write_local(output);
            output.push_str(", bits=");
            output.push_str(&bit_size.to_string());
            output.push_str(", ");
            output.push_str(endianness(*order));
            output.push(')');
        }
        BitArraySegment::EvaluatedInt {
            value,
            size,
            endianness: order,
            ..
        } => {
            output.push_str("int(");
            value.write_local(output);
            output.push_str(", bits=");
            write_evaluated_size(output, size);
            output.push_str(", ");
            output.push_str(endianness(*order));
            output.push(')');
        }
        BitArraySegment::Float {
            value,
            bit_size,
            endianness: order,
        } => {
            output.push_str("float(");
            value.write_local(output);
            output.push_str(", bits=");
            output.push_str(&float_size(*bit_size).to_string());
            output.push_str(", ");
            output.push_str(endianness(*order));
            output.push(')');
        }
        BitArraySegment::EvaluatedFloat {
            value,
            size,
            endianness: order,
            ..
        } => {
            output.push_str("float(");
            value.write_local(output);
            output.push_str(", bits=");
            write_evaluated_size(output, size);
            output.push_str(", ");
            output.push_str(endianness(*order));
            output.push(')');
        }
        BitArraySegment::String { value, encoding } => {
            output.push_str("string(");
            value.write_local(output);
            output.push_str(", ");
            output.push_str(string_encoding(*encoding));
            output.push(')');
        }
        BitArraySegment::UtfCodepoint { value, encoding } => {
            output.push_str("utf_codepoint(");
            value.write_local(output);
            output.push_str(", ");
            output.push_str(string_encoding(*encoding));
            output.push(')');
        }
        BitArraySegment::Bits(value) => {
            output.push_str("bits(");
            value.write_local(output);
            output.push(')');
        }
        BitArraySegment::SizedBits { value, size, .. } => {
            output.push_str("bits(");
            value.write_local(output);
            output.push_str(", bits=");
            match size {
                BitArrayBitsSize::Fixed(size) => output.push_str(&size.to_string()),
                BitArrayBitsSize::Evaluated(size) => write_evaluated_size(output, size),
            }
            output.push(')');
        }
    }
}

fn write_evaluated_size(output: &mut String, size: &BitArrayEvaluatedSize) {
    size.value().write_local(output);
    output.push('*');
    output.push_str(&size.unit().to_string());
}

fn write_utf_codepoint(output: &mut String, instruction: &UtfCodepointInstruction) {
    match instruction {
        UtfCodepointInstruction::Call { function, args } => {
            write_call(output, "utf_codepoint.call", function, args);
        }
        UtfCodepointInstruction::FunctionCall { function, args } => {
            write_function_call(output, "utf_codepoint.function_call", function, args);
        }
        UtfCodepointInstruction::TupleIndex { tuple, index } => {
            write_projection(output, "utf_codepoint.tuple_index", tuple, *index);
        }
        UtfCodepointInstruction::CustomField { source, index } => {
            write_projection(output, "utf_codepoint.custom_field", source, *index);
        }
        UtfCodepointInstruction::ListIndex { list, index } => {
            write_projection(output, "utf_codepoint.list_index", list, *index);
        }
    }
}

fn write_custom(output: &mut String, instruction: &CustomInstruction) {
    match instruction {
        CustomInstruction::Construct {
            constructor,
            fields,
        } => {
            output.push_str("custom.construct custom_type#");
            output.push_str(&constructor.type_id().index().to_string());
            output.push_str(".constructor#");
            output.push_str(&constructor.index().to_string());
            output.push_str(" fields=");
            write_locals(output, fields);
        }
        CustomInstruction::Constant(id) => write_constant(output, "custom", *id),
        CustomInstruction::Call { function, args } => {
            write_call(output, "custom.call", function, args);
        }
        CustomInstruction::FunctionCall { function, args } => {
            write_function_call(output, "custom.function_call", function, args);
        }
        CustomInstruction::TupleIndex { tuple, index } => {
            write_projection(output, "custom.tuple_index", tuple, *index);
        }
        CustomInstruction::CustomField { source, index } => {
            write_projection(output, "custom.custom_field", source, *index);
        }
        CustomInstruction::ListIndex { list, index } => {
            write_projection(output, "custom.list_index", list, *index);
        }
    }
}

fn write_bool(output: &mut String, instruction: &BoolInstruction) {
    match instruction {
        BoolInstruction::Value(value) => {
            write_literal(output, "bool.value", if *value { "True" } else { "False" });
        }
        BoolInstruction::Constant(id) => write_constant(output, "bool", *id),
        BoolInstruction::Call { function, args } => {
            write_call(output, "bool.call", function, args);
        }
        BoolInstruction::FunctionCall { function, args } => {
            write_function_call(output, "bool.function_call", function, args);
        }
        BoolInstruction::TupleIndex { tuple, index } => {
            write_projection(output, "bool.tuple_index", tuple, *index);
        }
        BoolInstruction::CustomField { source, index } => {
            write_projection(output, "bool.custom_field", source, *index);
        }
        BoolInstruction::ListIndex { list, index } => {
            write_projection(output, "bool.list_index", list, *index);
        }
        BoolInstruction::Not(value) => write_unary(output, "bool.not", value),
        BoolInstruction::LtInt { left, right } => write_binary(output, "bool.lt_int", left, right),
        BoolInstruction::LtEqInt { left, right } => {
            write_binary(output, "bool.lte_int", left, right);
        }
        BoolInstruction::GtInt { left, right } => write_binary(output, "bool.gt_int", left, right),
        BoolInstruction::GtEqInt { left, right } => {
            write_binary(output, "bool.gte_int", left, right);
        }
        BoolInstruction::LtFloat { left, right } => {
            write_binary(output, "bool.lt_float", left, right);
        }
        BoolInstruction::LtEqFloat { left, right } => {
            write_binary(output, "bool.lte_float", left, right);
        }
        BoolInstruction::GtFloat { left, right } => {
            write_binary(output, "bool.gt_float", left, right);
        }
        BoolInstruction::GtEqFloat { left, right } => {
            write_binary(output, "bool.gte_float", left, right);
        }
        BoolInstruction::Equal { left, right } => write_binary(output, "bool.equal", left, right),
        BoolInstruction::NotEqual { left, right } => {
            write_binary(output, "bool.not_equal", left, right);
        }
        BoolInstruction::StringStartsWith { value, prefix } => {
            output.push_str("bool.string_starts_with ");
            value.write_local(output);
            output.push_str(" prefix=");
            output.push_str(&format!("{prefix:?}"));
        }
        BoolInstruction::ListLengthEquals { value, length } => {
            write_length(output, "bool.list_length_equals", value, *length);
        }
        BoolInstruction::ListLengthAtLeast { value, length } => {
            write_length(output, "bool.list_length_at_least", value, *length);
        }
    }
}

fn write_nil(output: &mut String, instruction: &NilInstruction) {
    match instruction {
        NilInstruction::Value => output.push_str("nil.value"),
        NilInstruction::Constant(id) => write_constant(output, "nil", *id),
        NilInstruction::Call { function, args } => write_call(output, "nil.call", function, args),
        NilInstruction::FunctionCall { function, args } => {
            write_function_call(output, "nil.function_call", function, args);
        }
        NilInstruction::TupleIndex { tuple, index } => {
            write_projection(output, "nil.tuple_index", tuple, *index);
        }
        NilInstruction::CustomField { source, index } => {
            write_projection(output, "nil.custom_field", source, *index);
        }
        NilInstruction::ListIndex { list, index } => {
            write_projection(output, "nil.list_index", list, *index);
        }
    }
}

fn write_tuple(output: &mut String, instruction: &TupleInstruction) {
    match instruction {
        TupleInstruction::Value(elements) => {
            output.push_str("tuple.value elements=");
            write_locals(output, elements);
        }
        TupleInstruction::Constant(id) => write_constant(output, "tuple", *id),
        TupleInstruction::Call { function, args } => {
            write_call(output, "tuple.call", function, args);
        }
        TupleInstruction::FunctionCall { function, args } => {
            write_function_call(output, "tuple.function_call", function, args);
        }
        TupleInstruction::TupleIndex { tuple, index } => {
            write_projection(output, "tuple.tuple_index", tuple, *index);
        }
        TupleInstruction::CustomField { source, index } => {
            write_projection(output, "tuple.custom_field", source, *index);
        }
        TupleInstruction::ListIndex { list, index } => {
            write_projection(output, "tuple.list_index", list, *index);
        }
    }
}

fn write_list_instruction(output: &mut String, instruction: &ListInstruction) {
    match instruction {
        ListInstruction::Parameter(type_id, instruction) => {
            output.push_str("list.parameter[type#");
            output.push_str(&type_id.list_type().index().to_string());
            output.push_str("] ");
            write_parameter_list(output, instruction);
        }
        ListInstruction::ParameterList(type_id, instruction) => write_typed_list(
            output,
            "parameter_list",
            type_id.list_type().index(),
            instruction,
        ),
        ListInstruction::Int(type_id, instruction) => {
            write_typed_list(output, "int", type_id.list_type().index(), instruction);
        }
        ListInstruction::String(type_id, instruction) => {
            write_typed_list(output, "string", type_id.list_type().index(), instruction);
        }
        ListInstruction::BitArray(type_id, instruction) => {
            write_typed_list(
                output,
                "bit_array",
                type_id.list_type().index(),
                instruction,
            );
        }
        ListInstruction::UtfCodepoint(type_id, instruction) => write_typed_list(
            output,
            "utf_codepoint",
            type_id.list_type().index(),
            instruction,
        ),
        ListInstruction::Custom(type_id, instruction) => {
            write_typed_list(output, "custom", type_id.list_type().index(), instruction);
        }
        ListInstruction::Float(type_id, instruction) => {
            write_typed_list(output, "float", type_id.list_type().index(), instruction);
        }
        ListInstruction::Bool(type_id, instruction) => {
            write_typed_list(output, "bool", type_id.list_type().index(), instruction);
        }
        ListInstruction::Nil(type_id, instruction) => {
            write_typed_list(output, "nil", type_id.list_type().index(), instruction);
        }
        ListInstruction::Tuple(type_id, instruction) => {
            write_typed_list(output, "tuple", type_id.list_type().index(), instruction);
        }
        ListInstruction::List(type_id, instruction) => {
            write_typed_list(output, "list", type_id.list_type().index(), instruction);
        }
        ListInstruction::Function(type_id, instruction) => {
            write_typed_list(output, "function", type_id.list_type().index(), instruction);
        }
    }
}

fn write_parameter_list(output: &mut String, instruction: &ParameterListInstruction) {
    match instruction {
        ParameterListInstruction::Empty => output.push_str("empty"),
        ParameterListInstruction::Constant(id) => write_constant(output, "list.parameter", *id),
        ParameterListInstruction::Call { function, args } => {
            write_call(output, "call", function, args);
        }
        ParameterListInstruction::FunctionCall { function, args } => {
            write_function_call(output, "function_call", function, args);
        }
        ParameterListInstruction::TupleIndex { tuple, index } => {
            write_projection(output, "tuple_index", tuple, *index);
        }
        ParameterListInstruction::CustomField { source, index } => {
            write_projection(output, "custom_field", source, *index);
        }
        ParameterListInstruction::ListIndex { list, index } => {
            write_projection(output, "list_index", list, *index);
        }
    }
}

fn write_typed_list<Element, Local, Function>(
    output: &mut String,
    family: &'static str,
    type_id: usize,
    instruction: &TypedListInstruction<Element, Local, Function>,
) where
    Element: ExplainLocal,
    Local: ExplainLocal,
    Function: ExplainFunctionId,
{
    output.push_str("list.");
    output.push_str(family);
    output.push_str("[type#");
    output.push_str(&type_id.to_string());
    output.push_str("] ");
    match instruction {
        TypedListInstruction::Value(elements) => {
            output.push_str("value elements=");
            write_list(output, elements, |output, element| {
                element.write_local(output);
            });
        }
        TypedListInstruction::Constant(id) => {
            write_constant(output, &format!("list.{family}"), *id);
        }
        TypedListInstruction::Spread { elements, tail } => {
            output.push_str("spread elements=");
            write_list(output, elements, |output, element| {
                element.write_local(output);
            });
            output.push_str(" tail=");
            tail.write_local(output);
        }
        TypedListInstruction::Call { function, args } => write_call(output, "call", function, args),
        TypedListInstruction::FunctionCall { function, args } => {
            write_function_call(output, "function_call", function, args);
        }
        TypedListInstruction::TupleIndex { tuple, index } => {
            write_projection(output, "tuple_index", tuple, *index);
        }
        TypedListInstruction::CustomField { source, index } => {
            write_projection(output, "custom_field", source, *index);
        }
        TypedListInstruction::ListIndex { list, index } => {
            write_projection(output, "list_index", list, *index);
        }
        TypedListInstruction::DropFirst { list, count } => {
            output.push_str("drop_first ");
            list.write_local(output);
            output.push_str(" count=");
            output.push_str(&count.to_string());
        }
    }
}

fn write_function(output: &mut String, instruction: &FunctionInstruction) {
    output.push_str("function[");
    output.push_str(&instruction.family().to_string());
    output.push_str("] ");
    match instruction.kind() {
        FunctionInstructionKind::Constant(id) => write_constant(output, "function", *id),
        FunctionInstructionKind::Reference(target) => {
            output.push_str("reference ");
            write_function_target(output, target);
        }
        FunctionInstructionKind::Closure { target, captures } => {
            output.push_str("closure target=");
            write_function_target(output, target);
            output.push_str(" captures=");
            write_list(output, captures, write_capture);
        }
        FunctionInstructionKind::Constructor(constructor) => {
            output.push_str("constructor custom_type#");
            output.push_str(&constructor.type_id().index().to_string());
            output.push_str(".constructor#");
            output.push_str(&constructor.index().to_string());
        }
        FunctionInstructionKind::Call { function, args } => {
            output.push_str("call ");
            function_function_label(function).push_to(output);
            write_args(output, args);
        }
        FunctionInstructionKind::FunctionCall { function, args } => {
            write_function_call(output, "function_call", function, args);
        }
        FunctionInstructionKind::TupleIndex { tuple, index } => {
            write_projection(output, "tuple_index", tuple, *index);
        }
        FunctionInstructionKind::CustomField { source, index } => {
            write_projection(output, "custom_field", source, *index);
        }
        FunctionInstructionKind::ListIndex { list, index } => {
            write_projection(output, "list_index", list, *index);
        }
    }
}

fn write_function_target(output: &mut String, target: &FunctionTarget) {
    match target {
        FunctionTarget::Generic(GenericCallableId::Function {
            template,
            substitution,
        }) => {
            output.push_str("template#");
            output.push_str(&template.to_string());
            output.push_str(" shapes=");
            write_list(output, substitution, |output, shape| {
                output.push_str("shape#");
                output.push_str(&shape.index().to_string());
            });
        }
        FunctionTarget::Generic(GenericCallableId::Constructor(constructor)) => {
            output.push_str("custom_type#");
            output.push_str(&constructor.type_id().index().to_string());
            output.push_str(".constructor#");
            output.push_str(&constructor.index().to_string());
        }
        FunctionTarget::Never(function) => function.label().push_to(output),
        FunctionTarget::Int(function) => function.label().push_to(output),
        FunctionTarget::Float(function) => function.label().push_to(output),
        FunctionTarget::String(function) => function.label().push_to(output),
        FunctionTarget::BitArray(function) => function.label().push_to(output),
        FunctionTarget::UtfCodepoint(function) => function.label().push_to(output),
        FunctionTarget::Custom(function) => function.label().push_to(output),
        FunctionTarget::Bool(function) => function.label().push_to(output),
        FunctionTarget::Nil(function) => function.label().push_to(output),
        FunctionTarget::Tuple(function) => function.label().push_to(output),
        FunctionTarget::List(function) => list_function_label(function).push_to(output),
        FunctionTarget::Function(function) => function_function_label(function).push_to(output),
    }
}

fn write_capture(output: &mut String, capture: &FunctionCapture) {
    macro_rules! capture_pair {
        ($target:expr, $source:expr) => {{
            $target.write_local(output);
            output.push_str("<-");
            $source.write_local(output);
        }};
    }

    match capture {
        FunctionCapture::Int { target, source } => capture_pair!(target, source),
        FunctionCapture::Float { target, source } => capture_pair!(target, source),
        FunctionCapture::String { target, source } => capture_pair!(target, source),
        FunctionCapture::BitArray { target, source } => capture_pair!(target, source),
        FunctionCapture::UtfCodepoint { target, source } => capture_pair!(target, source),
        FunctionCapture::Custom { target, source } => capture_pair!(target, source),
        FunctionCapture::Bool { target, source } => capture_pair!(target, source),
        FunctionCapture::Nil { target, source } => capture_pair!(target, source),
        FunctionCapture::Tuple { target, source } => capture_pair!(target, source),
        FunctionCapture::ParameterList { target, source } => capture_pair!(target, source),
        FunctionCapture::ParameterListList { target, source } => capture_pair!(target, source),
        FunctionCapture::IntList { target, source } => capture_pair!(target, source),
        FunctionCapture::StringList { target, source } => capture_pair!(target, source),
        FunctionCapture::BitArrayList { target, source } => capture_pair!(target, source),
        FunctionCapture::UtfCodepointList { target, source } => capture_pair!(target, source),
        FunctionCapture::CustomList { target, source } => capture_pair!(target, source),
        FunctionCapture::FloatList { target, source } => capture_pair!(target, source),
        FunctionCapture::BoolList { target, source } => capture_pair!(target, source),
        FunctionCapture::NilList { target, source } => capture_pair!(target, source),
        FunctionCapture::TupleList { target, source } => capture_pair!(target, source),
        FunctionCapture::ListList { target, source } => capture_pair!(target, source),
        FunctionCapture::FunctionList { target, source } => capture_pair!(target, source),
        FunctionCapture::IntFunction { target, source } => capture_pair!(target, source),
        FunctionCapture::FloatFunction { target, source } => capture_pair!(target, source),
        FunctionCapture::StringFunction { target, source } => capture_pair!(target, source),
        FunctionCapture::BitArrayFunction { target, source } => capture_pair!(target, source),
        FunctionCapture::UtfCodepointFunction { target, source } => capture_pair!(target, source),
        FunctionCapture::GenericFunction { target, source } => capture_pair!(target, source),
        FunctionCapture::NeverFunction { target, source } => capture_pair!(target, source),
        FunctionCapture::CustomFunction { target, source } => capture_pair!(target, source),
        FunctionCapture::BoolFunction { target, source } => capture_pair!(target, source),
        FunctionCapture::NilFunction { target, source } => capture_pair!(target, source),
        FunctionCapture::TupleFunction { target, source } => capture_pair!(target, source),
        FunctionCapture::ListFunction { target, source } => capture_pair!(target, source),
        FunctionCapture::FunctionFunction { target, source } => capture_pair!(target, source),
    }
}

fn write_literal(output: &mut String, opcode: &str, value: &str) {
    output.push_str(opcode);
    output.push(' ');
    output.push_str(value);
}

fn write_constant<Value>(output: &mut String, family: &str, id: ConstantId<Value>) {
    output.push_str("constant.");
    output.push_str(family);
    output.push('#');
    output.push_str(&id.index().to_string());
}

fn write_call<Function: ExplainFunctionId>(
    output: &mut String,
    opcode: &str,
    function: &Function,
    args: &[ParamLocal],
) {
    output.push_str(opcode);
    output.push(' ');
    function.label().push_to(output);
    write_args(output, args);
}

fn write_function_call<Function: ExplainLocal>(
    output: &mut String,
    opcode: &str,
    function: &Function,
    args: &[ParamLocal],
) {
    output.push_str(opcode);
    output.push(' ');
    function.write_local(output);
    write_args(output, args);
}

fn write_args(output: &mut String, args: &[ParamLocal]) {
    output.push_str(" args=");
    write_locals(output, args);
}

fn write_projection<Source: ExplainLocal>(
    output: &mut String,
    opcode: &str,
    source: &Source,
    index: usize,
) {
    output.push_str(opcode);
    output.push(' ');
    source.write_local(output);
    output.push_str(" index=");
    output.push_str(&index.to_string());
}

fn write_unary<Value: ExplainLocal>(output: &mut String, opcode: &str, value: &Value) {
    output.push_str(opcode);
    output.push(' ');
    value.write_local(output);
}

fn write_binary<Value: ExplainLocal>(
    output: &mut String,
    opcode: &str,
    left: &Value,
    right: &Value,
) {
    output.push_str(opcode);
    output.push(' ');
    left.write_local(output);
    output.push(' ');
    right.write_local(output);
}

fn write_length<Value: ExplainLocal>(
    output: &mut String,
    opcode: &str,
    value: &Value,
    length: usize,
) {
    output.push_str(opcode);
    output.push(' ');
    value.write_local(output);
    output.push_str(" length=");
    output.push_str(&length.to_string());
}
