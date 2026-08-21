use crate::planner::error::{PlanError, UnsupportedBitArraySegmentReason};
use gleam_compiler_core::ast::BitArrayOption;
use num_bigint::BigInt;

pub(super) fn validate_supported_endianness_option<Value>(
    option: &BitArrayOption<Value>,
) -> Result<(), PlanError> {
    if matches!(option, BitArrayOption::Native { .. }) {
        return Err(PlanError::UnsupportedBitArraySegment {
            reason: UnsupportedBitArraySegmentReason::NativeEndianness,
        });
    }

    Ok(())
}

pub(super) fn fixed_bit_size(value: BigInt, unit: u8) -> Result<usize, PlanError> {
    let value = if value < BigInt::from(0) {
        BigInt::from(0)
    } else {
        value
    };
    usize::try_from(value * BigInt::from(unit)).map_err(|_| PlanError::UnsupportedBitArraySegment {
        reason: UnsupportedBitArraySegmentReason::SizeOutOfRange,
    })
}

#[cfg(test)]
mod tests {
    use super::{fixed_bit_size, validate_supported_endianness_option};
    use crate::planner::{PlanError, UnsupportedBitArraySegmentReason};
    use gleam_compiler_core::ast::{BitArrayOption, SrcSpan};
    use num_bigint::BigInt;

    #[test]
    fn owns_shared_bit_array_profile_rejections() {
        assert_eq!(
            validate_supported_endianness_option(&BitArrayOption::<()>::Native {
                location: SrcSpan::new(0, 0),
            }),
            Err(PlanError::UnsupportedBitArraySegment {
                reason: UnsupportedBitArraySegmentReason::NativeEndianness,
            }),
        );
        assert_eq!(fixed_bit_size(BigInt::from(-1), 8), Ok(0));
        assert_eq!(
            fixed_bit_size(BigInt::from(usize::MAX), 2),
            Err(PlanError::UnsupportedBitArraySegment {
                reason: UnsupportedBitArraySegmentReason::SizeOutOfRange,
            }),
        );
    }
}
