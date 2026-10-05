use crate::plan::execution::type_::CustomConstructorId;
use crate::runtime::evaluated::{EvaluatedCustomValue, EvaluatedValue};

/// Completed scalar and custom columns of an interrupted leaf callback.
/// Integers use the same i128 prefix as generated scalar execution, including
/// a completed Small overflow that the canonical owner must normalize.
#[derive(Default)]
pub struct CustomValues {
    pub ints: Vec<i128>,
    pub bools: Vec<bool>,
    pub customs: Vec<CustomInput>,
}

/// An existing immutable custom owner. Field reads borrow its original values
/// and do not materialize a public value or replace its shared read caches.
#[derive(Clone)]
pub struct CustomInput(pub(in crate::runtime) EvaluatedCustomValue);

impl CustomInput {
    pub fn matches_constructor(&self, constructor: CustomConstructorId) -> bool {
        self.0.constructor() == constructor
    }

    /// None returns before the canonical match or field instruction executes.
    pub fn integer(&self, index: usize) -> Option<i128> {
        match &self.0.fields()[index] {
            EvaluatedValue::Int(value) => value.small().map(i128::from),
            _ => None,
        }
    }

    pub fn boolean(&self, index: usize) -> Option<bool> {
        match &self.0.fields()[index] {
            EvaluatedValue::Bool(value) => Some(*value),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CustomInput;
    use crate::plan::execution::type_::{CustomConstructorId, CustomTypeId};
    use crate::runtime::evaluated::{EvaluatedCustomValue, EvaluatedValue};
    use num_bigint::BigInt;

    #[test]
    fn typed_reads_borrow_the_original_owner_and_defer_big_fields() {
        let constructor = CustomConstructorId {
            type_id: CustomTypeId(2),
            index: 1,
        };
        let big: BigInt = BigInt::from(1) << 130;
        let input = CustomInput(EvaluatedCustomValue::from_fields(
            constructor,
            vec![
                EvaluatedValue::Int(7.into()),
                EvaluatedValue::Bool(true),
                EvaluatedValue::Int(big.clone().into()),
            ]
            .into_boxed_slice(),
        ));
        let alias = input.clone();
        assert!(std::ptr::eq(input.0.fields(), alias.0.fields()));
        assert!(input.matches_constructor(constructor));
        assert!(!input.matches_constructor(CustomConstructorId {
            index: 0,
            ..constructor
        }));
        assert_eq!(input.integer(0), Some(7));
        assert_eq!(input.boolean(1), Some(true));
        assert_eq!(input.integer(1), None);
        assert_eq!(input.boolean(0), None);
        assert_eq!(input.integer(2), None);
        assert_eq!(input.boolean(2), None);
        assert!(input.0.integer_reads().get().is_none());
        drop(input);
        assert_eq!(
            alias.0.fields(),
            &[
                EvaluatedValue::Int(7.into()),
                EvaluatedValue::Bool(true),
                EvaluatedValue::Int(big.into())
            ]
        );
    }
}
