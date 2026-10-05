use super::BlockEnvironment;
use crate::runtime::compiled::string::StringValues;
use crate::runtime::integer::IntegerValue;

impl BlockEnvironment {
    pub(in crate::runtime::graph) fn load_string(&mut self, values: &mut StringValues) -> bool {
        values.ints.clear();
        values.bools.clear();
        for value in &self.values.ints {
            let Some(value) = value.small() else {
                return false;
            };
            values.ints.push(i128::from(value));
        }
        // A Big input leaves the existing environment and String owners intact.
        values.bools.extend_from_slice(&self.values.bools);
        values.load_strings(&mut self.values.strings);
        true
    }

    pub(in crate::runtime::graph) fn restore_string(&mut self, values: &mut StringValues) {
        self.values.ints.clear();
        self.values
            .ints
            .extend(values.ints.drain(..).map(IntegerValue::from));
        self.values.bools.clear();
        self.values.bools.append(&mut values.bools);
        values.restore_strings(&mut self.values.strings);
    }
}

#[cfg(test)]
mod tests {
    use super::{BlockEnvironment, StringValues};
    use crate::StringValue;
    use crate::runtime::graph::RetainedValues;
    use num_bigint::BigInt;

    #[test]
    fn eligibility_precedes_moves_and_restore_keeps_completed_prefix_and_other_families() {
        let mut environment = BlockEnvironment::from_retained(RetainedValues::empty());
        let text = StringValue::from("prefix:abcdefghijklmnopqrstuvwxyz");
        let pointer = text.as_ptr();
        environment.values.strings = vec![text];
        let big: BigInt = BigInt::from(1) << 180;
        environment.values.ints = vec![i64::MAX.into(), big.into()];
        environment.values.bools = vec![false, true];
        environment.values.floats = vec![1.5];
        let mut values = StringValues::default();
        assert!(!environment.load_string(&mut values));
        assert!(values.strings.is_empty());
        assert!(values.bools.is_empty());
        assert_eq!(environment.values.strings[0].as_ptr(), pointer);
        environment.values.ints.pop();
        assert!(environment.load_string(&mut values));
        assert!(environment.values.strings.is_empty());
        values.strings = vec![values.strings[0].drop_prefix(7), values.strings[0]];
        values.ints.push(i128::from(i64::MAX) + 1);
        values.bools = vec![true];
        environment.restore_string(&mut values);
        assert_eq!(
            environment
                .values
                .strings
                .iter()
                .map(|value| value.as_str().unwrap())
                .collect::<Vec<_>>(),
            [
                "abcdefghijklmnopqrstuvwxyz",
                "prefix:abcdefghijklmnopqrstuvwxyz",
            ]
        );
        assert_eq!(
            environment.values.strings[0].as_ptr(),
            pointer.wrapping_add(7)
        );
        assert_eq!(environment.values.strings[1].as_ptr(), pointer);
        assert_eq!(
            environment.values.ints[1].bigint().as_ref(),
            &(BigInt::from(i64::MAX) + 1)
        );
        assert_eq!(environment.values.bools, [true]);
        assert_eq!(environment.values.floats, [1.5]);
        assert!(values.ints.is_empty());
        assert!(values.bools.is_empty());
        assert!(values.strings.is_empty());
        assert!(!environment.load_string(&mut values));
        assert_eq!(environment.values.strings.len(), 2);
    }
}
