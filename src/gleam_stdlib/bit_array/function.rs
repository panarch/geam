mod codec;
mod slice;

pub(super) use self::codec::{base16_decode, base16_encode, base64_encode, decode64};
pub(super) use self::slice::slice;

use super::schema::IntPair;
use crate::gleam_stdlib::{GleamStdlibHostProfile, GleamStdlibRunState, stdlib_state};
use crate::{
    BitArrayValue, HostCall, HostCallCompletion, HostCallError, HostFailure, HostList, HostProvider,
};
use bitvec::order::Msb0;
use bitvec::vec::BitVec;
use ecow::EcoString;
use geam_core::provider_support::{bit_array_bits, bit_array_from_bits, bit_array_pad_to_bytes};
use num_bigint::{BigInt, Sign};
use std::marker::PhantomData;

pub(super) struct BitArrayProvider<Profile>(PhantomData<Profile>);

impl<Profile> HostProvider<Profile> for BitArrayProvider<Profile>
where
    Profile: GleamStdlibHostProfile,
{
    type State = GleamStdlibRunState<Profile::Io>;

    fn project(state: &mut Profile::RunState) -> &mut Self::State {
        stdlib_state::<Profile>(state)
    }
}

pub(super) fn from_string(value: EcoString) -> BitArrayValue {
    BitArrayValue::from_bytes(value.as_bytes().to_vec())
}

pub(super) fn bit_size(value: BitArrayValue) -> BigInt {
    BigInt::from(value.bit_len())
}

pub(super) fn byte_size(value: BitArrayValue) -> BigInt {
    BigInt::from(value.bytes().len())
}

pub(super) fn pad_to_bytes(value: BitArrayValue) -> BitArrayValue {
    bit_array_pad_to_bytes(&value)
}

pub(super) fn unsafe_to_string(value: BitArrayValue) -> Result<EcoString, HostFailure> {
    if !value.bit_len().is_multiple_of(8) {
        return Err(HostFailure::new("bit array is not byte-aligned UTF-8"));
    }
    std::str::from_utf8(value.bytes())
        .map(EcoString::from)
        .map_err(|_| HostFailure::new("bit array is not valid UTF-8"))
}

pub(super) fn concat<'call, Profile>(
    mut call: HostCall<'call, Profile, BitArrayProvider<Profile>, BitArrayValue>,
    values: HostList<'call, BitArrayValue>,
) -> Result<HostCallCompletion<'call, BitArrayValue>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    let mut bits = BitVec::<u8, Msb0>::new();
    let mut index = 0;
    while let Some(value) = call.list_item(values, index) {
        bits.extend_from_bitslice(bit_array_bits(&value));
        index += 1;
    }
    Ok(call.return_value(bit_array_from_bits(bits)))
}

pub(super) fn bit_array_to_int_and_size<'call, Profile>(
    call: HostCall<'call, Profile, BitArrayProvider<Profile>, IntPair>,
    value: BitArrayValue,
) -> Result<HostCallCompletion<'call, IntPair>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    let unused_bits = value.bytes().len() * 8 - value.bit_len();
    let integer = BigInt::from_bytes_be(Sign::Plus, value.bytes()) >> unused_bits;
    Ok(call.return_tuple((integer, (BigInt::from(value.bit_len()), ()))))
}

#[cfg(test)]
mod tests {
    use super::{
        BitArrayProvider, bit_size, byte_size, from_string, pad_to_bytes, unsafe_to_string,
    };
    use crate::gleam_stdlib::{GleamStdlibProfile, GleamStdlibRunState};
    use crate::{BitArrayValue, HostProvider};
    use num_bigint::BigInt;

    #[test]
    fn projects_the_stdlib_run_state() {
        let mut state = GleamStdlibRunState::from_seed([0; 32]);
        let original = std::ptr::from_ref(&state);
        let projected = <BitArrayProvider<GleamStdlibProfile> as HostProvider<
            GleamStdlibProfile,
        >>::project(&mut state);

        assert_eq!(std::ptr::from_mut(projected).cast_const(), original);
    }

    #[test]
    fn converts_sizes_padding_and_utf8() {
        let text = from_string("AB".into());
        let unaligned = BitArrayValue::try_from_parts(vec![0b1010_0000], 4)
            .expect("four supplied bits should be valid");

        assert_eq!(text, BitArrayValue::from_bytes(vec![65, 66]));
        assert_eq!(bit_size(unaligned.clone()), BigInt::from(4));
        assert_eq!(byte_size(unaligned.clone()), BigInt::from(1));
        assert_eq!(pad_to_bytes(unaligned).bit_len(), 8);
        assert_eq!(unsafe_to_string(text), Ok("AB".into()));
        assert_eq!(
            unsafe_to_string(
                BitArrayValue::try_from_parts(vec![0], 1)
                    .expect("one supplied bit should be valid"),
            )
            .expect_err("unaligned bits should not be UTF-8")
            .message(),
            "bit array is not byte-aligned UTF-8",
        );
        assert_eq!(
            unsafe_to_string(BitArrayValue::from_bytes(vec![0xff]))
                .expect_err("invalid bytes should not be UTF-8")
                .message(),
            "bit array is not valid UTF-8",
        );
    }
}
