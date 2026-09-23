use std::sync::Arc;

/// A persistent forest of complete binary trees in skew-binary order.
///
/// Only the first two trees may have equal weights. Prepending either links
/// those trees under the new item or adds a leaf. A suffix shares descendants,
/// never a removed ancestor, so it cannot retain an excluded item's payload.
pub(in crate::runtime) struct ListSequence<Item> {
    len: usize,
    first: Spine<Item>,
}

pub(in crate::runtime) struct ListSequenceIter<'a, Item> {
    next: &'a Spine<Item>,
    pending: Vec<&'a Tree<Item>>,
    remaining: usize,
}

enum Spine<Item> {
    Empty,
    Node {
        weight: usize,
        tree: Arc<Tree<Item>>,
        next: Arc<Spine<Item>>,
    },
}

enum Tree<Item> {
    Leaf(Item),
    Branch(Item, Arc<Tree<Item>>, Arc<Tree<Item>>),
}

impl<Item> ListSequence<Item> {
    pub(in crate::runtime) fn len(&self) -> usize {
        self.len
    }

    pub(in crate::runtime) fn get(&self, mut index: usize) -> Option<&Item> {
        let mut current = &self.first;
        while let Spine::Node { weight, tree, next } = current {
            if index < *weight {
                return Some(tree.get(*weight, index));
            }
            index -= weight;
            current = next;
        }
        None
    }

    pub(in crate::runtime) fn prepend(&self, prefix: Vec<Item>) -> Self {
        let mut result = self.clone();
        for item in prefix.into_iter().rev() {
            result.push_front(item);
        }
        result
    }

    pub(in crate::runtime) fn suffix(&self, count: usize) -> Self {
        let count = count.min(self.len);
        let mut remaining = count;
        let mut current = &self.first;
        while let Spine::Node { weight, tree, next } = current {
            if remaining == 0 {
                return Self {
                    len: self.len - count,
                    first: current.clone(),
                };
            }
            if remaining > *weight {
                remaining -= weight;
                current = next;
            } else {
                return Self {
                    len: self.len - count,
                    first: tree.suffix(*weight, remaining, next),
                };
            }
        }
        Self::default()
    }

    pub(in crate::runtime) fn iter(&self) -> ListSequenceIter<'_, Item> {
        ListSequenceIter {
            next: &self.first,
            pending: Vec::new(),
            remaining: self.len,
        }
    }

    fn push_front(&mut self, item: Item) {
        self.first = if let Spine::Node {
            weight: first_weight,
            tree: first,
            next,
        } = &self.first
            && let Spine::Node {
                weight: second_weight,
                tree: second,
                next,
            } = next.as_ref()
            && first_weight == second_weight
        {
            Spine::Node {
                weight: 1 + first_weight + second_weight,
                tree: Arc::new(Tree::Branch(item, Arc::clone(first), Arc::clone(second))),
                next: Arc::clone(next),
            }
        } else {
            Spine::Node {
                weight: 1,
                tree: Arc::new(Tree::Leaf(item)),
                next: Arc::new(self.first.clone()),
            }
        };
        self.len += 1;
    }
}

impl<Item> Default for ListSequence<Item> {
    fn default() -> Self {
        Self {
            len: 0,
            first: Spine::Empty,
        }
    }
}

impl<Item> Clone for ListSequence<Item> {
    fn clone(&self) -> Self {
        Self {
            len: self.len,
            first: self.first.clone(),
        }
    }
}

impl<Item> From<Vec<Item>> for ListSequence<Item> {
    fn from(items: Vec<Item>) -> Self {
        if items.len() < 2 {
            return Self::default().prepend(items);
        }
        let len = items.len();
        // Build the forest before linking its O(log n) spine. Repeated persistent
        // prepending would allocate n temporary spine nodes for this fresh input.
        let mut trees: Vec<(usize, Arc<Tree<Item>>)> = Vec::new();
        for item in items.into_iter().rev() {
            let (weight, tree) = match trees.pop() {
                None => (1, Tree::Leaf(item)),
                Some((first_weight, first)) => match trees.pop() {
                    Some((second_weight, second)) if first_weight == second_weight => (
                        1 + first_weight + second_weight,
                        Tree::Branch(item, first, second),
                    ),
                    Some(second) => {
                        trees.push(second);
                        trees.push((first_weight, first));
                        (1, Tree::Leaf(item))
                    }
                    None => {
                        trees.push((first_weight, first));
                        (1, Tree::Leaf(item))
                    }
                },
            };
            trees.push((weight, Arc::new(tree)));
        }
        let mut first = Spine::Empty;
        for (weight, tree) in trees {
            first = Spine::Node {
                weight,
                tree,
                next: Arc::new(first),
            };
        }
        Self { len, first }
    }
}

impl<Item> Clone for Spine<Item> {
    fn clone(&self) -> Self {
        match self {
            Self::Empty => Self::Empty,
            Self::Node { weight, tree, next } => Self::Node {
                weight: *weight,
                tree: Arc::clone(tree),
                next: Arc::clone(next),
            },
        }
    }
}

impl<Item> Tree<Item> {
    fn get(&self, mut weight: usize, mut index: usize) -> &Item {
        let mut tree = self;
        loop {
            match tree {
                Self::Leaf(item) => return item,
                Self::Branch(item, left, right) => {
                    if index == 0 {
                        return item;
                    }
                    index -= 1;
                    weight /= 2;
                    if index < weight {
                        tree = left;
                    } else {
                        index -= weight;
                        tree = right;
                    }
                }
            }
        }
    }

    // The count is in 0..=weight, including complete removal of a leaf.
    // Reconstruct only the boundary spine, sharing all remaining item trees.
    fn suffix(
        self: &Arc<Self>,
        weight: usize,
        count: usize,
        next: &Arc<Spine<Item>>,
    ) -> Spine<Item> {
        if count == 0 {
            return Spine::Node {
                weight,
                tree: Arc::clone(self),
                next: Arc::clone(next),
            };
        }
        match self.as_ref() {
            Self::Leaf(_) => next.as_ref().clone(),
            Self::Branch(_, left, right) => {
                let half = weight / 2;
                let count = count - 1;
                if count < half {
                    left.suffix(
                        half,
                        count,
                        &Arc::new(Spine::Node {
                            weight: half,
                            tree: Arc::clone(right),
                            next: Arc::clone(next),
                        }),
                    )
                } else {
                    right.suffix(half, count - half, next)
                }
            }
        }
    }
}

impl<'a, Item> Iterator for ListSequenceIter<'a, Item> {
    type Item = &'a Item;

    fn next(&mut self) -> Option<Self::Item> {
        let tree = match self.pending.pop() {
            Some(tree) => tree,
            None => match self.next {
                Spine::Empty => return None,
                Spine::Node { tree, next, .. } => {
                    self.next = next;
                    tree
                }
            },
        };
        self.remaining -= 1;
        match tree {
            Tree::Leaf(item) => Some(item),
            Tree::Branch(item, left, right) => {
                self.pending.push(right);
                self.pending.push(left);
                Some(item)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

#[cfg(test)]
mod tests {
    use super::ListSequence;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::{ptr, thread};

    #[test]
    fn every_suffix_and_prepend_preserves_order_and_indexed_access() {
        for len in 0..=128 {
            let expected: Vec<_> = (0..len).collect();
            let original = ListSequence::from(expected.clone());
            for count in 0..=len + 1 {
                let suffix = original.suffix(count);
                let retained = suffix.clone();
                let remaining = &expected[count.min(len)..];
                assert_eq!(suffix.len(), remaining.len());
                assert_eq!(suffix.iter().copied().collect::<Vec<_>>(), remaining);
                for index in 0..=remaining.len() {
                    assert_eq!(suffix.get(index), remaining.get(index));
                }
                assert_eq!(suffix.get(usize::MAX), None);
                let prepended = suffix.prepend(vec![len + 1, len + 2]);
                let combined: Vec<_> = [len + 1, len + 2]
                    .into_iter()
                    .chain(remaining.iter().copied())
                    .collect();
                assert_eq!(prepended.iter().copied().collect::<Vec<_>>(), combined);
                for (index, item) in combined.iter().enumerate() {
                    assert_eq!(prepended.get(index), Some(item));
                }
                assert_eq!(retained.iter().copied().collect::<Vec<_>>(), remaining);
            }
            assert_eq!(original.iter().copied().collect::<Vec<_>>(), expected);
            assert_eq!(original.suffix(usize::MAX).len(), 0);
        }
    }

    #[test]
    fn cursor_reports_remaining_length_across_tree_boundaries() {
        for len in [0, 1, 2, 3, 4, 7, 8, 31, 32, 1_000] {
            let values = ListSequence::from((0..len).collect::<Vec<_>>());
            let mut cursor = values.iter();
            assert_eq!(cursor.size_hint(), (len, Some(len)));
            for expected in 0..len {
                assert_eq!(cursor.next(), Some(&expected));
                let remaining = len - expected - 1;
                assert_eq!(cursor.size_hint(), (remaining, Some(remaining)));
            }
            assert_eq!(cursor.next(), None);
            assert_eq!(cursor.next(), None);
            assert_eq!(cursor.size_hint(), (0, Some(0)));
        }
    }

    #[test]
    fn suffix_shares_non_clone_items_without_retaining_removed_payloads() {
        struct Item {
            id: usize,
            drops: Arc<Vec<AtomicUsize>>,
        }

        impl Drop for Item {
            fn drop(&mut self) {
                self.drops[self.id].fetch_add(1, Ordering::Relaxed);
            }
        }

        for len in [1, 2, 3, 4, 7, 8, 15, 16, 63, 64, 65, 127, 128, 129, 1_000] {
            for count in [0, 1, len / 2, len, usize::MAX] {
                let drops = Arc::new((0..len).map(|_| AtomicUsize::new(0)).collect::<Vec<_>>());
                let original = ListSequence::from(
                    (0..len)
                        .map(|id| Item {
                            id,
                            drops: Arc::clone(&drops),
                        })
                        .collect::<Vec<_>>(),
                );
                let alias = original.clone();
                let suffix = original.suffix(count);
                for (index, item) in suffix.iter().enumerate() {
                    assert!(ptr::eq(
                        item,
                        original.get(count + index).expect("retained source item")
                    ));
                }
                drop(original);
                assert!(drops.iter().all(|count| count.load(Ordering::Relaxed) == 0));
                drop(alias);
                for (index, drops) in drops.iter().enumerate() {
                    assert_eq!(drops.load(Ordering::Relaxed), usize::from(index < count));
                }
                drop(suffix);
                assert!(drops.iter().all(|count| count.load(Ordering::Relaxed) == 1));
            }
        }
    }

    #[test]
    fn large_sequences_drop_on_a_small_stack_and_cursors_can_be_shared() {
        thread::Builder::new()
            .stack_size(128 * 1_024)
            .spawn(|| {
                let values = ListSequence::from((0..100_000).collect::<Vec<_>>());
                let suffix = values.suffix(1);
                let sum = thread::scope(|scope| {
                    let mut cursor = suffix.iter();
                    scope
                        .spawn(move || cursor.by_ref().copied().sum::<u64>())
                        .join()
                        .expect("cursor worker")
                });
                assert_eq!(sum, 4_999_950_000);
                drop(values);
                drop(suffix);
            })
            .expect("small-stack worker")
            .join()
            .expect("bounded drop stack");
    }
}
