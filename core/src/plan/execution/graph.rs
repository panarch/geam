mod bit_array;
mod block;
mod exit;
mod value;

pub(crate) use bit_array::{Endianness, FloatBitSize, StringEncoding};
pub(in crate::plan::execution::graph) use bit_array::{endianness, float_size, string_encoding};
pub(crate) use block::{
    BitArrayBindingPattern, BitArrayBitsSize, BitArrayEvaluatedSize, BitArrayInstruction,
    BitArrayPattern, BitArrayPatternSegment, BitArrayPatternSize, BitArrayPatternSizeExpr,
    BitArrayPatternValue, BitArraySegment, BitArrayStringPattern, Block, BlockId, BoolBranch,
    BoolInstruction, CustomInstruction, Echo, Edge, ExternalFunctionCallTarget,
    ExternalFunctionInstruction, ExternalFunctionInstructionKind, ExternalFunctionInstructionView,
    ExternalFunctionTarget, ExternalInstruction, ExternalInstructionRef, ExternalInstructionView,
    ExternalListInstruction, ExternalListInstructionView, FloatInstruction, FloatSwitch,
    FunctionCapture, FunctionInstruction, FunctionInstructionKind, FunctionTarget, Instruction,
    InstructionKind, IntInstruction, IntSwitch, Jump, LetAssertPanic, ListInstruction, Match,
    MatchEdge, MatchEdgeArgument, MatchIntBindingId, MatchPattern, MatchPatternBinding,
    MatchPatternList, MatchPatternListTail, NeverCall, NeverCallTarget, NilInstruction,
    ParameterListInstruction, ProfiledBlock, ProfiledInstruction, ProfiledInstructionKind,
    Signedness, SourceStop, SourceStopKind, StringInstruction, StringSwitch, Terminator,
    TupleInstruction, TypedListInstruction, UtfCodepointInstruction,
};
pub(crate) use exit::BlockGraphExitId;
pub(crate) use value::{
    BitArrayFunctionLocalId, BitArrayListFunctionLocalId, BitArrayListLocalId, BitArrayLocalId,
    BoolFunctionLocalId, BoolListFunctionLocalId, BoolListLocalId, BoolLocalId,
    CoreFunctionFunctionLocal, CoreFunctionFunctionLocalId, CustomFunctionLocal,
    CustomFunctionLocalId, CustomListFunctionLocalId, CustomListLocalId, CustomLocal,
    CustomLocalId, ExternalFunctionFunctionLocal, ExternalFunctionFunctionLocalId,
    ExternalFunctionLocal, ExternalFunctionLocalId, ExternalListFunctionLocalId,
    ExternalListLocalId, ExternalLocal, ExternalLocalId, FloatFunctionLocalId,
    FloatListFunctionLocalId, FloatListLocalId, FloatLocalId, FunctionFunctionLocal,
    FunctionListFunctionLocalId, FunctionListLocalId, FunctionLocal, GenericFunctionLocal,
    GenericFunctionLocalId, IntFunctionLocalId, IntListFunctionLocalId, IntListLocalId, IntLocalId,
    ListFunctionLocal, ListListFunctionLocalId, ListListLocalId, ListLocal, NeverFunctionLocal,
    NeverFunctionLocalId, NilFunctionLocalId, NilListFunctionLocalId, NilListLocalId, NilLocalId,
    ParamLocal, ParamSlot, ParameterListFunctionLocalId, ParameterListListFunctionLocalId,
    ParameterListListLocalId, ParameterListLocalId, StoredListLocal, StringFunctionLocalId,
    StringListFunctionLocalId, StringListLocalId, StringLocalId, TupleFunctionLocalId,
    TupleListFunctionLocalId, TupleListLocalId, TupleLocalId, UtfCodepointFunctionLocalId,
    UtfCodepointListFunctionLocalId, UtfCodepointListLocalId, UtfCodepointLocalId,
};
pub(in crate::plan::execution) use value::{LocalLabel, write_local_labels};

use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::function::{
    ExecutionGraphProfile, FunctionLabelSource, HostedExecutionGraph,
};

pub(crate) struct ProfiledBlockGraph<Graph: ExecutionGraphProfile> {
    entry: BlockId,
    blocks: Box<[ProfiledBlock<Graph>]>,
}

pub(crate) type BlockGraph = ProfiledBlockGraph<HostedExecutionGraph>;

pub(in crate::plan::execution) trait BlockGraphExitExplanation {
    fn write_exit(&self, context: &mut ExplainContext<'_, '_>, exit: BlockGraphExitId);
}

pub(in crate::plan::execution::graph) struct BlockGraphExplainContext<'a, 'plan, 'output> {
    context: &'a mut ExplainContext<'plan, 'output>,
    exits: &'a dyn BlockGraphExitExplanation,
}

impl<Graph: ExecutionGraphProfile> ProfiledBlockGraph<Graph> {
    pub(in crate::plan::execution) fn from_parts(
        entry: BlockId,
        blocks: Vec<ProfiledBlock<Graph>>,
    ) -> Self {
        Self {
            entry,
            blocks: blocks.into_boxed_slice(),
        }
    }

    pub(crate) fn entry(&self) -> BlockId {
        self.entry
    }

    pub(crate) fn blocks(&self) -> &[ProfiledBlock<Graph>] {
        &self.blocks
    }

    pub(crate) fn block(&self, id: BlockId) -> &ProfiledBlock<Graph> {
        &self.blocks[id.index()]
    }

    pub(in crate::plan::execution) fn into_parts(self) -> (BlockId, Box<[ProfiledBlock<Graph>]>) {
        (self.entry, self.blocks)
    }

    pub(in crate::plan::execution) fn write_explanation(
        &self,
        context: &mut ExplainContext<'_, '_>,
        entry_params: &[ParamSlot],
        entry_captures: &[ParamSlot],
        exits: &dyn BlockGraphExitExplanation,
    ) where
        Graph::ExternalFunctionId: FunctionLabelSource,
        Graph::ExternalListFunctionId: FunctionLabelSource,
        Graph::ExternalFunctionFunctionId: FunctionLabelSource,
        Graph::ExternalListFunctionFunctionId: FunctionLabelSource,
        Graph::ExternalInstruction: Explain,
        Graph::ExternalListInstruction: Explain,
        Graph::ExternalFunctionInstruction: Explain,
    {
        context.push_str("  entry b");
        context.push_str(&self.entry().index().to_string());
        context.push_str(" params=");
        context.write_list(entry_params, |context, slot| context.write(slot));
        context.push_str(" captures=");
        context.write_list(entry_captures, |context, slot| context.write(slot));
        context.push('\n');

        let mut graph_context = BlockGraphExplainContext { context, exits };
        for (index, block) in self.blocks().iter().enumerate() {
            block.write_explanation(&mut graph_context, index);
        }
    }
}

impl BlockGraphExplainContext<'_, '_, '_> {
    pub(in crate::plan::execution::graph) fn push(&mut self, character: char) {
        self.context.push(character);
    }

    pub(in crate::plan::execution::graph) fn push_str(&mut self, text: &str) {
        self.context.push_str(text);
    }

    pub(in crate::plan::execution::graph) fn write<Value>(&mut self, value: &Value)
    where
        Value: Explain + ?Sized,
    {
        self.context.write(value);
    }

    pub(in crate::plan::execution::graph) fn write_list<Value>(
        &mut self,
        values: &[Value],
        mut write_value: impl FnMut(&mut Self, &Value),
    ) {
        self.push('[');
        for (index, value) in values.iter().enumerate() {
            if index > 0 {
                self.push_str(", ");
            }
            write_value(self, value);
        }
        self.push(']');
    }

    pub(in crate::plan::execution::graph) fn write_exit(&mut self, exit: BlockGraphExitId) {
        self.exits.write_exit(self.context, exit);
    }
}

#[cfg(test)]
mod explain_tests {
    use crate::plan::execution::explain;
    use crate::plan::execution::function::IntFunctionId;

    #[test]
    fn writes_complete_graph_entry_and_block_order() {
        let source = r#"
fn choose(flag: Bool) { case flag { True -> 1 False -> 0 } }
pub fn main() { choose(True) }
"#;
        let expected = concat!(
            "  entry b0 params=[%bool#0:shape#0(Bool)] captures=[]\n",
            "  block b0 params=[%bool#0:shape#0(Bool)]\n",
            "    branch %bool#0 true=b1() false=b2()\n",
            "  block b1 params=[]\n",
            "    %int#0:shape#1(Int) = int.value 1\n",
            "    return %int#0\n",
            "  block b2 params=[]\n",
            "    %int#0:shape#1(Int) = int.value 0\n",
            "    return %int#0\n",
        );

        assert_explanation(source, expected);
    }

    #[test]
    fn writes_echo_terminator_through_block_graph() {
        let source = r#"
fn emit(value: Int) {
  echo value
}

pub fn main() {
  emit(1)
}
"#;
        let expected = concat!(
            "  entry b0 params=[%int#0:shape#0(Int)] captures=[]\n",
            "  block b0 params=[%int#0:shape#0(Int)]\n",
            "    echo subject=%int#0 message=none site=main::emit@25..35 next=b1(%int#0)\n",
            "  block b1 params=[%int#0:shape#0(Int)]\n",
            "    return %int#0\n",
        );

        assert_explanation(source, expected);
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let function = plan.int_function(IntFunctionId(1));
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
