use bitvec::order::Msb0;
use bitvec::slice::BitSlice;
use bitvec::vec::BitVec;
use bitvec::view::BitView;
use ecow::EcoString;
use num_bigint::BigInt;

use crate::plan::execution::{
    BitArrayBindingPattern, BitArrayPattern, BitArrayPatternSegment, BitArrayPatternSize,
    BitArrayPatternSizeExpr, BitArrayPatternValue, BitArrayStringPattern, Endianness, Signedness,
    StringEncoding,
};
use crate::runtime::evaluated::EvaluatedBitArray;
use crate::runtime::frame::Frame;

pub(in crate::runtime) fn match_bit_array_pattern(
    frame: &mut Frame,
    subject: &EvaluatedBitArray,
    pattern: &BitArrayPattern,
) -> bool {
    let mut cursor = 0;
    for segment in pattern.segments() {
        let matched = match segment {
            BitArrayPatternSegment::Int {
                pattern,
                size,
                endianness,
                signedness,
            } => match_int_segment(
                frame,
                subject.bits(),
                &mut cursor,
                pattern,
                size,
                *endianness,
                *signedness,
            ),
            BitArrayPatternSegment::Float {
                pattern,
                size,
                endianness,
            } => match_float_segment(
                frame,
                subject.bits(),
                &mut cursor,
                pattern,
                size,
                *endianness,
            ),
            BitArrayPatternSegment::Bits {
                pattern,
                size,
                unit,
            } => match_bits_segment(
                frame,
                subject.bits(),
                &mut cursor,
                pattern,
                size.as_ref(),
                *unit,
            ),
            BitArrayPatternSegment::String { pattern, encoding } => {
                match_string_segment(subject.bits(), &mut cursor, pattern, *encoding)
            }
        };
        if !matched {
            return false;
        }
    }

    cursor == subject.bits().len()
}

#[allow(clippy::too_many_arguments)]
fn match_int_segment(
    frame: &mut Frame,
    subject: &BitSlice<u8, Msb0>,
    cursor: &mut usize,
    pattern: &BitArrayPatternValue<BigInt, crate::plan::execution::IntLocalId>,
    size: &BitArrayPatternSize,
    endianness: Endianness,
    signedness: Signedness,
) -> bool {
    let Some(bit_size) = eval_size(frame, size) else {
        return false;
    };
    let Some(bits) = take_bits(subject, cursor, bit_size) else {
        return false;
    };
    let value = decode_integer(bits, endianness, signedness);
    match_int_pattern(frame, pattern, &value)
}

#[allow(clippy::too_many_arguments)]
fn match_float_segment(
    frame: &mut Frame,
    subject: &BitSlice<u8, Msb0>,
    cursor: &mut usize,
    pattern: &BitArrayPatternValue<f64, crate::plan::execution::FloatLocalId>,
    size: &BitArrayPatternSize,
    endianness: Endianness,
) -> bool {
    let Some(bit_size) = eval_size(frame, size) else {
        return false;
    };
    let width = match bit_size {
        16 => FloatWidth::Sixteen,
        32 => FloatWidth::ThirtyTwo,
        64 => FloatWidth::SixtyFour,
        _ => return false,
    };
    let Some(bits) = take_bits(subject, cursor, width.bit_size()) else {
        return false;
    };
    let value = decode_float(bits, width, endianness);
    match_float_pattern(frame, pattern, value)
}

fn match_bits_segment(
    frame: &mut Frame,
    subject: &BitSlice<u8, Msb0>,
    cursor: &mut usize,
    pattern: &BitArrayBindingPattern<crate::plan::execution::BitArrayLocalId>,
    size: Option<&BitArrayPatternSize>,
    unit: u8,
) -> bool {
    let bit_size = match size {
        Some(size) => {
            let Some(size) = eval_size(frame, size) else {
                return false;
            };
            size
        }
        None => {
            let remaining = subject.len() - *cursor;
            if !remaining.is_multiple_of(usize::from(unit)) {
                return false;
            }
            remaining
        }
    };
    let Some(bits) = take_bits(subject, cursor, bit_size) else {
        return false;
    };
    let value = EvaluatedBitArray::new(BitVec::from_bitslice(bits));
    bind_bits_pattern(frame, pattern, &value);
    true
}

fn match_string_segment(
    subject: &BitSlice<u8, Msb0>,
    cursor: &mut usize,
    pattern: &BitArrayStringPattern,
    encoding: StringEncoding,
) -> bool {
    match pattern {
        BitArrayStringPattern::Literal(literal) => {
            let encoded = encode_string(literal, encoding);
            let Some(bits) = take_bits(subject, cursor, encoded.len()) else {
                return false;
            };
            if bits != encoded.as_bitslice() {
                return false;
            }
            true
        }
        BitArrayStringPattern::Discard => {
            let Some((_, bit_size)) = decode_one_string(&subject[*cursor..], encoding) else {
                return false;
            };
            *cursor += bit_size;
            true
        }
    }
}

fn eval_size(frame: &Frame, size: &BitArrayPatternSize) -> Option<usize> {
    let value = eval_size_expr(frame, size.value());
    let Ok(value) = usize::try_from(value) else {
        return None;
    };
    if value == 0 {
        return None;
    }
    value.checked_mul(usize::from(size.unit()))
}

fn eval_size_expr(frame: &Frame, expression: &BitArrayPatternSizeExpr) -> BigInt {
    match expression {
        BitArrayPatternSizeExpr::Value(value) => value.clone(),
        BitArrayPatternSizeExpr::LocalGet(local) => frame.get_int(*local),
        BitArrayPatternSizeExpr::Add { left, right } => {
            eval_size_expr(frame, left) + eval_size_expr(frame, right)
        }
        BitArrayPatternSizeExpr::Subtract { left, right } => {
            eval_size_expr(frame, left) - eval_size_expr(frame, right)
        }
        BitArrayPatternSizeExpr::Multiply { left, right } => {
            eval_size_expr(frame, left) * eval_size_expr(frame, right)
        }
        BitArrayPatternSizeExpr::Divide { left, right } => {
            let right = eval_size_expr(frame, right);
            if right == BigInt::from(0) {
                BigInt::from(0)
            } else {
                eval_size_expr(frame, left) / right
            }
        }
        BitArrayPatternSizeExpr::Remainder { left, right } => {
            let right = eval_size_expr(frame, right);
            if right == BigInt::from(0) {
                BigInt::from(0)
            } else {
                eval_size_expr(frame, left) % right
            }
        }
    }
}

fn take_bits<'a>(
    subject: &'a BitSlice<u8, Msb0>,
    cursor: &mut usize,
    bit_size: usize,
) -> Option<&'a BitSlice<u8, Msb0>> {
    let end = cursor.checked_add(bit_size)?;
    let bits = subject.get(*cursor..end)?;
    *cursor = end;
    Some(bits)
}

fn decode_integer(
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

#[derive(Clone, Copy)]
enum FloatWidth {
    Sixteen,
    ThirtyTwo,
    SixtyFour,
}

impl FloatWidth {
    fn bit_size(self) -> usize {
        match self {
            Self::Sixteen => 16,
            Self::ThirtyTwo => 32,
            Self::SixtyFour => 64,
        }
    }
}

fn decode_float(bits: &BitSlice<u8, Msb0>, width: FloatWidth, endianness: Endianness) -> f64 {
    let mut bytes = vec![0u8; bits.len() / 8];
    bytes.view_bits_mut::<Msb0>().copy_from_bitslice(bits);
    match (width, endianness) {
        (FloatWidth::Sixteen, Endianness::Big) => {
            half::f16::from_be_bytes([bytes[0], bytes[1]]).to_f64()
        }
        (FloatWidth::Sixteen, Endianness::Little) => {
            half::f16::from_le_bytes([bytes[0], bytes[1]]).to_f64()
        }
        (FloatWidth::ThirtyTwo, Endianness::Big) => {
            f32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as f64
        }
        (FloatWidth::ThirtyTwo, Endianness::Little) => {
            f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as f64
        }
        (FloatWidth::SixtyFour, Endianness::Big) => f64::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]),
        (FloatWidth::SixtyFour, Endianness::Little) => f64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]),
    }
}

fn encode_string(value: &str, encoding: StringEncoding) -> BitVec<u8, Msb0> {
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

fn decode_one_string(
    bits: &BitSlice<u8, Msb0>,
    encoding: StringEncoding,
) -> Option<(EcoString, usize)> {
    match encoding {
        StringEncoding::Utf8 => decode_utf8(bits),
        StringEncoding::Utf16(endianness) => decode_utf16(bits, endianness),
        StringEncoding::Utf32(endianness) => decode_utf32(bits, endianness),
    }
}

fn decode_utf8(bits: &BitSlice<u8, Msb0>) -> Option<(EcoString, usize)> {
    let first = read_byte(bits, 0)?;
    let bytes = match first {
        0x00..=0x7f => 1,
        0xc2..=0xdf => 2,
        0xe0..=0xef => 3,
        0xf0..=0xf4 => 4,
        _ => return None,
    };
    let mut encoded = Vec::with_capacity(bytes);
    for index in 0..bytes {
        encoded.push(read_byte(bits, index * 8)?);
    }
    let Ok(value) = std::str::from_utf8(&encoded) else {
        return None;
    };
    Some((value.into(), bytes * 8))
}

fn decode_utf16(bits: &BitSlice<u8, Msb0>, endianness: Endianness) -> Option<(EcoString, usize)> {
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
    let character = char::from_u32(codepoint)?;
    Some((character.to_string().into(), bit_size))
}

fn decode_utf32(bits: &BitSlice<u8, Msb0>, endianness: Endianness) -> Option<(EcoString, usize)> {
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
    Some((char::from_u32(value)?.to_string().into(), 32))
}

fn read_u16(bits: &BitSlice<u8, Msb0>, offset: usize, endianness: Endianness) -> Option<u16> {
    let bytes = [read_byte(bits, offset)?, read_byte(bits, offset + 8)?];
    Some(match endianness {
        Endianness::Big => u16::from_be_bytes(bytes),
        Endianness::Little => u16::from_le_bytes(bytes),
    })
}

fn read_byte(bits: &BitSlice<u8, Msb0>, offset: usize) -> Option<u8> {
    let end = offset.checked_add(8)?;
    let bits = bits.get(offset..end)?;
    let mut byte = 0;
    for bit in bits {
        byte = (byte << 1) | u8::from(*bit);
    }
    Some(byte)
}

fn match_int_pattern(
    frame: &mut Frame,
    pattern: &BitArrayPatternValue<BigInt, crate::plan::execution::IntLocalId>,
    value: &BigInt,
) -> bool {
    match pattern {
        BitArrayPatternValue::Literal(expected) => expected == value,
        BitArrayPatternValue::Bind(binding) => {
            frame.set_int(*binding.local(), value.clone());
            true
        }
        BitArrayPatternValue::Discard => true,
        BitArrayPatternValue::Alias { pattern, binding } => {
            if !match_int_pattern(frame, pattern, value) {
                return false;
            }
            frame.set_int(*binding.local(), value.clone());
            true
        }
    }
}

fn match_float_pattern(
    frame: &mut Frame,
    pattern: &BitArrayPatternValue<f64, crate::plan::execution::FloatLocalId>,
    value: f64,
) -> bool {
    match pattern {
        BitArrayPatternValue::Literal(expected) => *expected == value,
        BitArrayPatternValue::Bind(binding) => {
            frame.set_float(*binding.local(), value);
            true
        }
        BitArrayPatternValue::Discard => true,
        BitArrayPatternValue::Alias { pattern, binding } => {
            if !match_float_pattern(frame, pattern, value) {
                return false;
            }
            frame.set_float(*binding.local(), value);
            true
        }
    }
}

fn bind_bits_pattern(
    frame: &mut Frame,
    pattern: &BitArrayBindingPattern<crate::plan::execution::BitArrayLocalId>,
    value: &EvaluatedBitArray,
) {
    match pattern {
        BitArrayBindingPattern::Bind(binding) => {
            frame.set_bit_array(*binding.local(), value.clone());
        }
        BitArrayBindingPattern::Discard => {}
        BitArrayBindingPattern::Alias { pattern, binding } => {
            bind_bits_pattern(frame, pattern, value);
            frame.set_bit_array(*binding.local(), value.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        FloatWidth, decode_float, decode_integer, decode_utf8, decode_utf16, decode_utf32,
        read_byte, read_u16, take_bits,
    };
    use crate::plan::execution::{Endianness, Signedness};
    use crate::runtime::Value;
    use bitvec::order::Msb0;
    use bitvec::view::BitView;

    #[test]
    fn source_matcher_handles_every_segment_binding_family_and_miss() {
        let source = r#"
pub fn main() {
  #(
    case <<1>> {
      <<value>> -> value
      _ -> 0
    },
    case <<1>> {
      <<1 as alias>> -> alias
      _ -> 0
    },
    case <<1>> {
      <<_>> -> True
      _ -> False
    },
    case <<1.5:float-size(16)>> {
      <<value:float-size(16)>> -> value
      _ -> 0.0
    },
    case <<1.5:float-size(16)>> {
      <<1.5 as alias:float-size(16)>> -> alias
      _ -> 0.0
    },
    case <<1.5:float-size(16)>> {
      <<_:float-size(16)>> -> True
      _ -> False
    },
    case <<1, 2>> {
      <<first:bytes-size(1), _ as rest:bits>> -> #(first, rest)
      _ -> #(<<>>, <<>>)
    },
    case <<"A":utf16-big>> {
      <<_:utf16-big>> -> True
      _ -> False
    },
    case <<"A":utf8>> {
      <<"A":utf8>> -> True
      _ -> False
    },
    case <<255>> {
      <<_:utf8>> -> True
      _ -> False
    },
  )
}
"#;

        assert_eq!(
            crate::runtime::run_src(source),
            Value::Tuple(vec![
                Value::Int(1.into()),
                Value::Int(1.into()),
                Value::Bool(true),
                Value::Float(1.5),
                Value::Float(1.5),
                Value::Bool(true),
                Value::Tuple(vec![
                    Value::BitArray(crate::BitArrayValue::from_bytes(vec![1])),
                    Value::BitArray(crate::BitArrayValue::from_bytes(vec![2])),
                ]),
                Value::Bool(true),
                Value::Bool(true),
                Value::Bool(false),
            ]),
        );
    }

    #[test]
    fn source_matcher_evaluates_every_size_operator_and_boundary() {
        assert_eq!(
            crate::runtime::run_src(include_str!(
                "../../../tests/fixtures/execution/control_flow/case/bit_array_pattern_integers.gleam"
            )),
            Value::Tuple(vec![
                Value::Int((-2).into()),
                Value::Int(4094.into()),
                Value::Int(564.into()),
                Value::Int(564.into()),
                Value::Int(564.into()),
                Value::Int(0.into()),
                Value::Int(0.into()),
                Value::Int(0.into()),
                Value::Int(0.into()),
                Value::Int(0.into()),
                Value::Int(15.into()),
            ]),
        );
        assert_eq!(
            crate::runtime::run_src(
                r#"
pub fn main() {
  #(
    case <<>> {
      <<_:bits-size(1 / 0)>> -> 1
      _ -> 0
    },
    case <<>> {
      <<_:bits-size(1 % 0)>> -> 1
      _ -> 0
    },
  )
}
"#,
            ),
            Value::Tuple(vec![Value::Int(0.into()), Value::Int(0.into())]),
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
        assert_eq!(FloatWidth::Sixteen.bit_size(), 16);
        assert_eq!(FloatWidth::ThirtyTwo.bit_size(), 32);
        assert_eq!(FloatWidth::SixtyFour.bit_size(), 64);
        assert_eq!(
            decode_float(
                [0x3e, 0x00].view_bits::<Msb0>(),
                FloatWidth::Sixteen,
                Endianness::Big,
            ),
            1.5,
        );
        assert_eq!(
            decode_float(
                [0x00, 0x3e].view_bits::<Msb0>(),
                FloatWidth::Sixteen,
                Endianness::Little,
            ),
            1.5,
        );
        assert_eq!(
            decode_float(
                [0x3f, 0xc0, 0x00, 0x00].view_bits::<Msb0>(),
                FloatWidth::ThirtyTwo,
                Endianness::Big,
            ),
            1.5,
        );
        assert_eq!(
            decode_float(
                [0x00, 0x00, 0xc0, 0x3f].view_bits::<Msb0>(),
                FloatWidth::ThirtyTwo,
                Endianness::Little,
            ),
            1.5,
        );
        assert_eq!(
            decode_float(
                [0x3f, 0xf8, 0, 0, 0, 0, 0, 0].view_bits::<Msb0>(),
                FloatWidth::SixtyFour,
                Endianness::Big,
            ),
            1.5,
        );
        assert_eq!(
            decode_float(
                [0, 0, 0, 0, 0, 0, 0xf8, 0x3f].view_bits::<Msb0>(),
                FloatWidth::SixtyFour,
                Endianness::Little,
            ),
            1.5,
        );
    }

    #[test]
    fn string_decoders_accept_one_scalar_and_reject_invalid_encoding() {
        assert_eq!(decode_utf8([].view_bits::<Msb0>()), None);
        assert_eq!(
            decode_utf8([b'A'].view_bits::<Msb0>()),
            Some(("A".into(), 8)),
        );
        assert_eq!(
            decode_utf8([0xc2, 0xa2].view_bits::<Msb0>()),
            Some(("¢".into(), 16)),
        );
        assert_eq!(
            decode_utf8("안".as_bytes().view_bits::<Msb0>()),
            Some(("안".into(), 24)),
        );
        assert_eq!(
            decode_utf8([0xf0, 0x9f, 0x98, 0x80].view_bits::<Msb0>()),
            Some(("😀".into(), 32)),
        );
        assert_eq!(decode_utf8([0xc2].view_bits::<Msb0>()), None);
        assert_eq!(decode_utf8([0xc2, 0x20].view_bits::<Msb0>()), None);
        assert_eq!(decode_utf8([0xff].view_bits::<Msb0>()), None);
        assert_eq!(decode_utf16([].view_bits::<Msb0>(), Endianness::Big), None);
        assert_eq!(
            decode_utf16([0, 0x41].view_bits::<Msb0>(), Endianness::Big),
            Some(("A".into(), 16)),
        );
        assert_eq!(
            decode_utf16([0x41, 0].view_bits::<Msb0>(), Endianness::Little),
            Some(("A".into(), 16)),
        );
        assert_eq!(
            decode_utf16(
                [0xd8, 0x3d, 0xde, 0x00].view_bits::<Msb0>(),
                Endianness::Big
            ),
            Some(("😀".into(), 32)),
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
        assert_eq!(decode_utf32([0].view_bits::<Msb0>(), Endianness::Big), None,);
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
            Some(("A".into(), 32)),
        );
        assert_eq!(
            decode_utf32([0x41, 0, 0, 0].view_bits::<Msb0>(), Endianness::Little),
            Some(("A".into(), 32)),
        );
        assert_eq!(
            decode_utf32([0, 0x11, 0, 0].view_bits::<Msb0>(), Endianness::Big),
            None,
        );
        assert_eq!(read_u16([0].view_bits::<Msb0>(), 0, Endianness::Big), None);
        assert_eq!(
            read_u16([0x41, 0].view_bits::<Msb0>(), 0, Endianness::Little),
            Some(0x41),
        );
        assert_eq!(read_byte([0].view_bits::<Msb0>(), usize::MAX), None);
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
}
