mod inputs;

pub(crate) use crate::host::{
    RetainedValueEquality, RetainedValueHashing, RetainedValueInspection,
};
pub(crate) use inputs::{CallbackInputs, RetainedInputs};

use crate::runtime::EvaluatedValue;
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct RetainedCallable {
    value: Arc<crate::runtime::function::InvocableFunctionValue>,
}

impl RetainedCallable {
    pub(in crate::runtime) fn new(value: crate::runtime::function::InvocableFunctionValue) -> Self {
        Self {
            value: Arc::new(value),
        }
    }

    pub(in crate::runtime) fn with_value<Output>(
        &self,
        use_value: impl FnOnce(&crate::runtime::function::InvocableFunctionValue) -> Output,
    ) -> Output {
        use_value(&self.value)
    }

    pub(in crate::runtime) fn into_evaluated(
        self,
    ) -> crate::runtime::evaluated::EvaluatedFunctionValue {
        self.value.as_ref().clone().into_evaluated()
    }
}

pub(crate) struct RetainedValueRef<'value> {
    value: &'value EvaluatedValue,
}

impl<'value> RetainedValueRef<'value> {
    pub(in crate::runtime) fn new(value: &'value EvaluatedValue) -> Self {
        Self { value }
    }

    pub(in crate::runtime) fn value(&self) -> &EvaluatedValue {
        self.value
    }
}
