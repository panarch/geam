use super::BlockEnvironment;
use crate::runtime::integer::IntegerValue;
use crate::runtime::numeric::NumericValues;

impl BlockEnvironment {
    pub(in crate::runtime::graph) fn load_numeric(&self, numeric: &mut NumericValues) -> bool {
        numeric.ints.clear();
        numeric.bools.clear();
        for value in &self.values.ints {
            let Some(value) = value.small() else {
                return false;
            };
            numeric.ints.push(i128::from(value));
        }
        numeric.bools.extend_from_slice(&self.values.bools);
        true
    }

    pub(in crate::runtime::graph) fn restore_numeric(&mut self, numeric: &NumericValues) {
        self.values.ints.clear();
        self.values
            .ints
            .extend(numeric.ints.iter().copied().map(IntegerValue::from));
        self.values.bools.clear();
        self.values.bools.extend_from_slice(&numeric.bools);
    }
}

#[cfg(test)]
mod tests {
    use super::{BlockEnvironment, NumericValues};
    use crate::runtime::graph::RetainedValues;
    use crate::runtime::integer::IntegerValue;
    use num_bigint::BigInt;

    #[test]
    fn numeric_loading_keeps_the_source_and_reuses_only_actual_small_columns() {
        let mut environment = BlockEnvironment::from_retained(RetainedValues::empty());
        environment.values.ints = vec![i64::MIN.into(), i64::MAX.into()];
        environment.values.bools = vec![false, true];
        let mut numeric = NumericValues {
            ints: vec![999],
            bools: vec![true],
        };
        assert!(environment.load_numeric(&mut numeric));
        assert_eq!(numeric.ints, [i128::from(i64::MIN), i128::from(i64::MAX)]);
        assert_eq!(numeric.bools, [false, true]);
        assert_eq!(environment.values.ints[0].small(), Some(i64::MIN));
        assert_eq!(environment.values.ints[1].small(), Some(i64::MAX));
        let big: BigInt = BigInt::from(1) << 100;
        environment.values.ints.push(big.clone().into());
        assert!(!environment.load_numeric(&mut numeric));
        assert_eq!(numeric.ints, [i128::from(i64::MIN), i128::from(i64::MAX)]);
        assert!(numeric.bools.is_empty());
        assert_eq!(environment.values.ints[2].bigint().as_ref(), &big);
        assert_eq!(environment.values.bools, [false, true]);
    }

    #[test]
    fn restoring_a_completed_prefix_normalizes_big_outputs_without_filling_other_columns() {
        let mut environment = BlockEnvironment::from_retained(RetainedValues::empty());
        environment.values.ints = vec![IntegerValue::from(99_i64)];
        environment.values.bools = vec![false];
        environment.values.strings = vec!["retained".into()];
        environment.restore_numeric(&NumericValues {
            ints: vec![3, i128::from(i64::MAX) + 1, i128::from(i64::MIN) - 1],
            bools: vec![true, false],
        });
        assert_eq!(environment.values.ints[0].small(), Some(3));
        assert_eq!(environment.values.ints[1].small(), None);
        assert_eq!(
            environment.values.ints[1].bigint().as_ref(),
            &(BigInt::from(i64::MAX) + 1)
        );
        assert_eq!(
            environment.values.ints[2].bigint().as_ref(),
            &(BigInt::from(i64::MIN) - 1)
        );
        assert_eq!(environment.values.bools, [true, false]);
        assert_eq!(environment.values.strings[0].as_str(), "retained");
    }
}
