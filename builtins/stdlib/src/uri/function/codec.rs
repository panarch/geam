use ecow::EcoString;

const HEX_DIGITS: &[u8; 16] = b"0123456789ABCDEF";

pub(super) fn parse_query(query: &str) -> Option<Vec<(EcoString, EcoString)>> {
    if query.is_empty() {
        return Some(Vec::new());
    }

    query
        .split('&')
        .map(|section| {
            let (key, value) = section.split_once('=').unwrap_or((section, ""));
            Some((decode(key, true)?, decode(value, true)?))
        })
        .collect()
}

pub(super) fn percent_encode(value: &str) -> EcoString {
    let mut encoded = String::with_capacity(value.len());
    for byte in value.bytes() {
        if is_unescaped(byte) {
            encoded.push(char::from(byte));
        } else {
            encoded.push('%');
            encoded.push(char::from(HEX_DIGITS[usize::from(byte >> 4)]));
            encoded.push(char::from(HEX_DIGITS[usize::from(byte & 0x0f)]));
        }
    }
    encoded.into()
}

pub(super) fn percent_decode(value: &str) -> Option<EcoString> {
    decode(value, false)
}

fn decode(value: &str, plus_as_space: bool) -> Option<EcoString> {
    let input = value.as_bytes();
    let mut decoded = Vec::with_capacity(input.len());
    let mut index = 0;
    while index < input.len() {
        match input[index] {
            b'+' if plus_as_space => decoded.push(b' '),
            b'%' => {
                let high = hex_value(*input.get(index + 1)?)?;
                let low = hex_value(*input.get(index + 2)?)?;
                decoded.push((high << 4) | low);
                index += 2;
            }
            byte => decoded.push(byte),
        }
        index += 1;
    }

    String::from_utf8(decoded).ok().map(Into::into)
}

fn is_unescaped(byte: u8) -> bool {
    byte.is_ascii_alphanumeric()
        || matches!(
            byte,
            b'!' | b'$' | b'\'' | b'(' | b')' | b'*' | b'+' | b'-' | b'.' | b'_' | b'~'
        )
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_query, percent_decode, percent_encode};

    #[test]
    fn matches_the_official_erlang_percent_codec() {
        for (decoded, encoded) in [
            (
                "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
                "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
            ),
            ("!$'()*+-._~", "!$'()*+-._~"),
            (" ,;:?[]@/\\&#=", "%20%2C%3B%3A%3F%5B%5D%40%2F%5C%26%23%3D"),
            ("ñ", "%C3%B1"),
            ("100% great+fun", "100%25%20great+fun"),
        ] {
            assert_eq!(percent_encode(decoded), encoded);
            assert_eq!(percent_decode(encoded).as_deref(), Some(decoded));
        }
        assert_eq!(percent_decode("%c3%b1").as_deref(), Some("ñ"));
        assert_eq!(percent_decode("+").as_deref(), Some("+"));
    }

    #[test]
    fn rejects_malformed_percent_encoding_and_invalid_utf8() {
        for invalid in ["%", "%0", "%GG", "%0G", "%C2", "%FF"] {
            assert_eq!(percent_decode(invalid), None, "{invalid:?}");
        }
    }

    #[test]
    fn parses_official_erlang_query_segments_in_source_order() {
        for (query, expected) in [
            ("", vec![]),
            ("a", vec![("a", "")]),
            ("=x", vec![("", "x")]),
            ("a=", vec![("a", "")]),
            ("a=b=c", vec![("a", "b=c")]),
            ("&&", vec![("", ""), ("", ""), ("", "")]),
            ("a[]=1&a[]=2", vec![("a[]", "1"), ("a[]", "2")]),
            ("one+two=three+four", vec![("one two", "three four")]),
        ] {
            assert_eq!(
                parse_query(query),
                Some(
                    expected
                        .into_iter()
                        .map(|(key, value)| (key.into(), value.into()))
                        .collect(),
                ),
                "{query:?}",
            );
        }
    }

    #[test]
    fn rejects_query_when_any_component_is_invalid() {
        for invalid in ["%C2=value", "key=%", "ok=1&bad=%GG"] {
            assert_eq!(parse_query(invalid), None, "{invalid:?}");
        }
    }
}
