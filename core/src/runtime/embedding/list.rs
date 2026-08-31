use super::input::EmbeddingListInput;
use super::output::EmbeddingOutput;
use crate::runtime::retained_list::RetainedList;
use crate::runtime::state::list::StoredListValueId;

pub(crate) struct EmbeddingList {
    retained: RetainedList<StoredListValueId>,
}

impl EmbeddingList {
    pub(super) fn new(value: StoredListValueId) -> Self {
        Self {
            retained: RetainedList::new(value),
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.retained.len()
    }

    pub(crate) fn item(&self, index: usize) -> Option<EmbeddingOutput> {
        self.retained.item(index).map(EmbeddingOutput::from_value)
    }

    pub(crate) fn input(&self) -> EmbeddingListInput {
        EmbeddingListInput(self.retained.handle().clone())
    }

    #[cfg(test)]
    pub(crate) fn item_reads(&self) -> usize {
        self.retained.item_reads()
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
