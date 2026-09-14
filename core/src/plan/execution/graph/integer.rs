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
        BigInt::new(self.sign, self.digits.to_vec())
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
