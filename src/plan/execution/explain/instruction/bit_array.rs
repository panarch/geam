use super::super::super::graph::{
    BitArrayBitsSize, BitArrayEvaluatedSize, BitArrayInstruction, BitArraySegment,
};
use super::super::bit_array::{endianness, float_size, string_encoding};
use super::super::value::{ExplainLocal, write_list};
use super::operand::{write_call, write_constant, write_function_call, write_projection};

pub(super) fn write_bit_array(output: &mut String, instruction: &BitArrayInstruction) {
    match instruction {
        BitArrayInstruction::Value(segments) => {
            output.push_str("bit_array.value ");
            write_list(output, segments, write_segment);
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

fn write_segment(output: &mut String, segment: &BitArraySegment) {
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

#[cfg(test)]
mod tests {
    use crate::plan::execution::BitArrayFunctionId;

    #[test]
    fn writes_bit_array_instruction_and_segment_grammar() {
        let source = r#"
pub fn main() {
  let size = 8
  let bits = <<1, 2>>
  <<
    1:4-big,
    2:size(size)-little,
    1.5:float-size(16)-big,
    2.5:float-size(size * 4)-little,
    "a":utf8,
    bits:bits-size(size),
  >>
}
"#;
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = crate::ExecutionPlan::from_module_plan(module_plan);
        let graph = plan.bit_array_function(BitArrayFunctionId(0)).graph();
        let mut output = String::new();

        for instruction in graph.blocks()[0].instructions() {
            super::super::write_instruction(&mut output, &plan, instruction);
        }

        assert_eq!(
            output,
            concat!(
                "    %int#0:shape#0(Int) = int.value 8\n",
                "    %int#1:shape#0(Int) = int.value 1\n",
                "    %int#2:shape#0(Int) = int.value 2\n",
                "    %bit_array#0:shape#1(BitArray) = bit_array.value ",
                "[int(%int#1, bits=8, big), int(%int#2, bits=8, big)]\n",
                "    %int#3:shape#0(Int) = int.value 1\n",
                "    %int#4:shape#0(Int) = int.value 2\n",
                "    %float#0:shape#2(Float) = float.value 1.5\n",
                "    %float#1:shape#2(Float) = float.value 2.5\n",
                "    %int#5:shape#0(Int) = int.value 4\n",
                "    %int#6:shape#0(Int) = int.mult %int#0 %int#5\n",
                "    %string#0:shape#3(String) = string.value \"a\"\n",
                "    %bit_array#1:shape#1(BitArray) = bit_array.value ",
                "[int(%int#3, bits=4, big), int(%int#4, bits=%int#0*1, little), ",
                "float(%float#0, bits=16, big), float(%float#1, bits=%int#6*1, little), ",
                "string(%string#0, utf8), bits(%bit_array#0, bits=%int#0*1)]\n",
            ),
        );
    }
}
