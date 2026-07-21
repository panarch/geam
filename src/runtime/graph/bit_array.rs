use bitvec::order::Msb0;
use bitvec::slice::BitSlice;
use bitvec::vec::BitVec;
use bitvec::view::BitView;
use num_bigint::BigInt;

use crate::plan::execution::{
    Endianness, ExecutionPlan, FloatBitSize, GraphBitArrayBitsSize, GraphBitArrayEvaluatedSize,
    GraphBitArraySegment, Signedness, StringEncoding,
};
use crate::runtime::environment::BlockEnvironment;
use crate::runtime::evaluated::EvaluatedBitArray;
use crate::runtime::{BitArraySegmentPanicReason, ExecutionError};

pub(super) fn evaluate(
    plan: &ExecutionPlan,
    environment: &BlockEnvironment,
    segments: &[GraphBitArraySegment],
) -> Result<EvaluatedBitArray, ExecutionError> {
    let mut bits = BitVec::<u8, Msb0>::new();
    for segment in segments {
        append_segment(plan, environment, &mut bits, segment)?;
    }
    Ok(EvaluatedBitArray::new(bits))
}

fn append_segment(
    plan: &ExecutionPlan,
    environment: &BlockEnvironment,
    output: &mut BitVec<u8, Msb0>,
    segment: &GraphBitArraySegment,
) -> Result<(), ExecutionError> {
    match segment {
        GraphBitArraySegment::Int {
            value,
            bit_size,
            endianness,
        } => append_integer(output, &environment.int(*value), *bit_size, *endianness),
        GraphBitArraySegment::EvaluatedInt {
            value,
            size,
            endianness,
            site,
        } => {
            let bit_size = evaluate_size(plan, environment, size, site)?;
            append_integer(output, &environment.int(*value), bit_size, *endianness);
        }
        GraphBitArraySegment::Float {
            value,
            bit_size,
            endianness,
        } => append_float(output, environment.float(*value), *bit_size, *endianness),
        GraphBitArraySegment::EvaluatedFloat {
            value,
            size,
            endianness,
            site,
        } => {
            let bit_size = evaluate_size(plan, environment, size, site)?;
            let bit_size = match bit_size {
                16 => FloatBitSize::Sixteen,
                32 => FloatBitSize::ThirtyTwo,
                64 => FloatBitSize::SixtyFour,
                bit_size => {
                    return Err(ExecutionError::bit_array_segment_panic(
                        plan.source_context(),
                        BitArraySegmentPanicReason::InvalidFloatSize {
                            bit_size: BigInt::from(bit_size),
                        },
                        site.clone(),
                    ));
                }
            };
            append_float(output, environment.float(*value), bit_size, *endianness);
        }
        GraphBitArraySegment::String { value, encoding } => {
            append_string(output, environment.string(*value).as_str(), *encoding);
        }
        GraphBitArraySegment::UtfCodepoint { value, encoding } => {
            append_utf_codepoint(output, environment.utf_codepoint(*value), *encoding);
        }
        GraphBitArraySegment::Bits(value) => {
            let value = environment.bit_array(*value);
            output.extend_from_bitslice(value.bits());
        }
        GraphBitArraySegment::SizedBits { value, size, site } => {
            let value = environment.bit_array(*value);
            let bit_size = match size {
                GraphBitArrayBitsSize::Fixed(bit_size) => *bit_size,
                GraphBitArrayBitsSize::Evaluated(size) => {
                    evaluate_size(plan, environment, size, site)?
                }
            };
            let Some(bits) = value.bits().get(..bit_size) else {
                return Err(ExecutionError::bit_array_segment_panic(
                    plan.source_context(),
                    BitArraySegmentPanicReason::InsufficientBits {
                        requested: bit_size,
                        available: value.bits().len(),
                    },
                    site.clone(),
                ));
            };
            output.extend_from_bitslice(bits);
        }
    }
    Ok(())
}

fn evaluate_size(
    plan: &ExecutionPlan,
    environment: &BlockEnvironment,
    size: &GraphBitArrayEvaluatedSize,
    site: &crate::plan::PanicSite,
) -> Result<usize, ExecutionError> {
    let value = environment.int(size.value());
    let bit_size = if value < BigInt::from(0) {
        BigInt::from(0)
    } else {
        value * BigInt::from(size.unit())
    };
    usize::try_from(bit_size.clone()).map_err(|_| {
        ExecutionError::bit_array_segment_panic(
            plan.source_context(),
            BitArraySegmentPanicReason::SizeOutOfRange { bit_size },
            site.clone(),
        )
    })
}

fn append_integer(
    output: &mut BitVec<u8, Msb0>,
    value: &BigInt,
    bit_size: usize,
    endianness: Endianness,
) {
    if bit_size == 0 {
        return;
    }
    let mask = (BigInt::from(1u8) << bit_size) - BigInt::from(1u8);
    let truncated = value & mask;
    match endianness {
        Endianness::Big => append_low_bits(output, &truncated.to_bytes_be().1, bit_size),
        Endianness::Little => {
            let mut bytes = truncated.to_bytes_le().1;
            bytes.resize(bit_size.div_ceil(8), 0);
            let full_bytes = bit_size / 8;
            for byte in &bytes[..full_bytes] {
                output.extend_from_bitslice(byte.view_bits::<Msb0>());
            }
            let remaining = bit_size % 8;
            if remaining > 0 {
                let byte = bytes[full_bytes];
                output.extend_from_bitslice(&byte.view_bits::<Msb0>()[8 - remaining..]);
            }
        }
    }
}

fn append_low_bits(output: &mut BitVec<u8, Msb0>, bytes: &[u8], bit_size: usize) {
    let byte_len = bit_size.div_ceil(8);
    let padding = byte_len.saturating_sub(bytes.len());
    let mut padded = vec![0; padding];
    padded.extend_from_slice(bytes);
    let leading = byte_len * 8 - bit_size;
    output.extend_from_bitslice(&padded.view_bits::<Msb0>()[leading..]);
}

fn append_float(
    output: &mut BitVec<u8, Msb0>,
    value: f64,
    bit_size: FloatBitSize,
    endianness: Endianness,
) {
    let bytes = match (bit_size, endianness) {
        (FloatBitSize::Sixteen, Endianness::Big) => {
            half::f16::from_f64(value).to_be_bytes().to_vec()
        }
        (FloatBitSize::Sixteen, Endianness::Little) => {
            half::f16::from_f64(value).to_le_bytes().to_vec()
        }
        (FloatBitSize::ThirtyTwo, Endianness::Big) => (value as f32).to_be_bytes().to_vec(),
        (FloatBitSize::ThirtyTwo, Endianness::Little) => (value as f32).to_le_bytes().to_vec(),
        (FloatBitSize::SixtyFour, Endianness::Big) => value.to_be_bytes().to_vec(),
        (FloatBitSize::SixtyFour, Endianness::Little) => value.to_le_bytes().to_vec(),
    };
    output.extend_from_bitslice(bytes.view_bits::<Msb0>());
}

pub(super) fn encode_string(value: &str, encoding: StringEncoding) -> BitVec<u8, Msb0> {
    let mut bits = BitVec::new();
    match encoding {
        StringEncoding::Utf8 => bits.extend_from_bitslice(value.as_bytes().view_bits::<Msb0>()),
        StringEncoding::Utf16(endianness) => {
            for code in value.encode_utf16() {
                let bytes = match endianness {
                    Endianness::Big => code.to_be_bytes(),
                    Endianness::Little => code.to_le_bytes(),
                };
                bits.extend_from_bitslice(bytes.view_bits::<Msb0>());
            }
        }
        StringEncoding::Utf32(endianness) => {
            for character in value.chars() {
                let bytes = match endianness {
                    Endianness::Big => u32::from(character).to_be_bytes(),
                    Endianness::Little => u32::from(character).to_le_bytes(),
                };
                bits.extend_from_bitslice(bytes.view_bits::<Msb0>());
            }
        }
    }
    bits
}

fn append_string(output: &mut BitVec<u8, Msb0>, value: &str, encoding: StringEncoding) {
    output.extend_from_bitslice(&encode_string(value, encoding));
}

fn append_utf_codepoint(output: &mut BitVec<u8, Msb0>, value: char, encoding: StringEncoding) {
    match encoding {
        StringEncoding::Utf8 => {
            let mut buffer = [0; 4];
            output.extend_from_bitslice(
                value
                    .encode_utf8(&mut buffer)
                    .as_bytes()
                    .view_bits::<Msb0>(),
            );
        }
        StringEncoding::Utf16(endianness) => {
            let mut buffer = [0; 2];
            for code in value.encode_utf16(&mut buffer) {
                let bytes = match endianness {
                    Endianness::Big => code.to_be_bytes(),
                    Endianness::Little => code.to_le_bytes(),
                };
                output.extend_from_bitslice(bytes.view_bits::<Msb0>());
            }
        }
        StringEncoding::Utf32(endianness) => {
            let bytes = match endianness {
                Endianness::Big => u32::from(value).to_be_bytes(),
                Endianness::Little => u32::from(value).to_le_bytes(),
            };
            output.extend_from_bitslice(bytes.view_bits::<Msb0>());
        }
    }
}

pub(super) fn take_bits<'a>(
    subject: &'a BitSlice<u8, Msb0>,
    cursor: &mut usize,
    bit_size: usize,
) -> Option<&'a BitSlice<u8, Msb0>> {
    let end = cursor.checked_add(bit_size)?;
    let bits = subject.get(*cursor..end)?;
    *cursor = end;
    Some(bits)
}

pub(super) fn decode_integer(
    bits: &BitSlice<u8, Msb0>,
    endianness: Endianness,
    signedness: Signedness,
) -> BigInt {
    let mut value = BigInt::from(0u8);
    match endianness {
        Endianness::Big => {
            for bit in bits {
                value = (value << 1) + BigInt::from(u8::from(*bit));
            }
        }
        Endianness::Little => {
            for (index, chunk) in bits.chunks(8).enumerate() {
                let mut byte = 0u8;
                for bit in chunk {
                    byte = (byte << 1) | u8::from(*bit);
                }
                value += BigInt::from(byte) << (index * 8);
            }
        }
    }
    if signedness == Signedness::Signed && !bits.is_empty() {
        let sign_bit = BigInt::from(1u8) << (bits.len() - 1);
        if (&value & sign_bit) != BigInt::from(0u8) {
            value -= BigInt::from(1u8) << bits.len();
        }
    }
    value
}

pub(super) fn decode_float(
    bits: &BitSlice<u8, Msb0>,
    bit_size: FloatBitSize,
    endianness: Endianness,
) -> f64 {
    let mut bytes = vec![0u8; bits.len() / 8];
    bytes.view_bits_mut::<Msb0>().copy_from_bitslice(bits);
    match (bit_size, endianness) {
        (FloatBitSize::Sixteen, Endianness::Big) => {
            half::f16::from_be_bytes([bytes[0], bytes[1]]).to_f64()
        }
        (FloatBitSize::Sixteen, Endianness::Little) => {
            half::f16::from_le_bytes([bytes[0], bytes[1]]).to_f64()
        }
        (FloatBitSize::ThirtyTwo, Endianness::Big) => {
            f32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as f64
        }
        (FloatBitSize::ThirtyTwo, Endianness::Little) => {
            f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as f64
        }
        (FloatBitSize::SixtyFour, Endianness::Big) => f64::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]),
        (FloatBitSize::SixtyFour, Endianness::Little) => f64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]),
    }
}

pub(super) fn decode_codepoint(
    bits: &BitSlice<u8, Msb0>,
    encoding: StringEncoding,
) -> Option<(char, usize)> {
    match encoding {
        StringEncoding::Utf8 => decode_utf8(bits),
        StringEncoding::Utf16(endianness) => decode_utf16(bits, endianness),
        StringEncoding::Utf32(endianness) => decode_utf32(bits, endianness),
    }
}

fn decode_utf8(bits: &BitSlice<u8, Msb0>) -> Option<(char, usize)> {
    let first = read_byte(bits, 0)?;
    let byte_count = match first {
        0x00..=0x7f => 1,
        0xc2..=0xdf => 2,
        0xe0..=0xef => 3,
        0xf0..=0xf4 => 4,
        _ => return None,
    };
    let mut encoded = Vec::with_capacity(byte_count);
    for index in 0..byte_count {
        encoded.push(read_byte(bits, index * 8)?);
    }
    let Ok(value) = std::str::from_utf8(&encoded) else {
        return None;
    };
    value
        .chars()
        .next()
        .map(|character| (character, byte_count * 8))
}

fn decode_utf16(bits: &BitSlice<u8, Msb0>, endianness: Endianness) -> Option<(char, usize)> {
    let first = read_u16(bits, 0, endianness)?;
    let (codepoint, bit_size) = if (0xd800..=0xdbff).contains(&first) {
        let second = read_u16(bits, 16, endianness)?;
        if !(0xdc00..=0xdfff).contains(&second) {
            return None;
        }
        let high = u32::from(first - 0xd800);
        let low = u32::from(second - 0xdc00);
        (0x10000 + (high << 10) + low, 32)
    } else {
        (u32::from(first), 16)
    };
    Some((char::from_u32(codepoint)?, bit_size))
}

fn decode_utf32(bits: &BitSlice<u8, Msb0>, endianness: Endianness) -> Option<(char, usize)> {
    let bytes = [
        read_byte(bits, 0)?,
        read_byte(bits, 8)?,
        read_byte(bits, 16)?,
        read_byte(bits, 24)?,
    ];
    let value = match endianness {
        Endianness::Big => u32::from_be_bytes(bytes),
        Endianness::Little => u32::from_le_bytes(bytes),
    };
    Some((char::from_u32(value)?, 32))
}

fn read_u16(bits: &BitSlice<u8, Msb0>, offset: usize, endianness: Endianness) -> Option<u16> {
    let bytes = [read_byte(bits, offset)?, read_byte(bits, offset + 8)?];
    Some(match endianness {
        Endianness::Big => u16::from_be_bytes(bytes),
        Endianness::Little => u16::from_le_bytes(bytes),
    })
}

fn read_byte(bits: &BitSlice<u8, Msb0>, offset: usize) -> Option<u8> {
    let bits = bits.get(offset..offset.checked_add(8)?)?;
    let mut byte = 0;
    for bit in bits {
        byte = (byte << 1) | u8::from(*bit);
    }
    Some(byte)
}

#[cfg(test)]
mod tests {
    use super::{decode_float, decode_integer, decode_utf8, decode_utf16, decode_utf32, take_bits};
    use crate::plan::execution::{Endianness, FloatBitSize, Signedness};
    use crate::plan::{PanicSite, SourceSpan};
    use crate::runtime::{BitArraySegmentPanicReason, BitArrayValue, ExecutionError, Value};
    use bitvec::order::Msb0;
    use bitvec::vec::BitVec;
    use bitvec::view::BitView;
    use num_bigint::BigInt;

    #[test]
    fn evaluated_segment_failures_preserve_exact_panic_reasons() {
        for (source, segment, reason) in [
            (
                "pub fn main() { let size = 24 <<1.5:float-size(size)>> }",
                "1.5:float-size(size)",
                BitArraySegmentPanicReason::InvalidFloatSize {
                    bit_size: 24.into(),
                },
            ),
            (
                "pub fn main() { let bits = <<1>> <<bits:bits-size(9)>> }",
                "bits:bits-size(9)",
                BitArraySegmentPanicReason::InsufficientBits {
                    requested: 9,
                    available: 8,
                },
            ),
            (
                "pub fn main() { let size = 99999999999999999999999999999999999999 <<1:size(size)>> }",
                "1:size(size)",
                BitArraySegmentPanicReason::SizeOutOfRange {
                    bit_size: BigInt::parse_bytes(b"99999999999999999999999999999999999999", 10)
                        .expect("integer"),
                },
            ),
            (
                "pub fn main() { let size = 99999999999999999999999999999999999999 <<1.5:float-size(size)>> }",
                "1.5:float-size(size)",
                BitArraySegmentPanicReason::SizeOutOfRange {
                    bit_size: BigInt::parse_bytes(b"99999999999999999999999999999999999999", 10)
                        .expect("integer"),
                },
            ),
            (
                "pub fn main() { let bits = <<1>> let size = 99999999999999999999999999999999999999 <<bits:bits-size(size)>> }",
                "bits:bits-size(size)",
                BitArraySegmentPanicReason::SizeOutOfRange {
                    bit_size: BigInt::parse_bytes(b"99999999999999999999999999999999999999", 10)
                        .expect("integer"),
                },
            ),
        ] {
            let start = source.find(segment).expect("segment should exist");
            assert_eq!(
                crate::runtime::run_src_error(source),
                ExecutionError::bit_array_segment_panic(
                    None,
                    reason,
                    PanicSite::new(
                        "main".into(),
                        "main".into(),
                        SourceSpan::new(start, start + segment.len()),
                    ),
                ),
            );
        }
    }

    #[test]
    fn source_bit_array_instruction_paths_preserve_exact_values() {
        assert_eq!(
            crate::runtime::run_src("pub fn main() { <<0x1234:little-size(16)>> }"),
            Value::BitArray(BitArrayValue::from_bytes(vec![0x34, 0x12])),
        );
        assert_eq!(
            crate::runtime::run_src(include_str!(
                "../../../tests/fixtures/execution/values/bit_array_expression_paths.gleam"
            )),
            Value::Tuple(
                [
                    1, 2, 3, 11, 12, 3, 4, 5, 6, 7, 8, 9, 10, 14, 15, 16, 17, 18, 19, 20, 21, 13,
                ]
                .into_iter()
                .map(|byte| Value::BitArray(BitArrayValue::from_bytes(vec![byte])))
                .collect(),
            ),
        );
        assert_eq!(
            crate::runtime::run_src(include_str!(
                "../../../tests/fixtures/execution/values/bit_array_segments.gleam"
            )),
            expected_segment_values(),
        );
        assert_eq!(
            crate::runtime::run_src(include_str!(
                "../../../tests/fixtures/execution/module_items/constant_bit_array_segments.gleam"
            )),
            expected_segment_values(),
        );
    }

    #[test]
    fn integer_decoder_preserves_signedness_endianness_and_unaligned_bits() {
        assert_eq!(
            decode_integer(
                [0xfe].view_bits::<Msb0>(),
                Endianness::Big,
                Signedness::Signed,
            ),
            (-2).into(),
        );
        assert_eq!(
            decode_integer(
                &[0x34, 0x20].view_bits::<Msb0>()[..12],
                Endianness::Little,
                Signedness::Unsigned,
            ),
            0x234.into(),
        );
        assert_eq!(
            decode_integer(
                &[0xfe, 0xf0].view_bits::<Msb0>()[..12],
                Endianness::Little,
                Signedness::Signed,
            ),
            (-2).into(),
        );
        assert_eq!(
            decode_integer(
                [0x7f].view_bits::<Msb0>(),
                Endianness::Big,
                Signedness::Signed,
            ),
            127.into(),
        );
    }

    #[test]
    fn float_decoder_accepts_every_supported_width_and_endianness() {
        assert_eq!(
            decode_float(
                [0x3e, 0x00].view_bits::<Msb0>(),
                FloatBitSize::Sixteen,
                Endianness::Big,
            ),
            1.5,
        );
        assert_eq!(
            decode_float(
                [0x00, 0x3e].view_bits::<Msb0>(),
                FloatBitSize::Sixteen,
                Endianness::Little,
            ),
            1.5,
        );
        assert_eq!(
            decode_float(
                [0x3f, 0xc0, 0x00, 0x00].view_bits::<Msb0>(),
                FloatBitSize::ThirtyTwo,
                Endianness::Big,
            ),
            1.5,
        );
        assert_eq!(
            decode_float(
                [0x00, 0x00, 0xc0, 0x3f].view_bits::<Msb0>(),
                FloatBitSize::ThirtyTwo,
                Endianness::Little,
            ),
            1.5,
        );
        assert_eq!(
            decode_float(
                [0x3f, 0xf8, 0, 0, 0, 0, 0, 0].view_bits::<Msb0>(),
                FloatBitSize::SixtyFour,
                Endianness::Big,
            ),
            1.5,
        );
        assert_eq!(
            decode_float(
                [0, 0, 0, 0, 0, 0, 0xf8, 0x3f].view_bits::<Msb0>(),
                FloatBitSize::SixtyFour,
                Endianness::Little,
            ),
            1.5,
        );
    }

    #[test]
    fn codepoint_decoders_accept_scalars_and_reject_invalid_encoding() {
        assert_eq!(decode_utf8([].view_bits::<Msb0>()), None);
        assert_eq!(decode_utf8([b'A'].view_bits::<Msb0>()), Some(('A', 8)));
        assert_eq!(
            decode_utf8([0xc2, 0xa2].view_bits::<Msb0>()),
            Some(('¢', 16)),
        );
        assert_eq!(
            decode_utf8("안".as_bytes().view_bits::<Msb0>()),
            Some(('안', 24)),
        );
        assert_eq!(
            decode_utf8([0xf0, 0x9f, 0x98, 0x80].view_bits::<Msb0>()),
            Some(('😀', 32)),
        );
        assert_eq!(decode_utf8([0xc2].view_bits::<Msb0>()), None);
        assert_eq!(decode_utf8([0xc2, 0x20].view_bits::<Msb0>()), None);
        assert_eq!(decode_utf8([0xff].view_bits::<Msb0>()), None);
        assert_eq!(decode_utf16([].view_bits::<Msb0>(), Endianness::Big), None);
        assert_eq!(
            decode_utf16([0, 0x41].view_bits::<Msb0>(), Endianness::Big),
            Some(('A', 16)),
        );
        assert_eq!(
            decode_utf16([0x41, 0].view_bits::<Msb0>(), Endianness::Little),
            Some(('A', 16)),
        );
        assert_eq!(
            decode_utf16(
                [0xd8, 0x3d, 0xde, 0x00].view_bits::<Msb0>(),
                Endianness::Big,
            ),
            Some(('😀', 32)),
        );
        assert_eq!(
            decode_utf16([0xd8, 0x00].view_bits::<Msb0>(), Endianness::Big),
            None,
        );
        assert_eq!(
            decode_utf16(
                [0xd8, 0x00, 0x00, 0x41].view_bits::<Msb0>(),
                Endianness::Big,
            ),
            None,
        );
        assert_eq!(
            decode_utf16([0xdc, 0x00].view_bits::<Msb0>(), Endianness::Big),
            None,
        );
        assert_eq!(decode_utf32([].view_bits::<Msb0>(), Endianness::Big), None);
        assert_eq!(decode_utf32([0].view_bits::<Msb0>(), Endianness::Big), None);
        assert_eq!(
            decode_utf32([0, 0].view_bits::<Msb0>(), Endianness::Big),
            None,
        );
        assert_eq!(
            decode_utf32([0, 0, 0].view_bits::<Msb0>(), Endianness::Big),
            None,
        );
        assert_eq!(
            decode_utf32([0, 0, 0, 0x41].view_bits::<Msb0>(), Endianness::Big),
            Some(('A', 32)),
        );
        assert_eq!(
            decode_utf32([0x41, 0, 0, 0].view_bits::<Msb0>(), Endianness::Little),
            Some(('A', 32)),
        );
        assert_eq!(
            decode_utf32([0, 0x11, 0, 0].view_bits::<Msb0>(), Endianness::Big),
            None,
        );
    }

    #[test]
    fn bit_cursor_advances_only_for_available_ranges() {
        let bits = [0xaa].view_bits::<Msb0>();
        let mut cursor = 0;
        assert_eq!(take_bits(bits, &mut cursor, 4), bits.get(0..4));
        assert_eq!(cursor, 4);
        assert_eq!(take_bits(bits, &mut cursor, usize::MAX), None);
        assert_eq!(cursor, 4);
        assert_eq!(take_bits(bits, &mut cursor, 5), None);
        assert_eq!(cursor, 4);
    }

    fn expected_segment_values() -> Value {
        let bit_array = |bytes, bit_len| {
            let mut bits = BitVec::<u8, Msb0>::from_vec(bytes);
            bits.truncate(bit_len);
            Value::BitArray(BitArrayValue::from_evaluated(&bits))
        };

        Value::Tuple(vec![
            bit_array(vec![], 0),
            bit_array(vec![], 0),
            bit_array(vec![0x23, 0x40], 12),
            bit_array(vec![0x34, 0x20], 12),
            bit_array(vec![0xf0], 4),
            bit_array(vec![0x01], 8),
            bit_array(vec![0x3e, 0x00], 16),
            bit_array(vec![0x00, 0x3e], 16),
            bit_array(vec![0x3f, 0xc0, 0x00, 0x00], 32),
            bit_array(vec![0x00, 0x00, 0xc0, 0x3f], 32),
            bit_array(vec![0x3f, 0xf8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], 64),
            bit_array(vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf8, 0x3f], 64),
            bit_array(vec![0xec, 0x95, 0x88], 24),
            bit_array(vec![0xec, 0x95, 0x88], 24),
            bit_array(vec![0xc5, 0x48], 16),
            bit_array(vec![0x48, 0xc5], 16),
            bit_array(vec![0x00, 0x00, 0x00, 0x41], 32),
            bit_array(vec![0x41, 0x00, 0x00, 0x00], 32),
            bit_array(vec![0x12], 8),
        ])
    }
}
