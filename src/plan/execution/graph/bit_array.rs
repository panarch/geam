#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Endianness {
    Big,
    Little,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StringEncoding {
    Utf8,
    Utf16(Endianness),
    Utf32(Endianness),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FloatBitSize {
    Sixteen,
    ThirtyTwo,
    SixtyFour,
}
