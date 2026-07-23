use crate::plan::PanicSite;
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::graph::StringLocalId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SourceStopKind {
    Panic,
    Todo,
    Assert,
    EmptyFunction,
    EmptyBlock,
    IncompleteUse,
}

pub(crate) struct SourceStop {
    kind: SourceStopKind,
    message: Option<StringLocalId>,
    site: PanicSite,
}

impl SourceStop {
    pub(in crate::plan::execution) fn new(
        kind: SourceStopKind,
        message: Option<StringLocalId>,
        site: PanicSite,
    ) -> Self {
        Self {
            kind,
            message,
            site,
        }
    }

    pub(crate) fn kind(&self) -> SourceStopKind {
        self.kind
    }

    pub(crate) fn message(&self) -> Option<StringLocalId> {
        self.message
    }

    pub(crate) fn site(&self) -> &PanicSite {
        &self.site
    }
}

impl Explain for SourceStop {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        context.push_str("source_stop kind=");
        context.push_str(source_stop_kind(self.kind()));
        context.push_str(" message=");
        match self.message() {
            Some(message) => context.write(&message),
            None => context.push_str("none"),
        }
    }
}

fn source_stop_kind(kind: SourceStopKind) -> &'static str {
    match kind {
        SourceStopKind::Panic => "panic",
        SourceStopKind::Todo => "todo",
        SourceStopKind::Assert => "assert",
        SourceStopKind::EmptyFunction => "empty_function",
        SourceStopKind::EmptyBlock => "empty_block",
        SourceStopKind::IncompleteUse => "incomplete_use",
    }
}

#[cfg(test)]
mod explain_tests {
    use super::super::Terminator;
    use super::{SourceStop, SourceStopKind};
    use crate::plan::execution::explain;
    use crate::plan::execution::function::IntFunctionId;

    #[test]
    fn writes_source_stop() {
        let source = r#"
pub fn main() -> Int {
  panic as "stopped"
}
"#;
        let expected = "source_stop kind=panic message=%string#0";

        assert_explanation(source, expected);
    }

    #[test]
    fn writes_every_source_stop_kind_token() {
        let cases = [
            (SourceStopKind::Panic, "panic"),
            (SourceStopKind::Todo, "todo"),
            (SourceStopKind::Assert, "assert"),
            (SourceStopKind::EmptyFunction, "empty_function"),
            (SourceStopKind::EmptyBlock, "empty_block"),
            (SourceStopKind::IncompleteUse, "incomplete_use"),
        ];

        for (kind, expected) in cases {
            explain::assert_written(expected, |output| {
                output.push_str(super::source_stop_kind(kind));
            });
        }
    }

    #[test]
    #[should_panic(expected = "source should lower one source-stop terminator")]
    fn source_stop_shape_guard_is_visible() {
        explain::with_execution_plan("pub fn main() { 1 }", |plan| {
            source_stop(&terminators(plan));
        });
    }

    #[test]
    #[should_panic(expected = "source should lower one source-stop terminator")]
    fn source_stop_uniqueness_guard_is_visible() {
        explain::with_execution_plan("pub fn main() -> Int { panic }", |plan| {
            let stop = source_stop(&terminators(plan));
            source_stop_from_nodes(&[stop, stop]);
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

    fn source_stop<'a>(terminators: &[&'a Terminator]) -> &'a SourceStop {
        let stops = terminators
            .iter()
            .copied()
            .filter_map(|terminator| match terminator {
                Terminator::SourceStop(stop) => Some(stop),
                _ => None,
            })
            .collect::<Vec<_>>();
        source_stop_from_nodes(&stops)
    }

    fn source_stop_from_nodes<'a>(stops: &[&'a SourceStop]) -> &'a SourceStop {
        let [stop] = stops else {
            panic!("source should lower one source-stop terminator");
        };
        stop
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let stop = source_stop(&terminators(plan));
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(stop);
        });
    }
}
