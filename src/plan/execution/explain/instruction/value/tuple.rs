use super::super::super::value::write_locals;
use super::super::operand::{write_call, write_constant, write_function_call, write_projection};
use crate::plan::execution::graph::TupleInstruction;

pub(in super::super) fn write_tuple(output: &mut String, instruction: &TupleInstruction) {
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

#[cfg(test)]
mod tests {
    use crate::plan::execution::{InstructionKind, TupleFunctionId};

    #[test]
    fn writes_tuple_construction() {
        let source = "pub fn main() { #(1, True) }";
        let expected = "tuple.value elements=[%int#0, %bool#0]";

        assert_explanation(source, expected);
    }

    #[test]
    fn writes_tuple_constants_calls_and_projections() {
        let source = r#"
const saved = #(0)

pub type Holder {
  Holder(value: #(Int))
}

fn tuple_value(value: #(Int)) { value }
fn tuple_values(values: List(#(Int))) { values }

pub fn main() {
  let function = tuple_value
  let values = tuple_values([#(1)])
  let selected = case values {
    [value, ..] -> value
    _ -> #(2)
  }
  let nested = #(#(3))
  let holder = Holder(#(4))
  let ignored = #(
    saved,
    tuple_value(#(5)),
    function(#(6)),
    nested.0,
    holder.value,
  )
  selected
}
"#;
        let expected = concat!(
            "tuple.value elements=[%int#0] | tuple.list_index %list.tuple#0 index=0 | ",
            "tuple.value elements=[%int#0] | tuple.value elements=[%tuple#1] | ",
            "tuple.value elements=[%int#1] | constant.tuple#0 | ",
            "tuple.value elements=[%int#2] | tuple.call tuple#1 args=[%tuple#5] | ",
            "tuple.value elements=[%int#3] | ",
            "tuple.function_call %function.tuple#0 args=[%tuple#7] | ",
            "tuple.tuple_index %tuple#2 index=0 | tuple.custom_field %custom#0 index=0 | ",
            "tuple.value elements=[%tuple#4, %tuple#6, %tuple#8, %tuple#9, %tuple#10] | ",
            "tuple.value elements=[%int#0]",
        );

        assert_explanation(source, expected);
    }

    fn assert_explanation(source: &str, expected: &str) {
        super::super::super::super::assert_rendered(source, expected, |plan, output| {
            let graph = plan.tuple_function(TupleFunctionId(0)).graph();
            let mut first = true;
            for instruction in graph.blocks().iter().flat_map(|block| block.instructions()) {
                if let InstructionKind::Tuple(instruction) = instruction.kind() {
                    if first {
                        first = false;
                    } else {
                        output.push_str(" | ");
                    }
                    super::write_tuple(output, instruction);
                }
            }
        });
    }
}
