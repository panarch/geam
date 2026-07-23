use super::super::super::value::ExplainLocal;

pub(in super::super) fn write_projection<Source: ExplainLocal>(
    output: &mut String,
    opcode: &str,
    source: &Source,
    index: usize,
) {
    output.push_str(opcode);
    output.push(' ');
    source.write_local(output);
    output.push_str(" index=");
    output.push_str(&index.to_string());
}
