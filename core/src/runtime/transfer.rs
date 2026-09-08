mod external;
mod inputs;
mod list;

pub(crate) use crate::host::{
    TransferExternalEquality, TransferExternalHashing, TransferExternalInspection,
};
pub(crate) use external::{
    TransferExternalPayloadLease, TransferExternalPayloadView, TransferExternalStore,
};
pub(crate) use inputs::{TransferCallbackInputs, TransferInputs};
pub(in crate::runtime) use list::TransferListHandleCore;
pub(crate) use list::TransferListStorage;

use crate::runtime::{EvaluatedValue, TransferValues};

#[derive(Clone)]
pub(crate) struct TransferCallable {
    value: crate::runtime::shared::Shared<
        crate::runtime::function::InvocableFunctionValue<TransferValues>,
    >,
}

impl TransferCallable {
    pub(in crate::runtime) fn new(
        value: crate::runtime::function::InvocableFunctionValue<TransferValues>,
    ) -> Self {
        Self {
            value: crate::runtime::shared::Shared::new(value),
        }
    }

    pub(in crate::runtime) fn with_value<Output>(
        &self,
        use_value: impl FnOnce(
            &crate::runtime::function::InvocableFunctionValue<TransferValues>,
        ) -> Output,
    ) -> Output {
        self.value.read(use_value)
    }
}

impl crate::runtime::function::StoredCallable<TransferValues> for TransferCallable {
    fn from_callable(
        value: crate::runtime::function::InvocableFunctionValue<TransferValues>,
    ) -> Self {
        Self::new(value)
    }

    fn into_evaluated(self) -> crate::runtime::evaluated::EvaluatedFunctionValue<TransferValues> {
        self.value.read(|value| value.clone().into_evaluated())
    }
}

#[derive(Clone)]
pub(crate) struct TransferStoredRuntimeValue {
    value: EvaluatedValue<TransferValues>,
}

impl TransferStoredRuntimeValue {
    pub(in crate::runtime) fn new(value: EvaluatedValue<TransferValues>) -> Self {
        Self { value }
    }

    pub(in crate::runtime) fn value(&self) -> &EvaluatedValue<TransferValues> {
        &self.value
    }
}
