use super::{
    write_binary, write_call, write_constant, write_function_call, write_length, write_literal,
    write_projection, write_unary,
};
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::graph::ExplainLocal;
use crate::plan::execution::{
    BoolFunctionId, BoolFunctionLocalId, BoolListLocalId, BoolLocalId, ConstantId, CustomLocal,
    FloatLocalId, IntLocalId, ListLocal, ParamLocal, StringLocalId, TupleLocalId,
};
use ecow::EcoString;

pub(crate) enum BoolInstruction {
    Value(bool),
    Constant(ConstantId<BoolLocalId>),
    Call {
        function: BoolFunctionId,
        args: Box<[ParamLocal]>,
    },
    FunctionCall {
        function: BoolFunctionLocalId,
        args: Box<[ParamLocal]>,
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
        list: BoolListLocalId,
        index: usize,
    },
    Not(BoolLocalId),
    LtInt {
        left: IntLocalId,
        right: IntLocalId,
    },
    LtEqInt {
        left: IntLocalId,
        right: IntLocalId,
    },
    GtInt {
        left: IntLocalId,
        right: IntLocalId,
    },
    GtEqInt {
        left: IntLocalId,
        right: IntLocalId,
    },
    LtFloat {
        left: FloatLocalId,
        right: FloatLocalId,
    },
    LtEqFloat {
        left: FloatLocalId,
        right: FloatLocalId,
    },
    GtFloat {
        left: FloatLocalId,
        right: FloatLocalId,
    },
    GtEqFloat {
        left: FloatLocalId,
        right: FloatLocalId,
    },
    Equal {
        left: ParamLocal,
        right: ParamLocal,
    },
    NotEqual {
        left: ParamLocal,
        right: ParamLocal,
    },
    StringStartsWith {
        value: StringLocalId,
        prefix: EcoString,
    },
    ListLengthEquals {
        value: ListLocal,
        length: usize,
    },
    ListLengthAtLeast {
        value: ListLocal,
        length: usize,
    },
}

impl Explain for BoolInstruction {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        let output = context.output();
        match self {
            BoolInstruction::Value(value) => {
                write_literal(output, "bool.value", if *value { "True" } else { "False" });
            }
            BoolInstruction::Constant(id) => write_constant(output, "bool", *id),
            BoolInstruction::Call { function, args } => {
                write_call(output, "bool.call", function, args);
            }
            BoolInstruction::FunctionCall { function, args } => {
                write_function_call(output, "bool.function_call", function, args);
            }
            BoolInstruction::TupleIndex { tuple, index } => {
                write_projection(output, "bool.tuple_index", tuple, *index);
            }
            BoolInstruction::CustomField { source, index } => {
                write_projection(output, "bool.custom_field", source, *index);
            }
            BoolInstruction::ListIndex { list, index } => {
                write_projection(output, "bool.list_index", list, *index);
            }
            BoolInstruction::Not(value) => write_unary(output, "bool.not", value),
            BoolInstruction::LtInt { left, right } => {
                write_binary(output, "bool.lt_int", left, right)
            }
            BoolInstruction::LtEqInt { left, right } => {
                write_binary(output, "bool.lte_int", left, right);
            }
            BoolInstruction::GtInt { left, right } => {
                write_binary(output, "bool.gt_int", left, right)
            }
            BoolInstruction::GtEqInt { left, right } => {
                write_binary(output, "bool.gte_int", left, right);
            }
            BoolInstruction::LtFloat { left, right } => {
                write_binary(output, "bool.lt_float", left, right);
            }
            BoolInstruction::LtEqFloat { left, right } => {
                write_binary(output, "bool.lte_float", left, right);
            }
            BoolInstruction::GtFloat { left, right } => {
                write_binary(output, "bool.gt_float", left, right);
            }
            BoolInstruction::GtEqFloat { left, right } => {
                write_binary(output, "bool.gte_float", left, right);
            }
            BoolInstruction::Equal { left, right } => {
                write_binary(output, "bool.equal", left, right)
            }
            BoolInstruction::NotEqual { left, right } => {
                write_binary(output, "bool.not_equal", left, right);
            }
            BoolInstruction::StringStartsWith { value, prefix } => {
                output.push_str("bool.string_starts_with ");
                value.write_local(output);
                output.push_str(" prefix=");
                output.push_str(&format!("{prefix:?}"));
            }
            BoolInstruction::ListLengthEquals { value, length } => {
                write_length(output, "bool.list_length_equals", value, *length);
            }
            BoolInstruction::ListLengthAtLeast { value, length } => {
                write_length(output, "bool.list_length_at_least", value, *length);
            }
        }
    }
}

#[cfg(test)]
mod explain_tests {
    use crate::plan::execution::{BoolFunctionId, InstructionKind, explain};

    #[test]
    fn writes_bool_instruction_grammar() {
        let source = r#"
pub fn main() {
  let integer = 1
  let float = 1.0
  let values = [1]
  !True
  && integer < 2
  && integer <= 2
  && integer > 0
  && integer >= 0
  && float <. 2.0
  && float <=. 2.0
  && float >. 0.0
  && float >=. 0.0
  && integer == 1
  && integer != 2
  && values == [1]
}
"#;
        let expected = concat!(
            "bool.value True | bool.not %bool#0 | bool.lt_int %int#0 %int#1 | ",
            "bool.lte_int %int#0 %int#1 | bool.gt_int %int#0 %int#1 | ",
            "bool.gte_int %int#0 %int#1 | bool.lt_float %float#0 %float#1 | ",
            "bool.lte_float %float#0 %float#1 | bool.gt_float %float#0 %float#1 | ",
            "bool.gte_float %float#0 %float#1 | bool.equal %int#0 %int#1 | ",
            "bool.not_equal %int#0 %int#1 | bool.equal %list.int#0 %list.int#1 | ",
            "bool.value True | bool.value False",
        );

        assert_explanation(source, expected);
    }

    #[test]
    fn writes_bool_constants_calls_projections_and_pattern_checks() {
        let source = r#"
const saved = True

pub type Flag {
  Flag(value: Bool)
}

fn bool_value(value: Bool) { value }
fn bool_values(values: List(Bool)) { values }
fn string_value(value: String) { value }

pub fn main() {
  let function = bool_value
  let values = bool_values([True])
  let selected = case values {
    [value, ..] -> value
    _ -> False
  }
  let exact = case values {
    [value] -> value
    _ -> False
  }
  let tuple = #(True)
  let record = Flag(True)
  let text = string_value("prefix-tail")
  let prefix = case text {
    "prefix-" <> _ -> True
    _ -> False
  }

  saved
  && bool_value(True)
  && function(True)
  && tuple.0
  && record.value
  && selected
  && exact
  && prefix
}
"#;
        let expected = concat!(
            "bool.value True | bool.list_length_at_least %list.bool#1 length=1 | ",
            "bool.list_index %list.bool#0 index=0 | bool.value True | bool.value True | ",
            "bool.list_length_equals %list.bool#0 length=1 | ",
            "bool.list_index %list.bool#0 index=0 | bool.value True | bool.value True | ",
            "bool.value True | bool.value True | ",
            "bool.string_starts_with %string#1 prefix=\"prefix-\" | bool.value True | ",
            "bool.value True | constant.bool#0 | bool.value True | ",
            "bool.call bool#1 args=[%bool#3] | bool.value True | ",
            "bool.function_call %function.bool#0 args=[%bool#3] | ",
            "bool.tuple_index %tuple#0 index=0 | bool.custom_field %custom#0 index=0 | ",
            "bool.value True | bool.value False | bool.value False | bool.value False | ",
            "bool.value False | bool.value False | bool.value False | bool.value False | ",
            "bool.value False | bool.value False",
        );

        assert_explanation(source, expected);
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let graph = plan.bool_function(BoolFunctionId(0)).body().block_graph();
            let mut first = true;
            for instruction in graph.blocks().iter().flat_map(|block| block.instructions()) {
                if let InstructionKind::Bool(instruction) = instruction.kind() {
                    if first {
                        first = false;
                    } else {
                        output.push_str(" | ");
                    }
                    let mut context = explain::ExplainContext::new(plan, output);
                    context.write(instruction);
                }
            }
        });
    }
}
