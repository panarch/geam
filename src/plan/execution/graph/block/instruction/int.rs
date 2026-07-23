use super::{
    write_binary, write_call, write_constant, write_function_call, write_literal, write_projection,
    write_unary,
};
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::{
    ConstantId, CustomLocal, IntFunctionId, IntFunctionLocalId, IntListLocalId, IntLocalId,
    ParamLocal, TupleLocalId,
};
use num_bigint::BigInt;

pub(crate) enum IntInstruction {
    Value(BigInt),
    Constant(ConstantId<IntLocalId>),
    Call {
        function: IntFunctionId,
        args: Box<[ParamLocal]>,
    },
    FunctionCall {
        function: IntFunctionLocalId,
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
        list: IntListLocalId,
        index: usize,
    },
    Add {
        left: IntLocalId,
        right: IntLocalId,
    },
    Sub {
        left: IntLocalId,
        right: IntLocalId,
    },
    Mult {
        left: IntLocalId,
        right: IntLocalId,
    },
    Div {
        left: IntLocalId,
        right: IntLocalId,
    },
    Remainder {
        left: IntLocalId,
        right: IntLocalId,
    },
    Negate(IntLocalId),
}

impl Explain for IntInstruction {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        let output = context.output();
        match self {
            IntInstruction::Value(value) => write_literal(output, "int.value", &value.to_string()),
            IntInstruction::Constant(id) => write_constant(output, "int", *id),
            IntInstruction::Call { function, args } => {
                write_call(output, "int.call", function, args)
            }
            IntInstruction::FunctionCall { function, args } => {
                write_function_call(output, "int.function_call", function, args);
            }
            IntInstruction::TupleIndex { tuple, index } => {
                write_projection(output, "int.tuple_index", tuple, *index);
            }
            IntInstruction::CustomField { source, index } => {
                write_projection(output, "int.custom_field", source, *index);
            }
            IntInstruction::ListIndex { list, index } => {
                write_projection(output, "int.list_index", list, *index);
            }
            IntInstruction::Add { left, right } => write_binary(output, "int.add", left, right),
            IntInstruction::Sub { left, right } => write_binary(output, "int.sub", left, right),
            IntInstruction::Mult { left, right } => write_binary(output, "int.mult", left, right),
            IntInstruction::Div { left, right } => write_binary(output, "int.div", left, right),
            IntInstruction::Remainder { left, right } => {
                write_binary(output, "int.remainder", left, right);
            }
            IntInstruction::Negate(value) => write_unary(output, "int.negate", value),
        }
    }
}

#[cfg(test)]
mod explain_tests {
    use crate::plan::execution::{InstructionKind, TupleFunctionId, explain};

    #[test]
    fn writes_int_arithmetic() {
        let source = r#"
pub fn main() {
  let value = 6
  #(
    value + 2,
    value - 2,
    value * 2,
    value / 2,
    value % 2,
    -value,
  )
}
"#;
        let expected = concat!(
            "int.value 6 | int.value 2 | int.add %int#0 %int#1 | ",
            "int.value 2 | int.sub %int#0 %int#3 | ",
            "int.value 2 | int.mult %int#0 %int#5 | ",
            "int.value 2 | int.div %int#0 %int#7 | ",
            "int.value 2 | int.remainder %int#0 %int#9 | int.negate %int#0",
        );

        assert_explanation(source, expected);
    }

    #[test]
    fn writes_int_constants_calls_and_projections() {
        let source = r#"
const saved = 1

pub type Holder {
  Holder(value: Int)
}

fn int_value(value: Int) { value }
fn int_values(values: List(Int)) { values }

pub fn main() {
  let function = int_value
  let values = int_values([2])
  let selected = case values {
    [value, ..] -> value
    _ -> 0
  }
  let tuple = #(3)
  let holder = Holder(4)
  #(
    saved,
    int_value(5),
    function(6),
    tuple.0,
    holder.value,
    selected,
  )
}
"#;
        let expected = concat!(
            "int.value 2 | int.list_index %list.int#0 index=0 | int.value 3 | ",
            "int.value 4 | constant.int#0 | int.value 5 | ",
            "int.call int#0 args=[%int#4] | int.value 6 | ",
            "int.function_call %function.int#0 args=[%int#6] | ",
            "int.tuple_index %tuple#0 index=0 | int.custom_field %custom#0 index=0 | ",
            "int.value 0",
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
                if let InstructionKind::Int(instruction) = instruction.kind() {
                    write_separator(output, &mut first);
                    let mut context = explain::ExplainContext::new(plan, output);
                    context.write(instruction);
                }
            }
        });
    }
}
