use std::ops::Deref;

pub enum Storage<Data: ?Sized + 'static> {
    Owned(Box<Data>),
    Static(&'static Data),
}

pub(crate) type Table<Item> = Storage<[Item]>;
pub(crate) type Node<Value> = Storage<Value>;

impl<Item> Storage<[Item]> {
    pub(in crate::plan::execution) fn into_vec(self) -> Vec<Item>
    where
        Item: Clone,
    {
        match self {
            Self::Owned(items) => items.into_vec(),
            Self::Static(items) => items.to_vec(),
        }
    }
}

impl<Data: ?Sized> Deref for Storage<Data> {
    type Target = Data;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Owned(items) => items,
            Self::Static(items) => items,
        }
    }
}

impl<Data: ?Sized> AsRef<Data> for Storage<Data> {
    fn as_ref(&self) -> &Data {
        self
    }
}

impl<Item> From<Vec<Item>> for Storage<[Item]> {
    fn from(items: Vec<Item>) -> Self {
        Self::Owned(items.into_boxed_slice())
    }
}

impl<Data: ?Sized> From<Box<Data>> for Storage<Data> {
    fn from(data: Box<Data>) -> Self {
        Self::Owned(data)
    }
}

impl<Item> FromIterator<Item> for Storage<[Item]> {
    fn from_iter<Items: IntoIterator<Item = Item>>(items: Items) -> Self {
        Self::from(items.into_iter().collect::<Vec<_>>())
    }
}

impl<Data: ?Sized> Clone for Storage<Data>
where
    Box<Data>: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Self::Owned(items) => Self::Owned(items.clone()),
            Self::Static(items) => Self::Static(items),
        }
    }
}

impl<Data: std::fmt::Debug + ?Sized> std::fmt::Debug for Storage<Data> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.deref().fmt(formatter)
    }
}

impl<Data: PartialEq + ?Sized> PartialEq for Storage<Data> {
    fn eq(&self, other: &Self) -> bool {
        self.deref() == other.deref()
    }
}

impl<Data: Eq + ?Sized> Eq for Storage<Data> {}

impl<Data: PartialOrd + ?Sized> PartialOrd for Storage<Data> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.deref().partial_cmp(other.deref())
    }
}

impl<Data: Ord + ?Sized> Ord for Storage<Data> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.deref().cmp(other.deref())
    }
}

impl<Data: std::hash::Hash + ?Sized> std::hash::Hash for Storage<Data> {
    fn hash<Hasher: std::hash::Hasher>(&self, state: &mut Hasher) {
        self.deref().hash(state);
    }
}

impl<'table, Item> IntoIterator for &'table Storage<[Item]> {
    type Item = &'table Item;
    type IntoIter = std::slice::Iter<'table, Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::{Node, Table};
    use std::hash::{Hash, Hasher};
    use std::sync::Arc;

    #[test]
    fn ordering_compares_contents_and_preserves_partial_ordering() {
        use std::cmp::Ordering;
        let owned = Table::from(vec![2, 3]);
        let borrowed = Table::Static(&[2, 3]);
        assert_eq!(owned.cmp(&borrowed), Ordering::Equal);
        assert_eq!(
            owned.partial_cmp(&Table::Static(&[2, 5])),
            Some(Ordering::Less)
        );
        assert_eq!(Table::Static(&[2, 3, 5]).cmp(&owned), Ordering::Greater);
        assert_eq!(
            Node::Static(&f64::NAN).partial_cmp(&Node::Static(&1.0)),
            None
        );
    }

    #[test]
    fn owned_tables_move_allocations_and_release_non_clone_items() {
        struct Item {
            owner: Arc<()>,
        }

        let owner = Arc::new(());
        let items = vec![Item {
            owner: owner.clone(),
        }]
        .into_boxed_slice();
        let pointer = items.as_ptr();
        let table = Table::from(items);
        assert!(std::ptr::eq(table.as_ptr(), pointer));
        assert!(Arc::ptr_eq(&table[0].owner, &owner));
        assert_eq!(Arc::strong_count(&owner), 2);
        drop(table);
        assert_eq!(Arc::strong_count(&owner), 1);
    }

    #[test]
    fn static_reads_and_clones_borrow_the_original_array() {
        static ITEMS: [u32; 3] = [2, 3, 5];
        let table = Table::Static(&ITEMS);
        let copied = table.clone();
        assert!(std::ptr::eq(table.as_ptr(), ITEMS.as_ptr()));
        assert!(std::ptr::eq(copied.as_ptr(), ITEMS.as_ptr()));
        assert_eq!(
            (&table).into_iter().copied().collect::<Vec<_>>(),
            vec![2, 3, 5]
        );
    }

    #[test]
    fn explicit_owned_conversion_moves_owned_items_and_copies_static_items() {
        let items = vec![7u32, 11];
        let pointer = items.as_ptr();
        let table = Table::from(items);
        let cloned = table.clone();
        assert!(!std::ptr::eq(cloned.as_ptr(), pointer));
        assert_eq!(&*cloned, &[7, 11]);
        let items = table.into_vec();
        assert!(std::ptr::eq(items.as_ptr(), pointer));
        assert_eq!(items, vec![7, 11]);

        static ITEMS: [u32; 2] = [13, 17];
        let items = Table::Static(&ITEMS).into_vec();
        assert!(!std::ptr::eq(items.as_ptr(), ITEMS.as_ptr()));
        assert_eq!(items, vec![13, 17]);
    }

    #[test]
    fn comparison_hash_and_debug_describe_contents_not_storage() {
        static ITEMS: [u32; 2] = [19, 23];
        let owned = Table::from(vec![19u32, 23]);
        let borrowed = Table::Static(&ITEMS);
        assert_eq!(owned, borrowed);
        assert_ne!(owned, Table::from(vec![19u32, 29]));
        assert_eq!(format!("{owned:?}"), "[19, 23]");
        assert_eq!(format!("{borrowed:?}"), "[19, 23]");

        let mut expected = std::collections::hash_map::DefaultHasher::new();
        ITEMS.as_slice().hash(&mut expected);
        for table in [&owned, &borrowed] {
            let mut actual = std::collections::hash_map::DefaultHasher::new();
            table.hash(&mut actual);
            assert_eq!(actual.finish(), expected.finish());
        }
    }

    #[test]
    fn fixed_nodes_borrow_static_data_or_retain_the_owned_allocation() {
        static VALUE: (u32, bool) = (42, true);
        let borrowed = Node::Static(&VALUE);
        let value = Box::new((42u32, true));
        let pointer = &*value as *const _;
        let owned = Node::from(value);
        assert!(std::ptr::eq(&*borrowed, &VALUE));
        assert!(std::ptr::eq(borrowed.as_ref(), &VALUE));
        assert!(std::ptr::eq(&*owned, pointer));
        assert_eq!(owned, borrowed);
        assert_eq!(*borrowed, (42, true));
        assert_eq!(*owned, (42, true));
    }
}
