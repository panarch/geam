use super::super::operand::{write_call, write_function_call, write_projection};
use crate::plan::execution::graph::UtfCodepointInstruction;

pub(in super::super) fn write_utf_codepoint(
    output: &mut String,
    instruction: &UtfCodepointInstruction,
) {
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

#[cfg(test)]
mod tests {
    use crate::plan::execution::{InstructionKind, TupleFunctionId};

    #[test]
    fn writes_utf_codepoint_calls_and_projections() {
        assert_explanation(
            r#"
pub type Holder {
  Holder(value: UtfCodepoint)
}

fn point() -> UtfCodepoint {
  let assert <<value:utf8_codepoint>> = <<65>>
  value
}

fn point_value(value: UtfCodepoint) { value }
fn point_values(values: List(UtfCodepoint)) { values }

pub fn main() {
  let scalar = point()
  let function = point_value
  let values = point_values([scalar])
  let selected = case values {
    [value, ..] -> value
    _ -> scalar
  }
  let tuple = #(scalar)
  let holder = Holder(scalar)
  #(
    point_value(scalar),
    function(scalar),
    tuple.0,
    holder.value,
    selected,
  )
}
"#,
            concat!(
                "utf_codepoint.call utf_codepoint#0 args=[] | ",
                "utf_codepoint.list_index %list.utf_codepoint#0 index=0 | ",
                "utf_codepoint.call utf_codepoint#1 args=[%utf_codepoint#1] | ",
                "utf_codepoint.function_call %function.utf_codepoint#0 ",
                "args=[%utf_codepoint#1] | ",
                "utf_codepoint.tuple_index %tuple#0 index=0 | ",
                "utf_codepoint.custom_field %custom#0 index=0",
            ),
        );
    }

    fn assert_explanation(source: &str, expected: &str) {
        super::super::super::super::assert_rendered(source, expected, |plan, output| {
            let graph = plan.tuple_function(TupleFunctionId(0)).graph();
            let mut first = true;
            for instruction in graph.blocks().iter().flat_map(|block| block.instructions()) {
                if let InstructionKind::UtfCodepoint(instruction) = instruction.kind() {
                    write_separator(output, &mut first);
                    super::write_utf_codepoint(output, instruction);
                }
            }
        });
    }

    fn write_separator(output: &mut String, first: &mut bool) {
        if *first {
            *first = false;
        } else {
            output.push_str(" | ");
        }
    }
}
