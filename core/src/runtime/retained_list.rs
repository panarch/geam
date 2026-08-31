use super::evaluated::EvaluatedValue;
use super::state::list::{ListValueId, RuntimeListReader};

pub(in crate::runtime) struct RetainedList<Handle> {
    value: Handle,
    #[cfg(test)]
    item_reads: std::cell::Cell<usize>,
}

impl<Handle: Clone + Into<ListValueId>> RetainedList<Handle> {
    pub(in crate::runtime) fn new(value: Handle) -> Self {
        Self {
            value,
            #[cfg(test)]
            item_reads: std::cell::Cell::new(0),
        }
    }

    pub(in crate::runtime) fn len(&self) -> usize {
        RuntimeListReader.list_len(&self.value.clone().into())
    }

    pub(in crate::runtime) fn item(&self, index: usize) -> Option<EvaluatedValue> {
        #[cfg(test)]
        self.item_reads.set(self.item_reads.get() + 1);
        RuntimeListReader.evaluated_value_at(&self.value.clone().into(), index)
    }

    pub(in crate::runtime) fn handle(&self) -> &Handle {
        &self.value
    }

    #[cfg(test)]
    pub(in crate::runtime) fn item_reads(&self) -> usize {
        self.item_reads.get()
    }
}
