use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

pub(crate) struct ConstantId<Value> {
    index: usize,
    value: PhantomData<fn() -> Value>,
}

impl<Value> ConstantId<Value> {
    pub(in crate::plan::execution) fn new(index: usize) -> Self {
        Self {
            index,
            value: PhantomData,
        }
    }

    pub(in crate::plan::execution) fn index(self) -> usize {
        self.index
    }
}

impl<Value> Clone for ConstantId<Value> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Value> Copy for ConstantId<Value> {}

impl<Value> std::fmt::Debug for ConstantId<Value> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_tuple("ConstantId")
            .field(&self.index)
            .finish()
    }
}

impl<Value> PartialEq for ConstantId<Value> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl<Value> Eq for ConstantId<Value> {}

impl<Value> Hash for ConstantId<Value> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::ConstantId;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct UncomparableValue;

    #[test]
    fn traits_depend_only_on_the_typed_index() {
        fn assert_copy<T: Copy>() {}

        assert_copy::<ConstantId<UncomparableValue>>();
        let id = ConstantId::<UncomparableValue>::new(3);
        let copied = id;
        let cloned = <ConstantId<UncomparableValue> as Clone>::clone(&id);
        let different = ConstantId::<UncomparableValue>::new(4);

        assert_eq!(copied, id);
        assert_eq!(cloned, id);
        assert_ne!(id, different);
        assert_eq!(format!("{id:?}"), "ConstantId(3)");

        let mut id_hasher = DefaultHasher::new();
        id.hash(&mut id_hasher);
        let mut copied_hasher = DefaultHasher::new();
        copied.hash(&mut copied_hasher);
        let mut different_hasher = DefaultHasher::new();
        different.hash(&mut different_hasher);

        assert_eq!(id_hasher.finish(), copied_hasher.finish());
        assert_ne!(id_hasher.finish(), different_hasher.finish());
    }
}
