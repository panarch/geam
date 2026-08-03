use super::super::schema::{BitArrayError, BitArrayOk, BitArrayResult};
use super::BitArrayProvider;
use crate::gleam_stdlib::GleamStdlibHostProfile;
use crate::{BitArrayValue, HostCall, HostCallCompletion, HostCallError};
use num_bigint::{BigInt, Sign};
use num_traits::ToPrimitive;

pub(in crate::gleam_stdlib::bit_array) fn slice<'call, Profile>(
    call: HostCall<'call, Profile, BitArrayProvider<Profile>, BitArrayResult>,
    value: BitArrayValue,
    position: BigInt,
    length: BigInt,
) -> Result<HostCallCompletion<'call, BitArrayResult>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    let endpoint = &position + &length;
    let (start, end) = if position <= endpoint {
        (position, endpoint)
    } else {
        (endpoint, position)
    };
    let byte_len = BigInt::from(value.bit_len() / 8);
    let selected =
        if value.bit_len().is_multiple_of(8) && start.sign() != Sign::Minus && end <= byte_len {
            start
                .to_usize()
                .zip((&end - &start).to_usize())
                .and_then(|(start, length)| value.byte_slice(start, length))
        } else {
            None
        };

    Ok(match selected {
        Some(value) => call.return_custom::<BitArrayOk>((value, ())),
        None => call.return_custom::<BitArrayError>(((), ())),
    })
}
