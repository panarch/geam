use super::super::super::{Endianness, StringEncoding};

pub(in super::super) fn string_encoding(value: StringEncoding) -> &'static str {
    match value {
        StringEncoding::Utf8 => "utf8",
        StringEncoding::Utf16(Endianness::Big) => "utf16.big",
        StringEncoding::Utf16(Endianness::Little) => "utf16.little",
        StringEncoding::Utf32(Endianness::Big) => "utf32.big",
        StringEncoding::Utf32(Endianness::Little) => "utf32.little",
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::super::{Endianness, StringEncoding};

    #[test]
    fn writes_every_string_encoding_token() {
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
