use crate::plan::execution::prepared::rust::{Emit, Rust};
pub(in crate::plan::execution) mod bit_array;
pub(in crate::plan::execution) mod block;
pub(in crate::plan::execution) mod exit;
pub(in crate::plan::execution) mod integer;
pub(in crate::plan::execution) mod transfer;
pub(in crate::plan::execution) mod value;

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
pub(crate) use integer::IntegerLiteral;
pub(in crate::plan::execution) use transfer::StorageSlot;
pub(crate) use transfer::{FamilyTransfer, StorageFamily, Transfer, TransferStep};
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
use crate::plan::execution::storage::Table;
pub(in crate::plan::execution) use block::BlockHeader;
pub(crate) use block::BlockView;

pub struct ProfiledBlockGraph<Graph: ExecutionGraphProfile> {
    pub entry: BlockId,
    pub blocks: Table<BlockHeader>,
    pub params: Table<ParamSlot>,
    pub instructions: Table<ProfiledInstruction<Graph>>,
}

pub(crate) struct BlockGraphView<'graph, Graph: ExecutionGraphProfile> {
    entry: BlockId,
    blocks: &'graph [BlockHeader],
    params: &'graph [ParamSlot],
    instructions: &'graph [ProfiledInstruction<Graph>],
}

pub(in crate::plan::execution) type BlockGraphParts<Graph> = (
    BlockId,
    Table<BlockHeader>,
    Table<ParamSlot>,
    Table<ProfiledInstruction<Graph>>,
);

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
        let mut headers = Vec::with_capacity(blocks.len());
        let mut all_params = Vec::new();
        let mut all_instructions = Vec::new();
        for block in blocks {
            let (params, instructions, terminator) = block.into_parts();
            let param_start = all_params.len();
            let instruction_start = all_instructions.len();
            all_params.extend(params);
            all_instructions.extend(instructions);
            headers.push(BlockHeader {
                params: param_start..all_params.len(),
                instructions: instruction_start..all_instructions.len(),
                terminator,
            });
        }
        Self {
            entry,
            blocks: headers.into(),
            params: all_params.into(),
            instructions: all_instructions.into(),
        }
    }

    pub(in crate::plan::execution) fn from_tables(
        entry: BlockId,
        blocks: Table<BlockHeader>,
        params: Table<ParamSlot>,
        instructions: Table<ProfiledInstruction<Graph>>,
    ) -> Self {
        Self {
            entry,
            blocks,
            params,
            instructions,
        }
    }

    pub(crate) fn entry(&self) -> BlockId {
        self.as_view().entry()
    }

    pub(crate) fn blocks(&self) -> impl ExactSizeIterator<Item = BlockView<'_, Graph>> {
        self.as_view().blocks()
    }

    pub(crate) fn block(&self, id: BlockId) -> BlockView<'_, Graph> {
        self.as_view().block(id)
    }

    pub(crate) fn as_view(&self) -> BlockGraphView<'_, Graph> {
        BlockGraphView {
            entry: self.entry,
            blocks: &self.blocks,
            params: &self.params,
            instructions: &self.instructions,
        }
    }

    pub(in crate::plan::execution) fn into_parts(self) -> BlockGraphParts<Graph> {
        (self.entry, self.blocks, self.params, self.instructions)
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
        for (index, block) in self.blocks().enumerate() {
            block.write_explanation(&mut graph_context, index);
        }
    }
}

impl<Graph: ExecutionGraphProfile> Copy for BlockGraphView<'_, Graph> {}

impl<Graph: ExecutionGraphProfile> Clone for BlockGraphView<'_, Graph> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'graph, Graph: ExecutionGraphProfile> BlockGraphView<'graph, Graph> {
    pub(crate) fn entry(self) -> BlockId {
        self.entry
    }

    pub(crate) fn blocks(self) -> impl ExactSizeIterator<Item = BlockView<'graph, Graph>> {
        self.blocks.iter().map(move |block| self.view_block(block))
    }

    pub(crate) fn block(self, id: BlockId) -> BlockView<'graph, Graph> {
        self.view_block(&self.blocks[id.index()])
    }

    fn view_block(self, block: &'graph BlockHeader) -> BlockView<'graph, Graph> {
        BlockView {
            params: &self.params[block.params.clone()],
            instructions: &self.instructions[block.instructions.clone()],
            terminator: &block.terminator,
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

impl<Graph: ExecutionGraphProfile> Emit for ProfiledBlockGraph<Graph>
where
    Table<ProfiledInstruction<Graph>>: Emit,
{
    fn emit(&self, output: &mut Rust) {
        let Self {
            entry,
            blocks,
            params,
            instructions,
        } = self;
        output.structure(
            "graph::ProfiledBlockGraph",
            &[
                ("entry", entry),
                ("blocks", blocks),
                ("params", params),
                ("instructions", instructions),
            ],
        );
    }
}

#[cfg(test)]
mod explain_tests {
    use crate::plan::execution::explain;
    use crate::plan::execution::function::IntFunctionId;

    #[test]
    fn borrowed_graph_views_preserve_owned_block_and_instruction_addresses() {
        let source = "pub fn main() { 20 + 22 }";
        let module = crate::compile_typed_module("main", "main.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(module).unwrap());
        let graph = plan.int_function(IntFunctionId(0)).body().block_graph();
        let view = graph.as_view();
        let copied = Clone::clone(&view);

        assert_eq!(view.entry(), graph.entry());
        assert_eq!(view.blocks().len(), 1);
        assert!(std::ptr::eq(view.blocks.as_ptr(), graph.blocks.as_ptr()));
        assert!(std::ptr::eq(
            Clone::clone(&copied.block(view.entry())).terminator(),
            graph.block(graph.entry()).terminator()
        ));
        assert!(std::ptr::eq(
            copied.block(view.entry()).instructions().as_ptr(),
            graph.block(graph.entry()).instructions().as_ptr(),
        ));
        assert_eq!(
            crate::run_main(&plan, &mut Vec::new()).unwrap(),
            crate::Value::Int(42.into())
        );
    }

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
