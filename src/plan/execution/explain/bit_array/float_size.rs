use super::super::super::FloatBitSize;

pub(in super::super) fn float_size(value: FloatBitSize) -> usize {
    match value {
        FloatBitSize::Sixteen => 16,
        FloatBitSize::ThirtyTwo => 32,
        FloatBitSize::SixtyFour => 64,
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::super::FloatBitSize;

    #[test]
    fn writes_every_float_size() {
        assert_eq!(super::float_size(FloatBitSize::Sixteen), 16);
        assert_eq!(super::float_size(FloatBitSize::ThirtyTwo), 32);
        assert_eq!(super::float_size(FloatBitSize::SixtyFour), 64);
    }
}
