use super::evaluated::EvaluatedValue;
use super::state::list::ListValueId;

pub(in crate::runtime) struct RetainedList<Handle> {
    value: Handle,
    #[cfg(test)]
    item_reads: std::cell::Cell<usize>,
}

impl<Handle> RetainedList<Handle>
where
    Handle: Clone + Into<ListValueId>,
{
    pub(in crate::runtime) fn new(value: Handle) -> Self {
        Self {
            value,
            #[cfg(test)]
            item_reads: std::cell::Cell::new(0),
        }
    }

    pub(in crate::runtime) fn len(&self) -> usize {
        crate::runtime::RuntimeListStorage::default().list_len(&self.value.clone().into())
    }

    pub(in crate::runtime) fn item(&self, index: usize) -> Option<EvaluatedValue> {
        #[cfg(test)]
        self.item_reads.set(self.item_reads.get() + 1);
        crate::runtime::RuntimeListStorage::default()
            .evaluated_value_at(&self.value.clone().into(), index)
    }

    pub(in crate::runtime) fn handle(&self) -> &Handle {
        &self.value
    }

    #[cfg(test)]
    pub(in crate::runtime) fn item_reads(&self) -> usize {
        self.item_reads.get()
    }
}
