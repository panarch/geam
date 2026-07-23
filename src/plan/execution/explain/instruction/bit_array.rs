mod segment;

use self::segment::write_segment;
use super::super::super::graph::BitArrayInstruction;
use super::super::value::write_list;
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
        let expected = concat!(
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
        );

        assert_explanation(source, expected);
    }

    fn assert_explanation(source: &str, expected: &str) {
        super::super::super::assert_rendered(source, expected, |plan, output| {
            let graph = plan.bit_array_function(BitArrayFunctionId(0)).graph();
            for instruction in graph.blocks()[0].instructions() {
                super::super::write_instruction(output, plan, instruction);
            }
        });
    }
}
