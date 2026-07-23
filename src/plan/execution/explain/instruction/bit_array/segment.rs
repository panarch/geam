mod size;

use self::size::write_evaluated_size;
use super::super::super::bit_array::{endianness, float_size, string_encoding};
use super::super::super::value::ExplainLocal;
use crate::plan::execution::graph::{BitArrayBitsSize, BitArraySegment};

pub(super) fn write_segment(output: &mut String, segment: &BitArraySegment) {
    match segment {
        BitArraySegment::Int {
            value,
            bit_size,
            endianness: order,
        } => {
            output.push_str("int(");
            value.write_local(output);
            output.push_str(", bits=");
            output.push_str(&bit_size.to_string());
            output.push_str(", ");
            output.push_str(endianness(*order));
            output.push(')');
        }
        BitArraySegment::EvaluatedInt {
            value,
            size,
            endianness: order,
            ..
        } => {
            output.push_str("int(");
            value.write_local(output);
            output.push_str(", bits=");
            write_evaluated_size(output, size);
            output.push_str(", ");
            output.push_str(endianness(*order));
            output.push(')');
        }
        BitArraySegment::Float {
            value,
            bit_size,
            endianness: order,
        } => {
            output.push_str("float(");
            value.write_local(output);
            output.push_str(", bits=");
            output.push_str(&float_size(*bit_size).to_string());
            output.push_str(", ");
            output.push_str(endianness(*order));
            output.push(')');
        }
        BitArraySegment::EvaluatedFloat {
            value,
            size,
            endianness: order,
            ..
        } => {
            output.push_str("float(");
            value.write_local(output);
            output.push_str(", bits=");
            write_evaluated_size(output, size);
            output.push_str(", ");
            output.push_str(endianness(*order));
            output.push(')');
        }
        BitArraySegment::String { value, encoding } => {
            output.push_str("string(");
            value.write_local(output);
            output.push_str(", ");
            output.push_str(string_encoding(*encoding));
            output.push(')');
        }
        BitArraySegment::UtfCodepoint { value, encoding } => {
            output.push_str("utf_codepoint(");
            value.write_local(output);
            output.push_str(", ");
            output.push_str(string_encoding(*encoding));
            output.push(')');
        }
        BitArraySegment::Bits(value) => {
            output.push_str("bits(");
            value.write_local(output);
            output.push(')');
        }
        BitArraySegment::SizedBits { value, size, .. } => {
            output.push_str("bits(");
            value.write_local(output);
            output.push_str(", bits=");
            match size {
                BitArrayBitsSize::Fixed(size) => output.push_str(&size.to_string()),
                BitArrayBitsSize::Evaluated(size) => write_evaluated_size(output, size),
            }
            output.push(')');
        }
    }
}
