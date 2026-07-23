use super::super::super::write_binding;
use crate::plan::execution::graph::BitArrayBindingPattern;

pub(super) fn write_binding_pattern(output: &mut String, pattern: &BitArrayBindingPattern) {
    match pattern {
        BitArrayBindingPattern::Bind(binding) => write_binding(output, binding),
        BitArrayBindingPattern::Discard => output.push('_'),
        BitArrayBindingPattern::Alias { pattern, binding } => {
            output.push_str("alias(");
            write_binding_pattern(output, pattern);
            output.push_str(", ");
            write_binding(output, binding);
            output.push(')');
        }
    }
}
