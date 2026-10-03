use super::BlockEnvironment;
use crate::runtime::compiled::int_list::{IntList, IntListValues};
use crate::runtime::integer::IntegerValue;

impl BlockEnvironment {
    pub(in crate::runtime::graph) fn load_int_list(&mut self, values: &mut IntListValues) -> bool {
        values.ints.clear();
        values.bools.clear();
        values.int_lists.clear();
        for value in &self.values.ints {
            let Some(value) = value.small() else {
                return false;
            };
            values.ints.push(i128::from(value));
        }
        values.bools.extend_from_slice(&self.values.bools);
        // Eligibility is established before any list moves out of its frame.
        values
            .int_lists
            .extend(self.values.int_lists.drain(..).map(IntList));
        true
    }

    pub(in crate::runtime::graph) fn restore_int_list(&mut self, values: &mut IntListValues) {
        self.values.ints.clear();
        self.values
            .ints
            .extend(values.ints.drain(..).map(IntegerValue::from));
        self.values.bools.clear();
        self.values.bools.append(&mut values.bools);
        self.values
            .int_lists
            .extend(values.int_lists.drain(..).map(|value| value.0));
    }
}

#[cfg(test)]
mod tests {
    use super::{BlockEnvironment, IntListValues};
    use crate::runtime::graph::RetainedValues;
    use crate::runtime::state::list::RuntimeListStorage;
    use num_bigint::BigInt;
    use std::ptr;

    #[test]
    fn loading_moves_only_eligible_lists_and_restoring_drains_the_completed_prefix() {
        let storage = RuntimeListStorage::default();
        let plan = crate::runtime::plan_src("pub fn main() { [1] }");
        let type_id = plan.int_list_function_id(0).type_id();
        let first = storage.int(type_id, vec![7_i64.into(), 8_i64.into()]);
        let second = storage.int(type_id, vec![9_i64.into()]);
        let first_address = ptr::from_ref(first.values().get(0).unwrap()).addr();
        let second_address = ptr::from_ref(second.values().get(0).unwrap()).addr();
        let mut environment = BlockEnvironment::from_retained(RetainedValues::empty());
        environment.values.ints = vec![i64::MIN.into(), i64::MAX.into()];
        environment.values.bools = vec![false, true];
        environment.values.strings = vec!["caller value".into()];
        environment.values.int_lists = vec![first, second];
        let big: BigInt = BigInt::from(1) << 180;
        environment.values.ints.push(big.clone().into());
        let mut values = IntListValues::default();
        assert!(!environment.load_int_list(&mut values));
        assert!(values.int_lists.is_empty());
        assert!(values.bools.is_empty());
        assert_eq!(environment.values.int_lists.len(), 2);
        assert_eq!(environment.values.ints[2].bigint().as_ref(), &big);
        environment.values.ints.pop();
        assert!(environment.load_int_list(&mut values));
        assert!(environment.values.int_lists.is_empty());
        assert_eq!(values.ints, [i128::from(i64::MIN), i128::from(i64::MAX)]);
        assert_eq!(values.bools, [false, true]);
        assert_eq!(values.int_lists.len(), 2);
        let second = values.int_lists.pop().unwrap();
        let first = values.int_lists.pop().unwrap();
        values.int_lists = vec![second.clone(), first, second];
        values.ints = vec![3, i128::from(i64::MAX) + 1];
        values.bools = vec![true];
        environment.restore_int_list(&mut values);
        assert!(values.ints.is_empty());
        assert!(values.bools.is_empty());
        assert!(values.int_lists.is_empty());
        assert_eq!(environment.values.ints[0].small(), Some(3));
        assert_eq!(
            environment.values.ints[1].bigint().as_ref(),
            &(BigInt::from(i64::MAX) + 1)
        );
        assert_eq!(environment.values.bools, [true]);
        assert_eq!(environment.values.strings[0].as_str(), "caller value");
        let addresses = environment
            .values
            .int_lists
            .iter()
            .map(|value| ptr::from_ref(value.values().get(0).unwrap()).addr())
            .collect::<Vec<_>>();
        assert_eq!(addresses, [second_address, first_address, second_address]);
        // A promoted completed output sends the following activation to the
        // graph without moving its lists or retaining a duplicate in scratch.
        assert!(!environment.load_int_list(&mut values));
        assert!(values.int_lists.is_empty());
        assert_eq!(environment.values.int_lists.len(), 3);
    }
}
