mod branch;
mod echo;
mod edge;
mod jump;
mod let_assert;
mod match_;
mod never;
mod pattern;
mod source_stop;
mod switch;

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

pub(crate) enum Terminator {
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
