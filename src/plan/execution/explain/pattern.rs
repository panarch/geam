mod binding;
mod bit_array;
mod list;

pub(super) use self::binding::write_binding;
use self::bit_array::write_bit_array;
use self::list::write_list;
use super::super::graph::{MatchPattern, MatchPatternBinding};

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

fn write_optional_binding(output: &mut String, binding: Option<&MatchPatternBinding>) {
    match binding {
        Some(binding) => write_binding(output, binding),
        None => output.push('_'),
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::{IntFunctionId, Terminator};
    use super::MatchPattern;

    #[test]
    fn writes_nested_patterns_from_a_lowered_match() {
        assert_explanation(
            r#"
pub type Payload { Payload(Int) }

fn identity(value: #(List(Int), Payload, String)) { value }

pub fn main() {
  let value = identity(#([1, 2], Payload(3), "prefix"))
  let assert #([1, ..rest], Payload(number), "pre" <> suffix) as whole = value
  number
}
"#,
            "alias(#([1, ..binding#0], custom_type#0.constructor#0(binding#1), string_prefix(\"pre\", left=_, right=binding#2)), binding#3)",
        );
    }

    #[test]
    fn writes_bound_list_tail() {
        assert_explanation(
            r#"
fn identity(values: List(Int)) { values }

pub fn main() {
  let values = identity([1])
  let assert [head, ..tail] = values
  head
}
"#,
            "[binding#0, ..binding#1]",
        );
    }

    #[test]
    fn writes_ignored_list_tail() {
        assert_explanation(
            r#"
fn identity(values: List(Int)) { values }

pub fn main() {
  let values = identity([1])
  let assert [head, ..] = values
  head
}
"#,
            "[binding#0, .._]",
        );
    }

    fn assert_explanation(source: &str, expected: &str) {
        super::super::assert_rendered(source, expected, |plan, output| {
            let function = plan.int_function(IntFunctionId(0));
            let terminators = function
                .graph()
                .blocks()
                .iter()
                .map(|block| block.terminator())
                .collect::<Vec<_>>();
            super::write_pattern(output, match_pattern(&terminators));
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

    #[test]
    #[should_panic(expected = "let assert should lower to a match terminator")]
    fn match_pattern_shape_guard_is_visible() {
        super::super::with_execution_plan("pub fn main() { 1 }", |plan| {
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
    #[should_panic(expected = "let assert should lower to one match terminator")]
    fn match_pattern_uniqueness_guard_is_visible() {
        let source = r#"
pub fn main() {
  let values = [1]
  let assert [head, ..] = values
  head
}
"#;
        super::super::with_execution_plan(source, |plan| {
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
