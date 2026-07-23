use super::super::super::write_binding;
use crate::plan::execution::graph::BitArrayPatternValue;

pub(super) fn write_value<Value>(
    output: &mut String,
    pattern: &BitArrayPatternValue<Value>,
    write_literal: impl Copy + Fn(&mut String, &Value),
) {
    match pattern {
        BitArrayPatternValue::Literal(value) => write_literal(output, value),
        BitArrayPatternValue::Bind(binding) => write_binding(output, binding),
        BitArrayPatternValue::Discard => output.push('_'),
        BitArrayPatternValue::Alias { pattern, binding } => {
            output.push_str("alias(");
            write_value(output, pattern, write_literal);
            output.push_str(", ");
            write_binding(output, binding);
            output.push(')');
        }
    }
}
