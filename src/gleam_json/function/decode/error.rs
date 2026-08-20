use ecow::EcoString;
use jiter::{JiterError, JiterErrorType, JsonErrorType};

#[derive(Debug, PartialEq, Eq)]
pub(super) enum DecodeFailure {
    EndOfInput,
    Byte(EcoString),
    Sequence(EcoString),
}

impl DecodeFailure {
    pub(super) fn from_jiter(input: &[u8], error: JiterError) -> Self {
        match error.error_type {
            JiterErrorType::JsonError(
                JsonErrorType::EofWhileParsingList
                | JsonErrorType::EofWhileParsingObject
                | JsonErrorType::EofWhileParsingString
                | JsonErrorType::EofWhileParsingValue
                | JsonErrorType::UnexpectedEndOfHexEscape,
            ) => Self::EndOfInput,
            JiterErrorType::JsonError(
                JsonErrorType::InvalidEscape
                | JsonErrorType::InvalidUnicodeCodePoint
                | JsonErrorType::LoneLeadingSurrogateInHexEscape,
            ) => Self::Sequence(unicode_sequence(input, error.index)),
            _ => match input.get(error.index).copied() {
                Some(byte) => Self::Byte(format!("0x{byte:02X}").into()),
                None => Self::EndOfInput,
            },
        }
    }

    pub(super) fn overflow(number: &[u8]) -> Self {
        Self::Sequence(normalize_number(number))
    }
}

fn unicode_sequence(input: &[u8], index: usize) -> EcoString {
    let search_end = index.min(input.len());
    let start = (0..search_end)
        .rev()
        .find(|start| input[*start] == b'\\' && input.get(*start + 1) == Some(&b'u'))
        .unwrap_or(search_end);
    let end = (start + 6).min(input.len());
    String::from_utf8_lossy(&input[start..end])
        .into_owned()
        .into()
}

fn normalize_number(number: &[u8]) -> EcoString {
    let input = String::from_utf8_lossy(number);
    let normalized = input.replace('E', "e");
    let Some(exponent_index) = normalized.find('e') else {
        return normalized.into();
    };
    let (mantissa, exponent) = normalized.split_at(exponent_index);
    if mantissa.contains('.') {
        normalized.into()
    } else {
        format!("{mantissa}.0{exponent}").into()
    }
}

#[cfg(test)]
mod tests {
    use super::{DecodeFailure, normalize_number, unicode_sequence};
    use crate::gleam_json::test_support::{execution, run_state};
    use jiter::{JiterError, JiterErrorType, JsonErrorType};

    #[test]
    fn maps_jiter_errors_to_the_official_decode_error_families() {
        assert_eq!(
            DecodeFailure::from_jiter(
                b"[",
                JiterError {
                    error_type: JiterErrorType::JsonError(JsonErrorType::EofWhileParsingList),
                    index: 1,
                },
            ),
            DecodeFailure::EndOfInput,
        );
        assert_eq!(
            DecodeFailure::from_jiter(
                b"[}",
                JiterError {
                    error_type: JiterErrorType::JsonError(JsonErrorType::ExpectedSomeValue),
                    index: 1,
                },
            ),
            DecodeFailure::Byte("0x7D".into()),
        );
        assert_eq!(
            DecodeFailure::from_jiter(
                br#""\uxxxx""#,
                JiterError {
                    error_type: JiterErrorType::JsonError(JsonErrorType::InvalidEscape),
                    index: 4,
                },
            ),
            DecodeFailure::Sequence(r#"\uxxxx"#.into()),
        );
        assert_eq!(
            DecodeFailure::from_jiter(
                b"x",
                JiterError {
                    error_type: JiterErrorType::JsonError(JsonErrorType::ExpectedSomeValue),
                    index: 1,
                },
            ),
            DecodeFailure::EndOfInput,
        );
    }

    #[test]
    fn extracts_unicode_sequences_and_normalizes_overflow_numbers() {
        assert_eq!(unicode_sequence(br#""x\u12xx""#, 7), r#"\u12xx"#);
        assert_eq!(normalize_number(b"1e400"), "1.0e400");
        assert_eq!(normalize_number(b"-1E+400"), "-1.0e+400");
        assert_eq!(normalize_number(b"1.25e400"), "1.25e400");
        assert_eq!(normalize_number(b"123"), "123");
    }

    #[test]
    fn maps_every_json_parse_error_without_turning_it_into_a_host_failure() {
        let execution = execution(
            r#"
pub fn main() {
  #(
    decode_to_dynamic(<<>>),
    decode_to_dynamic(<<91>>),
    decode_to_dynamic(<<125>>),
    decode_to_dynamic(<<34, 92, 117, 120, 120, 120, 120, 34>>),
    decode_to_dynamic(<<34, 92, 117, 68, 56, 48, 48, 34>>),
    decode_to_dynamic(<<49, 101, 52, 48, 48>>),
    decode_to_dynamic(<<255>>),
    decode_to_dynamic(<<116, 114, 117, 101, 32, 102, 97, 108, 115, 101>>),
    decode_to_dynamic(<<1:size(1)>>),
    decode_to_dynamic(<<"nul":utf8>>),
    decode_to_dynamic(<<"tru":utf8>>),
    decode_to_dynamic(<<"{":utf8>>),
    decode_to_dynamic(<<"{\"a\":":utf8>>),
    decode_to_dynamic(<<"1e":utf8>>),
    decode_to_dynamic(<<"[true":utf8>>),
    decode_to_dynamic(<<"{\"a\":true":utf8>>),
    decode_to_dynamic(<<"{\"a\":true,\"b\":":utf8>>),
  )
}
"#,
        );
        let value = execution
            .run_main(&mut run_state([0; 32]), &mut Vec::new())
            .expect("malformed JSON should remain source-level DecodeError values");

        assert_eq!(
            value.inspect().to_string(),
            r#"#(Error(UnexpectedEndOfInput), Error(UnexpectedEndOfInput), Error(UnexpectedByte("0x7D")), Error(UnexpectedSequence("\\uxxxx")), Error(UnexpectedEndOfInput), Error(UnexpectedSequence("1.0e400")), Error(UnexpectedByte("0xFF")), Error(UnexpectedByte("0x66")), Error(UnexpectedByte("")), Error(UnexpectedEndOfInput), Error(UnexpectedEndOfInput), Error(UnexpectedEndOfInput), Error(UnexpectedEndOfInput), Error(UnexpectedEndOfInput), Error(UnexpectedEndOfInput), Error(UnexpectedEndOfInput), Error(UnexpectedEndOfInput))"#,
        );
    }
}
