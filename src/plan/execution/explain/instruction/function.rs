use super::super::super::GenericCallableId;
use super::super::super::graph::{
    FunctionCapture, FunctionInstruction, FunctionInstructionKind, FunctionTarget,
};
use super::super::label::{function_function_label, list_function_label};
use super::super::value::{ExplainLocal, write_list};
use super::operand::{
    ExplainFunctionId, write_args, write_constant, write_function_call, write_projection,
};

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

fn write_target(output: &mut String, target: &FunctionTarget) {
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

fn write_capture(output: &mut String, capture: &FunctionCapture) {
    match capture {
        FunctionCapture::Int { target, source } => write_capture_pair(output, target, source),
        FunctionCapture::Float { target, source } => write_capture_pair(output, target, source),
        FunctionCapture::String { target, source } => write_capture_pair(output, target, source),
        FunctionCapture::BitArray { target, source } => write_capture_pair(output, target, source),
        FunctionCapture::UtfCodepoint { target, source } => {
            write_capture_pair(output, target, source);
        }
        FunctionCapture::Custom { target, source } => write_capture_pair(output, target, source),
        FunctionCapture::Bool { target, source } => write_capture_pair(output, target, source),
        FunctionCapture::Nil { target, source } => write_capture_pair(output, target, source),
        FunctionCapture::Tuple { target, source } => write_capture_pair(output, target, source),
        FunctionCapture::ParameterList { target, source } => {
            write_capture_pair(output, target, source);
        }
        FunctionCapture::ParameterListList { target, source } => {
            write_capture_pair(output, target, source);
        }
        FunctionCapture::IntList { target, source } => write_capture_pair(output, target, source),
        FunctionCapture::StringList { target, source } => {
            write_capture_pair(output, target, source);
        }
        FunctionCapture::BitArrayList { target, source } => {
            write_capture_pair(output, target, source);
        }
        FunctionCapture::UtfCodepointList { target, source } => {
            write_capture_pair(output, target, source);
        }
        FunctionCapture::CustomList { target, source } => {
            write_capture_pair(output, target, source);
        }
        FunctionCapture::FloatList { target, source } => {
            write_capture_pair(output, target, source);
        }
        FunctionCapture::BoolList { target, source } => write_capture_pair(output, target, source),
        FunctionCapture::NilList { target, source } => write_capture_pair(output, target, source),
        FunctionCapture::TupleList { target, source } => {
            write_capture_pair(output, target, source);
        }
        FunctionCapture::ListList { target, source } => write_capture_pair(output, target, source),
        FunctionCapture::FunctionList { target, source } => {
            write_capture_pair(output, target, source);
        }
        FunctionCapture::IntFunction { target, source } => {
            write_capture_pair(output, target, source);
        }
        FunctionCapture::FloatFunction { target, source } => {
            write_capture_pair(output, target, source);
        }
        FunctionCapture::StringFunction { target, source } => {
            write_capture_pair(output, target, source);
        }
        FunctionCapture::BitArrayFunction { target, source } => {
            write_capture_pair(output, target, source);
        }
        FunctionCapture::UtfCodepointFunction { target, source } => {
            write_capture_pair(output, target, source);
        }
        FunctionCapture::GenericFunction { target, source } => {
            write_capture_pair(output, target, source);
        }
        FunctionCapture::NeverFunction { target, source } => {
            write_capture_pair(output, target, source);
        }
        FunctionCapture::CustomFunction { target, source } => {
            write_capture_pair(output, target, source);
        }
        FunctionCapture::BoolFunction { target, source } => {
            write_capture_pair(output, target, source);
        }
        FunctionCapture::NilFunction { target, source } => {
            write_capture_pair(output, target, source);
        }
        FunctionCapture::TupleFunction { target, source } => {
            write_capture_pair(output, target, source);
        }
        FunctionCapture::ListFunction { target, source } => {
            write_capture_pair(output, target, source);
        }
        FunctionCapture::FunctionFunction { target, source } => {
            write_capture_pair(output, target, source);
        }
    }
}

fn write_capture_pair<Target, Source>(output: &mut String, target: &Target, source: &Source)
where
    Target: ExplainLocal,
    Source: ExplainLocal,
{
    target.write_local(output);
    output.push_str("<-");
    source.write_local(output);
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
            ),
        );
    }
}
