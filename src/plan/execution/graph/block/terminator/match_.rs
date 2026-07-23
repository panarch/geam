use super::{Edge, MatchEdge, MatchPattern};
use crate::plan::execution::ParamLocal;
use crate::plan::execution::explain::{Explain, ExplainContext};

pub(crate) struct Match {
    subject: ParamLocal,
    pattern: MatchPattern,
    success: MatchEdge,
    failure: Edge,
}

impl Match {
    pub(in crate::plan::execution) fn new(
        subject: ParamLocal,
        pattern: MatchPattern,
        success: MatchEdge,
        failure: Edge,
    ) -> Self {
        Self {
            subject,
            pattern,
            success,
            failure,
        }
    }

    pub(crate) fn subject(&self) -> &ParamLocal {
        &self.subject
    }

    pub(crate) fn pattern(&self) -> &MatchPattern {
        &self.pattern
    }

    pub(crate) fn success(&self) -> &MatchEdge {
        &self.success
    }

    pub(crate) fn failure(&self) -> &Edge {
        &self.failure
    }
}

impl Explain for Match {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        context.push_str("match ");
        context.write(self.subject());
        context.push_str(" pattern=");
        context.write(self.pattern());
        context.push_str(" success=");
        context.write(self.success());
        context.push_str(" failure=");
        context.write(self.failure());
    }
}

#[cfg(test)]
mod explain_tests {
    use super::super::Terminator;
    use super::Match;
    use crate::plan::execution::{IntFunctionId, explain};

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

    #[test]
    #[should_panic(expected = "source should lower one match terminator")]
    fn match_shape_guard_is_visible() {
        explain::with_execution_plan("pub fn main() { 1 }", |plan| {
            matcher(&terminators(plan));
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
        explain::with_execution_plan(source, |plan| {
            let matcher = matcher(&terminators(plan));
            match_from_nodes(&[matcher, matcher]);
        });
    }

    fn terminators(
        plan: &crate::plan::execution::ExecutionPlan,
    ) -> Vec<&crate::plan::execution::Terminator> {
        plan.int_function(IntFunctionId(0))
            .graph()
            .blocks()
            .iter()
            .map(|block| block.terminator())
            .collect()
    }

    fn matcher<'a>(terminators: &[&'a Terminator]) -> &'a Match {
        let matches = terminators
            .iter()
            .copied()
            .filter_map(|terminator| match terminator {
                Terminator::Match(matcher) => Some(matcher),
                _ => None,
            })
            .collect::<Vec<_>>();
        match_from_nodes(&matches)
    }

    fn match_from_nodes<'a>(matches: &[&'a Match]) -> &'a Match {
        let [matcher] = matches else {
            panic!("source should lower one match terminator");
        };
        matcher
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let matcher = matcher(&terminators(plan));
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(matcher);
        });
    }
}
