use super::super::super::value::ExplainLocal;

pub(in super::super) fn write_length<Value: ExplainLocal>(
    output: &mut String,
    opcode: &str,
    value: &Value,
    length: usize,
) {
    output.push_str(opcode);
    output.push(' ');
    value.write_local(output);
    output.push_str(" length=");
    output.push_str(&length.to_string());
}
