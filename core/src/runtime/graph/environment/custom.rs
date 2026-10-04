use super::RetainedValues;
use crate::runtime::compiled::custom::CustomValues;
use crate::runtime::integer::IntegerValue;
use std::mem;

impl RetainedValues {
    pub(in crate::runtime::graph) fn restore_custom(&mut self, values: &mut CustomValues) {
        self.values.ints.clear();
        self.values
            .ints
            .extend(values.ints.drain(..).map(IntegerValue::from));
        mem::swap(&mut self.values.bools, &mut values.bools);
        self.values
            .customs
            .extend(values.customs.drain(..).map(|value| value.0));
    }
}

#[cfg(test)]
mod tests {
    use super::RetainedValues;
    use crate::plan::execution::type_::{CustomConstructorId, CustomTypeId};
    use crate::runtime::compiled::custom::{CustomInput, CustomValues};
    use crate::runtime::evaluated::{EvaluatedCustomValue, EvaluatedValue};
    use crate::runtime::integer::IntegerValue;

    #[test]
    fn interrupted_prefix_normalizes_completed_overflow_and_moves_custom_owners() {
        let original = EvaluatedCustomValue::from_fields(
            CustomConstructorId {
                type_id: CustomTypeId(0),
                index: 0,
            },
            vec![EvaluatedValue::Int(9.into())].into_boxed_slice(),
        );
        let mut prefix = CustomValues {
            ints: vec![7, i128::from(i64::MAX) + 1, i128::from(i64::MIN) - 1],
            bools: vec![true, false],
            customs: vec![CustomInput(original.clone())],
        };
        let mut inputs = RetainedValues::empty();
        inputs.restore_custom(&mut prefix);
        assert_eq!(
            inputs.values.ints,
            vec![
                IntegerValue::from(7),
                IntegerValue::from(i128::from(i64::MAX) + 1),
                IntegerValue::from(i128::from(i64::MIN) - 1)
            ]
        );
        assert_eq!(inputs.values.ints[0].small(), Some(7));
        assert_eq!(inputs.values.ints[1].small(), None);
        assert_eq!(inputs.values.ints[2].small(), None);
        assert_eq!(inputs.values.bools, [true, false]);
        assert!(std::ptr::eq(
            inputs.values.customs[0].fields(),
            original.fields()
        ));
        assert!(prefix.ints.is_empty() && prefix.bools.is_empty() && prefix.customs.is_empty());
        drop(inputs);
        assert_eq!(original.fields(), &[EvaluatedValue::Int(9.into())]);
    }
}
