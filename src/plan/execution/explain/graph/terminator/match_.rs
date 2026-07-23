use super::super::super::pattern::write_pattern;
use super::super::super::value::ExplainLocal;
use super::super::edge::{write_edge, write_match_edge};
use crate::plan::execution::{Edge, MatchEdge, MatchPattern, ParamLocal};

pub(super) fn write_match(
    output: &mut String,
    subject: &ParamLocal,
    pattern: &MatchPattern,
    success: &MatchEdge,
    failure: &Edge,
) {
    output.push_str("match ");
    subject.write_local(output);
    output.push_str(" pattern=");
    write_pattern(output, pattern);
    output.push_str(" success=");
    write_match_edge(output, success);
    output.push_str(" failure=");
    write_edge(output, failure);
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::{IntFunctionId, Terminator};

    #[test]
    fn writes_refutable_match() {
        let source = r#"
pub fn main() {
  let values = [1]
  let assert [head, ..] = values
  head
}
"#;
        let expected = "match %list.int#0 pattern=[binding#0, .._] success=b1(binding#0) failure=b2(%list.int#0)";

        assert_explanation(source, expected);
    }

    fn assert_explanation(source: &str, expected: &str) {
        super::super::super::super::assert_rendered(source, expected, |plan, output| {
            let function = plan.int_function(IntFunctionId(0));
            let terminators = function
                .graph()
                .blocks()
                .iter()
                .map(|block| block.terminator())
                .collect::<Vec<_>>();
            let (subject, pattern, success, failure) = match_terminator(&terminators);
            super::write_match(output, subject, pattern, success, failure);
        });
    }

    type MatchTerminator<'a> = (
        &'a crate::plan::execution::ParamLocal,
        &'a crate::plan::execution::MatchPattern,
        &'a crate::plan::execution::MatchEdge,
        &'a crate::plan::execution::Edge,
    );

    fn match_terminator<'a>(terminators: &[&'a Terminator]) -> MatchTerminator<'a> {
        let mut matches = terminators
            .iter()
            .copied()
            .filter_map(|terminator| match terminator {
                Terminator::Match(matcher) => Some((
                    matcher.subject(),
                    matcher.pattern(),
                    matcher.success(),
                    matcher.failure(),
                )),
                _ => None,
            });
        let Some(match_) = matches.next() else {
            panic!("source should lower one match terminator");
        };
        if matches.next().is_some() {
            panic!("source should lower one match terminator");
        }
        match_
    }

    #[test]
    #[should_panic(expected = "source should lower one match terminator")]
    fn match_shape_guard_is_visible() {
        super::super::super::super::with_execution_plan("pub fn main() { 1 }", |plan| {
            let terminators = plan
                .int_function(IntFunctionId(0))
                .graph()
                .blocks()
                .iter()
                .map(|block| block.terminator())
                .collect::<Vec<_>>();
            match_terminator(&terminators);
        });
    }

    #[test]
    #[should_panic(expected = "source should lower one match terminator")]
    fn match_uniqueness_guard_is_visible() {
        let source = r#"
pub fn main() {
  let values = [1]
  let assert [head, ..] = values
  head
}
"#;
        super::super::super::super::with_execution_plan(source, |plan| {
            let function = plan.int_function(IntFunctionId(0));
            let terminator = function
                .graph()
                .blocks()
                .iter()
                .map(|block| block.terminator())
                .find(|terminator| matches!(terminator, Terminator::Match(_)))
                .expect("source should lower a match terminator");
            match_terminator(&[terminator, terminator]);
        });
    }
}
