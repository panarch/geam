use ecow::EcoString;

pub(super) fn parse_literal(source: &EcoString) -> Option<f64> {
    let bytes = source.as_bytes();
    let mut index = usize::from(matches!(bytes.first(), Some(b'+' | b'-')));
    let integer_start = index;
    while bytes.get(index).is_some_and(u8::is_ascii_digit) {
        index += 1;
    }
    if index == integer_start || bytes.get(index) != Some(&b'.') {
        return None;
    }

    index += 1;
    let fraction_start = index;
    while bytes.get(index).is_some_and(u8::is_ascii_digit) {
        index += 1;
    }
    if index == fraction_start {
        return None;
    }

    if matches!(bytes.get(index), Some(b'e' | b'E')) {
        index += 1;
        if matches!(bytes.get(index), Some(b'+' | b'-')) {
            index += 1;
        }
        let exponent_start = index;
        while bytes.get(index).is_some_and(u8::is_ascii_digit) {
            index += 1;
        }
        if index == exponent_start {
            return None;
        }
    }

    (index == bytes.len())
        .then(|| source.parse::<f64>().ok())
        .flatten()
}

pub(super) fn format(value: f64) -> EcoString {
    let mut output = format!("{value:?}");
    if let Some(exponent) = output.find('e')
        && !output[..exponent].contains('.')
    {
        output.insert_str(exponent, ".0");
    }
    output.into()
}

#[cfg(test)]
mod tests {
    use super::{format, parse_literal};

    #[test]
    fn parses_only_the_official_decimal_and_exponent_grammar() {
        let accepted = [
            ("0.0", 0.0),
            ("-0.0", -0.0),
            ("+12.5", 12.5),
            ("1.25e3", 1250.0),
            ("1.25E-2", 0.0125),
        ];
        for (source, expected) in accepted {
            assert_eq!(parse_literal(&source.into()), Some(expected));
        }

        for source in [
            "", "+", "1", ".5", "1.", "1.0e", "1.0e+", "1.0x", " 1.0", "1.0 ", "NaN", "inf",
        ] {
            assert_eq!(parse_literal(&source.into()), None, "{source}");
        }
    }

    #[test]
    fn formats_floats_with_explicit_float_mantissas() {
        assert_eq!(format(1.0), "1.0");
        assert_eq!(format(-0.0), "-0.0");
        assert_eq!(format(1e100), "1.0e100");
        assert_eq!(format(1.5e100), "1.5e100");
        assert_eq!(format(f64::INFINITY), "inf");
        assert_eq!(format(f64::NEG_INFINITY), "-inf");
        assert_eq!(format(f64::NAN), "NaN");
    }
}
