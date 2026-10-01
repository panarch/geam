use super::format::{write_binary, write_integer_binary, write_unary};
use crate::plan::execution::prepared::rust::{Emit, Rust};
pub(in crate::plan::execution) mod bit_array;
pub(in crate::plan::execution) mod bool;
pub(in crate::plan::execution) mod custom;
pub(in crate::plan::execution) mod external;
pub(in crate::plan::execution) mod float;
pub(in crate::plan::execution) mod function;
pub(in crate::plan::execution) mod int;
pub(in crate::plan::execution) mod list;
pub(in crate::plan::execution) mod nil;
pub(in crate::plan::execution) mod string;
pub(in crate::plan::execution) mod tuple;
pub(in crate::plan::execution) mod utf_codepoint;

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
use crate::plan::execution::graph::{
    ArithmeticRegion, LocalLabel, ParamLocal, ParamSlot, write_local_labels,
};

#[expect(
    clippy::large_enum_variant,
    reason = "scalar instructions remain inline without a separate allocation and indirection for each instruction"
)]
#[derive(Clone)]
pub enum ProfiledInstruction<Graph: ExecutionGraphProfile> {
    Value(ProfiledValueInstruction<Graph>),
    IntegerRegion(ArithmeticRegion),
}

#[derive(Clone)]
pub struct ProfiledValueInstruction<Graph: ExecutionGraphProfile> {
    pub output: ParamSlot,
    pub kind: ProfiledInstructionKind<Graph>,
}

#[derive(Clone)]
pub enum ProfiledInstructionKind<Graph: ExecutionGraphProfile> {
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
        Self::Value(ProfiledValueInstruction { output, kind })
    }

    pub(crate) fn value(&self) -> Option<&ProfiledValueInstruction<Graph>> {
        match self {
            Self::Value(value) => Some(value),
            Self::IntegerRegion(_) => None,
        }
    }

    pub(crate) fn outputs(&self) -> impl Iterator<Item = &ParamSlot> {
        let (single, region) = match self {
            Self::Value(value) => (Some(&value.output), &[][..]),
            Self::IntegerRegion(region) => (None, region.outputs.as_ref()),
        };
        single
            .into_iter()
            .chain(region.iter().map(|output| &output.slot))
    }
}

impl<Graph: ExecutionGraphProfile> ProfiledValueInstruction<Graph> {
    pub(crate) fn output(&self) -> &ParamSlot {
        &self.output
    }
    pub(crate) fn kind(&self) -> &ProfiledInstructionKind<Graph> {
        &self.kind
    }
}

impl<Graph: ExecutionGraphProfile> Explain for ProfiledInstruction<Graph>
where
    Graph::ExternalInstruction: Explain,
    Graph::ExternalListInstruction: Explain,
    Graph::ExternalFunctionInstruction: Explain,
{
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        match self {
            Self::Value(value) => {
                context.push_str("    ");
                context.write(value.output());
                context.push_str(" = ");
                context.write(value.kind());
                context.push('\n');
            }
            Self::IntegerRegion(region) => context.write(region),
        }
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

impl<Graph: ExecutionGraphProfile> Emit for ProfiledInstruction<Graph>
where
    ProfiledInstructionKind<Graph>: Emit,
{
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Value(value) => output.call("graph::ProfiledInstruction::Value", &[value]),
            Self::IntegerRegion(value) => {
                output.call("graph::ProfiledInstruction::IntegerRegion", &[value])
            }
        }
    }
}

impl<Graph: ExecutionGraphProfile> Emit for ProfiledValueInstruction<Graph>
where
    ProfiledInstructionKind<Graph>: Emit,
{
    fn emit(&self, output: &mut Rust) {
        output.structure(
            "graph::ProfiledValueInstruction",
            &[("output", &self.output), ("kind", &self.kind)],
        );
    }
}

impl<Graph: ExecutionGraphProfile> Emit for ProfiledInstructionKind<Graph>
where
    Graph::ExternalInstruction: Emit,
    Graph::ExternalListInstruction: Emit,
    Graph::ExternalFunctionInstruction: Emit,
{
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Int(field_0) => output.call("graph::ProfiledInstructionKind::Int", &[field_0]),
            Self::Float(field_0) => {
                output.call("graph::ProfiledInstructionKind::Float", &[field_0])
            }
            Self::String(field_0) => {
                output.call("graph::ProfiledInstructionKind::String", &[field_0])
            }
            Self::BitArray(field_0) => {
                output.call("graph::ProfiledInstructionKind::BitArray", &[field_0])
            }
            Self::UtfCodepoint(field_0) => {
                output.call("graph::ProfiledInstructionKind::UtfCodepoint", &[field_0])
            }
            Self::Custom(field_0) => {
                output.call("graph::ProfiledInstructionKind::Custom", &[field_0])
            }
            Self::External(field_0) => {
                output.call("graph::ProfiledInstructionKind::External", &[field_0])
            }
            Self::ExternalList(field_0) => {
                output.call("graph::ProfiledInstructionKind::ExternalList", &[field_0])
            }
            Self::ExternalFunction(field_0) => output.call(
                "graph::ProfiledInstructionKind::ExternalFunction",
                &[field_0],
            ),
            Self::Bool(field_0) => output.call("graph::ProfiledInstructionKind::Bool", &[field_0]),
            Self::Nil(field_0) => output.call("graph::ProfiledInstructionKind::Nil", &[field_0]),
            Self::Tuple(field_0) => {
                output.call("graph::ProfiledInstructionKind::Tuple", &[field_0])
            }
            Self::List(field_0) => output.call("graph::ProfiledInstructionKind::List", &[field_0]),
            Self::Function(field_0) => {
                output.call("graph::ProfiledInstructionKind::Function", &[field_0])
            }
        }
    }
}

#[cfg(test)]
mod emission_tests {
    use super::ProfiledValueInstruction;
    use super::{
        BitArrayInstruction, BoolInstruction, CustomInstruction, ExternalFunctionInstruction,
        ExternalFunctionInstructionKind, ExternalFunctionTarget, ExternalInstruction,
        ExternalListInstruction, FloatInstruction, FunctionInstruction, FunctionInstructionKind,
        FunctionTarget, HostedExecutionGraph, IntInstruction, ListInstruction, NilInstruction,
        ProfiledInstruction, ProfiledInstructionKind, Rust, StringInstruction, TupleInstruction,
        TypedListInstruction, UtfCodepointInstruction,
    };
    use crate::plan::execution::function::{
        ExternalFunctionId, FunctionReturnFamily, IntFunctionId,
    };
    use crate::plan::execution::graph::{BoolLocalId, ParamLocal, ParamSlot, TupleLocalId};
    use crate::plan::execution::storage::Table;
    use crate::plan::execution::type_::{
        ExternalListTypeId, ExternalTypeId, FunctionType, IntListTypeId, ListTypeId, ValueShapeId,
        ValueType,
    };

    #[test]
    fn arithmetic_instruction_emits_its_complete_region() {
        use crate::plan::execution::graph::{
            ArithmeticNode, ArithmeticOperand, ArithmeticOutput, ArithmeticRegion, IntLocalId,
        };
        let instruction =
            ProfiledInstruction::<HostedExecutionGraph>::IntegerRegion(ArithmeticRegion {
                inputs: vec![].into(),
                nodes: vec![
                    ArithmeticNode::Negate(ArithmeticOperand::Immediate(0)),
                    ArithmeticNode::Negate(ArithmeticOperand::Value(0)),
                ]
                .into(),
                outputs: vec![ArithmeticOutput {
                    value: 1,
                    slot: ParamSlot::new(ParamLocal::Int(IntLocalId(0)), ValueShapeId(0)),
                }]
                .into(),
                native: true,
            });
        assert_eq!(
            Rust::expression(&instruction),
            r#"data::graph::ProfiledInstruction::IntegerRegion(data::graph::ArithmeticRegion {
    inputs: data::Storage::Static(&[]),
    nodes: data::Storage::Static(&[
        data::graph::ArithmeticNode::Negate(data::graph::ArithmeticOperand::Immediate(0)),
        data::graph::ArithmeticNode::Negate(data::graph::ArithmeticOperand::Value(0)),
    ]),
    outputs: data::Storage::Static(&[
        data::graph::ArithmeticOutput {
            value: 1,
            slot: data::graph::ParamSlot {
                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                shape: data::type_::ValueShapeId(0),
            },
        },
    ]),
    native: true,
})"#
        );
    }

    #[test]
    fn emits_each_instruction_family_and_preserves_the_output_slot() {
        type Kind = ProfiledInstructionKind<HostedExecutionGraph>;
        let cases = [
            (
                Kind::Int(IntInstruction::TupleIndex {
                    tuple: TupleLocalId(2),
                    index: 1,
                }),
                "Int",
                r#"
data::graph::IntInstruction::TupleIndex {
    tuple: data::graph::TupleLocalId(2),
    index: 1,
}"#.trim_start_matches('\n'),
            ),
            (
                Kind::Float(FloatInstruction::TupleIndex {
                    tuple: TupleLocalId(2),
                    index: 1,
                }),
                "Float",
                r#"
data::graph::FloatInstruction::TupleIndex {
    tuple: data::graph::TupleLocalId(2),
    index: 1,
}"#.trim_start_matches('\n'),
            ),
            (
                Kind::String(StringInstruction::TupleIndex {
                    tuple: TupleLocalId(2),
                    index: 1,
                }),
                "String",
                r#"
data::graph::StringInstruction::TupleIndex {
    tuple: data::graph::TupleLocalId(2),
    index: 1,
}"#.trim_start_matches('\n'),
            ),
            (
                Kind::BitArray(BitArrayInstruction::TupleIndex {
                    tuple: TupleLocalId(2),
                    index: 1,
                }),
                "BitArray",
                r#"
data::graph::BitArrayInstruction::TupleIndex {
    tuple: data::graph::TupleLocalId(2),
    index: 1,
}"#.trim_start_matches('\n'),
            ),
            (
                Kind::UtfCodepoint(UtfCodepointInstruction::TupleIndex {
                    tuple: TupleLocalId(2),
                    index: 1,
                }),
                "UtfCodepoint",
                r#"
data::graph::UtfCodepointInstruction::TupleIndex {
    tuple: data::graph::TupleLocalId(2),
    index: 1,
}"#.trim_start_matches('\n'),
            ),
            (
                Kind::Custom(CustomInstruction::TupleIndex {
                    tuple: TupleLocalId(2),
                    index: 1,
                }),
                "Custom",
                r#"
data::graph::CustomInstruction::TupleIndex {
    tuple: data::graph::TupleLocalId(2),
    index: 1,
}"#.trim_start_matches('\n'),
            ),
            (
                Kind::External(ExternalInstruction::TupleIndex {
                    tuple: TupleLocalId(2),
                    index: 1,
                }),
                "External",
                r#"
data::graph::ExternalInstruction::TupleIndex {
    tuple: data::graph::TupleLocalId(2),
    index: 1,
}"#.trim_start_matches('\n'),
            ),
            (
                Kind::Bool(BoolInstruction::Value(true)),
                "Bool",
                "data::graph::BoolInstruction::Value(true)",
            ),
            (
                Kind::Nil(NilInstruction::Value),
                "Nil",
                "data::graph::NilInstruction::Value",
            ),
            (
                Kind::Tuple(TupleInstruction::Value(Table::Static(&[]))),
                "Tuple",
                "data::graph::TupleInstruction::Value(data::Storage::Static(&[]))",
            ),
            (
                Kind::List(ListInstruction::Int(
                    IntListTypeId {
                        list_type: ListTypeId(3),
                    },
                    TypedListInstruction::Value(Table::Static(&[])),
                )),
                "List",
                r#"
data::graph::ListInstruction::Int(data::type_::IntListTypeId {
    list_type: data::type_::ListTypeId(3),
}, data::graph::TypedListInstruction::Value(data::Storage::Static(&[])))"#.trim_start_matches('\n'),
            ),
            (
                Kind::ExternalList(ExternalListInstruction::new(
                    ExternalListTypeId::new(ListTypeId(3), ExternalTypeId(4)),
                    TypedListInstruction::Value(Table::Static(&[])),
                )),
                "ExternalList",
                r#"
data::graph::ExternalListInstruction {
    type_id: data::type_::ExternalListTypeId {
        list_type: data::type_::ListTypeId(3),
        item_type: data::type_::ExternalTypeId(4),
    },
    instruction: data::graph::TypedListInstruction::Value(data::Storage::Static(&[])),
}"#.trim_start_matches('\n'),
            ),
            (
                Kind::Function(FunctionInstruction {
                    type_: FunctionType::new(Vec::new(), ValueType::Int),
                    family: FunctionReturnFamily::Int,
                    kind: FunctionInstructionKind::Reference(FunctionTarget::Int(IntFunctionId(3))),
                }),
                "Function",
                r#"
data::graph::FunctionInstruction {
    type_: data::type_::FunctionType {
        arguments: data::Storage::Static(&[]),
        return_: data::Storage::Static(&data::type_::ValueType::Int),
    },
    family: data::function::FunctionReturnFamily::Int,
    kind: data::graph::FunctionInstructionKind::Reference(data::graph::FunctionTarget::Int(data::function::IntFunctionId(3))),
}"#.trim_start_matches('\n'),
            ),
            (
                Kind::ExternalFunction(ExternalFunctionInstruction {
                    type_: FunctionType::new(Vec::new(), ValueType::External(ExternalTypeId(4))),
                    family: FunctionReturnFamily::External,
                    kind: ExternalFunctionInstructionKind::Reference(
                        ExternalFunctionTarget::Value(ExternalFunctionId::new(
                            3,
                            ExternalTypeId(4),
                        )),
                    ),
                }),
                "ExternalFunction",
                r#"
data::graph::ExternalFunctionInstruction {
    type_: data::type_::FunctionType {
        arguments: data::Storage::Static(&[]),
        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(4))),
    },
    family: data::function::FunctionReturnFamily::External,
    kind: data::graph::ExternalFunctionInstructionKind::Reference(data::graph::ExternalFunctionTarget::Value(data::function::ExternalFunctionId {
        index: 3,
        return_type: data::type_::ExternalTypeId(4),
    })),
}"#.trim_start_matches('\n'),
            ),
        ];
        for (instruction, family, expected) in cases {
            assert_eq!(
                Rust::expression(&instruction),
                format!("data::graph::ProfiledInstructionKind::{family}({expected})")
            );
        }
        let instruction =
            ProfiledInstruction::<HostedExecutionGraph>::Value(ProfiledValueInstruction {
                output: ParamSlot::new(ParamLocal::Bool(BoolLocalId(2)), ValueShapeId(9)),
                kind: Kind::Bool(BoolInstruction::Value(true)),
            });
        assert_eq!(
            Rust::expression(&instruction),
            r#"
data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
    output: data::graph::ParamSlot {
        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(2)),
        shape: data::type_::ValueShapeId(9),
    },
    kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(true)),
})"#
            .trim_start_matches('\n')
        );
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
                .blocks()
                .next()
                .unwrap()
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
                args: Vec::new().into(),
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
                .blocks()
                .next()
                .unwrap()
                .instructions();
            let instruction = &instructions[instructions.len() - 2];
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(instruction.value().unwrap().kind());
        });
    }
}
