use super::super::super::value::ExplainLocal;

pub(in super::super) fn write_binary<Value: ExplainLocal>(
    output: &mut String,
    opcode: &str,
    left: &Value,
    right: &Value,
) {
    output.push_str(opcode);
    output.push(' ');
    left.write_local(output);
    output.push(' ');
    right.write_local(output);
}
