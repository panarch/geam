use crate::plan::execution::graph::IntegerLiteral;
use num_bigint::{BigInt, Sign};
use std::borrow::Cow;
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

/// Canonical runtime integer. Small values stay inline through stored values
/// and calls; boxing the uncommon large payload keeps those columns compact.
#[derive(Clone)]
pub(crate) struct IntegerValue(Representation);

#[derive(Clone)]
enum Representation {
    Small(i64),
    Big(Box<BigInt>),
}

impl IntegerValue {
    pub(crate) fn from_literal(value: &IntegerLiteral) -> Self {
        if value.sign == Sign::NoSign {
            return Self::from(0_i64);
        }
        if value.digits.len() <= 2 {
            let low = value.digits.first().copied().unwrap_or(0);
            let high = value.digits.get(1).copied().unwrap_or(0);
            let magnitude = i128::from(u64::from(low) | (u64::from(high) << 32));
            Self::from(if value.sign == Sign::Minus {
                -magnitude
            } else {
                magnitude
            })
        } else {
            value.materialize().into()
        }
    }

    pub(crate) fn matches_literal(&self, value: &IntegerLiteral) -> bool {
        if self.sign() != value.sign {
            return false;
        }
        match &self.0 {
            Representation::Small(number) => {
                let magnitude = number.unsigned_abs();
                let digits = [magnitude as u32, (magnitude >> 32) as u32];
                let count = (64 - magnitude.leading_zeros()).div_ceil(32) as usize;
                value.digits.as_ref() == &digits[..count]
            }
            Representation::Big(number) => value.matches(number),
        }
    }

    pub(crate) fn small(&self) -> Option<i64> {
        match &self.0 {
            Representation::Small(value) => Some(*value),
            Representation::Big(_) => None,
        }
    }

    pub(crate) fn to_usize(&self) -> Option<usize> {
        match &self.0 {
            Representation::Small(value) => usize::try_from(*value).ok(),
            Representation::Big(value) => usize::try_from(value.as_ref()).ok(),
        }
    }

    pub(crate) fn to_u8(&self) -> Option<u8> {
        self.small().and_then(|value| u8::try_from(value).ok())
    }

    pub(crate) fn sign(&self) -> Sign {
        match &self.0 {
            Representation::Small(value) => match value.cmp(&0) {
                Ordering::Less => Sign::Minus,
                Ordering::Equal => Sign::NoSign,
                Ordering::Greater => Sign::Plus,
            },
            Representation::Big(value) => value.sign(),
        }
    }

    pub(crate) fn bigint(&self) -> Cow<'_, BigInt> {
        match &self.0 {
            Representation::Small(value) => Cow::Owned(BigInt::from(*value)),
            Representation::Big(value) => Cow::Borrowed(value),
        }
    }

    pub(crate) fn into_bigint(self) -> BigInt {
        match self.0 {
            Representation::Small(value) => BigInt::from(value),
            Representation::Big(value) => *value,
        }
    }

    pub(crate) fn compare_small(&self, right: i64) -> Ordering {
        match &self.0 {
            Representation::Small(left) => left.cmp(&right),
            Representation::Big(left) => {
                if left.sign() == Sign::Minus {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
        }
    }

    pub(crate) fn negate(&self) -> Self {
        match &self.0 {
            Representation::Small(value) => Self::from(-i128::from(*value)),
            Representation::Big(value) => (-value.as_ref()).into(),
        }
    }

    pub(crate) fn add(&self, right: &Self) -> Self {
        match (&self.0, &right.0) {
            (Representation::Small(left), Representation::Small(right)) => {
                Self::from(i128::from(*left) + i128::from(*right))
            }
            (Representation::Big(left), Representation::Small(right)) => {
                (left.as_ref() + *right).into()
            }
            (Representation::Small(left), Representation::Big(right)) => {
                (*left + right.as_ref()).into()
            }
            (Representation::Big(left), Representation::Big(right)) => {
                (left.as_ref() + right.as_ref()).into()
            }
        }
    }

    pub(crate) fn subtract(&self, right: &Self) -> Self {
        match (&self.0, &right.0) {
            (Representation::Small(left), Representation::Small(right)) => {
                Self::from(i128::from(*left) - i128::from(*right))
            }
            (Representation::Big(left), Representation::Small(right)) => {
                (left.as_ref() - *right).into()
            }
            (Representation::Small(left), Representation::Big(right)) => {
                (*left - right.as_ref()).into()
            }
            (Representation::Big(left), Representation::Big(right)) => {
                (left.as_ref() - right.as_ref()).into()
            }
        }
    }

    pub(crate) fn multiply(&self, right: &Self) -> Self {
        match (&self.0, &right.0) {
            (Representation::Small(left), Representation::Small(right)) => {
                Self::from(i128::from(*left) * i128::from(*right))
            }
            (Representation::Big(left), Representation::Small(right)) => {
                (left.as_ref() * *right).into()
            }
            (Representation::Small(left), Representation::Big(right)) => {
                (*left * right.as_ref()).into()
            }
            (Representation::Big(left), Representation::Big(right)) => {
                (left.as_ref() * right.as_ref()).into()
            }
        }
    }

    pub(crate) fn divide(&self, right: &Self) -> Self {
        if right.sign() == Sign::NoSign {
            return Self::from(0);
        }
        match (&self.0, &right.0) {
            (Representation::Small(left), Representation::Small(right)) => {
                Self::from(i128::from(*left) / i128::from(*right))
            }
            (Representation::Big(left), Representation::Small(right)) => {
                (left.as_ref() / *right).into()
            }
            (Representation::Small(left), Representation::Big(right)) => {
                (*left / right.as_ref()).into()
            }
            (Representation::Big(left), Representation::Big(right)) => {
                (left.as_ref() / right.as_ref()).into()
            }
        }
    }

    pub(crate) fn remainder(&self, right: &Self) -> Self {
        if right.sign() == Sign::NoSign {
            return Self::from(0);
        }
        match (&self.0, &right.0) {
            (Representation::Small(left), Representation::Small(right)) => {
                Self::from(i128::from(*left) % i128::from(*right))
            }
            (Representation::Big(left), Representation::Small(right)) => {
                (left.as_ref() % *right).into()
            }
            (Representation::Small(left), Representation::Big(right)) => {
                (*left % right.as_ref()).into()
            }
            (Representation::Big(left), Representation::Big(right)) => {
                (left.as_ref() % right.as_ref()).into()
            }
        }
    }
}

impl From<BigInt> for IntegerValue {
    fn from(value: BigInt) -> Self {
        match i64::try_from(&value) {
            Ok(value) => value.into(),
            Err(_) => Self(Representation::Big(Box::new(value))),
        }
    }
}

impl From<i128> for IntegerValue {
    fn from(value: i128) -> Self {
        match i64::try_from(value) {
            Ok(value) => value.into(),
            Err(_) => BigInt::from(value).into(),
        }
    }
}

macro_rules! small_from {
    ($($type:ty),+) => {$(
        impl From<$type> for IntegerValue {
            fn from(value: $type) -> Self {
                Self(Representation::Small(i64::from(value)))
            }
        }
    )+};
}
small_from!(i8, i16, i32, i64, u8, u16, u32);

impl From<u64> for IntegerValue {
    fn from(value: u64) -> Self {
        Self::from(i128::from(value))
    }
}

impl From<usize> for IntegerValue {
    fn from(value: usize) -> Self {
        Self::from(value as u64)
    }
}

impl From<IntegerValue> for BigInt {
    fn from(value: IntegerValue) -> Self {
        value.into_bigint()
    }
}

impl fmt::Display for IntegerValue {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Representation::Small(value) => fmt::Display::fmt(value, output),
            Representation::Big(value) => fmt::Display::fmt(value, output),
        }
    }
}

impl fmt::Debug for IntegerValue {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, output)
    }
}

impl PartialEq for IntegerValue {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}
impl Eq for IntegerValue {}

#[cfg(test)]
impl PartialEq<BigInt> for IntegerValue {
    fn eq(&self, other: &BigInt) -> bool {
        match &self.0 {
            Representation::Small(value) => i64::try_from(other).ok() == Some(*value),
            Representation::Big(value) => value.as_ref() == other,
        }
    }
}

impl PartialOrd for IntegerValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for IntegerValue {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self.0, &other.0) {
            (Representation::Small(left), Representation::Small(right)) => left.cmp(right),
            (Representation::Big(left), Representation::Big(right)) => left.cmp(right),
            (Representation::Big(_), Representation::Small(right)) => self.compare_small(*right),
            (Representation::Small(left), Representation::Big(_)) => {
                other.compare_small(*left).reverse()
            }
        }
    }
}

impl Hash for IntegerValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.sign().hash(state);
        match &self.0 {
            Representation::Small(value) => {
                let magnitude = value.unsigned_abs();
                let count = (64 - magnitude.leading_zeros()).div_ceil(32) as usize;
                count.hash(state);
                if count > 0 {
                    (magnitude as u32).hash(state);
                }
                if count > 1 {
                    ((magnitude >> 32) as u32).hash(state);
                }
            }
            Representation::Big(value) => {
                (value.bits().div_ceil(32) as usize).hash(state);
                for digit in value.iter_u32_digits() {
                    digit.hash(state);
                }
            }
        }
    }
}

impl Neg for IntegerValue {
    type Output = Self;
    fn neg(self) -> Self {
        match self.0 {
            Representation::Small(value) => Self::from(-i128::from(value)),
            Representation::Big(value) => (-*value).into(),
        }
    }
}

macro_rules! binary_operator {
    ($trait:ident, $method:ident, $kernel:ident) => {
        impl $trait<IntegerValue> for IntegerValue {
            type Output = IntegerValue;
            fn $method(self, right: IntegerValue) -> Self::Output {
                IntegerValue::$kernel(&self, &right)
            }
        }
    };
}
binary_operator!(Add, add, add);
binary_operator!(Sub, sub, subtract);
binary_operator!(Mul, mul, multiply);
binary_operator!(Div, div, divide);
binary_operator!(Rem, rem, remainder);

#[cfg(test)]
mod tests {
    use super::IntegerValue;
    use num_bigint::BigInt;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    #[test]
    fn normalization_preserves_boundaries_and_promotes_and_demotes_exactly() {
        assert!(size_of::<IntegerValue>() <= 2 * size_of::<i64>());
        for value in [i64::MIN, -1, 0, 1, i64::MAX] {
            let integer = IntegerValue::from(BigInt::from(value));
            assert_eq!(integer.small(), Some(value));
            assert_eq!(integer.into_bigint(), BigInt::from(value));
        }
        let min = IntegerValue::from(i64::MIN);
        let max = IntegerValue::from(i64::MAX);
        let above = max.add(&1.into());
        let below = min.subtract(&1.into());
        assert_eq!(above.small(), None);
        assert_eq!(below.small(), None);
        assert_eq!(above.subtract(&1.into()).small(), Some(i64::MAX));
        assert_eq!(below.add(&1.into()).small(), Some(i64::MIN));
        assert_eq!((-min.clone()).into_bigint(), -BigInt::from(i64::MIN));
        assert_eq!(
            min.divide(&(-1).into()).into_bigint(),
            -BigInt::from(i64::MIN)
        );
        assert_eq!(min.remainder(&(-1).into()).small(), Some(0));
    }

    #[test]
    fn kernels_match_independent_arbitrary_precision_arithmetic() {
        let huge: BigInt = BigInt::from(1u8) << 256;
        let numbers = [
            BigInt::from(i64::MIN),
            BigInt::from(i64::MAX),
            (-7).into(),
            3.into(),
            0.into(),
            huge.clone(),
            -huge,
        ];
        for left in &numbers {
            for right in &numbers {
                let a = IntegerValue::from(left.clone());
                let b = IntegerValue::from(right.clone());
                assert_eq!(a.add(&b).into_bigint(), left + right);
                assert_eq!(a.subtract(&b).into_bigint(), left - right);
                assert_eq!(a.multiply(&b).into_bigint(), left * right);
                let quotient = if right == &BigInt::from(0) {
                    0.into()
                } else {
                    left / right
                };
                let remainder = if right == &BigInt::from(0) {
                    0.into()
                } else {
                    left % right
                };
                assert_eq!(a.divide(&b).into_bigint(), quotient);
                assert_eq!(a.remainder(&b).into_bigint(), remainder);
                assert_eq!(a.cmp(&b), left.cmp(right));
                assert_eq!(a.partial_cmp(&b), Some(left.cmp(right)));
                assert_eq!(a.to_string(), left.to_string());
            }
            let value = IntegerValue::from(left.clone());
            assert_eq!(value.negate().into_bigint(), -left);
            assert_eq!((-value).into_bigint(), -left);
        }
    }

    #[test]
    fn normalized_values_have_identical_hashes_after_large_intermediate_arithmetic() {
        let huge = IntegerValue::from(BigInt::from(1u8) << 256);
        for value in [i64::MIN, -1, 0, 1, i64::MAX] {
            let direct = IntegerValue::from(value);
            let returned = direct.add(&huge).subtract(&huge);
            let hash = |value: &IntegerValue| {
                let mut state = DefaultHasher::new();
                value.hash(&mut state);
                state.finish()
            };
            assert_eq!(returned.small(), Some(value));
            assert_eq!(direct, returned);
            assert_eq!(hash(&direct), hash(&returned));
        }
        for number in [
            BigInt::from(1_u8) << 256_u32,
            -(BigInt::from(1_u8) << 256_u32),
        ] {
            let direct = IntegerValue::from(number.clone());
            let returned = direct.add(&7.into()).subtract(&7.into());
            let hash = |value: &IntegerValue| {
                let mut state = DefaultHasher::new();
                value.hash(&mut state);
                state.finish()
            };
            assert_eq!(direct, returned);
            assert_eq!(hash(&direct), hash(&returned));
            assert_eq!(direct.compare_small(0), number.cmp(&BigInt::from(0)));
        }
        assert_eq!(
            IntegerValue::from(7).compare_small(8),
            std::cmp::Ordering::Less
        );
    }
}
