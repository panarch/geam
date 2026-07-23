mod binding;
mod size;
mod value;

use self::binding::write_binding_pattern;
use self::size::write_size;
use self::value::write_value;
use super::super::super::bit_array::{endianness, string_encoding};
use crate::plan::execution::graph::{BitArrayPatternSegment, BitArrayStringPattern, Signedness};

pub(super) fn write_segment(output: &mut String, segment: &BitArrayPatternSegment) {
    match segment {
        BitArrayPatternSegment::Int {
            pattern,
            size,
            endianness: order,
            signedness,
        } => {
            output.push_str("int(");
            write_value(output, pattern, |output, value| {
                output.push_str(&value.to_string());
            });
            output.push_str(", size=");
            write_size(output, size);
            output.push_str(", ");
            output.push_str(endianness(*order));
            output.push_str(", ");
            output.push_str(match signedness {
                Signedness::Signed => "signed",
                Signedness::Unsigned => "unsigned",
            });
            output.push(')');
        }
        BitArrayPatternSegment::Float {
            pattern,
            size,
            endianness: order,
        } => {
            output.push_str("float(");
            write_value(output, pattern, |output, value| {
                output.push_str(&format!("{value:?}"));
            });
            output.push_str(", size=");
            write_size(output, size);
            output.push_str(", ");
            output.push_str(endianness(*order));
            output.push(')');
        }
        BitArrayPatternSegment::Bits {
            pattern,
            size,
            unit,
        } => {
            output.push_str("bits(");
            write_binding_pattern(output, pattern);
            output.push_str(", size=");
            match size {
                Some(size) => write_size(output, size),
                None => output.push_str("rest"),
            }
            output.push_str(", unit=");
            output.push_str(&unit.to_string());
            output.push(')');
        }
        BitArrayPatternSegment::String { pattern, encoding } => {
            output.push_str("string(");
            match pattern {
                BitArrayStringPattern::Literal(value) => {
                    output.push_str(&format!("{value:?}"));
                }
                BitArrayStringPattern::Discard => output.push('_'),
            }
            output.push_str(", ");
            output.push_str(string_encoding(*encoding));
            output.push(')');
        }
        BitArrayPatternSegment::UtfCodepoint { pattern, encoding } => {
            output.push_str("utf_codepoint(");
            write_binding_pattern(output, pattern);
            output.push_str(", ");
            output.push_str(string_encoding(*encoding));
            output.push(')');
        }
    }
}
