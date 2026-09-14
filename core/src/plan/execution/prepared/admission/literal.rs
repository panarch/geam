use crate::plan::execution::graph::IntegerLiteral;
use num_bigint::Sign;

#[derive(Debug, PartialEq, Eq)]
pub(super) enum IntegerError {
    ZeroSign,
    EmptyMagnitude,
    TrailingZero,
}

pub(super) fn integer(value: &IntegerLiteral) -> Result<(), IntegerError> {
    match (value.sign, value.digits.last()) {
        (Sign::NoSign, None) => Ok(()),
        (Sign::NoSign, Some(_)) => Err(IntegerError::ZeroSign),
        (_, None) => Err(IntegerError::EmptyMagnitude),
        (_, Some(0)) => Err(IntegerError::TrailingZero),
        (_, Some(_)) => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::{IntegerError, IntegerLiteral, Sign, integer};

    #[test]
    fn accepts_exact_canonical_limbs_without_materialization() {
        for (sign, digits, expected) in [
            (Sign::NoSign, vec![], Ok(())),
            (Sign::Plus, vec![42], Ok(())),
            (Sign::Minus, vec![0, 0, 1], Ok(())),
            (Sign::NoSign, vec![42], Err(IntegerError::ZeroSign)),
            (Sign::Plus, vec![], Err(IntegerError::EmptyMagnitude)),
            (Sign::Minus, vec![42, 0], Err(IntegerError::TrailingZero)),
        ] {
            assert_eq!(
                integer(&IntegerLiteral {
                    sign,
                    digits: digits.into()
                }),
                expected
            );
        }
    }
}
