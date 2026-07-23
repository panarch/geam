use super::super::super::value::ExplainLocal;
use crate::plan::execution::{SourceStopKind, StringLocalId};

pub(super) fn write_source_stop(
    output: &mut String,
    kind: SourceStopKind,
    message: Option<&StringLocalId>,
) {
    output.push_str("source_stop kind=");
    output.push_str(source_stop_kind(kind));
    output.push_str(" message=");
    match message {
        Some(message) => message.write_local(output),
        None => output.push_str("none"),
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
mod tests {
    use crate::plan::execution::{IntFunctionId, Terminator};

    #[test]
    fn writes_source_stop() {
        assert_explanation(
            r#"
pub fn main() -> Int {
  panic as "stopped"
}
"#,
            "source_stop kind=panic message=%string#0",
        );
    }

    #[test]
    fn writes_every_source_stop_kind_token() {
        let cases = [
            (super::SourceStopKind::Panic, "panic"),
            (super::SourceStopKind::Todo, "todo"),
            (super::SourceStopKind::Assert, "assert"),
            (super::SourceStopKind::EmptyFunction, "empty_function"),
            (super::SourceStopKind::EmptyBlock, "empty_block"),
            (super::SourceStopKind::IncompleteUse, "incomplete_use"),
        ];

        for (kind, expected) in cases {
            super::super::super::super::assert_written(expected, |output| {
                output.push_str(super::source_stop_kind(kind))
            });
        }
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
            let (kind, message) = source_stop(&terminators);
            super::write_source_stop(output, kind, message);
        });
    }

    fn source_stop<'a>(
        terminators: &[&'a Terminator],
    ) -> (
        crate::plan::execution::SourceStopKind,
        Option<&'a crate::plan::execution::StringLocalId>,
    ) {
        let mut stops = terminators
            .iter()
            .copied()
            .filter_map(|terminator| match terminator {
                Terminator::SourceStop { kind, message, .. } => Some((*kind, message.as_ref())),
                _ => None,
            });
        let Some(stop) = stops.next() else {
            panic!("source should lower one source-stop terminator");
        };
        if stops.next().is_some() {
            panic!("source should lower one source-stop terminator");
        }
        stop
    }

    #[test]
    #[should_panic(expected = "source should lower one source-stop terminator")]
    fn source_stop_shape_guard_is_visible() {
        super::super::super::super::with_execution_plan("pub fn main() { 1 }", |plan| {
            let terminators = plan
                .int_function(IntFunctionId(0))
                .graph()
                .blocks()
                .iter()
                .map(|block| block.terminator())
                .collect::<Vec<_>>();
            source_stop(&terminators);
        });
    }

    #[test]
    #[should_panic(expected = "source should lower one source-stop terminator")]
    fn source_stop_uniqueness_guard_is_visible() {
        super::super::super::super::with_execution_plan("pub fn main() -> Int { panic }", |plan| {
            let terminator = plan.int_function(IntFunctionId(0)).graph().blocks()[0].terminator();
            source_stop(&[terminator, terminator]);
        });
    }
}
