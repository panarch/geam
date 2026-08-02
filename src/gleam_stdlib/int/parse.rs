use ecow::EcoString;
use num_bigint::BigInt;
use num_traits::ToPrimitive;

pub(super) fn decimal(source: &EcoString) -> Option<BigInt> {
    parse_digits(source, 10, true)
}

pub(super) fn radix(source: &EcoString, base: &BigInt) -> Option<BigInt> {
    let base = base.to_u32().filter(|base| (2..=36).contains(base))?;
    parse_digits(source, base, false)
}

pub(super) fn format_radix(value: &BigInt, base: &BigInt) -> Option<EcoString> {
    let base = base.to_u32().filter(|base| (2..=36).contains(base))?;
    Some(value.to_str_radix(base).to_uppercase().into())
}

fn parse_digits(source: &EcoString, base: u32, allow_plus: bool) -> Option<BigInt> {
    let bytes = source.as_bytes();
    let digits = match bytes.first() {
        Some(b'-') => &bytes[1..],
        Some(b'+') if allow_plus => &bytes[1..],
        _ => bytes,
    };
    if digits.is_empty() || !digits.iter().all(|byte| (*byte as char).is_digit(base)) {
        return None;
    }
    BigInt::parse_bytes(bytes, base)
}

#[cfg(test)]
mod tests {
    use super::{decimal, format_radix, radix};
    use num_bigint::BigInt;

    #[test]
    fn parses_the_official_signed_decimal_grammar() {
        assert_eq!(decimal(&"0".into()), Some(BigInt::from(0)));
        assert_eq!(decimal(&"-12".into()), Some(BigInt::from(-12)));
        assert_eq!(decimal(&"+12".into()), Some(BigInt::from(12)));

        for source in ["", "+", "-", "1.0", " 1", "1 ", "12x"] {
            assert_eq!(decimal(&source.into()), None, "{source}");
        }
    }

    #[test]
    fn parses_and_formats_radices_from_two_through_thirty_six() {
        assert_eq!(radix(&"101".into(), &2.into()), Some(BigInt::from(5)));
        assert_eq!(radix(&"-1C".into(), &36.into()), Some(BigInt::from(-48)));
        assert_eq!(radix(&"ff".into(), &16.into()), Some(BigInt::from(255)));
        assert_eq!(radix(&"+10".into(), &2.into()), None);
        assert_eq!(radix(&"2".into(), &2.into()), None);
        assert_eq!(radix(&"".into(), &10.into()), None);
        assert_eq!(radix(&"10".into(), &1.into()), None);
        assert_eq!(radix(&"10".into(), &37.into()), None);
        assert_eq!(radix(&"10".into(), &BigInt::from(10u8).pow(100)), None);

        assert_eq!(format_radix(&255.into(), &16.into()), Some("FF".into()));
        assert_eq!(format_radix(&(-48).into(), &36.into()), Some("-1C".into()));
        assert_eq!(format_radix(&1.into(), &1.into()), None);
        assert_eq!(format_radix(&1.into(), &37.into()), None);
        assert_eq!(format_radix(&1.into(), &BigInt::from(10u8).pow(100)), None,);
    }
}
