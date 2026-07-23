mod segment;

use self::segment::write_segment;
use super::super::super::graph::BitArrayPattern;

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

#[cfg(test)]
mod tests {
    use super::super::super::super::{IntFunctionId, MatchPattern, Terminator};
    use crate::plan::execution::graph::BitArrayPattern;

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

    fn assert_explanation(source: &str, expected: &str) {
        super::super::super::assert_rendered(source, expected, |plan, output| {
            let function = plan.int_function(IntFunctionId(0));
            let terminators = function
                .graph()
                .blocks()
                .iter()
                .map(|block| block.terminator())
                .collect::<Vec<_>>();
            super::write_bit_array(output, bit_array_pattern(match_pattern(&terminators)));
        });
    }

    fn match_pattern<'a>(terminators: &[&'a Terminator]) -> &'a MatchPattern {
        let mut patterns = terminators
            .iter()
            .copied()
            .filter_map(|terminator| match terminator {
                Terminator::Match { pattern, .. } => Some(pattern),
                _ => None,
            });
        let Some(pattern) = patterns.next() else {
            panic!("let assert should lower to a match terminator");
        };
        if patterns.next().is_some() {
            panic!("let assert should lower to one match terminator");
        }
        pattern
    }

    fn bit_array_pattern(pattern: &MatchPattern) -> &BitArrayPattern {
        let MatchPattern::BitArray(pattern) = pattern else {
            panic!("source should lower a BitArray match pattern");
        };
        pattern
    }

    #[test]
    #[should_panic(expected = "let assert should lower to a match terminator")]
    fn match_pattern_shape_guard_is_visible() {
        super::super::super::with_execution_plan("pub fn main() { 1 }", |plan| {
            let terminators = plan
                .int_function(IntFunctionId(0))
                .graph()
                .blocks()
                .iter()
                .map(|block| block.terminator())
                .collect::<Vec<_>>();
            match_pattern(&terminators);
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
        super::super::super::with_execution_plan(source, |plan| {
            let function = plan.int_function(IntFunctionId(1));
            let terminators = function
                .graph()
                .blocks()
                .iter()
                .map(|block| block.terminator())
                .collect::<Vec<_>>();
            bit_array_pattern(match_pattern(&terminators));
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
        super::super::super::with_execution_plan(source, |plan| {
            let function = plan.int_function(IntFunctionId(0));
            let terminator = function
                .graph()
                .blocks()
                .iter()
                .map(|block| block.terminator())
                .find(|terminator| matches!(terminator, Terminator::Match { .. }))
                .expect("source should lower a match terminator");
            match_pattern(&[terminator, terminator]);
        });
    }
}
