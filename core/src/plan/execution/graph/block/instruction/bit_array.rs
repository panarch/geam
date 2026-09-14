use super::super::super::{Endianness, FloatBitSize, StringEncoding};
use super::{write_call, write_constant, write_function_call, write_projection};
use crate::plan::PanicSite;
use crate::plan::execution::constant::ConstantId;
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::function::BitArrayFunctionId;
use crate::plan::execution::graph::{
    BitArrayListLocalId, CustomLocal, FloatLocalId, IntLocalId, ParamLocal, StringLocalId,
    TupleLocalId, UtfCodepointLocalId,
};
use crate::plan::execution::graph::{LocalLabel, endianness, float_size, string_encoding};
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::Table;

#[derive(Clone)]
pub struct BitArrayEvaluatedSize {
    pub value: IntLocalId,
    pub unit: u8,
}

#[derive(Clone)]
pub enum BitArrayBitsSize {
    Fixed(usize),
    Evaluated(BitArrayEvaluatedSize),
}

#[derive(Clone)]
pub enum BitArraySegment {
    Int {
        value: IntLocalId,
        bit_size: usize,
        endianness: Endianness,
    },
    EvaluatedInt {
        value: IntLocalId,
        size: BitArrayEvaluatedSize,
        endianness: Endianness,
        site: PanicSite,
    },
    Float {
        value: FloatLocalId,
        bit_size: FloatBitSize,
        endianness: Endianness,
    },
    EvaluatedFloat {
        value: FloatLocalId,
        size: BitArrayEvaluatedSize,
        endianness: Endianness,
        site: PanicSite,
    },
    String {
        value: StringLocalId,
        encoding: StringEncoding,
    },
    UtfCodepoint {
        value: UtfCodepointLocalId,
        encoding: StringEncoding,
    },
    Bits(crate::plan::execution::graph::BitArrayLocalId),
    SizedBits {
        value: crate::plan::execution::graph::BitArrayLocalId,
        size: BitArrayBitsSize,
        site: PanicSite,
    },
}

#[derive(Clone)]
pub enum BitArrayInstruction {
    Value(Table<BitArraySegment>),
    Constant(ConstantId<crate::plan::execution::graph::BitArrayLocalId>),
    Call {
        function: BitArrayFunctionId,
        args: Table<ParamLocal>,
        site: crate::plan::HostCallSite,
    },
    FunctionCall {
        function: crate::plan::execution::graph::BitArrayFunctionLocalId,
        args: Table<ParamLocal>,
        site: crate::plan::HostCallSite,
    },
    TupleIndex {
        tuple: TupleLocalId,
        index: usize,
    },
    CustomField {
        source: CustomLocal,
        index: usize,
    },
    ListIndex {
        list: BitArrayListLocalId,
        index: usize,
    },
}

impl BitArrayEvaluatedSize {
    pub(in crate::plan::execution) fn new(value: IntLocalId, unit: u8) -> Self {
        Self { value, unit }
    }

    pub(crate) fn value(&self) -> IntLocalId {
        self.value
    }

    pub(crate) fn unit(&self) -> u8 {
        self.unit
    }
}

impl Explain for BitArrayInstruction {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        match self {
            Self::Value(segments) => {
                context.push_str("bit_array.value ");
                context.write_list(segments, |context, segment| context.write(segment));
            }
            Self::Constant(id) => write_constant(context.output(), "bit_array", *id),
            Self::Call { function, args, .. } => {
                write_call(context.output(), "bit_array.call", function, args);
            }
            Self::FunctionCall { function, args, .. } => {
                write_function_call(context.output(), "bit_array.function_call", function, args);
            }
            Self::TupleIndex { tuple, index } => {
                write_projection(context.output(), "bit_array.tuple_index", tuple, *index);
            }
            Self::CustomField { source, index } => {
                write_projection(context.output(), "bit_array.custom_field", source, *index);
            }
            Self::ListIndex { list, index } => {
                write_projection(context.output(), "bit_array.list_index", list, *index);
            }
        }
    }
}

impl Explain for BitArrayEvaluatedSize {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        self.value().write_local_label(context.output());
        context.push('*');
        context.push_str(&self.unit().to_string());
    }
}

impl Explain for BitArrayBitsSize {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        match self {
            Self::Fixed(size) => context.push_str(&size.to_string()),
            Self::Evaluated(size) => context.write(size),
        }
    }
}

impl Explain for BitArraySegment {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        match self {
            Self::Int {
                value,
                bit_size,
                endianness: order,
            } => {
                context.push_str("int(");
                value.write_local_label(context.output());
                context.push_str(", bits=");
                context.push_str(&bit_size.to_string());
                context.push_str(", ");
                context.push_str(endianness(*order));
                context.push(')');
            }
            Self::EvaluatedInt {
                value,
                size,
                endianness: order,
                ..
            } => {
                context.push_str("int(");
                value.write_local_label(context.output());
                context.push_str(", bits=");
                context.write(size);
                context.push_str(", ");
                context.push_str(endianness(*order));
                context.push(')');
            }
            Self::Float {
                value,
                bit_size,
                endianness: order,
            } => {
                context.push_str("float(");
                value.write_local_label(context.output());
                context.push_str(", bits=");
                context.push_str(&float_size(*bit_size).to_string());
                context.push_str(", ");
                context.push_str(endianness(*order));
                context.push(')');
            }
            Self::EvaluatedFloat {
                value,
                size,
                endianness: order,
                ..
            } => {
                context.push_str("float(");
                value.write_local_label(context.output());
                context.push_str(", bits=");
                context.write(size);
                context.push_str(", ");
                context.push_str(endianness(*order));
                context.push(')');
            }
            Self::String { value, encoding } => {
                context.push_str("string(");
                value.write_local_label(context.output());
                context.push_str(", ");
                context.push_str(string_encoding(*encoding));
                context.push(')');
            }
            Self::UtfCodepoint { value, encoding } => {
                context.push_str("utf_codepoint(");
                value.write_local_label(context.output());
                context.push_str(", ");
                context.push_str(string_encoding(*encoding));
                context.push(')');
            }
            Self::Bits(value) => {
                context.push_str("bits(");
                value.write_local_label(context.output());
                context.push(')');
            }
            Self::SizedBits { value, size, .. } => {
                context.push_str("bits(");
                value.write_local_label(context.output());
                context.push_str(", bits=");
                context.write(size);
                context.push(')');
            }
        }
    }
}

impl Emit for BitArrayEvaluatedSize {
    fn emit(&self, output: &mut Rust) {
        let Self { value, unit } = self;
        output.structure(
            "graph::BitArrayEvaluatedSize",
            &[("value", value), ("unit", unit)],
        );
    }
}

impl Emit for BitArrayBitsSize {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Fixed(field_0) => output.call("graph::BitArrayBitsSize::Fixed", &[field_0]),
            Self::Evaluated(field_0) => {
                output.call("graph::BitArrayBitsSize::Evaluated", &[field_0])
            }
        }
    }
}

impl Emit for BitArraySegment {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Int {
                value,
                bit_size,
                endianness,
            } => output.structure(
                "graph::BitArraySegment::Int",
                &[
                    ("value", value),
                    ("bit_size", bit_size),
                    ("endianness", endianness),
                ],
            ),
            Self::EvaluatedInt {
                value,
                size,
                endianness,
                site,
            } => output.structure(
                "graph::BitArraySegment::EvaluatedInt",
                &[
                    ("value", value),
                    ("size", size),
                    ("endianness", endianness),
                    ("site", site),
                ],
            ),
            Self::Float {
                value,
                bit_size,
                endianness,
            } => output.structure(
                "graph::BitArraySegment::Float",
                &[
                    ("value", value),
                    ("bit_size", bit_size),
                    ("endianness", endianness),
                ],
            ),
            Self::EvaluatedFloat {
                value,
                size,
                endianness,
                site,
            } => output.structure(
                "graph::BitArraySegment::EvaluatedFloat",
                &[
                    ("value", value),
                    ("size", size),
                    ("endianness", endianness),
                    ("site", site),
                ],
            ),
            Self::String { value, encoding } => output.structure(
                "graph::BitArraySegment::String",
                &[("value", value), ("encoding", encoding)],
            ),
            Self::UtfCodepoint { value, encoding } => output.structure(
                "graph::BitArraySegment::UtfCodepoint",
                &[("value", value), ("encoding", encoding)],
            ),
            Self::Bits(field_0) => output.call("graph::BitArraySegment::Bits", &[field_0]),
            Self::SizedBits { value, size, site } => output.structure(
                "graph::BitArraySegment::SizedBits",
                &[("value", value), ("size", size), ("site", site)],
            ),
        }
    }
}

impl Emit for BitArrayInstruction {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Value(field_0) => output.call("graph::BitArrayInstruction::Value", &[field_0]),
            Self::Constant(field_0) => {
                output.call("graph::BitArrayInstruction::Constant", &[field_0])
            }
            Self::Call {
                function,
                args,
                site,
            } => output.structure(
                "graph::BitArrayInstruction::Call",
                &[("function", function), ("args", args), ("site", site)],
            ),
            Self::FunctionCall {
                function,
                args,
                site,
            } => output.structure(
                "graph::BitArrayInstruction::FunctionCall",
                &[("function", function), ("args", args), ("site", site)],
            ),
            Self::TupleIndex { tuple, index } => output.structure(
                "graph::BitArrayInstruction::TupleIndex",
                &[("tuple", tuple), ("index", index)],
            ),
            Self::CustomField { source, index } => output.structure(
                "graph::BitArrayInstruction::CustomField",
                &[("source", source), ("index", index)],
            ),
            Self::ListIndex { list, index } => output.structure(
                "graph::BitArrayInstruction::ListIndex",
                &[("list", list), ("index", index)],
            ),
        }
    }
}

#[cfg(test)]
mod emission_tests {
    use super::{BitArrayBitsSize, BitArrayEvaluatedSize, BitArrayInstruction, BitArraySegment};
    use crate::plan::execution::constant::ConstantId;
    use crate::plan::execution::function::BitArrayFunctionId;
    use crate::plan::execution::graph::{
        BitArrayFunctionLocalId, BitArrayListLocalId, BitArrayLocalId, CustomLocal, CustomLocalId,
        Endianness, FloatBitSize, FloatLocalId, IntLocalId, ParamLocal, StringEncoding,
        StringLocalId, TupleLocalId, UtfCodepointLocalId,
    };
    use crate::plan::execution::prepared::rust::Rust;
    use crate::plan::execution::type_::{CustomTypeId, CustomValueShape, CustomValueShapeId};
    use crate::plan::{HostCallSite, PanicSite, SourceSpan};

    #[test]
    fn emits_bit_segments_with_fixed_and_evaluated_sizes() {
        let site = PanicSite::new("example".into(), "main".into(), SourceSpan::new(3, 8));
        let cases = [
            (
                BitArraySegment::Int {
                    value: IntLocalId(2),
                    bit_size: 24,
                    endianness: Endianness::Little,
                },
                r#"
data::graph::BitArraySegment::Int {
    value: data::graph::IntLocalId(2),
    bit_size: 24,
    endianness: data::graph::Endianness::Little,
}"#.trim_start_matches('\n'),
            ),
            (
                BitArraySegment::EvaluatedInt {
                    value: IntLocalId(2),
                    size: BitArrayEvaluatedSize::new(IntLocalId(3), 8),
                    endianness: Endianness::Big,
                    site: site.clone(),
                },
                r#"
data::graph::BitArraySegment::EvaluatedInt {
    value: data::graph::IntLocalId(2),
    size: data::graph::BitArrayEvaluatedSize {
        value: data::graph::IntLocalId(3),
        unit: 8,
    },
    endianness: data::graph::Endianness::Big,
    site: data::source::PanicSite::from_static("example", "main", data::source::SourceSpan::new(3, 8)),
}"#.trim_start_matches('\n'),
            ),
            (
                BitArraySegment::Float {
                    value: FloatLocalId(2),
                    bit_size: FloatBitSize::ThirtyTwo,
                    endianness: Endianness::Little,
                },
                r#"
data::graph::BitArraySegment::Float {
    value: data::graph::FloatLocalId(2),
    bit_size: data::graph::FloatBitSize::ThirtyTwo,
    endianness: data::graph::Endianness::Little,
}"#.trim_start_matches('\n'),
            ),
            (
                BitArraySegment::EvaluatedFloat {
                    value: FloatLocalId(2),
                    size: BitArrayEvaluatedSize::new(IntLocalId(3), 1),
                    endianness: Endianness::Little,
                    site: site.clone(),
                },
                r#"
data::graph::BitArraySegment::EvaluatedFloat {
    value: data::graph::FloatLocalId(2),
    size: data::graph::BitArrayEvaluatedSize {
        value: data::graph::IntLocalId(3),
        unit: 1,
    },
    endianness: data::graph::Endianness::Little,
    site: data::source::PanicSite::from_static("example", "main", data::source::SourceSpan::new(3, 8)),
}"#.trim_start_matches('\n'),
            ),
            (
                BitArraySegment::String {
                    value: StringLocalId(2),
                    encoding: StringEncoding::Utf8,
                },
                r#"
data::graph::BitArraySegment::String {
    value: data::graph::StringLocalId(2),
    encoding: data::graph::StringEncoding::Utf8,
}"#.trim_start_matches('\n'),
            ),
            (
                BitArraySegment::UtfCodepoint {
                    value: UtfCodepointLocalId(2),
                    encoding: StringEncoding::Utf16(Endianness::Little),
                },
                r#"
data::graph::BitArraySegment::UtfCodepoint {
    value: data::graph::UtfCodepointLocalId(2),
    encoding: data::graph::StringEncoding::Utf16(data::graph::Endianness::Little),
}"#.trim_start_matches('\n'),
            ),
            (
                BitArraySegment::Bits(BitArrayLocalId(2)),
                "data::graph::BitArraySegment::Bits(data::graph::BitArrayLocalId(2))",
            ),
            (
                BitArraySegment::SizedBits {
                    value: BitArrayLocalId(2),
                    size: BitArrayBitsSize::Fixed(13),
                    site: site.clone(),
                },
                r#"
data::graph::BitArraySegment::SizedBits {
    value: data::graph::BitArrayLocalId(2),
    size: data::graph::BitArrayBitsSize::Fixed(13),
    site: data::source::PanicSite::from_static("example", "main", data::source::SourceSpan::new(3, 8)),
}"#.trim_start_matches('\n'),
            ),
            (
                BitArraySegment::SizedBits {
                    value: BitArrayLocalId(2),
                    size: BitArrayBitsSize::Evaluated(BitArrayEvaluatedSize::new(IntLocalId(3), 8)),
                    site,
                },
                r#"
data::graph::BitArraySegment::SizedBits {
    value: data::graph::BitArrayLocalId(2),
    size: data::graph::BitArrayBitsSize::Evaluated(data::graph::BitArrayEvaluatedSize {
        value: data::graph::IntLocalId(3),
        unit: 8,
    }),
    site: data::source::PanicSite::from_static("example", "main", data::source::SourceSpan::new(3, 8)),
}"#.trim_start_matches('\n'),
            ),
        ];
        for (segment, expected) in cases {
            assert_eq!(Rust::expression(&segment), expected);
        }
    }

    #[test]
    fn emits_every_bit_array_instruction_with_its_operands_and_source_site() {
        let site = HostCallSite::new("example".into(), "main".into(), SourceSpan::new(3, 8));
        let cases = [
            (
                BitArrayInstruction::Value(vec![BitArraySegment::Bits(BitArrayLocalId(7))].into()),
                r#"
data::graph::BitArrayInstruction::Value(data::Storage::Static(&[
    data::graph::BitArraySegment::Bits(data::graph::BitArrayLocalId(7)),
]))"#.trim_start_matches('\n'),
            ),
            (
                BitArrayInstruction::Constant(ConstantId::new(3)),
                r#"
data::graph::BitArrayInstruction::Constant(data::constant::ConstantId {
    index: 3,
    value: ::core::marker::PhantomData,
})"#.trim_start_matches('\n'),
            ),
            (
                BitArrayInstruction::Call {
                    function: BitArrayFunctionId(2),
                    args: vec![ParamLocal::BitArray(BitArrayLocalId(5))].into(),
                    site: site.clone(),
                },
                r#"
data::graph::BitArrayInstruction::Call {
    function: data::function::BitArrayFunctionId(2),
    args: data::Storage::Static(&[
        data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(5)),
    ]),
    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3, 8)),
}"#.trim_start_matches('\n'),
            ),
            (
                BitArrayInstruction::FunctionCall {
                    function: BitArrayFunctionLocalId(2),
                    args: vec![ParamLocal::BitArray(BitArrayLocalId(5))].into(),
                    site,
                },
                r#"
data::graph::BitArrayInstruction::FunctionCall {
    function: data::graph::BitArrayFunctionLocalId(2),
    args: data::Storage::Static(&[
        data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(5)),
    ]),
    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3, 8)),
}"#.trim_start_matches('\n'),
            ),
            (
                BitArrayInstruction::TupleIndex {
                    tuple: TupleLocalId(2),
                    index: 1,
                },
                r#"
data::graph::BitArrayInstruction::TupleIndex {
    tuple: data::graph::TupleLocalId(2),
    index: 1,
}"#.trim_start_matches('\n'),
            ),
            (
                BitArrayInstruction::CustomField {
                    source: CustomLocal::new(
                        CustomLocalId(2),
                        CustomValueShape::new(CustomTypeId(3), CustomValueShapeId(4)),
                    ),
                    index: 1,
                },
                r#"
data::graph::BitArrayInstruction::CustomField {
    source: data::graph::CustomLocal {
        id: data::graph::CustomLocalId(2),
        shape: data::type_::CustomValueShape {
            type_id: data::type_::CustomTypeId(3),
            shape_id: data::type_::CustomValueShapeId(4),
        },
    },
    index: 1,
}"#.trim_start_matches('\n'),
            ),
            (
                BitArrayInstruction::ListIndex {
                    list: BitArrayListLocalId(2),
                    index: 1,
                },
                r#"
data::graph::BitArrayInstruction::ListIndex {
    list: data::graph::BitArrayListLocalId(2),
    index: 1,
}"#.trim_start_matches('\n'),
            ),
        ];
        for (instruction, expected) in cases {
            assert_eq!(Rust::expression(&instruction), expected);
        }
    }
}

#[cfg(test)]
mod explain_tests {
    use super::{BitArrayBitsSize, BitArrayEvaluatedSize, BitArraySegment};
    use crate::plan::execution::explain;
    use crate::plan::execution::function::BitArrayFunctionId;
    use crate::plan::execution::graph::IntLocalId;

    #[test]
    fn writes_bit_array_instruction_and_segment_grammar() {
        let source = r#"
pub fn main() {
  let size = 8
  let bits = <<1, 2>>
  <<
    1:4-big,
    2:size(size)-little,
    1.5:float-size(16)-big,
    2.5:float-size(size * 4)-little,
    "a":utf8,
    bits:bits-size(size),
  >>
}
"#;
        let expected = concat!(
            "    %int#0:shape#0(Int) = int.value 8\n",
            "    %int#1:shape#0(Int) = int.value 1\n",
            "    %int#2:shape#0(Int) = int.value 2\n",
            "    %bit_array#0:shape#1(BitArray) = bit_array.value ",
            "[int(%int#1, bits=8, big), int(%int#2, bits=8, big)]\n",
            "    %int#3:shape#0(Int) = int.value 1\n",
            "    %int#4:shape#0(Int) = int.value 2\n",
            "    %float#0:shape#2(Float) = float.value 1.5\n",
            "    %float#1:shape#2(Float) = float.value 2.5\n",
            "    %int#5:shape#0(Int) = int.value 4\n",
            "    %int#6:shape#0(Int) = int.mult %int#0 %int#5\n",
            "    %string#0:shape#3(String) = string.value \"a\"\n",
            "    %bit_array#1:shape#1(BitArray) = bit_array.value ",
            "[int(%int#3, bits=4, big), int(%int#4, bits=%int#0*1, little), ",
            "float(%float#0, bits=16, big), float(%float#1, bits=%int#6*1, little), ",
            "string(%string#0, utf8), bits(%bit_array#0, bits=%int#0*1)]\n",
        );

        assert_explanation(source, expected);
    }

    #[test]
    fn writes_bit_array_evaluated_size() {
        let source = "pub fn main() { <<>> }";
        let expected = "%int#2*4";

        explain::assert_rendered(source, expected, |plan, output| {
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(&BitArrayEvaluatedSize::new(IntLocalId(2), 4));
        });
    }

    #[test]
    fn writes_fixed_and_evaluated_bit_array_bits_size() {
        let source = "pub fn main() { <<>> }";
        let expected = "8 | %int#2*4";

        explain::assert_rendered(source, expected, |plan, output| {
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(&BitArrayBitsSize::Fixed(8));
            context.push_str(" | ");
            context.write(&BitArrayBitsSize::Evaluated(BitArrayEvaluatedSize::new(
                IntLocalId(2),
                4,
            )));
        });
    }

    #[test]
    fn writes_bit_array_segment() {
        let source = "pub fn main() { <<1>> }";
        let expected = "int(%int#2, bits=4, big)";

        explain::assert_rendered(source, expected, |plan, output| {
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(&BitArraySegment::Int {
                value: IntLocalId(2),
                bit_size: 4,
                endianness: crate::plan::execution::graph::Endianness::Big,
            });
        });
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let graph = plan
                .bit_array_function(BitArrayFunctionId(0))
                .body()
                .block_graph();
            let mut context = explain::ExplainContext::new(plan, output);
            for instruction in graph.blocks().next().unwrap().instructions() {
                context.write(instruction);
            }
        });
    }
}
