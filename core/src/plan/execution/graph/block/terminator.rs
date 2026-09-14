use crate::plan::execution::prepared::rust::{Emit, Rust};
pub(in crate::plan::execution) mod branch;
pub(in crate::plan::execution) mod echo;
pub(in crate::plan::execution) mod edge;
pub(in crate::plan::execution) mod jump;
pub(in crate::plan::execution) mod let_assert;
pub(in crate::plan::execution) mod match_;
pub(in crate::plan::execution) mod never;
pub(in crate::plan::execution) mod pattern;
pub(in crate::plan::execution) mod source_stop;
pub(in crate::plan::execution) mod switch;

pub(crate) use branch::BoolBranch;
pub(crate) use echo::Echo;
pub(crate) use edge::{Edge, MatchEdge, MatchEdgeArgument};
pub(crate) use jump::Jump;
pub(crate) use let_assert::LetAssertPanic;
pub(crate) use match_::Match;
pub(crate) use never::{NeverCall, NeverCallTarget};
pub(crate) use pattern::{
    BitArrayBindingPattern, BitArrayPattern, BitArrayPatternSegment, BitArrayPatternSize,
    BitArrayPatternSizeExpr, BitArrayPatternValue, BitArrayStringPattern, MatchIntBindingId,
    MatchPattern, MatchPatternBinding, MatchPatternList, MatchPatternListTail, Signedness,
};
pub(crate) use source_stop::{SourceStop, SourceStopKind};
pub(crate) use switch::{FloatSwitch, IntSwitch, StringSwitch};

use crate::plan::execution::graph::{BlockGraphExitId, BlockGraphExplainContext};

#[derive(Clone)]
pub enum Terminator {
    Jump(Jump),
    BoolBranch(BoolBranch),
    IntSwitch(IntSwitch),
    FloatSwitch(FloatSwitch),
    StringSwitch(StringSwitch),
    Match(Match),
    Echo(Echo),
    Exit(BlockGraphExitId),
    SourceStop(SourceStop),
    LetAssertPanic(LetAssertPanic),
    NeverCall(NeverCall),
}

impl Terminator {
    pub(in crate::plan::execution::graph) fn write_explanation(
        &self,
        context: &mut BlockGraphExplainContext<'_, '_, '_>,
    ) {
        match self {
            Self::Jump(jump) => context.write(jump),
            Self::BoolBranch(branch) => context.write(branch),
            Self::IntSwitch(switch) => context.write(switch),
            Self::FloatSwitch(switch) => context.write(switch),
            Self::StringSwitch(switch) => context.write(switch),
            Self::Match(matcher) => context.write(matcher),
            Self::Echo(echo) => context.write(echo),
            Self::Exit(exit) => context.write_exit(*exit),
            Self::SourceStop(stop) => context.write(stop),
            Self::LetAssertPanic(panic) => context.write(panic),
            Self::NeverCall(call) => context.write(call),
        }
    }
}

impl Emit for Terminator {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Jump(field_0) => output.call("graph::Terminator::Jump", &[field_0]),
            Self::BoolBranch(field_0) => output.call("graph::Terminator::BoolBranch", &[field_0]),
            Self::IntSwitch(field_0) => output.call("graph::Terminator::IntSwitch", &[field_0]),
            Self::FloatSwitch(field_0) => output.call("graph::Terminator::FloatSwitch", &[field_0]),
            Self::StringSwitch(field_0) => {
                output.call("graph::Terminator::StringSwitch", &[field_0])
            }
            Self::Match(field_0) => output.call("graph::Terminator::Match", &[field_0]),
            Self::Echo(field_0) => output.call("graph::Terminator::Echo", &[field_0]),
            Self::Exit(field_0) => output.call("graph::Terminator::Exit", &[field_0]),
            Self::SourceStop(field_0) => output.call("graph::Terminator::SourceStop", &[field_0]),
            Self::LetAssertPanic(field_0) => {
                output.call("graph::Terminator::LetAssertPanic", &[field_0])
            }
            Self::NeverCall(field_0) => output.call("graph::Terminator::NeverCall", &[field_0]),
        }
    }
}

#[cfg(test)]
mod emission_tests {
    use super::{
        BlockGraphExitId, BoolBranch, Echo, Edge, FloatSwitch, IntSwitch, Jump, LetAssertPanic,
        Match, MatchEdge, MatchPattern, NeverCall, NeverCallTarget, Rust, SourceStop,
        SourceStopKind, StringSwitch, Terminator,
    };
    use crate::plan::execution::function::NeverFunctionId;
    use crate::plan::execution::graph::{
        BlockId, BoolLocalId, FloatLocalId, IntLocalId, ParamLocal, StringLocalId,
    };
    use crate::plan::execution::storage::Table;
    use crate::plan::{EchoSite, HostCallSite, PanicSite, SourceSpan};

    #[test]
    fn emits_every_terminator_with_its_own_control_and_source_data() {
        let edge = Edge::new(BlockId(2), Vec::new());
        let span = SourceSpan::new(3, 12);
        let panic = PanicSite::from_static("example", "main", span);
        let cases = [
            (
                Terminator::Jump(Jump::new(edge.clone())),
                r#"
data::graph::Terminator::Jump(data::graph::Jump {
    edge: data::graph::Edge {
        target: data::graph::BlockId(2),
        args: data::Storage::Static(&[]),
    },
})"#.trim_start_matches('\n'),
            ),
            (
                Terminator::BoolBranch(BoolBranch::new(BoolLocalId(0), edge.clone(), edge.clone())),
                r#"
data::graph::Terminator::BoolBranch(data::graph::BoolBranch {
    subject: data::graph::BoolLocalId(0),
    true_: data::graph::Edge {
        target: data::graph::BlockId(2),
        args: data::Storage::Static(&[]),
    },
    false_: data::graph::Edge {
        target: data::graph::BlockId(2),
        args: data::Storage::Static(&[]),
    },
})"#.trim_start_matches('\n'),
            ),
            (
                Terminator::IntSwitch(IntSwitch::new(
                    IntLocalId(0),
                    Table::Static(&[]),
                    edge.clone(),
                )),
                r#"
data::graph::Terminator::IntSwitch(data::graph::IntSwitch {
    subject: data::graph::IntLocalId(0),
    clauses: data::Storage::Static(&[]),
    fallback: data::graph::Edge {
        target: data::graph::BlockId(2),
        args: data::Storage::Static(&[]),
    },
})"#.trim_start_matches('\n'),
            ),
            (
                Terminator::FloatSwitch(FloatSwitch::new(
                    FloatLocalId(0),
                    Table::Static(&[]),
                    edge.clone(),
                )),
                r#"
data::graph::Terminator::FloatSwitch(data::graph::FloatSwitch {
    subject: data::graph::FloatLocalId(0),
    clauses: data::Storage::Static(&[]),
    fallback: data::graph::Edge {
        target: data::graph::BlockId(2),
        args: data::Storage::Static(&[]),
    },
})"#.trim_start_matches('\n'),
            ),
            (
                Terminator::StringSwitch(StringSwitch::new(
                    StringLocalId(0),
                    Table::Static(&[]),
                    edge.clone(),
                )),
                r#"
data::graph::Terminator::StringSwitch(data::graph::StringSwitch {
    subject: data::graph::StringLocalId(0),
    clauses: data::Storage::Static(&[]),
    fallback: data::graph::Edge {
        target: data::graph::BlockId(2),
        args: data::Storage::Static(&[]),
    },
})"#.trim_start_matches('\n'),
            ),
            (
                Terminator::Match(Match::new(
                    ParamLocal::Int(IntLocalId(0)),
                    MatchPattern::Discard,
                    MatchEdge::new(BlockId(1), Vec::new()),
                    edge.clone(),
                )),
                r#"
data::graph::Terminator::Match(data::graph::Match {
    subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
    pattern: data::graph::MatchPattern::Discard,
    success: data::graph::MatchEdge {
        target: data::graph::BlockId(1),
        args: data::Storage::Static(&[]),
    },
    failure: data::graph::Edge {
        target: data::graph::BlockId(2),
        args: data::Storage::Static(&[]),
    },
})"#.trim_start_matches('\n'),
            ),
            (
                Terminator::Echo(Echo::new(
                    ParamLocal::Int(IntLocalId(0)),
                    None,
                    EchoSite::from_static("example", "main", span),
                    edge,
                )),
                r#"
data::graph::Terminator::Echo(data::graph::Echo {
    subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
    message: None,
    site: data::source::EchoSite::from_static("example", "main", data::source::SourceSpan::new(3, 12)),
    next: data::graph::Edge {
        target: data::graph::BlockId(2),
        args: data::Storage::Static(&[]),
    },
})"#.trim_start_matches('\n'),
            ),
            (
                Terminator::Exit(BlockGraphExitId(3)),
                "data::graph::Terminator::Exit(data::graph::BlockGraphExitId(3))",
            ),
            (
                Terminator::SourceStop(SourceStop::new(SourceStopKind::Panic, None, panic.clone())),
                r#"
data::graph::Terminator::SourceStop(data::graph::SourceStop {
    kind: data::graph::SourceStopKind::Panic,
    message: None,
    site: data::source::PanicSite::from_static("example", "main", data::source::SourceSpan::new(3, 12)),
})"#.trim_start_matches('\n'),
            ),
            (
                Terminator::LetAssertPanic(LetAssertPanic::new(
                    ParamLocal::Int(IntLocalId(0)),
                    None,
                    panic,
                    SourceSpan::new(5, 8),
                )),
                r#"
data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
    subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
    message: None,
    site: data::source::PanicSite::from_static("example", "main", data::source::SourceSpan::new(3, 12)),
    pattern_span: data::source::SourceSpan::new(5, 8),
})"#.trim_start_matches('\n'),
            ),
            (
                Terminator::NeverCall(NeverCall::new(
                    NeverCallTarget::Direct(NeverFunctionId(0)),
                    Table::Static(&[]),
                    HostCallSite::from_static("example", "main", span),
                )),
                r#"
data::graph::Terminator::NeverCall(data::graph::NeverCall {
    function: data::graph::NeverCallTarget::Direct(data::function::NeverFunctionId(0)),
    args: data::Storage::Static(&[]),
    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3, 12)),
})"#.trim_start_matches('\n'),
            ),
        ];
        for (terminator, expected) in cases {
            assert_eq!(Rust::expression(&terminator), expected);
        }
    }
}
