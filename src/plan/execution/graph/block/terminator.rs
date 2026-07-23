mod branch;
mod edge;
mod jump;
mod let_assert;
mod match_;
mod never;
mod pattern;
mod source_stop;
mod switch;

pub(crate) use branch::BoolBranch;
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

use crate::plan::execution::graph::GraphExitId;

pub(crate) enum Terminator {
    Jump(Jump),
    BoolBranch(BoolBranch),
    IntSwitch(IntSwitch),
    FloatSwitch(FloatSwitch),
    StringSwitch(StringSwitch),
    Match(Match),
    Exit(GraphExitId),
    SourceStop(SourceStop),
    LetAssertPanic(LetAssertPanic),
    NeverCall(NeverCall),
}

use crate::plan::execution::explain::ExplainContext;

pub(super) fn write_terminator(
    context: &mut ExplainContext<'_, '_>,
    terminator: &Terminator,
    write_graph_exit: &mut dyn FnMut(&mut ExplainContext<'_, '_>, GraphExitId),
) {
    match terminator {
        Terminator::Jump(jump) => context.write(jump),
        Terminator::BoolBranch(branch) => context.write(branch),
        Terminator::IntSwitch(switch) => context.write(switch),
        Terminator::FloatSwitch(switch) => context.write(switch),
        Terminator::StringSwitch(switch) => context.write(switch),
        Terminator::Match(matcher) => context.write(matcher),
        Terminator::Exit(exit) => write_graph_exit(context, *exit),
        Terminator::SourceStop(stop) => context.write(stop),
        Terminator::LetAssertPanic(panic) => context.write(panic),
        Terminator::NeverCall(call) => context.write(call),
    }
}
