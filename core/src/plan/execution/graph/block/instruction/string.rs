use super::{
    write_binary, write_call, write_constant, write_function_call, write_literal, write_projection,
};
use crate::plan::execution::constant::ConstantId;
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::function::StringFunctionId;
use crate::plan::execution::graph::LocalLabel;
use crate::plan::execution::graph::{
    CustomLocal, ParamLocal, StringFunctionLocalId, StringListLocalId, StringLocalId, TupleLocalId,
};
use ecow::EcoString;

pub(crate) enum StringInstruction {
    Value(EcoString),
    Constant(ConstantId<StringLocalId>),
    Call {
        function: StringFunctionId,
        args: Box<[ParamLocal]>,
        site: crate::plan::HostCallSite,
    },
    FunctionCall {
        function: StringFunctionLocalId,
        args: Box<[ParamLocal]>,
        site: crate::plan::HostCallSite,
    },
    TupleIndex {
        tuple: TupleLocalId,
        index: usize,
    },
    CustomField {
        source: CustomLocal,
        index: usize,
    },
    ListIndex {
        list: StringListLocalId,
        index: usize,
    },
    Concatenate {
        left: StringLocalId,
        right: StringLocalId,
    },
    DropPrefix {
        value: StringLocalId,
        prefix: EcoString,
    },
}

impl Explain for StringInstruction {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        let output = context.output();
        match self {
            StringInstruction::Value(value) => {
                write_literal(output, "string.value", &format!("{value:?}"));
            }
            StringInstruction::Constant(id) => write_constant(output, "string", *id),
            StringInstruction::Call { function, args, .. } => {
                write_call(output, "string.call", function, args);
            }
            StringInstruction::FunctionCall { function, args, .. } => {
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
                value.write_local_label(output);
                output.push_str(" prefix=");
                output.push_str(&format!("{prefix:?}"));
            }
        }
    }
}

#[cfg(test)]
mod explain_tests {
    use crate::plan::execution::explain;
    use crate::plan::execution::function::TupleFunctionId;
    use crate::plan::execution::graph::ProfiledInstructionKind;

    #[test]
    fn writes_string_values_and_concatenation() {
        let source = r#"
pub fn main() {
  let prefix = "pre"
  #(prefix <> "fix")
}
"#;
        let expected =
            "string.value \"pre\" | string.value \"fix\" | string.concatenate %string#0 %string#1";

        assert_explanation(source, expected);
    }

    #[test]
    fn writes_string_constants_calls_and_projections() {
        let source = r#"
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
"#;
        let expected = concat!(
            "string.value \"list\" | string.list_index %list.string#0 index=0 | ",
            "string.value \"tuple\" | string.value \"record\" | ",
            "string.value \"prefix-tail\" | string.call string#0 args=[%string#3] | ",
            "string.drop_prefix %string#1 prefix=\"prefix-\" | constant.string#0 | ",
            "string.value \"call\" | string.call string#0 args=[%string#3] | ",
            "string.value \"function\" | ",
            "string.function_call %function.string#0 args=[%string#5] | ",
            "string.tuple_index %tuple#0 index=0 | ",
            "string.custom_field %custom#0 index=0 | string.value \"\" | string.value \"\"",
        );

        assert_explanation(source, expected);
    }

    fn write_separator(output: &mut String, first: &mut bool) {
        if *first {
            *first = false;
        } else {
            output.push_str(" | ");
        }
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let graph = plan.tuple_function(TupleFunctionId(0)).body().block_graph();
            let mut first = true;
            for instruction in graph.blocks().iter().flat_map(|block| block.instructions()) {
                if let ProfiledInstructionKind::String(instruction) = instruction.kind() {
                    write_separator(output, &mut first);
                    let mut context = explain::ExplainContext::new(plan, output);
                    context.write(instruction);
                }
            }
        });
    }
}
