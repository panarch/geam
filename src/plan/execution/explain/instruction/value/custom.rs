use super::super::super::value::write_locals;
use super::super::operand::{write_call, write_constant, write_function_call, write_projection};
use crate::plan::execution::graph::CustomInstruction;

pub(in super::super) fn write_custom(output: &mut String, instruction: &CustomInstruction) {
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

#[cfg(test)]
mod tests {
    use crate::plan::execution::InstructionKind;

    #[test]
    fn writes_custom_construction() {
        let source = r#"
pub type Boxed { Boxed(Int) }
pub fn main() { Boxed(1) }
"#;
        let expected = "custom.construct custom_type#0.constructor#0 fields=[%int#0]";

        assert_explanation(source, expected);
    }

    #[test]
    fn writes_custom_constants_calls_and_projections() {
        let source = r#"
pub type Boxed {
  Boxed(Int)
}

pub type Holder {
  Holder(value: Boxed)
}

const saved = Boxed(0)

fn custom_value(value: Boxed) { value }
fn custom_values(values: List(Boxed)) { values }

pub fn main() {
  let function = custom_value
  let values = custom_values([Boxed(1)])
  let selected = case values {
    [value, ..] -> value
    _ -> Boxed(2)
  }
  let tuple = #(Boxed(3))
  let holder = Holder(Boxed(4))
  let ignored = #(
    saved,
    custom_value(Boxed(5)),
    function(Boxed(6)),
    tuple.0,
    holder.value,
  )
  selected
}
"#;
        let expected = concat!(
            "custom.construct custom_type#0.constructor#0 fields=[%int#0] | ",
            "custom.list_index %list.custom#0 index=0 | ",
            "custom.construct custom_type#0.constructor#0 fields=[%int#0] | ",
            "custom.construct custom_type#0.constructor#0 fields=[%int#1] | ",
            "custom.construct custom_type#1.constructor#0 fields=[%custom#2] | ",
            "constant.custom#0 | ",
            "custom.construct custom_type#0.constructor#0 fields=[%int#2] | ",
            "custom.call custom#1 args=[%custom#5] | ",
            "custom.construct custom_type#0.constructor#0 fields=[%int#3] | ",
            "custom.function_call %function.custom#0 args=[%custom#7] | ",
            "custom.tuple_index %tuple#0 index=0 | ",
            "custom.custom_field %custom#3 index=0 | ",
            "custom.construct custom_type#0.constructor#0 fields=[%int#0]",
        );

        assert_explanation(source, expected);
    }

    fn assert_explanation(source: &str, expected: &str) {
        super::super::super::super::assert_rendered(source, expected, |plan, output| {
            let function = plan.custom_function(plan.custom_function_id(0));
            let graph = function.graph().body();
            let mut first = true;
            for instruction in graph.blocks().iter().flat_map(|block| block.instructions()) {
                if let InstructionKind::Custom(instruction) = instruction.kind() {
                    if first {
                        first = false;
                    } else {
                        output.push_str(" | ");
                    }
                    super::write_custom(output, instruction);
                }
            }
        });
    }
}
