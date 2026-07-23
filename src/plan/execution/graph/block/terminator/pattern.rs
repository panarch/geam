mod bit_array;
mod list;

pub(crate) use bit_array::{
    BitArrayBindingPattern, BitArrayPattern, BitArrayPatternSegment, BitArrayPatternSize,
    BitArrayPatternSizeExpr, BitArrayPatternValue, BitArrayStringPattern, Signedness,
};
pub(crate) use list::{MatchPatternList, MatchPatternListTail};

use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::type_::CustomConstructorId;
use ecow::EcoString;
use num_bigint::BigInt;

pub(crate) enum MatchPattern {
    Bind(MatchPatternBinding),
    Discard,
    Int(BigInt),
    Float(f64),
    String(EcoString),
    Bool(bool),
    Nil,
    Tuple(Box<[MatchPattern]>),
    List(MatchPatternList),
    BitArray(BitArrayPattern),
    Custom {
        constructor: CustomConstructorId,
        fields: Box<[MatchPattern]>,
    },
    StringPrefix {
        prefix: EcoString,
        left: Option<MatchPatternBinding>,
        right: Option<MatchPatternBinding>,
    },
    Alias {
        pattern: Box<MatchPattern>,
        binding: MatchPatternBinding,
    },
}

pub(crate) struct MatchPatternBinding {
    index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct MatchIntBindingId(usize);

impl MatchPatternBinding {
    pub(in crate::plan::execution) fn new(index: usize) -> Self {
        Self { index }
    }

    pub(crate) fn int_id(&self) -> MatchIntBindingId {
        MatchIntBindingId(self.index)
    }

    pub(in crate::plan::execution) fn index(&self) -> usize {
        self.index
    }
}

impl MatchIntBindingId {
    pub(in crate::plan::execution) fn new(index: usize) -> Self {
        Self(index)
    }

    pub(in crate::plan::execution) fn index(self) -> usize {
        self.0
    }
}

impl Explain for MatchPattern {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        match self {
            Self::Bind(binding) => context.write(binding),
            Self::Discard => context.push('_'),
            Self::Int(value) => context.push_str(&value.to_string()),
            Self::Float(value) => context.push_str(&format!("{value:?}")),
            Self::String(value) => context.push_str(&format!("{value:?}")),
            Self::Bool(value) => context.push_str(if *value { "True" } else { "False" }),
            Self::Nil => context.push_str("Nil"),
            Self::Tuple(elements) => {
                context.push_str("#(");
                write_patterns(context, elements);
                context.push(')');
            }
            Self::List(list) => context.write(list),
            Self::BitArray(pattern) => context.write(pattern),
            Self::Custom {
                constructor,
                fields,
            } => {
                context.push_str("custom_type#");
                context.push_str(&constructor.type_id().index().to_string());
                context.push_str(".constructor#");
                context.push_str(&constructor.index().to_string());
                context.push('(');
                write_patterns(context, fields);
                context.push(')');
            }
            Self::StringPrefix {
                prefix,
                left,
                right,
            } => {
                context.push_str("string_prefix(");
                context.push_str(&format!("{prefix:?}"));
                context.push_str(", left=");
                write_optional_binding(context, left.as_ref());
                context.push_str(", right=");
                write_optional_binding(context, right.as_ref());
                context.push(')');
            }
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

impl Explain for MatchPatternBinding {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        context.push_str("binding#");
        context.push_str(&self.index().to_string());
    }
}

fn write_patterns(context: &mut ExplainContext<'_, '_>, patterns: &[MatchPattern]) {
    for (index, pattern) in patterns.iter().enumerate() {
        if index > 0 {
            context.push_str(", ");
        }
        context.write(pattern);
    }
}

fn write_optional_binding(
    context: &mut ExplainContext<'_, '_>,
    binding: Option<&MatchPatternBinding>,
) {
    match binding {
        Some(binding) => context.write(binding),
        None => context.push('_'),
    }
}

#[cfg(test)]
mod explain_tests {
    use super::super::Terminator;
    use super::MatchPattern;
    use crate::plan::execution::explain;
    use crate::plan::execution::function::IntFunctionId;

    #[test]
    fn writes_nested_patterns_from_a_lowered_match() {
        let source = r#"
pub type Payload { Payload(Int) }

fn identity(value: #(List(Int), Payload, String)) { value }

pub fn main() {
  let value = identity(#([1, 2], Payload(3), "prefix"))
  let assert #([1, ..rest], Payload(number), "pre" <> suffix) as whole = value
  number
}
"#;
        let expected = "alias(#([1, ..binding#0], custom_type#0.constructor#0(binding#1), string_prefix(\"pre\", left=_, right=binding#2)), binding#3)";

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
    #[should_panic(expected = "let assert should lower to one match terminator")]
    fn match_pattern_uniqueness_guard_is_visible() {
        let source = r#"
pub fn main() {
  let values = [1]
  let assert [head, ..] = values
  head
}
"#;
        explain::with_execution_plan(source, |plan| {
            let pattern = match_pattern(&terminators(plan));
            match_pattern_from_nodes(&[pattern, pattern]);
        });
    }

    fn terminators(
        plan: &crate::plan::execution::ExecutionPlan,
    ) -> Vec<&crate::plan::execution::graph::Terminator> {
        plan.int_function(IntFunctionId(0))
            .body()
            .block_graph()
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

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let pattern = match_pattern(&terminators(plan));
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(pattern);
        });
    }
}
