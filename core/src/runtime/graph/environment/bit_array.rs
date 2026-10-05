use super::BlockEnvironment;
use crate::runtime::compiled::bit_array::BitArrayValues;
use crate::runtime::evaluated::EvaluatedBitArray;
use crate::runtime::integer::IntegerValue;

impl BlockEnvironment {
    pub(in crate::runtime::graph) fn load_bit_array(&self, values: &mut BitArrayValues) -> bool {
        values.clear();
        for value in &self.values.ints {
            let Some(value) = value.small() else {
                return false;
            };
            values.ints.push(i128::from(value));
        }
        values.bools.extend_from_slice(&self.values.bools);
        for input in &self.values.bit_arrays {
            values.push_input(input.value());
        }
        true
    }

    pub(in crate::runtime::graph) fn restore_bit_array(&mut self, values: &BitArrayValues) {
        self.values.ints.clear();
        self.values
            .ints
            .extend(values.ints.iter().copied().map(IntegerValue::from));
        self.values.bools.clear();
        self.values.bools.extend_from_slice(&values.bools);
        self.values.bit_arrays.clear();
        self.values.bit_arrays.extend(
            values
                .bit_arrays
                .iter()
                .map(|range| EvaluatedBitArray::from_value(values.materialize(*range))),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::{BitArrayValues, BlockEnvironment, EvaluatedBitArray};
    use crate::BitArrayValue;
    use crate::runtime::graph::RetainedValues;
    use num_bigint::BigInt;

    #[test]
    fn bit_loading_and_restoring_preserve_only_the_completed_columns() {
        let mut environment = BlockEnvironment::from_retained(RetainedValues::empty());
        environment.values.ints = vec![7_i64.into()];
        environment.values.bools = vec![true];
        environment.values.bit_arrays = vec![EvaluatedBitArray::from_value(
            BitArrayValue::from_bytes(vec![0xab, 0xcd]),
        )];
        environment.values.strings = vec!["retained".into()];
        let mut values = BitArrayValues::default();
        assert!(environment.load_bit_array(&mut values));
        let original = values.bit_arrays[0];
        values.bit_arrays = vec![original.slice(4, 9).unwrap(), original.slice(8, 8).unwrap()];
        values.ints = vec![i128::from(i64::MAX) + 1];
        values.bools.clear();
        environment.restore_bit_array(&values);
        values.clear();
        assert_eq!(
            environment.values.ints[0].bigint().as_ref(),
            &(BigInt::from(i64::MAX) + 1)
        );
        assert!(environment.values.bools.is_empty());
        assert_eq!(
            environment.values.bit_arrays[0].value().bytes(),
            &[0xbc, 0x80]
        );
        assert_eq!(environment.values.bit_arrays[0].value().bit_len(), 9);
        assert_eq!(environment.values.bit_arrays[1].value().bytes(), &[0xcd]);
        assert_eq!(environment.values.strings[0].as_str().unwrap(), "retained");
        assert!(!environment.load_bit_array(&mut values));
        assert!(values.bit_arrays.is_empty());
        assert!(values.ints.is_empty());
        assert!(values.bools.is_empty());
        assert_eq!(environment.values.bit_arrays.len(), 2);
    }
}
