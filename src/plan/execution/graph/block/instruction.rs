mod bit_array;
mod bool;
mod custom;
mod external;
mod float;
mod function;
mod int;
mod list;
mod nil;
mod string;
mod tuple;
mod utf_codepoint;

pub(crate) use bit_array::{
    BitArrayBitsSize, BitArrayEvaluatedSize, BitArrayInstruction, BitArraySegment,
};
pub(crate) use bool::BoolInstruction;
pub(crate) use custom::CustomInstruction;
pub(crate) use external::{ExternalInstruction, ExternalInstructionRef, ExternalInstructionView};
pub(crate) use float::FloatInstruction;
pub(crate) use function::{
    ExternalFunctionCallTarget, ExternalFunctionInstruction, ExternalFunctionInstructionKind,
    ExternalFunctionInstructionView, ExternalFunctionTarget, FunctionCapture, FunctionInstruction,
    FunctionInstructionKind, FunctionTarget,
};
pub(crate) use int::IntInstruction;
pub(crate) use list::{
    ExternalListInstruction, ExternalListInstructionView, ListInstruction,
    ParameterListInstruction, TypedListInstruction,
};
pub(crate) use nil::NilInstruction;
pub(crate) use string::StringInstruction;
pub(crate) use tuple::TupleInstruction;
pub(crate) use utf_codepoint::UtfCodepointInstruction;

use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::function::FunctionLabelSource;
use crate::plan::execution::function::{ExecutionGraphProfile, HostedExecutionGraph};
use crate::plan::execution::graph::ParamSlot;
use crate::plan::execution::graph::{LocalLabel, ParamLocal, write_local_labels};

pub(crate) struct ProfiledInstruction<Graph: ExecutionGraphProfile> {
    output: ParamSlot,
    kind: ProfiledInstructionKind<Graph>,
}

pub(crate) enum ProfiledInstructionKind<Graph: ExecutionGraphProfile> {
    Int(IntInstruction),
    Float(FloatInstruction),
    String(StringInstruction),
    BitArray(BitArrayInstruction),
    UtfCodepoint(UtfCodepointInstruction),
    Custom(CustomInstruction),
    External(Graph::ExternalInstruction),
    ExternalList(Graph::ExternalListInstruction),
    ExternalFunction(Graph::ExternalFunctionInstruction),
    Bool(BoolInstruction),
    Nil(NilInstruction),
    Tuple(TupleInstruction),
    List(ListInstruction),
    Function(FunctionInstruction),
}

pub(crate) type Instruction = ProfiledInstruction<HostedExecutionGraph>;
pub(crate) type InstructionKind = ProfiledInstructionKind<HostedExecutionGraph>;

impl<Graph: ExecutionGraphProfile> ProfiledInstruction<Graph> {
    pub(in crate::plan::execution) fn new(
        output: ParamSlot,
        kind: ProfiledInstructionKind<Graph>,
    ) -> Self {
        Self { output, kind }
    }

    pub(crate) fn output(&self) -> &ParamSlot {
        &self.output
    }

    pub(crate) fn kind(&self) -> &ProfiledInstructionKind<Graph> {
        &self.kind
    }

    pub(in crate::plan::execution) fn into_parts(
        self,
    ) -> (ParamSlot, ProfiledInstructionKind<Graph>) {
        (self.output, self.kind)
    }
}

impl<Graph: ExecutionGraphProfile> Explain for ProfiledInstruction<Graph>
where
    Graph::ExternalInstruction: Explain,
    Graph::ExternalListInstruction: Explain,
    Graph::ExternalFunctionInstruction: Explain,
{
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        context.push_str("    ");
        context.write(self.output());
        context.push_str(" = ");
        context.write(self.kind());
        context.push('\n');
    }
}

impl<Graph: ExecutionGraphProfile> Explain for ProfiledInstructionKind<Graph>
where
    Graph::ExternalInstruction: Explain,
    Graph::ExternalListInstruction: Explain,
    Graph::ExternalFunctionInstruction: Explain,
{
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        match self {
            Self::Int(instruction) => context.write(instruction),
            Self::Float(instruction) => context.write(instruction),
            Self::String(instruction) => context.write(instruction),
            Self::BitArray(instruction) => context.write(instruction),
            Self::UtfCodepoint(instruction) => context.write(instruction),
            Self::Custom(instruction) => context.write(instruction),
            Self::External(instruction) => context.write(instruction),
            Self::ExternalList(instruction) => context.write(instruction),
            Self::ExternalFunction(instruction) => context.write(instruction),
            Self::Bool(instruction) => context.write(instruction),
            Self::Nil(instruction) => context.write(instruction),
            Self::Tuple(instruction) => context.write(instruction),
            Self::List(instruction) => context.write(instruction),
            Self::Function(instruction) => context.write(instruction),
        }
    }
}

#[cfg(test)]
mod instruction_explain_tests {
    use crate::plan::execution::explain;
    use crate::plan::execution::function::IntFunctionId;

    #[test]
    fn writes_instruction_output_and_payload() {
        let source = "pub fn main() { 1 }";
        let expected = "    %int#0:shape#0(Int) = int.value 1\n";

        assert_explanation(source, expected);
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let instruction = &plan
                .int_function(IntFunctionId(0))
                .body()
                .block_graph()
                .blocks()[0]
                .instructions()[0];
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(instruction);
        });
    }
}

#[cfg(test)]
mod instruction_kind_explain_tests {
    use super::{ExternalInstruction, InstructionKind};
    use crate::plan::execution::explain;
    use crate::plan::execution::function::{ExternalFunctionId, TupleFunctionId};
    use crate::plan::execution::type_::ExternalTypeId;

    #[test]
    fn dispatches_every_typed_instruction_family() {
        let cases = [
            ("pub fn main() { #(1) }", "int.value 1"),
            ("pub fn main() { #(1.0) }", "float.value 1.0"),
            ("pub fn main() { #(\"one\") }", "string.value \"one\""),
            (
                "pub fn main() { #(<<1>>) }",
                "bit_array.value [int(%int#0, bits=8, big)]",
            ),
            (
                r#"
fn scalar() -> UtfCodepoint { panic }
pub fn main() { #(scalar()) }
"#,
                "utf_codepoint.call utf_codepoint#0 args=[]",
            ),
            (
                r#"
pub type Boxed { Boxed }
pub fn main() { #(Boxed) }
"#,
                "custom.construct custom_type#0.constructor#0 fields=[]",
            ),
            ("pub fn main() { #(True) }", "bool.value True"),
            ("pub fn main() { #(Nil) }", "nil.value"),
            ("pub fn main() { #(#(1)) }", "tuple.value elements=[%int#0]"),
            (
                "pub fn main() { let values: List(Int) = [] #(values) }",
                "list.int[type#0] value elements=[]",
            ),
            (
                "pub fn main() { #(fn() { 1 }) }",
                "function[Int] closure target=int#0 captures=[]",
            ),
        ];

        for (source, expected) in cases {
            assert_explanation(source, expected);
        }
    }

    #[test]
    fn dispatches_external_instructions() {
        let source = "pub fn main() { 1 }";
        let expected = "external.call external#13 args=[]";

        explain::assert_rendered(source, expected, |plan, output| {
            let instruction = InstructionKind::External(ExternalInstruction::Call {
                function: ExternalFunctionId::new(13, ExternalTypeId::new(0)),
                args: Box::new([]),
                site: crate::plan::HostCallSite::unknown(),
            });
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(&instruction);
        });
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let instructions = plan
                .tuple_function(TupleFunctionId(0))
                .body()
                .block_graph()
                .blocks()[0]
                .instructions();
            let instruction = &instructions[instructions.len() - 2];
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(instruction.kind());
        });
    }
}

pub(super) fn write_binary<Value: LocalLabel>(
    output: &mut String,
    opcode: &str,
    left: &Value,
    right: &Value,
) {
    output.push_str(opcode);
    output.push(' ');
    left.write_local_label(output);
    output.push(' ');
    right.write_local_label(output);
}

pub(super) fn write_call<Function: FunctionLabelSource>(
    output: &mut String,
    opcode: &str,
    function: &Function,
    args: &[ParamLocal],
) {
    output.push_str(opcode);
    output.push(' ');
    function.function_label().write(output);
    write_args(output, args);
}

pub(super) fn write_function_call<Function: LocalLabel>(
    output: &mut String,
    opcode: &str,
    function: &Function,
    args: &[ParamLocal],
) {
    output.push_str(opcode);
    output.push(' ');
    function.write_local_label(output);
    write_args(output, args);
}

pub(super) fn write_args(output: &mut String, args: &[ParamLocal]) {
    output.push_str(" args=");
    write_local_labels(output, args);
}

pub(super) fn write_constant<Value>(
    output: &mut String,
    family: &str,
    id: crate::plan::execution::constant::ConstantId<Value>,
) {
    output.push_str("constant.");
    output.push_str(family);
    output.push('#');
    output.push_str(&id.index().to_string());
}

pub(super) fn write_length<Value: LocalLabel>(
    output: &mut String,
    opcode: &str,
    value: &Value,
    length: usize,
) {
    output.push_str(opcode);
    output.push(' ');
    value.write_local_label(output);
    output.push_str(" length=");
    output.push_str(&length.to_string());
}

pub(super) fn write_literal(output: &mut String, opcode: &str, value: &str) {
    output.push_str(opcode);
    output.push(' ');
    output.push_str(value);
}

pub(super) fn write_projection<Source: LocalLabel>(
    output: &mut String,
    opcode: &str,
    source: &Source,
    index: usize,
) {
    output.push_str(opcode);
    output.push(' ');
    source.write_local_label(output);
    output.push_str(" index=");
    output.push_str(&index.to_string());
}

pub(super) fn write_unary<Value: LocalLabel>(output: &mut String, opcode: &str, value: &Value) {
    output.push_str(opcode);
    output.push(' ');
    value.write_local_label(output);
}
