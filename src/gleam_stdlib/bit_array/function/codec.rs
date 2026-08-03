use super::super::schema::{BitArrayError, BitArrayOk, BitArrayResult};
use super::BitArrayProvider;
use crate::gleam_stdlib::GleamStdlibHostProfile;
use crate::{BitArrayValue, HostCall, HostCallCompletion, HostCallError};
use base64::Engine;
use base64::alphabet;
use base64::engine::general_purpose::{
    GeneralPurpose, GeneralPurposeConfig, STANDARD, STANDARD_NO_PAD,
};
use ecow::EcoString;

const ERLANG_BASE64: GeneralPurpose = GeneralPurpose::new(
    &alphabet::STANDARD,
    GeneralPurposeConfig::new().with_decode_allow_trailing_bits(true),
);

pub(in crate::gleam_stdlib::bit_array) fn base64_encode(
    value: BitArrayValue,
    padding: bool,
) -> EcoString {
    let value = value.pad_to_bytes();
    if padding {
        STANDARD.encode(value.bytes()).into()
    } else {
        STANDARD_NO_PAD.encode(value.bytes()).into()
    }
}

pub(in crate::gleam_stdlib::bit_array) fn decode64<'call, Profile>(
    call: HostCall<'call, Profile, BitArrayProvider<Profile>, BitArrayResult>,
    value: EcoString,
) -> Result<HostCallCompletion<'call, BitArrayResult>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    Ok(match decode_base64(&value) {
        Ok(bytes) => call.return_custom::<BitArrayOk>((BitArrayValue::from_bytes(bytes), ())),
        Err(_) => call.return_custom::<BitArrayError>(((), ())),
    })
}

pub(in crate::gleam_stdlib::bit_array) fn base16_encode(value: BitArrayValue) -> EcoString {
    hex::encode_upper(value.pad_to_bytes().bytes()).into()
}

pub(in crate::gleam_stdlib::bit_array) fn base16_decode<'call, Profile>(
    call: HostCall<'call, Profile, BitArrayProvider<Profile>, BitArrayResult>,
    value: EcoString,
) -> Result<HostCallCompletion<'call, BitArrayResult>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    Ok(match hex::decode(value.as_bytes()) {
        Ok(bytes) => call.return_custom::<BitArrayOk>((BitArrayValue::from_bytes(bytes), ())),
        Err(_) => call.return_custom::<BitArrayError>(((), ())),
    })
}

fn decode_base64(value: &str) -> Result<Vec<u8>, base64::DecodeError> {
    let encoded = value
        .bytes()
        .filter(|byte| !matches!(byte, b' ' | b'\t' | b'\r' | b'\n'))
        .collect::<Vec<_>>();
    ERLANG_BASE64.decode(encoded)
}

#[cfg(test)]
mod tests {
    use super::{base16_encode, base64_encode, decode_base64};
    use crate::BitArrayValue;

    #[test]
    fn matches_erlang_base64_whitespace_and_trailing_bit_acceptance() {
        assert_eq!(decode_base64("aG  \t\nVsbG8="), Ok(b"hello".to_vec()));
        assert_eq!(decode_base64("AB=="), Ok(vec![0]));
        for invalid in [
            "=", "A===", "AAAA====", "AA=A", "AA==junk", "A", "AA", "AAA",
        ] {
            assert!(decode_base64(invalid).is_err(), "{invalid:?}");
        }
        assert!(decode_base64("aG\u{000c}VsbG8=").is_err());
        assert!(decode_base64("aG\u{000b}VsbG8=").is_err());
    }

    #[test]
    fn zero_pads_unaligned_values_for_encoding() {
        let value = BitArrayValue::try_from_parts(vec![0b1010_0000], 3)
            .expect("three supplied bits should be valid");

        assert_eq!(base64_encode(value.clone(), true), "oA==");
        assert_eq!(base64_encode(value.clone(), false), "oA");
        assert_eq!(base16_encode(value), "A0");
    }
}
