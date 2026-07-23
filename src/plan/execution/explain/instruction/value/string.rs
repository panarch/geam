use super::super::super::value::ExplainLocal;
use super::super::operand::{
    write_binary, write_call, write_constant, write_function_call, write_literal, write_projection,
};
use crate::plan::execution::graph::StringInstruction;

pub(in super::super) fn write_string(output: &mut String, instruction: &StringInstruction) {
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

#[cfg(test)]
mod tests {
    use crate::plan::execution::{InstructionKind, TupleFunctionId};

    #[test]
    fn writes_string_values_and_concatenation() {
        assert_explanation(
            r#"
pub fn main() {
  let prefix = "pre"
  #(prefix <> "fix")
}
"#,
            "string.value \"pre\" | string.value \"fix\" | string.concatenate %string#0 %string#1",
        );
    }

    #[test]
    fn writes_string_constants_calls_and_projections() {
        assert_explanation(
            r#"
const saved = "saved"

pub type Holder {
  Holder(value: String)
}

fn string_value(value: String) { value }
fn string_values(values: List(String)) { values }

pub fn main() {
  let function = string_value
  let values = string_values(["list"])
  let selected = case values {
    [value, ..] -> value
    _ -> ""
  }
  let tuple = #("tuple")
  let holder = Holder("record")
  let text = string_value("prefix-tail")
  let suffix = case text {
    "prefix-" <> rest -> rest
    _ -> ""
  }
  #(
    saved,
    string_value("call"),
    function("function"),
    tuple.0,
    holder.value,
    selected,
    suffix,
  )
}
"#,
            concat!(
                "string.value \"list\" | string.list_index %list.string#0 index=0 | ",
                "string.value \"tuple\" | string.value \"record\" | ",
                "string.value \"prefix-tail\" | string.call string#0 args=[%string#3] | ",
                "string.drop_prefix %string#1 prefix=\"prefix-\" | constant.string#0 | ",
                "string.value \"call\" | string.call string#0 args=[%string#3] | ",
                "string.value \"function\" | ",
                "string.function_call %function.string#0 args=[%string#5] | ",
                "string.tuple_index %tuple#0 index=0 | ",
                "string.custom_field %custom#0 index=0 | string.value \"\" | string.value \"\"",
            ),
        );
    }

    fn assert_explanation(source: &str, expected: &str) {
        super::super::super::super::assert_rendered(source, expected, |plan, output| {
            let graph = plan.tuple_function(TupleFunctionId(0)).graph();
            let mut first = true;
            for instruction in graph.blocks().iter().flat_map(|block| block.instructions()) {
                if let InstructionKind::String(instruction) = instruction.kind() {
                    write_separator(output, &mut first);
                    super::write_string(output, instruction);
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
