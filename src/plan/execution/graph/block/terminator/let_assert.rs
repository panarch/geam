use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::{ParamLocal, StringLocalId};
use crate::plan::{PanicSite, SourceSpan};

pub(crate) struct LetAssertPanic {
    subject: ParamLocal,
    message: Option<StringLocalId>,
    site: PanicSite,
    pattern_span: SourceSpan,
}

impl LetAssertPanic {
    pub(in crate::plan::execution) fn new(
        subject: ParamLocal,
        message: Option<StringLocalId>,
        site: PanicSite,
        pattern_span: SourceSpan,
    ) -> Self {
        Self {
            subject,
            message,
            site,
            pattern_span,
        }
    }

    pub(crate) fn subject(&self) -> &ParamLocal {
        &self.subject
    }

    pub(crate) fn message(&self) -> Option<StringLocalId> {
        self.message
    }

    pub(crate) fn site(&self) -> &PanicSite {
        &self.site
    }

    pub(crate) fn pattern_span(&self) -> &SourceSpan {
        &self.pattern_span
    }
}

impl Explain for LetAssertPanic {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        context.push_str("let_assert_panic subject=");
        context.write(self.subject());
        context.push_str(" message=");
        match self.message() {
            Some(message) => context.write(&message),
            None => context.push_str("none"),
        }
    }
}

#[cfg(test)]
mod explain_tests {
    use super::super::Terminator;
    use super::LetAssertPanic;
    use crate::plan::execution::{IntFunctionId, explain};

    #[test]
    fn writes_let_assert_panic() {
        let source = r#"
pub fn main() {
  let values = [1]
  let assert [head, ..] = values
  head
}
"#;
        let expected = "let_assert_panic subject=%list.int#0 message=none";

        assert_explanation(source, expected);
    }

    #[test]
    #[should_panic(expected = "source should lower one let-assert panic terminator")]
    fn let_assert_panic_shape_guard_is_visible() {
        explain::with_execution_plan("pub fn main() { 1 }", |plan| {
            let_assert_panic(&terminators(plan));
        });
    }

    #[test]
    #[should_panic(expected = "source should lower one let-assert panic terminator")]
    fn let_assert_panic_uniqueness_guard_is_visible() {
        let source = r#"
pub fn main() {
  let values = [1]
  let assert [head, ..] = values
  head
}
"#;
        explain::with_execution_plan(source, |plan| {
            let panic = let_assert_panic(&terminators(plan));
            let_assert_panic_from_nodes(&[panic, panic]);
        });
    }

    fn terminators(
        plan: &crate::plan::execution::ExecutionPlan,
    ) -> Vec<&crate::plan::execution::Terminator> {
        plan.int_function(IntFunctionId(0))
            .body()
            .block_graph()
            .blocks()
            .iter()
            .map(|block| block.terminator())
            .collect()
    }

    fn let_assert_panic<'a>(terminators: &[&'a Terminator]) -> &'a LetAssertPanic {
        let panics = terminators
            .iter()
            .copied()
            .filter_map(|terminator| match terminator {
                Terminator::LetAssertPanic(panic) => Some(panic),
                _ => None,
            })
            .collect::<Vec<_>>();
        let_assert_panic_from_nodes(&panics)
    }

    fn let_assert_panic_from_nodes<'a>(panics: &[&'a LetAssertPanic]) -> &'a LetAssertPanic {
        let [panic] = panics else {
            panic!("source should lower one let-assert panic terminator");
        };
        panic
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let panic = let_assert_panic(&terminators(plan));
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(panic);
        });
    }
}
