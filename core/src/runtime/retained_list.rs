use super::evaluated::EvaluatedValue;
use super::state::list::ListValueId;
use super::{LocalValues, RuntimeListStorage, RuntimeValueProfile};

pub(in crate::runtime) struct RetainedList<Handle, Profile: RuntimeValueProfile = LocalValues> {
    value: Handle,
    profile: std::marker::PhantomData<fn() -> Profile>,
    #[cfg(test)]
    item_reads: std::cell::Cell<usize>,
}

impl<Handle, Profile> RetainedList<Handle, Profile>
where
    Handle: Clone + Into<ListValueId<Profile>>,
    Profile: RuntimeValueProfile,
{
    pub(in crate::runtime) fn new(value: Handle) -> Self {
        Self {
            value,
            profile: std::marker::PhantomData,
            #[cfg(test)]
            item_reads: std::cell::Cell::new(0),
        }
    }

    pub(in crate::runtime) fn len(&self) -> usize {
        Profile::ListStorage::default().list_len(&self.value.clone().into())
    }

    pub(in crate::runtime) fn item(&self, index: usize) -> Option<EvaluatedValue<Profile>> {
        #[cfg(test)]
        self.item_reads.set(self.item_reads.get() + 1);
        Profile::ListStorage::default().evaluated_value_at(&self.value.clone().into(), index)
    }

    pub(in crate::runtime) fn handle(&self) -> &Handle {
        &self.value
    }

    #[cfg(test)]
    pub(in crate::runtime) fn item_reads(&self) -> usize {
        self.item_reads.get()
    }
}
