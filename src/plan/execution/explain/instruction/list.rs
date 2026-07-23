mod parameter;
mod typed;

use self::parameter::write_parameter;
use self::typed::write_typed;
use super::super::super::graph::ListInstruction;

pub(super) fn write_list(output: &mut String, instruction: &ListInstruction) {
    match instruction {
        ListInstruction::Parameter(type_id, instruction) => {
            output.push_str("list.parameter[type#");
            output.push_str(&type_id.list_type().index().to_string());
            output.push_str("] ");
            write_parameter(output, instruction);
        }
        ListInstruction::ParameterList(type_id, instruction) => write_typed(
            output,
            "parameter_list",
            type_id.list_type().index(),
            instruction,
        ),
        ListInstruction::Int(type_id, instruction) => {
            write_typed(output, "int", type_id.list_type().index(), instruction);
        }
        ListInstruction::String(type_id, instruction) => {
            write_typed(output, "string", type_id.list_type().index(), instruction);
        }
        ListInstruction::BitArray(type_id, instruction) => {
            write_typed(
                output,
                "bit_array",
                type_id.list_type().index(),
                instruction,
            );
        }
        ListInstruction::UtfCodepoint(type_id, instruction) => write_typed(
            output,
            "utf_codepoint",
            type_id.list_type().index(),
            instruction,
        ),
        ListInstruction::Custom(type_id, instruction) => {
            write_typed(output, "custom", type_id.list_type().index(), instruction);
        }
        ListInstruction::Float(type_id, instruction) => {
            write_typed(output, "float", type_id.list_type().index(), instruction);
        }
        ListInstruction::Bool(type_id, instruction) => {
            write_typed(output, "bool", type_id.list_type().index(), instruction);
        }
        ListInstruction::Nil(type_id, instruction) => {
            write_typed(output, "nil", type_id.list_type().index(), instruction);
        }
        ListInstruction::Tuple(type_id, instruction) => {
            write_typed(output, "tuple", type_id.list_type().index(), instruction);
        }
        ListInstruction::List(type_id, instruction) => {
            write_typed(output, "list", type_id.list_type().index(), instruction);
        }
        ListInstruction::Function(type_id, instruction) => {
            write_typed(output, "function", type_id.list_type().index(), instruction);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::TupleFunctionId;

    #[test]
    fn writes_list_instruction_grammar() {
        assert_explanation(
            r#"
pub fn main() {
  let tail = [3]
  let values = [1, 2, ..tail]
  let assert [_, ..rest] = values
  #([], values, rest)
}
"#,
            concat!(
                "    %int#0:shape#0(Int) = int.value 3\n",
                "    %list.int#0:shape#1(list_type#1) = list.int[type#1] value ",
                "elements=[%int#0]\n",
                "    %int#1:shape#0(Int) = int.value 1\n",
                "    %int#2:shape#0(Int) = int.value 2\n",
                "    %list.int#1:shape#1(list_type#1) = list.int[type#1] spread ",
                "elements=[%int#1, %int#2] tail=%list.int#0\n",
                "    %list.parameter#0:shape#3(list_type#0) = list.parameter[type#0] empty\n",
                "    %tuple#0:shape#4(#(list_type#0, list_type#1, list_type#1)) = ",
                "tuple.value elements=[%list.parameter#0, %list.int#1, %list.int#0]\n",
            ),
        );
    }

    fn assert_explanation(source: &str, expected: &str) {
        super::super::super::assert_rendered(source, expected, |plan, output| {
            let graph = plan.tuple_function(TupleFunctionId(0)).graph();
            for block in graph.blocks() {
                for instruction in block.instructions() {
                    super::super::write_instruction(output, plan, instruction);
                }
            }
        });
    }
}
