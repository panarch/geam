use super::super::operand::{
    write_binary, write_call, write_constant, write_function_call, write_literal, write_projection,
};
use crate::plan::execution::graph::FloatInstruction;

pub(in super::super) fn write_float(output: &mut String, instruction: &FloatInstruction) {
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

#[cfg(test)]
mod tests {
    use crate::plan::execution::{InstructionKind, TupleFunctionId};

    #[test]
    fn writes_float_arithmetic() {
        let source = r#"
pub fn main() {
  let value = 6.0
  #(
    value +. 2.0,
    value -. 2.0,
    value *. 2.0,
    value /. 2.0,
  )
}
"#;
        let expected = concat!(
            "float.value 6.0 | float.value 2.0 | float.add %float#0 %float#1 | ",
            "float.value 2.0 | float.sub %float#0 %float#3 | ",
            "float.value 2.0 | float.mult %float#0 %float#5 | ",
            "float.value 2.0 | float.div %float#0 %float#7",
        );

        assert_explanation(source, expected);
    }

    #[test]
    fn writes_float_constants_calls_and_projections() {
        let source = r#"
const saved = 1.0

pub type Holder {
  Holder(value: Float)
}

fn float_value(value: Float) { value }
fn float_values(values: List(Float)) { values }

pub fn main() {
  let function = float_value
  let values = float_values([2.0])
  let selected = case values {
    [value, ..] -> value
    _ -> 0.0
  }
  let tuple = #(3.0)
  let holder = Holder(4.0)
  #(
    saved,
    float_value(5.0),
    function(6.0),
    tuple.0,
    holder.value,
    selected,
  )
}
"#;
        let expected = concat!(
            "float.value 2.0 | float.list_index %list.float#0 index=0 | ",
            "float.value 3.0 | float.value 4.0 | constant.float#0 | ",
            "float.value 5.0 | float.call float#0 args=[%float#4] | ",
            "float.value 6.0 | float.function_call %function.float#0 args=[%float#6] | ",
            "float.tuple_index %tuple#0 index=0 | ",
            "float.custom_field %custom#0 index=0 | float.value 0.0",
        );

        assert_explanation(source, expected);
    }

    fn assert_explanation(source: &str, expected: &str) {
        super::super::super::super::assert_rendered(source, expected, |plan, output| {
            let graph = plan.tuple_function(TupleFunctionId(0)).graph();
            let mut first = true;
            for instruction in graph.blocks().iter().flat_map(|block| block.instructions()) {
                if let InstructionKind::Float(instruction) = instruction.kind() {
                    write_separator(output, &mut first);
                    super::write_float(output, instruction);
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
