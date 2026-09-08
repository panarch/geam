use super::input::EmbeddingListInput;
use super::output::EmbeddingOutput;
use crate::runtime::retained_list::RetainedList;
use crate::runtime::state::list::StoredListValueId;
use crate::runtime::{LocalValues, RuntimeValueProfile};

pub(crate) struct EmbeddingList<Profile: RuntimeValueProfile = LocalValues> {
    retained: RetainedList<StoredListValueId<Profile>, Profile>,
}

impl<Profile: RuntimeValueProfile> EmbeddingList<Profile> {
    pub(crate) fn from_borrowed(value: crate::runtime::BorrowedValue<'_, Profile>) -> Self {
        Self::new(value.stored_list().clone())
    }

    pub(super) fn new(value: StoredListValueId<Profile>) -> Self {
        Self {
            retained: RetainedList::new(value),
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.retained.len()
    }

    pub(crate) fn item(&self, index: usize) -> Option<EmbeddingOutput<Profile>> {
        self.retained.item(index).map(EmbeddingOutput::from_value)
    }

    pub(crate) fn input(&self) -> EmbeddingListInput<Profile> {
        EmbeddingListInput(self.retained.handle().clone())
    }

    #[cfg(test)]
    pub(crate) fn item_reads(&self) -> usize {
        self.retained.item_reads()
    }

    #[cfg(test)]
    pub(crate) fn same_allocation(&self, other: &Self) -> bool {
        self.retained.handle() == other.retained.handle()
    }
}

impl EmbeddingList<crate::runtime::TransferValues> {
    pub(crate) fn read_item<Output>(
        &self,
        index: usize,
        read: impl FnOnce(crate::runtime::BorrowedValue<'_, crate::runtime::TransferValues>) -> Output,
    ) -> Option<Output> {
        crate::runtime::BorrowedValue::read_list_item(self.retained.handle(), index, read)
    }
}

#[cfg(test)]
mod tests {
    use super::EmbeddingList;
    use crate::runtime::state::list::{RuntimeListStorage, StoredListValueId};

    #[test]
    fn retained_input_preserves_the_exact_allocation_without_reading_items() {
        let plan = crate::runtime::plan_src("pub fn main() -> List(Int) { [1, 2] }");
        let mut storage = RuntimeListStorage::default();
        let allocation: StoredListValueId = storage
            .int(
                plan.int_list_function_id(0).type_id(),
                vec![1.into(), 2.into()],
            )
            .into();
        let value = EmbeddingList::new(allocation.clone());
        let input = value.input();

        assert_eq!(input.0, allocation);
        assert_eq!(value.item_reads(), 0);
        drop(allocation);
        drop(storage);
        assert_eq!(value.len(), 2);
        assert_eq!(value.item_reads(), 0);
        assert_eq!(value.item(1).expect("second item").take_int(), 2.into());
        assert_eq!(value.item_reads(), 1);
        drop(value);
        let retained = EmbeddingList::new(input.0);
        assert_eq!(retained.len(), 2);
        assert_eq!(retained.item(0).expect("first item").take_int(), 1.into());
    }
}
