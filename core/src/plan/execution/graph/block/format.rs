use crate::plan::execution::graph::{IntegerOperand, LocalLabel};

pub(in crate::plan::execution::graph::block) fn write_unary<Value: LocalLabel>(
    output: &mut String,
    opcode: &str,
    value: &Value,
) {
    output.push_str(opcode);
    output.push(' ');
    value.write_local_label(output);
}

pub(in crate::plan::execution::graph::block) fn write_binary<Value: LocalLabel>(
    output: &mut String,
    opcode: &str,
    left: &Value,
    right: &Value,
) {
    output.push_str(opcode);
    output.push(' ');
    left.write_local_label(output);
    output.push(' ');
    right.write_local_label(output);
}

pub(in crate::plan::execution::graph::block) fn write_integer_binary(
    output: &mut String,
    opcode: &str,
    left: &IntegerOperand,
    right: &IntegerOperand,
) {
    output.push_str(opcode);
    output.push(' ');
    left.write_operand(output);
    output.push(' ');
    right.write_operand(output);
}

pub(in crate::plan::execution::graph::block) fn write_length<Value: LocalLabel>(
    output: &mut String,
    opcode: &str,
    value: &Value,
    length: usize,
) {
    output.push_str(opcode);
    output.push(' ');
    value.write_local_label(output);
    output.push_str(" length=");
    output.push_str(&length.to_string());
}
