use super::super::super::graph::{
    BitArrayBindingPattern, BitArrayPattern, BitArrayPatternSegment, BitArrayPatternSize,
    BitArrayPatternSizeExpr, BitArrayPatternValue, BitArrayStringPattern, Signedness,
};
use super::super::bit_array::{endianness, string_encoding};
use super::super::value::ExplainLocal;
use super::write_binding;

pub(super) fn write_bit_array(output: &mut String, pattern: &BitArrayPattern) {
    output.push_str("<<");
    for (index, segment) in pattern.segments().iter().enumerate() {
        if index > 0 {
            output.push_str(", ");
        }
        write_segment(output, segment);
    }
    output.push_str(">>");
}

fn write_segment(output: &mut String, segment: &BitArrayPatternSegment) {
    match segment {
        BitArrayPatternSegment::Int {
            pattern,
            size,
            endianness: order,
            signedness,
        } => {
            output.push_str("int(");
            write_value(output, pattern, |output, value| {
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
            write_value(output, pattern, |output, value| {
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
            write_binding_pattern(output, pattern);
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
            write_binding_pattern(output, pattern);
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

fn write_value<Value>(
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

fn write_binding_pattern(output: &mut String, pattern: &BitArrayBindingPattern) {
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

#[cfg(test)]
mod tests {
    use super::super::super::super::{IntFunctionId, MatchPattern, Terminator};
    use crate::plan::execution::graph::BitArrayPattern;

    #[test]
    fn writes_dynamic_and_remainder_bit_array_segments() {
        let source = r#"
fn select(bits: BitArray, size: Int) {
  let assert <<value:size(size), rest:bits>> = bits
  value
}

pub fn main() { select(<<1, 2>>, 8) }
"#;
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = crate::ExecutionPlan::from_module_plan(module_plan);
        let terminator = plan.int_function(IntFunctionId(1)).graph().blocks()[0].terminator();
        let pattern = bit_array_pattern(match_pattern(terminator));
        let mut output = String::new();

        super::write_bit_array(&mut output, pattern);

        assert_eq!(
            output,
            "<<int(binding#0, size=%int#0*1, big, unsigned), bits(binding#1, size=rest, unit=1)>>",
        );
    }

    #[test]
    #[should_panic(expected = "let assert should lower to a match terminator")]
    fn match_pattern_shape_guard_is_visible() {
        let plan = execution_plan("pub fn main() { 1 }");
        let terminator = plan.int_function(IntFunctionId(0)).graph().blocks()[0].terminator();
        match_pattern(terminator);
    }

    #[test]
    #[should_panic(expected = "source should lower a BitArray match pattern")]
    fn bit_array_pattern_shape_guard_is_visible() {
        let source = r#"
fn select(value: Int) {
  let assert 1 = value
  value
}
pub fn main() { select(1) }
"#;
        let plan = execution_plan(source);
        let terminator = plan.int_function(IntFunctionId(1)).graph().blocks()[0].terminator();
        bit_array_pattern(match_pattern(terminator));
    }

    fn match_pattern(terminator: &Terminator) -> &MatchPattern {
        let Terminator::Match { pattern, .. } = terminator else {
            panic!("let assert should lower to a match terminator");
        };
        pattern
    }

    fn bit_array_pattern(pattern: &MatchPattern) -> &BitArrayPattern {
        let MatchPattern::BitArray(pattern) = pattern else {
            panic!("source should lower a BitArray match pattern");
        };
        pattern
    }

    fn execution_plan(source: &str) -> crate::ExecutionPlan {
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        crate::ExecutionPlan::from_module_plan(module_plan)
    }
}
