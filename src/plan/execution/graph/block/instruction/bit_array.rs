use super::super::super::{Endianness, FloatBitSize, StringEncoding};
use super::{write_call, write_constant, write_function_call, write_projection};
use crate::plan::PanicSite;
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::graph::{ExplainLocal, endianness, float_size, string_encoding};
use crate::plan::execution::{
    BitArrayFunctionId, BitArrayListLocalId, ConstantId, CustomLocal, FloatLocalId, IntLocalId,
    ParamLocal, StringLocalId, TupleLocalId, UtfCodepointLocalId,
};

pub(crate) struct BitArrayEvaluatedSize {
    value: IntLocalId,
    unit: u8,
}

pub(crate) enum BitArrayBitsSize {
    Fixed(usize),
    Evaluated(BitArrayEvaluatedSize),
}

pub(crate) enum BitArraySegment {
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
    Bits(crate::plan::execution::BitArrayLocalId),
    SizedBits {
        value: crate::plan::execution::BitArrayLocalId,
        size: BitArrayBitsSize,
        site: PanicSite,
    },
}

pub(crate) enum BitArrayInstruction {
    Value(Box<[BitArraySegment]>),
    Constant(ConstantId<crate::plan::execution::BitArrayLocalId>),
    Call {
        function: BitArrayFunctionId,
        args: Box<[ParamLocal]>,
    },
    FunctionCall {
        function: crate::plan::execution::BitArrayFunctionLocalId,
        args: Box<[ParamLocal]>,
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
            Self::Call { function, args } => {
                write_call(context.output(), "bit_array.call", function, args);
            }
            Self::FunctionCall { function, args } => {
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
        self.value().write_local(context.output());
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
                value.write_local(context.output());
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
                value.write_local(context.output());
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
                value.write_local(context.output());
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
                value.write_local(context.output());
                context.push_str(", bits=");
                context.write(size);
                context.push_str(", ");
                context.push_str(endianness(*order));
                context.push(')');
            }
            Self::String { value, encoding } => {
                context.push_str("string(");
                value.write_local(context.output());
                context.push_str(", ");
                context.push_str(string_encoding(*encoding));
                context.push(')');
            }
            Self::UtfCodepoint { value, encoding } => {
                context.push_str("utf_codepoint(");
                value.write_local(context.output());
                context.push_str(", ");
                context.push_str(string_encoding(*encoding));
                context.push(')');
            }
            Self::Bits(value) => {
                context.push_str("bits(");
                value.write_local(context.output());
                context.push(')');
            }
            Self::SizedBits { value, size, .. } => {
                context.push_str("bits(");
                value.write_local(context.output());
                context.push_str(", bits=");
                context.write(size);
                context.push(')');
            }
        }
    }
}

#[cfg(test)]
mod explain_tests {
    use crate::plan::execution::{BitArrayFunctionId, explain};

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

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let graph = plan
                .bit_array_function(BitArrayFunctionId(0))
                .body()
                .block_graph();
            let mut context = explain::ExplainContext::new(plan, output);
            for instruction in graph.blocks()[0].instructions() {
                context.write(instruction);
            }
        });
    }
}
