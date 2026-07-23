use super::super::super::label::{ExplainFunctionId, function_function_label, list_function_label};
use super::super::super::value::write_list;
use crate::plan::execution::GenericCallableId;
use crate::plan::execution::graph::FunctionTarget;

pub(super) fn write_target(output: &mut String, target: &FunctionTarget) {
    match target {
        FunctionTarget::Generic(GenericCallableId::Function {
            template,
            substitution,
        }) => {
            output.push_str("template#");
            output.push_str(&template.to_string());
            output.push_str(" shapes=");
            write_list(output, substitution, |output, shape| {
                output.push_str("shape#");
                output.push_str(&shape.index().to_string());
            });
        }
        FunctionTarget::Generic(GenericCallableId::Constructor(constructor)) => {
            output.push_str("custom_type#");
            output.push_str(&constructor.type_id().index().to_string());
            output.push_str(".constructor#");
            output.push_str(&constructor.index().to_string());
        }
        FunctionTarget::Never(function) => function.label().push_to(output),
        FunctionTarget::Int(function) => function.label().push_to(output),
        FunctionTarget::Float(function) => function.label().push_to(output),
        FunctionTarget::String(function) => function.label().push_to(output),
        FunctionTarget::BitArray(function) => function.label().push_to(output),
        FunctionTarget::UtfCodepoint(function) => function.label().push_to(output),
        FunctionTarget::Custom(function) => function.label().push_to(output),
        FunctionTarget::Bool(function) => function.label().push_to(output),
        FunctionTarget::Nil(function) => function.label().push_to(output),
        FunctionTarget::Tuple(function) => function.label().push_to(output),
        FunctionTarget::List(function) => list_function_label(function).push_to(output),
        FunctionTarget::Function(function) => function_function_label(function).push_to(output),
    }
}
