use super::super::super::value::ExplainLocal;

pub(in super::super) fn write_unary<Value: ExplainLocal>(
    output: &mut String,
    opcode: &str,
    value: &Value,
) {
    output.push_str(opcode);
    output.push(' ');
    value.write_local(output);
}
