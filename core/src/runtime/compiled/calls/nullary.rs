use crate::plan::execution::type_::CustomConstructorId;
use crate::runtime::evaluated::EvaluatedCustomValue;

/// A generated local for a constructor whose zero-field shape was admitted.
/// Canonical owners are created only when crossing a host/checkpoint boundary.
#[derive(Clone, Copy)]
pub struct CallNullary(CustomConstructorId);

impl CallNullary {
    pub fn matches_constructor(&self, constructor: CustomConstructorId) -> bool {
        self.0 == constructor
    }

    pub fn new(constructor: CustomConstructorId) -> Self {
        Self(constructor)
    }

    pub(in crate::runtime) fn into_evaluated(self) -> EvaluatedCustomValue {
        EvaluatedCustomValue::from_fields(self.0, Box::default())
    }
}
