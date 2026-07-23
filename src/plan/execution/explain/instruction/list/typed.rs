use super::super::super::label::ExplainFunctionId;
use super::super::super::value::{ExplainLocal, write_list};
use super::super::operand::{write_call, write_constant, write_function_call, write_projection};
use crate::plan::execution::graph::TypedListInstruction;

pub(super) fn write_typed<Element, Local, Function>(
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
            write_list(output, elements, |output, element| {
                element.write_local(output);
            });
        }
        TypedListInstruction::Constant(id) => {
            write_constant(output, &format!("list.{family}"), *id);
        }
        TypedListInstruction::Spread { elements, tail } => {
            output.push_str("spread elements=");
            write_list(output, elements, |output, element| {
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
