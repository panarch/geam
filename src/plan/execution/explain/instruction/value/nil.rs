use super::super::operand::{write_call, write_constant, write_function_call, write_projection};
use crate::plan::execution::graph::NilInstruction;

pub(in super::super) fn write_nil(output: &mut String, instruction: &NilInstruction) {
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

#[cfg(test)]
mod tests {
    use crate::plan::execution::{InstructionKind, NilFunctionId};

    #[test]
    fn writes_nil_value() {
        assert_explanation("pub fn main() { Nil }", "nil.value");
    }

    #[test]
    fn writes_nil_constants_calls_and_projections() {
        assert_explanation(
            r#"
const saved = Nil

pub type Holder {
  Holder(value: Nil)
}

fn nil_value(value: Nil) { value }
fn nil_values(values: List(Nil)) { values }

pub fn main() {
  let function = nil_value
  let values = nil_values([Nil])
  let selected = case values {
    [value, ..] -> value
    _ -> Nil
  }
  let tuple = #(Nil)
  let holder = Holder(Nil)
  let ignored = #(
    saved,
    nil_value(Nil),
    function(Nil),
    tuple.0,
    holder.value,
  )
  selected
}
"#,
            concat!(
                "nil.value | nil.list_index %list.nil#0 index=0 | nil.value | nil.value | ",
                "constant.nil#0 | nil.value | nil.call nil#1 args=[%nil#4] | nil.value | ",
                "nil.function_call %function.nil#0 args=[%nil#6] | ",
                "nil.tuple_index %tuple#0 index=0 | nil.custom_field %custom#0 index=0 | ",
                "nil.value",
            ),
        );
    }

    fn assert_explanation(source: &str, expected: &str) {
        super::super::super::super::assert_rendered(source, expected, |plan, output| {
            let graph = plan.nil_function(NilFunctionId(0)).graph();
            let mut first = true;
            for instruction in graph.blocks().iter().flat_map(|block| block.instructions()) {
                if let InstructionKind::Nil(instruction) = instruction.kind() {
                    if first {
                        first = false;
                    } else {
                        output.push_str(" | ");
                    }
                    super::write_nil(output, instruction);
                }
            }
        });
    }
}
