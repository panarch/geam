use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::Table;
use num_bigint::{BigInt, Sign};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegerLiteral {
    pub sign: Sign,
    pub digits: Table<u32>,
}

impl IntegerLiteral {
    pub(crate) fn materialize(&self) -> BigInt {
        BigInt::from_slice(self.sign, &self.digits)
    }

    pub(crate) fn matches(&self, value: &BigInt) -> bool {
        self.sign == value.sign() && self.digits.iter().copied().eq(value.iter_u32_digits())
    }
}

impl From<BigInt> for IntegerLiteral {
    fn from(value: BigInt) -> Self {
        Self::from(&value)
    }
}

impl From<&BigInt> for IntegerLiteral {
    fn from(value: &BigInt) -> Self {
        let (sign, digits) = value.to_u32_digits();
        Self {
            sign,
            digits: digits.into(),
        }
    }
}

impl std::fmt::Display for IntegerLiteral {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.materialize().fmt(formatter)
    }
}

impl Emit for IntegerLiteral {
    fn emit(&self, output: &mut Rust) {
        let Self { sign, digits } = self;
        output.structure(
            "graph::IntegerLiteral",
            &[("sign", sign), ("digits", digits)],
        );
    }
}

#[cfg(test)]
mod tests {
    use super::{IntegerLiteral, Table};
    use num_bigint::{BigInt, Sign};

    #[test]
    fn owns_canonical_digits_without_changing_integer_values() {
        for (value, sign, digits, text) in [
            (BigInt::from(0), Sign::NoSign, vec![], "0"),
            (BigInt::from(42), Sign::Plus, vec![42], "42"),
            (BigInt::from(-42), Sign::Minus, vec![42], "-42"),
            (
                BigInt::from(u64::MAX),
                Sign::Plus,
                vec![u32::MAX, u32::MAX],
                "18446744073709551615",
            ),
        ] {
            let literal = IntegerLiteral::from(value.clone());
            assert_eq!(literal.sign, sign);
            assert_eq!(&*literal.digits, digits.as_slice());
            assert_eq!(literal.materialize(), value);
            assert!(literal.matches(&value));
            assert_eq!(literal.to_string(), text);
        }
        assert!(!IntegerLiteral::from(BigInt::from(42)).matches(&BigInt::from(-42)));
        assert!(!IntegerLiteral::from(BigInt::from(42)).matches(&BigInt::from(43)));
        assert!(!IntegerLiteral::from(BigInt::from(42)).matches(&(BigInt::from(1) << 64)));
    }

    #[test]
    fn materializes_owned_and_static_digits_as_independent_values() {
        const CASES: &[(Sign, &[u32], &str)] = &[
            (Sign::NoSign, &[], "0"),
            (Sign::Plus, &[1], "1"),
            (Sign::Minus, &[1], "-1"),
            (Sign::Plus, &[u32::MAX], "4294967295"),
            (Sign::Minus, &[u32::MAX], "-4294967295"),
            (Sign::Plus, &[0, 1], "4294967296"),
            (Sign::Minus, &[0, 1], "-4294967296"),
            (Sign::Plus, &[u32::MAX, u32::MAX], "18446744073709551615"),
            (Sign::Minus, &[u32::MAX, u32::MAX], "-18446744073709551615"),
            (Sign::Plus, &[0, 0, 1], "18446744073709551616"),
            (Sign::Minus, &[0, 0, 1], "-18446744073709551616"),
            (
                Sign::Plus,
                &[1, 0, 0, 0, 0, 0, 0, 0, 1],
                "115792089237316195423570985008687907853269984665640564039457584007913129639937",
            ),
            (
                Sign::Minus,
                &[1, 0, 0, 0, 0, 0, 0, 0, 1],
                "-115792089237316195423570985008687907853269984665640564039457584007913129639937",
            ),
        ];

        for &(sign, digits, text) in CASES {
            let expected: BigInt = text.parse().unwrap();
            for storage in [Table::from(digits.to_vec()), Table::Static(digits)] {
                let pointer = storage.as_ptr();
                let kind = std::mem::discriminant(&storage);
                let literal = IntegerLiteral {
                    sign,
                    digits: storage,
                };
                let mut first = literal.materialize();
                let second = literal.materialize();
                assert_eq!(first, expected);
                assert_eq!(second, expected);

                first += 1;
                assert_eq!(first, &expected + 1);
                assert_eq!(second, expected);
                assert_eq!(literal.materialize(), expected);
                assert!(literal.matches(&expected));
                assert_eq!(literal.to_string(), text);
                assert_eq!(literal.sign, sign);
                assert_eq!(&*literal.digits, digits);
                assert!(std::ptr::eq(literal.digits.as_ptr(), pointer));
                assert_eq!(std::mem::discriminant(&literal.digits), kind);

                drop(first);
                drop(literal);
                assert_eq!(second, expected);
            }
        }
    }

    #[test]
    fn materializes_static_digits_without_changing_the_literal() {
        static DIGITS: [u32; 3] = [0, 0, 1];
        static LITERAL: IntegerLiteral = IntegerLiteral {
            sign: Sign::Minus,
            digits: Table::Static(&DIGITS),
        };
        let first = LITERAL.materialize();
        let second = LITERAL.materialize();
        assert_eq!(first.to_string(), "-18446744073709551616");
        assert_eq!(second, first);
        assert!(std::ptr::eq(LITERAL.digits.as_ptr(), DIGITS.as_ptr()));
        assert_eq!(LITERAL, IntegerLiteral::from(first));
        assert_eq!(LITERAL.clone(), LITERAL);
        assert_eq!(
            format!("{LITERAL:?}"),
            "IntegerLiteral { sign: Minus, digits: [0, 0, 1] }"
        );
    }
}
