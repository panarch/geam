pub(in super::super) fn write_literal(output: &mut String, opcode: &str, value: &str) {
    output.push_str(opcode);
    output.push(' ');
    output.push_str(value);
}
