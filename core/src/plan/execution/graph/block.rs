use crate::plan::execution::prepared::rust::{Emit, Rust};
pub(in crate::plan::execution) mod instruction;
pub(in crate::plan::execution) mod terminator;

pub(crate) use instruction::{
    BitArrayBitsSize, BitArrayEvaluatedSize, BitArrayInstruction, BitArraySegment, BoolInstruction,
    CustomInstruction, ExternalFunctionCallTarget, ExternalFunctionInstruction,
    ExternalFunctionInstructionKind, ExternalFunctionInstructionView, ExternalFunctionTarget,
    ExternalInstruction, ExternalInstructionRef, ExternalInstructionView, ExternalListInstruction,
    ExternalListInstructionView, FloatInstruction, FunctionCapture, FunctionInstruction,
    FunctionInstructionKind, FunctionTarget, Instruction, InstructionKind, IntInstruction,
    ListInstruction, NilInstruction, ParameterListInstruction, ProfiledInstruction,
    ProfiledInstructionKind, StringInstruction, TupleInstruction, TypedListInstruction,
    UtfCodepointInstruction,
};
pub(crate) use terminator::{
    BitArrayBindingPattern, BitArrayPattern, BitArrayPatternSegment, BitArrayPatternSize,
    BitArrayPatternSizeExpr, BitArrayPatternValue, BitArrayStringPattern, BoolBranch, Echo, Edge,
    FloatSwitch, IntSwitch, Jump, LetAssertPanic, Match, MatchEdge, MatchEdgeArgument,
    MatchIntBindingId, MatchPattern, MatchPatternBinding, MatchPatternList, MatchPatternListTail,
    NeverCall, NeverCallTarget, Signedness, SourceStop, SourceStopKind, StringSwitch, Terminator,
};

use crate::plan::execution::explain::Explain;
use crate::plan::execution::function::{
    ExecutionGraphProfile, FunctionLabelSource, HostedExecutionGraph,
};
use crate::plan::execution::graph::{BlockGraphExplainContext, ParamSlot};
use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockId(pub usize);

pub(crate) struct ProfiledBlock<Graph: ExecutionGraphProfile> {
    params: Box<[ParamSlot]>,
    instructions: Box<[ProfiledInstruction<Graph>]>,
    terminator: Terminator,
}

pub struct BlockHeader {
    pub params: Range<usize>,
    pub instructions: Range<usize>,
    pub terminator: Terminator,
}

pub(crate) struct BlockView<'graph, Graph: ExecutionGraphProfile> {
    pub(super) params: &'graph [ParamSlot],
    pub(super) instructions: &'graph [ProfiledInstruction<Graph>],
    pub(super) terminator: &'graph Terminator,
}

pub(crate) type Block = ProfiledBlock<HostedExecutionGraph>;
pub(in crate::plan::execution) type ProfiledBlockParts<Graph> = (
    Box<[ParamSlot]>,
    Box<[ProfiledInstruction<Graph>]>,
    Terminator,
);

impl BlockId {
    pub(in crate::plan::execution) fn new(index: usize) -> Self {
        Self(index)
    }

    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl<Graph: ExecutionGraphProfile> ProfiledBlock<Graph> {
    pub(in crate::plan::execution) fn new(
        params: Vec<ParamSlot>,
        instructions: Vec<ProfiledInstruction<Graph>>,
        terminator: Terminator,
    ) -> Self {
        Self {
            params: params.into_boxed_slice(),
            instructions: instructions.into_boxed_slice(),
            terminator,
        }
    }

    pub(in crate::plan::execution) fn into_parts(self) -> ProfiledBlockParts<Graph> {
        (self.params, self.instructions, self.terminator)
    }
}

impl<Graph: ExecutionGraphProfile> Copy for BlockView<'_, Graph> {}

impl<Graph: ExecutionGraphProfile> Clone for BlockView<'_, Graph> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'graph, Graph: ExecutionGraphProfile> BlockView<'graph, Graph> {
    pub(crate) fn params(self) -> &'graph [ParamSlot] {
        self.params
    }

    pub(crate) fn instructions(self) -> &'graph [ProfiledInstruction<Graph>] {
        self.instructions
    }

    pub(crate) fn terminator(self) -> &'graph Terminator {
        self.terminator
    }

    pub(in crate::plan::execution::graph) fn write_explanation(
        self,
        context: &mut BlockGraphExplainContext<'_, '_, '_>,
        index: usize,
    ) where
        Graph::ExternalFunctionId: FunctionLabelSource,
        Graph::ExternalListFunctionId: FunctionLabelSource,
        Graph::ExternalFunctionFunctionId: FunctionLabelSource,
        Graph::ExternalListFunctionFunctionId: FunctionLabelSource,
        Graph::ExternalInstruction: Explain,
        Graph::ExternalListInstruction: Explain,
        Graph::ExternalFunctionInstruction: Explain,
    {
        context.push_str("  block b");
        context.push_str(&index.to_string());
        context.push_str(" params=");
        context.write_list(self.params(), |context, slot| context.write(slot));
        context.push('\n');
        for instruction in self.instructions() {
            context.write(instruction);
        }
        context.push_str("    ");
        self.terminator().write_explanation(context);
        context.push('\n');
    }
}

impl Emit for BlockId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::BlockId", &[field_0]);
    }
}

impl Emit for BlockHeader {
    fn emit(&self, output: &mut Rust) {
        let Self {
            params,
            instructions,
            terminator,
        } = self;
        output.structure(
            "graph::BlockHeader",
            &[
                ("params", params),
                ("instructions", instructions),
                ("terminator", terminator),
            ],
        );
    }
}

#[cfg(test)]
mod explain_tests {
    use crate::plan::execution::explain;
    use crate::plan::execution::function::IntFunctionId;

    #[test]
    fn writes_block_parameters_instructions_and_terminator() {
        let source = "pub fn main() { 1 }";
        let expected = concat!(
            "  entry b0 params=[] captures=[]\n",
            "  block b0 params=[]\n",
            "    %int#0:shape#0(Int) = int.value 1\n",
            "    return %int#0\n",
        );

        assert_explanation(source, expected);
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let function = plan.int_function(IntFunctionId(0));
            let body = function.body();
            let mut context = explain::ExplainContext::new(plan, output);
            body.write_explanation(
                &mut context,
                "int",
                function.entry().params(body),
                function.entry().captures(body),
            );
        });
    }
}
