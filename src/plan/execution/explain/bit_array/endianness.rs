use super::super::super::Endianness;

pub(in super::super) fn endianness(value: Endianness) -> &'static str {
    match value {
        Endianness::Big => "big",
        Endianness::Little => "little",
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::super::Endianness;

    #[test]
    fn writes_every_endianness_token() {
        assert_eq!(super::endianness(Endianness::Big), "big");
        assert_eq!(super::endianness(Endianness::Little), "little");
    }
}
