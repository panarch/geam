mod instruction;
mod terminator;

pub(crate) use instruction::{
    BitArrayBitsSize, BitArrayEvaluatedSize, BitArrayInstruction, BitArraySegment, BoolInstruction,
    CustomInstruction, FloatInstruction, FunctionCapture, FunctionInstruction,
    FunctionInstructionKind, FunctionTarget, Instruction, InstructionKind, IntInstruction,
    ListInstruction, NilInstruction, ParameterListInstruction, StringInstruction, TupleInstruction,
    TypedListInstruction, UtfCodepointInstruction,
};
pub(crate) use terminator::{
    BitArrayBindingPattern, BitArrayPattern, BitArrayPatternSegment, BitArrayPatternSize,
    BitArrayPatternSizeExpr, BitArrayPatternValue, BitArrayStringPattern, BoolBranch, Echo, Edge,
    FloatSwitch, IntSwitch, Jump, LetAssertPanic, Match, MatchEdge, MatchEdgeArgument,
    MatchIntBindingId, MatchPattern, MatchPatternBinding, MatchPatternList, MatchPatternListTail,
    NeverCall, NeverCallTarget, Signedness, SourceStop, SourceStopKind, StringSwitch, Terminator,
};

use crate::plan::execution::graph::{BlockGraphExplainContext, ParamSlot};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct BlockId(usize);

pub(crate) struct Block {
    params: Box<[ParamSlot]>,
    instructions: Box<[Instruction]>,
    terminator: Terminator,
}

impl BlockId {
    pub(in crate::plan::execution) fn new(index: usize) -> Self {
        Self(index)
    }

    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl Block {
    pub(in crate::plan::execution) fn new(
        params: Vec<ParamSlot>,
        instructions: Vec<Instruction>,
        terminator: Terminator,
    ) -> Self {
        Self {
            params: params.into_boxed_slice(),
            instructions: instructions.into_boxed_slice(),
            terminator,
        }
    }

    pub(crate) fn params(&self) -> &[ParamSlot] {
        &self.params
    }

    pub(crate) fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }

    pub(crate) fn terminator(&self) -> &Terminator {
        &self.terminator
    }

    pub(in crate::plan::execution::graph) fn write_explanation(
        &self,
        context: &mut BlockGraphExplainContext<'_, '_, '_>,
        index: usize,
    ) {
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
