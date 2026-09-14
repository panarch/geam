use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::{Node, Table};
pub(in crate::plan::execution) mod bit_array;
pub(in crate::plan::execution) mod list;

pub(crate) use bit_array::{
    BitArrayBindingPattern, BitArrayPattern, BitArrayPatternSegment, BitArrayPatternSize,
    BitArrayPatternSizeExpr, BitArrayPatternValue, BitArrayStringPattern, Signedness,
};
pub(crate) use list::{MatchPatternList, MatchPatternListTail};

use crate::plan::Text;
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::graph::IntegerLiteral;
use crate::plan::execution::type_::CustomConstructorId;

#[derive(Clone)]
pub enum MatchPattern {
    Bind(MatchPatternBinding),
    Discard,
    Int(IntegerLiteral),
    Float(f64),
    String(Text),
    Bool(bool),
    Nil,
    Tuple(Table<MatchPattern>),
    List(MatchPatternList),
    BitArray(BitArrayPattern),
    Custom {
        constructor: CustomConstructorId,
        fields: Table<MatchPattern>,
    },
    StringPrefix {
        prefix: Text,
        left: Option<MatchPatternBinding>,
        right: Option<MatchPatternBinding>,
    },
    Alias {
        pattern: Node<MatchPattern>,
        binding: MatchPatternBinding,
    },
}

#[derive(Clone)]
pub struct MatchPatternBinding {
    pub index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MatchIntBindingId(pub usize);

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

impl Emit for MatchPattern {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Bind(field_0) => output.call("graph::MatchPattern::Bind", &[field_0]),
            Self::Discard => output.path("graph::MatchPattern::Discard"),
            Self::Int(field_0) => output.call("graph::MatchPattern::Int", &[field_0]),
            Self::Float(field_0) => output.call("graph::MatchPattern::Float", &[field_0]),
            Self::String(field_0) => output.call("graph::MatchPattern::String", &[field_0]),
            Self::Bool(field_0) => output.call("graph::MatchPattern::Bool", &[field_0]),
            Self::Nil => output.path("graph::MatchPattern::Nil"),
            Self::Tuple(field_0) => output.call("graph::MatchPattern::Tuple", &[field_0]),
            Self::List(field_0) => output.call("graph::MatchPattern::List", &[field_0]),
            Self::BitArray(field_0) => output.call("graph::MatchPattern::BitArray", &[field_0]),
            Self::Custom {
                constructor,
                fields,
            } => output.structure(
                "graph::MatchPattern::Custom",
                &[("constructor", constructor), ("fields", fields)],
            ),
            Self::StringPrefix {
                prefix,
                left,
                right,
            } => output.structure(
                "graph::MatchPattern::StringPrefix",
                &[("prefix", prefix), ("left", left), ("right", right)],
            ),
            Self::Alias { pattern, binding } => output.structure(
                "graph::MatchPattern::Alias",
                &[("pattern", pattern), ("binding", binding)],
            ),
        }
    }
}

impl Emit for MatchPatternBinding {
    fn emit(&self, output: &mut Rust) {
        let Self { index } = self;
        output.structure("graph::MatchPatternBinding", &[("index", index)]);
    }
}

impl Emit for MatchIntBindingId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::MatchIntBindingId", &[field_0]);
    }
}

#[cfg(test)]
mod emission_tests {
    use super::{
        BitArrayPattern, CustomConstructorId, MatchIntBindingId, MatchPattern, MatchPatternBinding,
        MatchPatternList, Node, Rust,
    };
    use crate::plan::execution::type_::CustomTypeId;

    #[test]
    fn emits_every_match_pattern_and_preserves_nested_bindings() {
        assert_eq!(
            Rust::expression(&MatchPatternBinding::new(3)),
            r#"
data::graph::MatchPatternBinding {
    index: 3,
}"#
            .trim_start_matches('\n')
        );
        assert_eq!(
            Rust::expression(&MatchIntBindingId::new(2)),
            "data::graph::MatchIntBindingId(2)"
        );
        let cases = [
            (
                MatchPattern::Bind(MatchPatternBinding::new(3)),
                r#"
data::graph::MatchPattern::Bind(data::graph::MatchPatternBinding {
    index: 3,
})"#
                .trim_start_matches('\n'),
            ),
            (MatchPattern::Discard, "data::graph::MatchPattern::Discard"),
            (
                MatchPattern::Int(num_bigint::BigInt::from(1).into()),
                r#"
data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
    sign: data::Sign::Plus,
    digits: data::Storage::Static(&[
        1,
    ]),
})"#
                .trim_start_matches('\n'),
            ),
            (
                MatchPattern::Float(-0.0),
                "data::graph::MatchPattern::Float(f64::from_bits(9223372036854775808))",
            ),
            (
                MatchPattern::String("test".into()),
                "data::graph::MatchPattern::String(data::Text::Static(\"test\"))",
            ),
            (
                MatchPattern::Bool(true),
                "data::graph::MatchPattern::Bool(true)",
            ),
            (MatchPattern::Nil, "data::graph::MatchPattern::Nil"),
            (
                MatchPattern::Tuple(vec![MatchPattern::Nil].into()),
                r#"
data::graph::MatchPattern::Tuple(data::Storage::Static(&[
    data::graph::MatchPattern::Nil,
]))"#
                    .trim_start_matches('\n'),
            ),
            (
                MatchPattern::List(MatchPatternList::new(vec![MatchPattern::Discard], None)),
                r#"
data::graph::MatchPattern::List(data::graph::MatchPatternList {
    elements: data::Storage::Static(&[
        data::graph::MatchPattern::Discard,
    ]),
    tail: None,
})"#
                .trim_start_matches('\n'),
            ),
            (
                MatchPattern::BitArray(BitArrayPattern::new(Vec::new())),
                r#"
data::graph::MatchPattern::BitArray(data::graph::BitArrayPattern {
    segments: data::Storage::Static(&[]),
})"#
                .trim_start_matches('\n'),
            ),
            (
                MatchPattern::Custom {
                    constructor: CustomConstructorId::new(CustomTypeId(2), 4),
                    fields: vec![MatchPattern::Nil].into(),
                },
                r#"
data::graph::MatchPattern::Custom {
    constructor: data::type_::CustomConstructorId {
        type_id: data::type_::CustomTypeId(2),
        index: 4,
    },
    fields: data::Storage::Static(&[
        data::graph::MatchPattern::Nil,
    ]),
}"#
                .trim_start_matches('\n'),
            ),
            (
                MatchPattern::StringPrefix {
                    prefix: "pre".into(),
                    left: None,
                    right: Some(MatchPatternBinding::new(3)),
                },
                r#"
data::graph::MatchPattern::StringPrefix {
    prefix: data::Text::Static("pre"),
    left: None,
    right: Some(data::graph::MatchPatternBinding {
        index: 3,
    }),
}"#
                .trim_start_matches('\n'),
            ),
            (
                MatchPattern::Alias {
                    pattern: Node::Static(&MatchPattern::Discard),
                    binding: MatchPatternBinding::new(3),
                },
                r#"
data::graph::MatchPattern::Alias {
    pattern: data::Storage::Static(&data::graph::MatchPattern::Discard),
    binding: data::graph::MatchPatternBinding {
        index: 3,
    },
}"#
                .trim_start_matches('\n'),
            ),
        ];
        for (pattern, expected) in cases {
            assert_eq!(Rust::expression(&pattern), expected);
        }
    }
}

#[cfg(test)]
mod explain_tests {
    use super::super::Terminator;
    use super::{MatchPattern, MatchPatternBinding};
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
    fn writes_match_pattern_binding() {
        let source = "pub fn main() { 1 }";
        let expected = "binding#3";

        explain::assert_rendered(source, expected, |plan, output| {
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(&MatchPatternBinding::new(3));
        });
    }

    #[test]
    fn writes_nil_inside_a_refutable_tuple_pattern() {
        assert_explanation(
            "pub fn main() { let assert #(1, Nil) = #(1, Nil) 42 }",
            "#(1, Nil)",
        );
    }

    #[test]
    fn writes_scalar_discard_and_binary_patterns_inside_a_tuple() {
        for (pattern, value, expected) in [
            ("_", "Nil", "_"),
            ("True", "True", "True"),
            ("False", "False", "False"),
            ("1.5", "1.5", "1.5"),
            ("\"text\"", "\"text\"", "\"text\""),
            ("<<42>>", "<<42>>", "<<int(42, size=8*1, big, unsigned)>>"),
        ] {
            assert_explanation(
                &format!("pub fn main() {{ let assert #(1, {pattern}) = #(1, {value}) 42 }}"),
                &format!("#(1, {expected})"),
            );
        }
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
