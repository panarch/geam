use super::{MatchIntBindingId, MatchPatternBinding};
use crate::plan::execution::{Endianness, IntLocalId, StringEncoding};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Signedness {
    Signed,
    Unsigned,
}

pub(crate) struct BitArrayPattern {
    segments: Box<[BitArrayPatternSegment]>,
}

pub(crate) enum BitArrayPatternSegment {
    Int {
        pattern: BitArrayPatternValue<BigInt>,
        size: BitArrayPatternSize,
        endianness: Endianness,
        signedness: Signedness,
    },
    Float {
        pattern: BitArrayPatternValue<f64>,
        size: BitArrayPatternSize,
        endianness: Endianness,
    },
    Bits {
        pattern: BitArrayBindingPattern,
        size: Option<BitArrayPatternSize>,
        unit: u8,
    },
    String {
        pattern: BitArrayStringPattern,
        encoding: StringEncoding,
    },
    UtfCodepoint {
        pattern: BitArrayBindingPattern,
        encoding: StringEncoding,
    },
}

pub(crate) struct BitArrayPatternSize {
    value: BitArrayPatternSizeExpr,
    unit: u8,
}

pub(crate) enum BitArrayPatternSizeExpr {
    Value(BigInt),
    Local(IntLocalId),
    Binding(MatchIntBindingId),
    Add { left: Box<Self>, right: Box<Self> },
    Subtract { left: Box<Self>, right: Box<Self> },
    Multiply { left: Box<Self>, right: Box<Self> },
    Divide { left: Box<Self>, right: Box<Self> },
    Remainder { left: Box<Self>, right: Box<Self> },
}

pub(crate) enum BitArrayPatternValue<Value> {
    Literal(Value),
    Bind(MatchPatternBinding),
    Discard,
    Alias {
        pattern: Box<Self>,
        binding: MatchPatternBinding,
    },
}

pub(crate) enum BitArrayStringPattern {
    Literal(EcoString),
    Discard,
}

pub(crate) enum BitArrayBindingPattern {
    Bind(MatchPatternBinding),
    Discard,
    Alias {
        pattern: Box<Self>,
        binding: MatchPatternBinding,
    },
}

impl BitArrayPattern {
    pub(in crate::plan::execution) fn new(segments: Vec<BitArrayPatternSegment>) -> Self {
        Self {
            segments: segments.into_boxed_slice(),
        }
    }

    pub(crate) fn segments(&self) -> &[BitArrayPatternSegment] {
        &self.segments
    }
}

impl BitArrayPatternSize {
    pub(in crate::plan::execution) fn new(value: BitArrayPatternSizeExpr, unit: u8) -> Self {
        Self { value, unit }
    }

    pub(crate) fn value(&self) -> &BitArrayPatternSizeExpr {
        &self.value
    }

    pub(crate) fn unit(&self) -> u8 {
        self.unit
    }
}

use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::graph::{ExplainLocal, endianness, string_encoding};

impl Explain for BitArrayPattern {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        context.push_str("<<");
        for (index, segment) in self.segments().iter().enumerate() {
            if index > 0 {
                context.push_str(", ");
            }
            context.write(segment);
        }
        context.push_str(">>");
    }
}

impl Explain for BitArrayPatternSegment {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        match self {
            Self::Int {
                pattern,
                size,
                endianness: order,
                signedness,
            } => {
                context.push_str("int(");
                write_value(context, pattern, |context, value| {
                    context.push_str(&value.to_string());
                });
                context.push_str(", size=");
                context.write(size);
                context.push_str(", ");
                context.push_str(endianness(*order));
                context.push_str(", ");
                context.push_str(match signedness {
                    Signedness::Signed => "signed",
                    Signedness::Unsigned => "unsigned",
                });
                context.push(')');
            }
            Self::Float {
                pattern,
                size,
                endianness: order,
            } => {
                context.push_str("float(");
                write_value(context, pattern, |context, value| {
                    context.push_str(&format!("{value:?}"));
                });
                context.push_str(", size=");
                context.write(size);
                context.push_str(", ");
                context.push_str(endianness(*order));
                context.push(')');
            }
            Self::Bits {
                pattern,
                size,
                unit,
            } => {
                context.push_str("bits(");
                context.write(pattern);
                context.push_str(", size=");
                match size {
                    Some(size) => context.write(size),
                    None => context.push_str("rest"),
                }
                context.push_str(", unit=");
                context.push_str(&unit.to_string());
                context.push(')');
            }
            Self::String { pattern, encoding } => {
                context.push_str("string(");
                match pattern {
                    BitArrayStringPattern::Literal(value) => {
                        context.push_str(&format!("{value:?}"));
                    }
                    BitArrayStringPattern::Discard => context.push('_'),
                }
                context.push_str(", ");
                context.push_str(string_encoding(*encoding));
                context.push(')');
            }
            Self::UtfCodepoint { pattern, encoding } => {
                context.push_str("utf_codepoint(");
                context.write(pattern);
                context.push_str(", ");
                context.push_str(string_encoding(*encoding));
                context.push(')');
            }
        }
    }
}

impl Explain for BitArrayBindingPattern {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        match self {
            Self::Bind(binding) => context.write(binding),
            Self::Discard => context.push('_'),
            Self::Alias { pattern, binding } => {
                context.push_str("alias(");
                context.write(pattern.as_ref());
                context.push_str(", ");
                context.write(binding);
                context.push(')');
            }
        }
    }
}

impl Explain for BitArrayPatternSize {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        context.write(self.value());
        context.push('*');
        context.push_str(&self.unit().to_string());
    }
}

impl Explain for BitArrayPatternSizeExpr {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        match self {
            Self::Value(value) => context.push_str(&value.to_string()),
            Self::Local(local) => local.write_local(context.output()),
            Self::Binding(binding) => {
                context.push_str("binding#");
                context.push_str(&binding.index().to_string());
            }
            Self::Add { left, right } => write_binary(context, "+", left, right),
            Self::Subtract { left, right } => write_binary(context, "-", left, right),
            Self::Multiply { left, right } => write_binary(context, "*", left, right),
            Self::Divide { left, right } => write_binary(context, "/", left, right),
            Self::Remainder { left, right } => write_binary(context, "%", left, right),
        }
    }
}

fn write_binary(
    context: &mut ExplainContext<'_, '_>,
    operator: &str,
    left: &BitArrayPatternSizeExpr,
    right: &BitArrayPatternSizeExpr,
) {
    context.push('(');
    context.write(left);
    context.push(' ');
    context.push_str(operator);
    context.push(' ');
    context.write(right);
    context.push(')');
}

fn write_value<Value>(
    context: &mut ExplainContext<'_, '_>,
    pattern: &BitArrayPatternValue<Value>,
    write_literal: impl Copy + Fn(&mut ExplainContext<'_, '_>, &Value),
) {
    match pattern {
        BitArrayPatternValue::Literal(value) => write_literal(context, value),
        BitArrayPatternValue::Bind(binding) => context.write(binding),
        BitArrayPatternValue::Discard => context.push('_'),
        BitArrayPatternValue::Alias { pattern, binding } => {
            context.push_str("alias(");
            write_value(context, pattern, write_literal);
            context.push_str(", ");
            context.write(binding);
            context.push(')');
        }
    }
}

#[cfg(test)]
mod explain_tests {
    use super::super::super::Terminator;
    use super::BitArrayPattern;
    use crate::plan::execution::{IntFunctionId, MatchPattern, explain};

    #[test]
    fn writes_dynamic_and_remainder_bit_array_segments() {
        let source = r#"
fn identity(bits: BitArray) { bits }

pub fn main() {
  let bits = identity(<<1, 2>>)
  let size = 8
  let assert <<value:size(size), rest:bits>> = bits
  value
}
"#;
        let expected =
            "<<int(binding#0, size=%int#2*1, big, unsigned), bits(binding#1, size=rest, unit=1)>>";

        assert_explanation(source, expected);
    }

    #[test]
    #[should_panic(expected = "let assert should lower to a match terminator")]
    fn match_pattern_shape_guard_is_visible() {
        explain::with_execution_plan("pub fn main() { 1 }", |plan| {
            match_pattern(&terminators(plan));
        });
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
        explain::with_execution_plan(source, |plan| {
            bit_array_pattern(match_pattern(&terminators_for(plan, IntFunctionId(1))));
        });
    }

    #[test]
    #[should_panic(expected = "let assert should lower to one match terminator")]
    fn match_pattern_uniqueness_guard_is_visible() {
        let source = r#"
fn identity(bits: BitArray) { bits }

pub fn main() {
  let bits = identity(<<1>>)
  let assert <<value, rest:bits>> = bits
  value
}
"#;
        explain::with_execution_plan(source, |plan| {
            let pattern = match_pattern(&terminators(plan));
            match_pattern_from_nodes(&[pattern, pattern]);
        });
    }

    fn terminators(
        plan: &crate::plan::execution::ExecutionPlan,
    ) -> Vec<&crate::plan::execution::Terminator> {
        terminators_for(plan, IntFunctionId(0))
    }

    fn terminators_for(
        plan: &crate::plan::execution::ExecutionPlan,
        function: IntFunctionId,
    ) -> Vec<&crate::plan::execution::Terminator> {
        plan.int_function(function)
            .graph()
            .blocks()
            .iter()
            .map(|block| block.terminator())
            .collect()
    }

    fn match_pattern<'a>(terminators: &[&'a Terminator]) -> &'a MatchPattern {
        let patterns = terminators
            .iter()
            .copied()
            .filter_map(|terminator| match terminator {
                Terminator::Match(matcher) => Some(matcher.pattern()),
                _ => None,
            })
            .collect::<Vec<_>>();
        match_pattern_from_nodes(&patterns)
    }

    fn match_pattern_from_nodes<'a>(patterns: &[&'a MatchPattern]) -> &'a MatchPattern {
        let [pattern] = patterns else {
            if patterns.is_empty() {
                panic!("let assert should lower to a match terminator");
            }
            panic!("let assert should lower to one match terminator");
        };
        pattern
    }

    fn bit_array_pattern(pattern: &MatchPattern) -> &BitArrayPattern {
        let MatchPattern::BitArray(pattern) = pattern else {
            panic!("source should lower a BitArray match pattern");
        };
        pattern
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let pattern = bit_array_pattern(match_pattern(&terminators(plan)));
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(pattern);
        });
    }
}
