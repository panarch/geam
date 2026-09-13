use crate::plan::execution::prepared::rust::{Emit, Rust};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endianness {
    Big,
    Little,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatBitSize {
    Sixteen,
    ThirtyTwo,
    SixtyFour,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringEncoding {
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

impl Emit for Endianness {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Big => output.path("graph::Endianness::Big"),
            Self::Little => output.path("graph::Endianness::Little"),
        }
    }
}

impl Emit for FloatBitSize {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Sixteen => output.path("graph::FloatBitSize::Sixteen"),
            Self::ThirtyTwo => output.path("graph::FloatBitSize::ThirtyTwo"),
            Self::SixtyFour => output.path("graph::FloatBitSize::SixtyFour"),
        }
    }
}

impl Emit for StringEncoding {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Utf8 => output.path("graph::StringEncoding::Utf8"),
            Self::Utf16(field_0) => output.call("graph::StringEncoding::Utf16", &[field_0]),
            Self::Utf32(field_0) => output.call("graph::StringEncoding::Utf32", &[field_0]),
        }
    }
}

#[cfg(test)]
mod emission_tests {
    use super::{Endianness, FloatBitSize, StringEncoding};
    use crate::plan::execution::prepared::rust::Rust;

    #[test]
    fn emits_bit_array_metadata_without_losing_width_or_byte_order() {
        for (value, expected) in [
            (Endianness::Big, "data::graph::Endianness::Big"),
            (Endianness::Little, "data::graph::Endianness::Little"),
        ] {
            assert_eq!(Rust::expression(&value), expected);
        }
        for (value, expected) in [
            (FloatBitSize::Sixteen, "data::graph::FloatBitSize::Sixteen"),
            (
                FloatBitSize::ThirtyTwo,
                "data::graph::FloatBitSize::ThirtyTwo",
            ),
            (
                FloatBitSize::SixtyFour,
                "data::graph::FloatBitSize::SixtyFour",
            ),
        ] {
            assert_eq!(Rust::expression(&value), expected);
        }
        for (value, expected) in [
            (StringEncoding::Utf8, "data::graph::StringEncoding::Utf8"),
            (
                StringEncoding::Utf16(Endianness::Big),
                "data::graph::StringEncoding::Utf16(data::graph::Endianness::Big,)",
            ),
            (
                StringEncoding::Utf16(Endianness::Little),
                "data::graph::StringEncoding::Utf16(data::graph::Endianness::Little,)",
            ),
            (
                StringEncoding::Utf32(Endianness::Big),
                "data::graph::StringEncoding::Utf32(data::graph::Endianness::Big,)",
            ),
            (
                StringEncoding::Utf32(Endianness::Little),
                "data::graph::StringEncoding::Utf32(data::graph::Endianness::Little,)",
            ),
        ] {
            assert_eq!(Rust::expression(&value), expected);
        }
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
