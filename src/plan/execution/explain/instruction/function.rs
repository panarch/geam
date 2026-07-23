mod capture;
mod target;

use self::capture::write_capture;
use self::target::write_target;
use super::super::super::graph::{FunctionInstruction, FunctionInstructionKind};
use super::super::label::function_function_label;
use super::super::value::write_list;
use super::operand::{write_args, write_constant, write_function_call, write_projection};

pub(super) fn write_function(output: &mut String, instruction: &FunctionInstruction) {
    output.push_str("function[");
    output.push_str(&instruction.family().to_string());
    output.push_str("] ");
    match instruction.kind() {
        FunctionInstructionKind::Constant(id) => write_constant(output, "function", *id),
        FunctionInstructionKind::Reference(target) => {
            output.push_str("reference ");
            write_target(output, target);
        }
        FunctionInstructionKind::Closure { target, captures } => {
            output.push_str("closure target=");
            write_target(output, target);
            output.push_str(" captures=");
            write_list(output, captures, write_capture);
        }
        FunctionInstructionKind::Constructor(constructor) => {
            output.push_str("constructor custom_type#");
            output.push_str(&constructor.type_id().index().to_string());
            output.push_str(".constructor#");
            output.push_str(&constructor.index().to_string());
        }
        FunctionInstructionKind::Call { function, args } => {
            output.push_str("call ");
            function_function_label(function).push_to(output);
            write_args(output, args);
        }
        FunctionInstructionKind::FunctionCall { function, args } => {
            write_function_call(output, "function_call", function, args);
        }
        FunctionInstructionKind::TupleIndex { tuple, index } => {
            write_projection(output, "tuple_index", tuple, *index);
        }
        FunctionInstructionKind::CustomField { source, index } => {
            write_projection(output, "custom_field", source, *index);
        }
        FunctionInstructionKind::ListIndex { list, index } => {
            write_projection(output, "list_index", list, *index);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::TupleFunctionId;

    #[test]
    fn writes_function_instruction_targets_calls_and_captures() {
        let source = r#"
fn identity(value: Int) { value }
fn returner(function: fn(Int) -> Int) { function }

pub fn main() {
  let captured = 1
  let reference = identity
  let closure = fn(value) { value + captured }
  let caller = returner
  let direct = returner(reference)
  let indirect = caller(reference)
  #(reference, closure, direct, indirect)
}
"#;
        let expected = concat!(
            "    %int#0:shape#0(Int) = int.value 1\n",
            "    %function.int#0:shape#1(fn(Int) -> Int) = function[Int] ",
            "reference int#0\n",
            "    %function.int#1:shape#1(fn(Int) -> Int) = function[Int] closure ",
            "target=int#1 captures=[%int#1<-%int#0]\n",
            "    %function.function#0:shape#2(fn(fn(Int) -> Int) -> fn(Int) -> Int) = ",
            "function[Function] reference function.int#0\n",
            "    %function.int#2:shape#1(fn(Int) -> Int) = function[Int] call ",
            "function.int#0 args=[%function.int#0]\n",
            "    %function.int#3:shape#1(fn(Int) -> Int) = function[Int] function_call ",
            "%function.function#0 args=[%function.int#0]\n",
            "    %tuple#0:shape#3(#(fn(Int) -> Int, fn(Int) -> Int, fn(Int) -> Int, ",
            "fn(Int) -> Int)) = tuple.value elements=[%function.int#0, ",
            "%function.int#1, %function.int#2, %function.int#3]\n",
        );

        assert_explanation(source, expected);
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
