use super::super::super::graph::{ListInstruction, ParameterListInstruction, TypedListInstruction};
use super::super::value::{ExplainLocal, write_list as write_values};
use super::operand::{
    ExplainFunctionId, write_call, write_constant, write_function_call, write_projection,
};

pub(super) fn write_list(output: &mut String, instruction: &ListInstruction) {
    match instruction {
        ListInstruction::Parameter(type_id, instruction) => {
            output.push_str("list.parameter[type#");
            output.push_str(&type_id.list_type().index().to_string());
            output.push_str("] ");
            write_parameter(output, instruction);
        }
        ListInstruction::ParameterList(type_id, instruction) => write_typed(
            output,
            "parameter_list",
            type_id.list_type().index(),
            instruction,
        ),
        ListInstruction::Int(type_id, instruction) => {
            write_typed(output, "int", type_id.list_type().index(), instruction);
        }
        ListInstruction::String(type_id, instruction) => {
            write_typed(output, "string", type_id.list_type().index(), instruction);
        }
        ListInstruction::BitArray(type_id, instruction) => {
            write_typed(
                output,
                "bit_array",
                type_id.list_type().index(),
                instruction,
            );
        }
        ListInstruction::UtfCodepoint(type_id, instruction) => write_typed(
            output,
            "utf_codepoint",
            type_id.list_type().index(),
            instruction,
        ),
        ListInstruction::Custom(type_id, instruction) => {
            write_typed(output, "custom", type_id.list_type().index(), instruction);
        }
        ListInstruction::Float(type_id, instruction) => {
            write_typed(output, "float", type_id.list_type().index(), instruction);
        }
        ListInstruction::Bool(type_id, instruction) => {
            write_typed(output, "bool", type_id.list_type().index(), instruction);
        }
        ListInstruction::Nil(type_id, instruction) => {
            write_typed(output, "nil", type_id.list_type().index(), instruction);
        }
        ListInstruction::Tuple(type_id, instruction) => {
            write_typed(output, "tuple", type_id.list_type().index(), instruction);
        }
        ListInstruction::List(type_id, instruction) => {
            write_typed(output, "list", type_id.list_type().index(), instruction);
        }
        ListInstruction::Function(type_id, instruction) => {
            write_typed(output, "function", type_id.list_type().index(), instruction);
        }
    }
}

fn write_parameter(output: &mut String, instruction: &ParameterListInstruction) {
    match instruction {
        ParameterListInstruction::Empty => output.push_str("empty"),
        ParameterListInstruction::Constant(id) => write_constant(output, "list.parameter", *id),
        ParameterListInstruction::Call { function, args } => {
            write_call(output, "call", function, args);
        }
        ParameterListInstruction::FunctionCall { function, args } => {
            write_function_call(output, "function_call", function, args);
        }
        ParameterListInstruction::TupleIndex { tuple, index } => {
            write_projection(output, "tuple_index", tuple, *index);
        }
        ParameterListInstruction::CustomField { source, index } => {
            write_projection(output, "custom_field", source, *index);
        }
        ParameterListInstruction::ListIndex { list, index } => {
            write_projection(output, "list_index", list, *index);
        }
    }
}

fn write_typed<Element, Local, Function>(
    output: &mut String,
    family: &'static str,
    type_id: usize,
    instruction: &TypedListInstruction<Element, Local, Function>,
) where
    Element: ExplainLocal,
    Local: ExplainLocal,
    Function: ExplainFunctionId,
{
    output.push_str("list.");
    output.push_str(family);
    output.push_str("[type#");
    output.push_str(&type_id.to_string());
    output.push_str("] ");
    match instruction {
        TypedListInstruction::Value(elements) => {
            output.push_str("value elements=");
            write_values(output, elements, |output, element| {
                element.write_local(output);
            });
        }
        TypedListInstruction::Constant(id) => {
            write_constant(output, &format!("list.{family}"), *id);
        }
        TypedListInstruction::Spread { elements, tail } => {
            output.push_str("spread elements=");
            write_values(output, elements, |output, element| {
                element.write_local(output);
            });
            output.push_str(" tail=");
            tail.write_local(output);
        }
        TypedListInstruction::Call { function, args } => write_call(output, "call", function, args),
        TypedListInstruction::FunctionCall { function, args } => {
            write_function_call(output, "function_call", function, args);
        }
        TypedListInstruction::TupleIndex { tuple, index } => {
            write_projection(output, "tuple_index", tuple, *index);
        }
        TypedListInstruction::CustomField { source, index } => {
            write_projection(output, "custom_field", source, *index);
        }
        TypedListInstruction::ListIndex { list, index } => {
            write_projection(output, "list_index", list, *index);
        }
        TypedListInstruction::DropFirst { list, count } => {
            output.push_str("drop_first ");
            list.write_local(output);
            output.push_str(" count=");
            output.push_str(&count.to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::TupleFunctionId;

    #[test]
    fn writes_list_instruction_grammar() {
        let source = r#"
pub fn main() {
  let tail = [3]
  let values = [1, 2, ..tail]
  let assert [_, ..rest] = values
  #([], values, rest)
}
"#;
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = crate::ExecutionPlan::from_module_plan(module_plan);
        let graph = plan.tuple_function(TupleFunctionId(0)).graph();
        let mut output = String::new();

        for block in graph.blocks() {
            for instruction in block.instructions() {
                super::super::write_instruction(&mut output, &plan, instruction);
            }
        }

        assert_eq!(
            output,
            concat!(
                "    %int#0:shape#0(Int) = int.value 3\n",
                "    %list.int#0:shape#1(list_type#1) = list.int[type#1] value ",
                "elements=[%int#0]\n",
                "    %int#1:shape#0(Int) = int.value 1\n",
                "    %int#2:shape#0(Int) = int.value 2\n",
                "    %list.int#1:shape#1(list_type#1) = list.int[type#1] spread ",
                "elements=[%int#1, %int#2] tail=%list.int#0\n",
                "    %list.parameter#0:shape#3(list_type#0) = list.parameter[type#0] empty\n",
                "    %tuple#0:shape#4(#(list_type#0, list_type#1, list_type#1)) = ",
                "tuple.value elements=[%list.parameter#0, %list.int#1, %list.int#0]\n",
            ),
        );
    }
}
