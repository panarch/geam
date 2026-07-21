use super::super::graph::{
    BitArrayBindingPattern, BitArrayPattern, BitArrayPatternSegment, BitArrayPatternSize,
    BitArrayPatternSizeExpr, BitArrayPatternValue, BitArrayStringPattern, MatchPattern,
    MatchPatternBinding, MatchPatternList, MatchPatternListTail, Signedness,
};
use super::bit_array::{endianness, string_encoding};
use super::value::ExplainLocal;

pub(super) fn write_pattern(output: &mut String, pattern: &MatchPattern) {
    match pattern {
        MatchPattern::Bind(binding) => write_binding(output, binding),
        MatchPattern::Discard => output.push('_'),
        MatchPattern::Int(value) => output.push_str(&value.to_string()),
        MatchPattern::Float(value) => output.push_str(&format!("{value:?}")),
        MatchPattern::String(value) => output.push_str(&format!("{value:?}")),
        MatchPattern::Bool(value) => output.push_str(if *value { "True" } else { "False" }),
        MatchPattern::Nil => output.push_str("Nil"),
        MatchPattern::Tuple(elements) => {
            output.push_str("#(");
            write_patterns(output, elements);
            output.push(')');
        }
        MatchPattern::List(list) => write_list(output, list),
        MatchPattern::BitArray(pattern) => write_bit_array(output, pattern),
        MatchPattern::Custom {
            constructor,
            fields,
        } => {
            output.push_str("custom_type#");
            output.push_str(&constructor.type_id().index().to_string());
            output.push_str(".constructor#");
            output.push_str(&constructor.index().to_string());
            output.push('(');
            write_patterns(output, fields);
            output.push(')');
        }
        MatchPattern::StringPrefix {
            prefix,
            left,
            right,
        } => {
            output.push_str("string_prefix(");
            output.push_str(&format!("{prefix:?}"));
            output.push_str(", left=");
            write_optional_binding(output, left.as_ref());
            output.push_str(", right=");
            write_optional_binding(output, right.as_ref());
            output.push(')');
        }
        MatchPattern::Alias { pattern, binding } => {
            output.push_str("alias(");
            write_pattern(output, pattern);
            output.push_str(", ");
            write_binding(output, binding);
            output.push(')');
        }
    }
}

fn write_patterns(output: &mut String, patterns: &[MatchPattern]) {
    for (index, pattern) in patterns.iter().enumerate() {
        if index > 0 {
            output.push_str(", ");
        }
        write_pattern(output, pattern);
    }
}

fn write_list(output: &mut String, list: &MatchPatternList) {
    output.push('[');
    let mut separator = "";
    for pattern in list.elements() {
        output.push_str(separator);
        write_pattern(output, pattern);
        separator = ", ";
    }
    if let Some(tail) = list.tail() {
        output.push_str(separator);
        output.push_str("..");
        match tail {
            MatchPatternListTail::Ignore => output.push('_'),
            MatchPatternListTail::Bind(binding) => write_binding(output, binding),
        }
    }
    output.push(']');
}

fn write_bit_array(output: &mut String, pattern: &BitArrayPattern) {
    output.push_str("<<");
    for (index, segment) in pattern.segments().iter().enumerate() {
        if index > 0 {
            output.push_str(", ");
        }
        write_bit_array_segment(output, segment);
    }
    output.push_str(">>");
}

fn write_bit_array_segment(output: &mut String, segment: &BitArrayPatternSegment) {
    match segment {
        BitArrayPatternSegment::Int {
            pattern,
            size,
            endianness: order,
            signedness,
        } => {
            output.push_str("int(");
            write_bit_array_value(output, pattern, |output, value| {
                output.push_str(&value.to_string());
            });
            output.push_str(", size=");
            write_size(output, size);
            output.push_str(", ");
            output.push_str(endianness(*order));
            output.push_str(", ");
            output.push_str(match signedness {
                Signedness::Signed => "signed",
                Signedness::Unsigned => "unsigned",
            });
            output.push(')');
        }
        BitArrayPatternSegment::Float {
            pattern,
            size,
            endianness: order,
        } => {
            output.push_str("float(");
            write_bit_array_value(output, pattern, |output, value| {
                output.push_str(&format!("{value:?}"));
            });
            output.push_str(", size=");
            write_size(output, size);
            output.push_str(", ");
            output.push_str(endianness(*order));
            output.push(')');
        }
        BitArrayPatternSegment::Bits {
            pattern,
            size,
            unit,
        } => {
            output.push_str("bits(");
            write_bit_array_binding(output, pattern);
            output.push_str(", size=");
            match size {
                Some(size) => write_size(output, size),
                None => output.push_str("rest"),
            }
            output.push_str(", unit=");
            output.push_str(&unit.to_string());
            output.push(')');
        }
        BitArrayPatternSegment::String { pattern, encoding } => {
            output.push_str("string(");
            match pattern {
                BitArrayStringPattern::Literal(value) => {
                    output.push_str(&format!("{value:?}"));
                }
                BitArrayStringPattern::Discard => output.push('_'),
            }
            output.push_str(", ");
            output.push_str(string_encoding(*encoding));
            output.push(')');
        }
        BitArrayPatternSegment::UtfCodepoint { pattern, encoding } => {
            output.push_str("utf_codepoint(");
            write_bit_array_binding(output, pattern);
            output.push_str(", ");
            output.push_str(string_encoding(*encoding));
            output.push(')');
        }
    }
}

fn write_size(output: &mut String, size: &BitArrayPatternSize) {
    write_size_expr(output, size.value());
    output.push('*');
    output.push_str(&size.unit().to_string());
}

fn write_size_expr(output: &mut String, expression: &BitArrayPatternSizeExpr) {
    match expression {
        BitArrayPatternSizeExpr::Value(value) => output.push_str(&value.to_string()),
        BitArrayPatternSizeExpr::Local(local) => local.write_local(output),
        BitArrayPatternSizeExpr::Binding(binding) => {
            output.push_str("binding#");
            output.push_str(&binding.index().to_string());
        }
        BitArrayPatternSizeExpr::Add { left, right } => write_binary(output, "+", left, right),
        BitArrayPatternSizeExpr::Subtract { left, right } => {
            write_binary(output, "-", left, right);
        }
        BitArrayPatternSizeExpr::Multiply { left, right } => {
            write_binary(output, "*", left, right);
        }
        BitArrayPatternSizeExpr::Divide { left, right } => write_binary(output, "/", left, right),
        BitArrayPatternSizeExpr::Remainder { left, right } => {
            write_binary(output, "%", left, right);
        }
    }
}

fn write_binary(
    output: &mut String,
    operator: &str,
    left: &BitArrayPatternSizeExpr,
    right: &BitArrayPatternSizeExpr,
) {
    output.push('(');
    write_size_expr(output, left);
    output.push(' ');
    output.push_str(operator);
    output.push(' ');
    write_size_expr(output, right);
    output.push(')');
}

fn write_bit_array_value<Value>(
    output: &mut String,
    pattern: &BitArrayPatternValue<Value>,
    write_value: impl Copy + Fn(&mut String, &Value),
) {
    match pattern {
        BitArrayPatternValue::Literal(value) => write_value(output, value),
        BitArrayPatternValue::Bind(binding) => write_binding(output, binding),
        BitArrayPatternValue::Discard => output.push('_'),
        BitArrayPatternValue::Alias { pattern, binding } => {
            output.push_str("alias(");
            write_bit_array_value(output, pattern, write_value);
            output.push_str(", ");
            write_binding(output, binding);
            output.push(')');
        }
    }
}

fn write_bit_array_binding(output: &mut String, pattern: &BitArrayBindingPattern) {
    match pattern {
        BitArrayBindingPattern::Bind(binding) => write_binding(output, binding),
        BitArrayBindingPattern::Discard => output.push('_'),
        BitArrayBindingPattern::Alias { pattern, binding } => {
            output.push_str("alias(");
            write_bit_array_binding(output, pattern);
            output.push_str(", ");
            write_binding(output, binding);
            output.push(')');
        }
    }
}

fn write_optional_binding(output: &mut String, binding: Option<&MatchPatternBinding>) {
    match binding {
        Some(binding) => write_binding(output, binding),
        None => output.push('_'),
    }
}

fn write_binding(output: &mut String, binding: &MatchPatternBinding) {
    output.push_str("binding#");
    output.push_str(&binding.index().to_string());
}

#[cfg(test)]
mod tests {
    use super::{MatchPattern, MatchPatternBinding, MatchPatternList, MatchPatternListTail};

    #[test]
    fn list_pattern_explanation_separates_elements_from_the_tail() {
        let pattern = MatchPattern::List(MatchPatternList::new(
            vec![MatchPattern::Bind(MatchPatternBinding::new(0))],
            Some(MatchPatternListTail::Bind(MatchPatternBinding::new(1))),
        ));
        let mut output = String::new();

        super::write_pattern(&mut output, &pattern);

        assert_eq!(output, "[binding#0, ..binding#1]");

        let pattern = MatchPattern::List(MatchPatternList::new(
            Vec::new(),
            Some(MatchPatternListTail::Ignore),
        ));
        output.clear();

        super::write_pattern(&mut output, &pattern);

        assert_eq!(output, "[.._]");
    }
}
