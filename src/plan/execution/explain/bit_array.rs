use super::super::{Endianness, FloatBitSize, StringEncoding};

pub(super) fn endianness(value: Endianness) -> &'static str {
    match value {
        Endianness::Big => "big",
        Endianness::Little => "little",
    }
}

pub(super) fn float_size(value: FloatBitSize) -> usize {
    match value {
        FloatBitSize::Sixteen => 16,
        FloatBitSize::ThirtyTwo => 32,
        FloatBitSize::SixtyFour => 64,
    }
}

pub(super) fn string_encoding(value: StringEncoding) -> &'static str {
    match value {
        StringEncoding::Utf8 => "utf8",
        StringEncoding::Utf16(Endianness::Big) => "utf16.big",
        StringEncoding::Utf16(Endianness::Little) => "utf16.little",
        StringEncoding::Utf32(Endianness::Big) => "utf32.big",
        StringEncoding::Utf32(Endianness::Little) => "utf32.little",
    }
}
