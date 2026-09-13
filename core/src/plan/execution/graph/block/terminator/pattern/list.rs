use super::{MatchPattern, MatchPatternBinding};
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::Table;

#[derive(Clone)]
pub struct MatchPatternList {
    pub elements: Table<MatchPattern>,
    pub tail: Option<MatchPatternListTail>,
}

#[derive(Clone)]
pub enum MatchPatternListTail {
    Ignore,
    Bind(MatchPatternBinding),
}

impl MatchPatternList {
    pub(in crate::plan::execution) fn new(
        elements: Vec<MatchPattern>,
        tail: Option<MatchPatternListTail>,
    ) -> Self {
        Self {
            elements: elements.into(),
            tail,
        }
    }

    pub(crate) fn elements(&self) -> &[MatchPattern] {
        &self.elements
    }

    pub(crate) fn tail(&self) -> Option<&MatchPatternListTail> {
        self.tail.as_ref()
    }
}

impl Explain for MatchPatternList {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        context.push('[');
        let mut separator = "";
        for pattern in self.elements() {
            context.push_str(separator);
            context.write(pattern);
            separator = ", ";
        }
        if let Some(tail) = self.tail() {
            context.push_str(separator);
            context.push_str("..");
            match tail {
                MatchPatternListTail::Ignore => context.push('_'),
                MatchPatternListTail::Bind(binding) => context.write(binding),
            }
        }
        context.push(']');
    }
}

impl Emit for MatchPatternList {
    fn emit(&self, output: &mut Rust) {
        let Self { elements, tail } = self;
        output.structure(
            "graph::MatchPatternList",
            &[("elements", elements), ("tail", tail)],
        );
    }
}

impl Emit for MatchPatternListTail {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Ignore => output.path("graph::MatchPatternListTail::Ignore"),
            Self::Bind(field_0) => output.call("graph::MatchPatternListTail::Bind", &[field_0]),
        }
    }
}

#[cfg(test)]
mod emission_tests {
    use super::{MatchPattern, MatchPatternBinding, MatchPatternList, MatchPatternListTail, Rust};

    #[test]
    fn emits_closed_ignored_and_bound_list_tails() {
        assert_eq!(
            Rust::expression(&MatchPatternListTail::Ignore),
            "data::graph::MatchPatternListTail::Ignore"
        );
        assert_eq!(
            Rust::expression(&MatchPatternListTail::Bind(MatchPatternBinding::new(2))),
            "data::graph::MatchPatternListTail::Bind(data::graph::MatchPatternBinding {index: 2,},)"
        );
        for (tail, expected) in [
            (
                None,
                "data::graph::MatchPatternList {elements: data::Storage::Static(&[data::graph::MatchPattern::Discard,]),tail: None,}",
            ),
            (
                Some(MatchPatternListTail::Ignore),
                "data::graph::MatchPatternList {elements: data::Storage::Static(&[data::graph::MatchPattern::Discard,]),tail: Some(data::graph::MatchPatternListTail::Ignore),}",
            ),
            (
                Some(MatchPatternListTail::Bind(MatchPatternBinding::new(2))),
                concat!(
                    "data::graph::MatchPatternList {elements: data::Storage::Static(&[data::graph::MatchPattern::Discard,]),",
                    "tail: Some(data::graph::MatchPatternListTail::Bind(data::graph::MatchPatternBinding {index: 2,},)),}"
                ),
            ),
        ] {
            assert_eq!(
                Rust::expression(&MatchPatternList::new(vec![MatchPattern::Discard], tail)),
                expected
            );
        }
    }
}

#[cfg(test)]
mod explain_tests {
    use super::{MatchPattern, MatchPatternList};
    use crate::plan::execution::explain;
    use crate::plan::execution::function::IntFunctionId;
    use crate::plan::execution::graph::Terminator;

    #[test]
    fn writes_bound_list_tail() {
        let source = r#"
fn identity(values: List(Int)) { values }

pub fn main() {
  let values = identity([1])
  let assert [head, ..tail] = values
  head
}
"#;
        let expected = "[binding#0, ..binding#1]";

        assert_explanation(source, expected);
    }

    #[test]
    fn writes_ignored_list_tail() {
        let source = r#"
fn identity(values: List(Int)) { values }

pub fn main() {
  let values = identity([1])
  let assert [head, ..] = values
  head
}
"#;
        let expected = "[binding#0, .._]";

        assert_explanation(source, expected);
    }

    #[test]
    #[should_panic(expected = "let assert should lower to a match terminator")]
    fn match_shape_guard_is_visible() {
        explain::with_execution_plan("pub fn main() { 1 }", |plan| {
            list_pattern(
                plan.int_function(IntFunctionId(0))
                    .body()
                    .block_graph()
                    .blocks()
                    .next()
                    .unwrap()
                    .terminator(),
            );
        });
    }

    #[test]
    #[should_panic(expected = "let assert should lower to a List pattern")]
    fn list_pattern_shape_guard_is_visible() {
        let source = r#"
fn identity(value: Int) { value }

pub fn main() {
  let value = identity(1)
  let assert 1 = value
  value
}
"#;
        explain::with_execution_plan(source, |plan| {
            list_pattern(
                plan.int_function(IntFunctionId(0))
                    .body()
                    .block_graph()
                    .blocks()
                    .next()
                    .unwrap()
                    .terminator(),
            );
        });
    }

    fn list_pattern(terminator: &Terminator) -> &MatchPatternList {
        let Terminator::Match(matcher) = terminator else {
            panic!("let assert should lower to a match terminator");
        };
        let MatchPattern::List(pattern) = matcher.pattern() else {
            panic!("let assert should lower to a List pattern");
        };
        pattern
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let terminator = plan
                .int_function(IntFunctionId(0))
                .body()
                .block_graph()
                .blocks()
                .next()
                .unwrap()
                .terminator();
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(list_pattern(terminator));
        });
    }
}
