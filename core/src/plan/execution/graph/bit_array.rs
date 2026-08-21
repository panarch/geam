#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Endianness {
    Big,
    Little,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FloatBitSize {
    Sixteen,
    ThirtyTwo,
    SixtyFour,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StringEncoding {
    Utf8,
    Utf16(Endianness),
    Utf32(Endianness),
}

pub(in crate::plan::execution::graph) fn endianness(value: Endianness) -> &'static str {
    match value {
        Endianness::Big => "big",
        Endianness::Little => "little",
    }
}

pub(in crate::plan::execution::graph) fn float_size(value: FloatBitSize) -> usize {
    match value {
        FloatBitSize::Sixteen => 16,
        FloatBitSize::ThirtyTwo => 32,
        FloatBitSize::SixtyFour => 64,
    }
}

pub(in crate::plan::execution::graph) fn string_encoding(value: StringEncoding) -> &'static str {
    match value {
        StringEncoding::Utf8 => "utf8",
        StringEncoding::Utf16(Endianness::Big) => "utf16.big",
        StringEncoding::Utf16(Endianness::Little) => "utf16.little",
        StringEncoding::Utf32(Endianness::Big) => "utf32.big",
        StringEncoding::Utf32(Endianness::Little) => "utf32.little",
    }
}

#[cfg(test)]
mod explain_tests {
    use super::{Endianness, FloatBitSize, StringEncoding};

    #[test]
    fn writes_every_bit_array_metadata_token() {
        assert_eq!(super::endianness(Endianness::Big), "big");
        assert_eq!(super::endianness(Endianness::Little), "little");
        assert_eq!(super::float_size(FloatBitSize::Sixteen), 16);
        assert_eq!(super::float_size(FloatBitSize::ThirtyTwo), 32);
        assert_eq!(super::float_size(FloatBitSize::SixtyFour), 64);
        assert_eq!(super::string_encoding(StringEncoding::Utf8), "utf8");
        assert_eq!(
            super::string_encoding(StringEncoding::Utf16(Endianness::Big)),
            "utf16.big",
        );
        assert_eq!(
            super::string_encoding(StringEncoding::Utf16(Endianness::Little)),
            "utf16.little",
        );
        assert_eq!(
            super::string_encoding(StringEncoding::Utf32(Endianness::Big)),
            "utf32.big",
        );
        assert_eq!(
            super::string_encoding(StringEncoding::Utf32(Endianness::Little)),
            "utf32.little",
        );
    }
}
