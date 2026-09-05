mod external;
mod list;

pub(crate) use crate::host::{
    TransferExternalEquality, TransferExternalHashing, TransferExternalInspection,
};
pub(crate) use external::TransferExternalPayloadLease;
pub(crate) use external::TransferExternalStore;
pub(in crate::runtime) use list::TransferListHandleCore;
pub(crate) use list::TransferListStorage;

use crate::runtime::{EvaluatedValue, TransferValues};

#[derive(Clone)]
pub(crate) struct TransferStoredRuntimeValue {
    value: EvaluatedValue<TransferValues>,
}

impl TransferStoredRuntimeValue {
    pub(crate) fn external(value: crate::runtime::EvaluatedExternalValue<TransferValues>) -> Self {
        Self::new(EvaluatedValue::External(value))
    }

    pub(in crate::runtime) fn new(value: EvaluatedValue<TransferValues>) -> Self {
        Self { value }
    }

    pub(in crate::runtime) fn value(&self) -> &EvaluatedValue<TransferValues> {
        &self.value
    }
}
