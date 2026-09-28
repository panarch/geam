use super::BlockEnvironment;
use crate::plan::execution::graph::IntegerOperand;
use num_bigint::{BigInt, Sign};
use std::cmp::Ordering;

pub(super) fn add(
    environment: &BlockEnvironment,
    left: IntegerOperand,
    right: IntegerOperand,
) -> BigInt {
    use IntegerOperand::{Immediate, Local};
    match (left, right) {
        (Local(left), Local(right)) => environment.int_ref(left) + environment.int_ref(right),
        (Local(left), Immediate(right)) => environment.int_ref(left) + right,
        (Immediate(left), Local(right)) => left + environment.int_ref(right),
        (Immediate(left), Immediate(right)) => match left.checked_add(right) {
            Some(value) => BigInt::from(value),
            None => BigInt::from(left) + right,
        },
    }
}

pub(super) fn subtract(
    environment: &BlockEnvironment,
    left: IntegerOperand,
    right: IntegerOperand,
) -> BigInt {
    use IntegerOperand::{Immediate, Local};
    match (left, right) {
        (Local(left), Local(right)) => environment.int_ref(left) - environment.int_ref(right),
        (Local(left), Immediate(right)) => environment.int_ref(left) - right,
        (Immediate(left), Local(right)) => left - environment.int_ref(right),
        (Immediate(left), Immediate(right)) => match left.checked_sub(right) {
            Some(value) => BigInt::from(value),
            None => BigInt::from(left) - right,
        },
    }
}

pub(super) fn multiply(
    environment: &BlockEnvironment,
    left: IntegerOperand,
    right: IntegerOperand,
) -> BigInt {
    use IntegerOperand::{Immediate, Local};
    match (left, right) {
        (Local(left), Local(right)) => environment.int_ref(left) * environment.int_ref(right),
        (Local(left), Immediate(right)) => environment.int_ref(left) * right,
        (Immediate(left), Local(right)) => left * environment.int_ref(right),
        (Immediate(left), Immediate(right)) => match left.checked_mul(right) {
            Some(value) => BigInt::from(value),
            None => BigInt::from(left) * right,
        },
    }
}

pub(super) fn divide(
    environment: &BlockEnvironment,
    left: IntegerOperand,
    right: IntegerOperand,
) -> BigInt {
    use IntegerOperand::{Immediate, Local};
    match (left, right) {
        (Local(left), Local(right)) => {
            let right = environment.int_ref(right);
            if right.sign() == Sign::NoSign {
                BigInt::from(0)
            } else {
                environment.int_ref(left) / right
            }
        }
        (Local(left), Immediate(right)) => {
            if right == 0 {
                BigInt::from(0)
            } else {
                environment.int_ref(left) / right
            }
        }
        (Immediate(left), Local(right)) => {
            let right = environment.int_ref(right);
            if right.sign() == Sign::NoSign {
                BigInt::from(0)
            } else {
                left / right
            }
        }
        (Immediate(left), Immediate(right)) => {
            if right == 0 {
                BigInt::from(0)
            } else {
                match left.checked_div(right) {
                    Some(value) => BigInt::from(value),
                    None => BigInt::from(left) / right,
                }
            }
        }
    }
}

pub(super) fn remainder(
    environment: &BlockEnvironment,
    left: IntegerOperand,
    right: IntegerOperand,
) -> BigInt {
    use IntegerOperand::{Immediate, Local};
    match (left, right) {
        (Local(left), Local(right)) => {
            let right = environment.int_ref(right);
            if right.sign() == Sign::NoSign {
                BigInt::from(0)
            } else {
                environment.int_ref(left) % right
            }
        }
        (Local(left), Immediate(right)) => {
            if right == 0 {
                BigInt::from(0)
            } else {
                environment.int_ref(left) % right
            }
        }
        (Immediate(left), Local(right)) => {
            let right = environment.int_ref(right);
            if right.sign() == Sign::NoSign {
                BigInt::from(0)
            } else {
                left % right
            }
        }
        (Immediate(left), Immediate(right)) => {
            if right == 0 {
                BigInt::from(0)
            } else {
                match left.checked_rem(right) {
                    Some(value) => BigInt::from(value),
                    None => BigInt::from(left) % right,
                }
            }
        }
    }
}

pub(super) fn equal(
    environment: &BlockEnvironment,
    left: IntegerOperand,
    right: IntegerOperand,
) -> bool {
    use IntegerOperand::{Immediate, Local};
    match (left, right) {
        (Local(left), Local(right)) => environment.int_ref(left) == environment.int_ref(right),
        (Local(left), Immediate(right)) => {
            compare_immediate(environment.int_ref(left), right).is_eq()
        }
        (Immediate(left), Local(right)) => {
            compare_immediate(environment.int_ref(right), left).is_eq()
        }
        (Immediate(left), Immediate(right)) => left == right,
    }
}

pub(super) fn compare(
    environment: &BlockEnvironment,
    left: IntegerOperand,
    right: IntegerOperand,
) -> Ordering {
    use IntegerOperand::{Immediate, Local};
    match (left, right) {
        (Local(left), Local(right)) => environment.int_ref(left).cmp(environment.int_ref(right)),
        (Local(left), Immediate(right)) => compare_immediate(environment.int_ref(left), right),
        (Immediate(left), Local(right)) => {
            compare_immediate(environment.int_ref(right), left).reverse()
        }
        (Immediate(left), Immediate(right)) => left.cmp(&right),
    }
}

fn compare_immediate(left: &BigInt, right: i64) -> Ordering {
    match i64::try_from(left) {
        Ok(left) => left.cmp(&right),
        Err(_) => {
            if left.sign() == Sign::Minus {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{add, compare, divide, equal, multiply, remainder, subtract};
    use crate::plan::execution::graph::{IntLocalId, IntegerOperand};
    use crate::runtime::graph::environment::{BlockEnvironment, RetainedValues};
    use num_bigint::BigInt;
    use std::cmp::Ordering;

    #[test]
    fn arithmetic_and_comparison_preserve_every_operand_form_and_integer_boundary() {
        let cases = [
            (
                "-7",
                "3",
                ["-4", "-10", "-21", "-2", "-1"],
                Ordering::Less,
                4,
            ),
            (
                "7",
                "-3",
                ["4", "10", "-21", "-2", "1"],
                Ordering::Greater,
                4,
            ),
            (
                "-7",
                "-3",
                ["-10", "-4", "21", "2", "-1"],
                Ordering::Less,
                4,
            ),
            ("0", "0", ["0", "0", "0", "0", "0"], Ordering::Equal, 4),
            ("7", "0", ["7", "7", "0", "0", "0"], Ordering::Greater, 4),
            ("0", "-3", ["-3", "3", "0", "0", "0"], Ordering::Greater, 4),
            (
                "9223372036854775807",
                "1",
                [
                    "9223372036854775808",
                    "9223372036854775806",
                    "9223372036854775807",
                    "9223372036854775807",
                    "0",
                ],
                Ordering::Greater,
                4,
            ),
            (
                "-9223372036854775808",
                "-1",
                [
                    "-9223372036854775809",
                    "-9223372036854775807",
                    "9223372036854775808",
                    "9223372036854775808",
                    "0",
                ],
                Ordering::Less,
                4,
            ),
            (
                "-9223372036854775808",
                "1",
                [
                    "-9223372036854775807",
                    "-9223372036854775809",
                    "-9223372036854775808",
                    "-9223372036854775808",
                    "0",
                ],
                Ordering::Less,
                4,
            ),
            (
                "9223372036854775807",
                "-1",
                [
                    "9223372036854775806",
                    "9223372036854775808",
                    "-9223372036854775807",
                    "-9223372036854775807",
                    "0",
                ],
                Ordering::Greater,
                4,
            ),
            (
                "-9223372036854775808",
                "-9223372036854775808",
                [
                    "-18446744073709551616",
                    "0",
                    "85070591730234615865843651857942052864",
                    "1",
                    "0",
                ],
                Ordering::Equal,
                4,
            ),
            (
                "9223372036854775807",
                "9223372036854775807",
                [
                    "18446744073709551614",
                    "0",
                    "85070591730234615847396907784232501249",
                    "1",
                    "0",
                ],
                Ordering::Equal,
                4,
            ),
            (
                "9223372036854775808",
                "3",
                [
                    "9223372036854775811",
                    "9223372036854775805",
                    "27670116110564327424",
                    "3074457345618258602",
                    "2",
                ],
                Ordering::Greater,
                2,
            ),
            (
                "-9223372036854775809",
                "3",
                [
                    "-9223372036854775806",
                    "-9223372036854775812",
                    "-27670116110564327427",
                    "-3074457345618258603",
                    "0",
                ],
                Ordering::Less,
                2,
            ),
            (
                "340282366920938463463374607431768211456",
                "-3",
                [
                    "340282366920938463463374607431768211453",
                    "340282366920938463463374607431768211459",
                    "-1020847100762815390390123822295304634368",
                    "-113427455640312821154458202477256070485",
                    "1",
                ],
                Ordering::Greater,
                2,
            ),
            (
                "-340282366920938463463374607431768211456",
                "-3",
                [
                    "-340282366920938463463374607431768211459",
                    "-340282366920938463463374607431768211453",
                    "1020847100762815390390123822295304634368",
                    "113427455640312821154458202477256070485",
                    "-1",
                ],
                Ordering::Less,
                2,
            ),
            (
                "3",
                "340282366920938463463374607431768211456",
                [
                    "340282366920938463463374607431768211459",
                    "-340282366920938463463374607431768211453",
                    "1020847100762815390390123822295304634368",
                    "0",
                    "3",
                ],
                Ordering::Less,
                2,
            ),
            (
                "3",
                "-340282366920938463463374607431768211456",
                [
                    "-340282366920938463463374607431768211453",
                    "340282366920938463463374607431768211459",
                    "-1020847100762815390390123822295304634368",
                    "0",
                    "3",
                ],
                Ordering::Greater,
                2,
            ),
            (
                "340282366920938463463374607431768211456",
                "340282366920938463463374607431768211456",
                [
                    "680564733841876926926749214863536422912",
                    "0",
                    "115792089237316195423570985008687907853269984665640564039457584007913129639936",
                    "1",
                    "0",
                ],
                Ordering::Equal,
                1,
            ),
        ];
        for (left, right, expected, ordering, forms) in cases {
            let left: BigInt = left.parse().unwrap();
            let right: BigInt = right.parse().unwrap();
            let expected = expected.map(|value| value.parse::<BigInt>().unwrap());
            let mut values = RetainedValues::empty();
            values.push_int(left.clone());
            values.push_int(right.clone());
            let environment = BlockEnvironment::from_retained(values);
            let mut lefts = vec![IntegerOperand::Local(IntLocalId(0))];
            let mut rights = vec![IntegerOperand::Local(IntLocalId(1))];
            if let Ok(value) = i64::try_from(&left) {
                lefts.push(IntegerOperand::Immediate(value));
            }
            if let Ok(value) = i64::try_from(&right) {
                rights.push(IntegerOperand::Immediate(value));
            }
            assert_eq!(lefts.len() * rights.len(), forms);
            for &a in &lefts {
                for &b in &rights {
                    assert_eq!(add(&environment, a, b), expected[0]);
                    assert_eq!(subtract(&environment, a, b), expected[1]);
                    assert_eq!(multiply(&environment, a, b), expected[2]);
                    assert_eq!(divide(&environment, a, b), expected[3]);
                    assert_eq!(remainder(&environment, a, b), expected[4]);
                    assert_eq!(compare(&environment, a, b), ordering);
                    assert_eq!(equal(&environment, a, b), ordering == Ordering::Equal);
                    assert_eq!(environment.int_ref(IntLocalId(0)), &left);
                    assert_eq!(environment.int_ref(IntLocalId(1)), &right);
                }
            }
        }
    }

    #[test]
    fn repeated_local_reads_leave_the_owned_input_unchanged() {
        let mut values = RetainedValues::empty();
        values.push_int(BigInt::from(-7));
        let environment = BlockEnvironment::from_retained(values);
        let value = IntegerOperand::Local(IntLocalId(0));
        assert_eq!(add(&environment, value, value), BigInt::from(-14));
        assert_eq!(subtract(&environment, value, value), BigInt::from(0));
        assert_eq!(multiply(&environment, value, value), BigInt::from(49));
        assert_eq!(divide(&environment, value, value), BigInt::from(1));
        assert_eq!(remainder(&environment, value, value), BigInt::from(0));
        assert!(equal(&environment, value, value));
        assert_eq!(compare(&environment, value, value), Ordering::Equal);
        assert_eq!(environment.int_ref(IntLocalId(0)), &BigInt::from(-7));
    }
}
