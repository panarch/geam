use super::super::super::label::ExplainFunctionId;
use super::super::super::value::{ExplainLocal, write_locals};
use crate::plan::execution::ParamLocal;

pub(in super::super) fn write_call<Function: ExplainFunctionId>(
    output: &mut String,
    opcode: &str,
    function: &Function,
    args: &[ParamLocal],
) {
    output.push_str(opcode);
    output.push(' ');
    function.label().push_to(output);
    write_args(output, args);
}

pub(in super::super) fn write_function_call<Function: ExplainLocal>(
    output: &mut String,
    opcode: &str,
    function: &Function,
    args: &[ParamLocal],
) {
    output.push_str(opcode);
    output.push(' ');
    function.write_local(output);
    write_args(output, args);
}

pub(in super::super) fn write_args(output: &mut String, args: &[ParamLocal]) {
    output.push_str(" args=");
    write_locals(output, args);
}
