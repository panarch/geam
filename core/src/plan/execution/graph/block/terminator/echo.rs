use crate::plan::EchoSite;
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::graph::{Edge, ParamLocal, StringLocalId};

pub(crate) struct Echo {
    subject: ParamLocal,
    message: Option<StringLocalId>,
    site: EchoSite,
    next: Edge,
}

impl Echo {
    pub(in crate::plan::execution) fn new(
        subject: ParamLocal,
        message: Option<StringLocalId>,
        site: EchoSite,
        next: Edge,
    ) -> Self {
        Self {
            subject,
            message,
            site,
            next,
        }
    }

    pub(crate) fn subject(&self) -> &ParamLocal {
        &self.subject
    }

    pub(crate) fn message(&self) -> Option<StringLocalId> {
        self.message
    }

    pub(crate) fn site(&self) -> &EchoSite {
        &self.site
    }

    pub(crate) fn next(&self) -> &Edge {
        &self.next
    }
}

impl Explain for Echo {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        context.push_str("echo subject=");
        context.write(self.subject());
        context.push_str(" message=");
        match self.message() {
            Some(message) => context.write(&message),
            None => context.push_str("none"),
        }
        context.push_str(" site=");
        context.push_str(self.site().module());
        context.push_str("::");
        context.push_str(self.site().function());
        context.push('@');
        context.push_str(&self.site().span().start().to_string());
        context.push_str("..");
        context.push_str(&self.site().span().end().to_string());
        context.push_str(" next=");
        context.write(self.next());
    }
}

#[cfg(test)]
mod explain_tests {
    use super::super::Terminator;
    use super::Echo;
    use crate::plan::execution::explain;
    use crate::plan::execution::function::IntFunctionId;

    #[test]
    fn writes_echo_subject_message_site_and_continuation() {
        let source = r#"
pub fn main() {
  echo 1 as "selected"
}
"#;
        let expected =
            "echo subject=%int#0 message=%string#0 site=main::main@19..39 next=b1(%int#0)";

        assert_explanation(source, expected);
    }

    #[test]
    fn writes_absent_message() {
        let source = "pub fn main() { echo 1 }";
        let expected = "echo subject=%int#0 message=none site=main::main@16..22 next=b1(%int#0)";

        assert_explanation(source, expected);
    }

    #[test]
    #[should_panic(expected = "source should lower one echo terminator")]
    fn echo_shape_guard_is_visible() {
        explain::with_execution_plan("pub fn main() { 1 }", |plan| {
            echo(&terminators(plan));
        });
    }

    #[test]
    #[should_panic(expected = "source should lower one echo terminator")]
    fn echo_uniqueness_guard_is_visible() {
        let source = "pub fn main() { echo 1 }";
        explain::with_execution_plan(source, |plan| {
            let echo = echo(&terminators(plan));
            echo_from_nodes(&[echo, echo]);
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

    fn echo<'a>(terminators: &[&'a Terminator]) -> &'a Echo {
        let echoes = terminators
            .iter()
            .copied()
            .filter_map(|terminator| match terminator {
                Terminator::Echo(echo) => Some(echo),
                _ => None,
            })
            .collect::<Vec<_>>();
        echo_from_nodes(&echoes)
    }

    fn echo_from_nodes<'a>(echoes: &[&'a Echo]) -> &'a Echo {
        let [echo] = echoes else {
            panic!("source should lower one echo terminator");
        };
        echo
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let echo = echo(&terminators(plan));
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(echo);
        });
    }
}
