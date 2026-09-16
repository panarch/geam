use super::{BlockEnvironment, RetainedValues};
use crate::plan::execution::graph::{StorageFamily, Transfer};

impl BlockEnvironment {
    pub(in crate::runtime::graph) fn into_retained(self, transfer: &Transfer) -> RetainedValues {
        let mut retained = RetainedValues {
            values: self.values,
        };
        retained.transfer(transfer);
        retained
    }

    pub(in crate::runtime::graph) fn into_match_retained(
        self,
        transfer: &Transfer,
        selected: &[usize],
        bindings: super::super::pattern::MatchBindings,
    ) -> RetainedValues {
        let mut retained = RetainedValues {
            values: self.values,
        };
        let mut selected = selected.iter().copied().peekable();
        for (index, value) in bindings.into_values().into_iter().enumerate() {
            if selected.peek() == Some(&index) {
                retained.push_evaluated(value);
                selected.next();
            }
        }
        retained.transfer(transfer);
        retained
    }
}

impl RetainedValues {
    fn transfer(&mut self, transfer: &Transfer) {
        for route in transfer.families.iter() {
            let positions = &route.positions;
            match route.family {
                StorageFamily::Int => rearrange(&mut self.values.ints, positions),
                StorageFamily::Float => rearrange(&mut self.values.floats, positions),
                StorageFamily::String => rearrange(&mut self.values.strings, positions),
                StorageFamily::BitArray => rearrange(&mut self.values.bit_arrays, positions),
                StorageFamily::UtfCodepoint => {
                    rearrange(&mut self.values.utf_codepoints, positions)
                }
                StorageFamily::Custom => rearrange(&mut self.values.customs, positions),
                StorageFamily::External => rearrange(&mut self.values.externals, positions),
                StorageFamily::Bool => rearrange(&mut self.values.bools, positions),
                StorageFamily::Tuple => rearrange(&mut self.values.tuples, positions),
                StorageFamily::ParameterList => {
                    rearrange(&mut self.values.parameter_lists, positions)
                }
                StorageFamily::ParameterListList => {
                    rearrange(&mut self.values.parameter_list_lists, positions)
                }
                StorageFamily::IntList => rearrange(&mut self.values.int_lists, positions),
                StorageFamily::StringList => rearrange(&mut self.values.string_lists, positions),
                StorageFamily::BitArrayList => {
                    rearrange(&mut self.values.bit_array_lists, positions)
                }
                StorageFamily::UtfCodepointList => {
                    rearrange(&mut self.values.utf_codepoint_lists, positions)
                }
                StorageFamily::CustomList => rearrange(&mut self.values.custom_lists, positions),
                StorageFamily::ExternalList => {
                    rearrange(&mut self.values.external_lists, positions)
                }
                StorageFamily::FloatList => rearrange(&mut self.values.float_lists, positions),
                StorageFamily::BoolList => rearrange(&mut self.values.bool_lists, positions),
                StorageFamily::NilList => rearrange(&mut self.values.nil_lists, positions),
                StorageFamily::TupleList => rearrange(&mut self.values.tuple_lists, positions),
                StorageFamily::ListList => rearrange(&mut self.values.list_lists, positions),
                StorageFamily::FunctionList => {
                    rearrange(&mut self.values.function_lists, positions)
                }
                StorageFamily::IntFunction => rearrange(&mut self.values.int_functions, positions),
                StorageFamily::FloatFunction => {
                    rearrange(&mut self.values.float_functions, positions)
                }
                StorageFamily::StringFunction => {
                    rearrange(&mut self.values.string_functions, positions)
                }
                StorageFamily::BitArrayFunction => {
                    rearrange(&mut self.values.bit_array_functions, positions)
                }
                StorageFamily::UtfCodepointFunction => {
                    rearrange(&mut self.values.utf_codepoint_functions, positions)
                }
                StorageFamily::CustomFunction => {
                    rearrange(&mut self.values.custom_functions, positions)
                }
                StorageFamily::ExternalFunction => {
                    rearrange(&mut self.values.external_functions, positions)
                }
                StorageFamily::BoolFunction => {
                    rearrange(&mut self.values.bool_functions, positions)
                }
                StorageFamily::NilFunction => rearrange(&mut self.values.nil_functions, positions),
                StorageFamily::TupleFunction => {
                    rearrange(&mut self.values.tuple_functions, positions)
                }
                StorageFamily::GenericFunction => {
                    rearrange(&mut self.values.generic_functions, positions)
                }
                StorageFamily::NeverFunction => {
                    rearrange(&mut self.values.never_functions, positions)
                }
                StorageFamily::ParameterListFunction => {
                    rearrange(&mut self.values.parameter_list_functions, positions)
                }
                StorageFamily::ParameterListListFunction => {
                    rearrange(&mut self.values.parameter_list_list_functions, positions)
                }
                StorageFamily::IntListFunction => {
                    rearrange(&mut self.values.int_list_functions, positions)
                }
                StorageFamily::StringListFunction => {
                    rearrange(&mut self.values.string_list_functions, positions)
                }
                StorageFamily::BitArrayListFunction => {
                    rearrange(&mut self.values.bit_array_list_functions, positions)
                }
                StorageFamily::UtfCodepointListFunction => {
                    rearrange(&mut self.values.utf_codepoint_list_functions, positions)
                }
                StorageFamily::CustomListFunction => {
                    rearrange(&mut self.values.custom_list_functions, positions)
                }
                StorageFamily::ExternalListFunction => {
                    rearrange(&mut self.values.external_list_functions, positions)
                }
                StorageFamily::FloatListFunction => {
                    rearrange(&mut self.values.float_list_functions, positions)
                }
                StorageFamily::BoolListFunction => {
                    rearrange(&mut self.values.bool_list_functions, positions)
                }
                StorageFamily::NilListFunction => {
                    rearrange(&mut self.values.nil_list_functions, positions)
                }
                StorageFamily::TupleListFunction => {
                    rearrange(&mut self.values.tuple_list_functions, positions)
                }
                StorageFamily::ListListFunction => {
                    rearrange(&mut self.values.list_list_functions, positions)
                }
                StorageFamily::FunctionListFunction => {
                    rearrange(&mut self.values.function_list_functions, positions)
                }
                StorageFamily::CoreFunctionFunction => {
                    rearrange(&mut self.values.core_function_functions, positions)
                }
                StorageFamily::ExternalFunctionFunction => {
                    rearrange(&mut self.values.external_function_functions, positions)
                }
            }
        }
    }
}

// The placed prefix holds the first retained use; only further uses need a copy.
fn rearrange<Value: Clone>(values: &mut Vec<Value>, positions: &[usize]) {
    for (destination, &source) in positions.iter().enumerate() {
        if source < destination {
            values.push(values[source].clone());
            let last = values.len() - 1;
            values.swap(last, destination);
        } else {
            values.swap(source, destination);
        }
    }
    values.truncate(positions.len());
}

#[cfg(test)]
mod tests {
    use super::rearrange;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct Tracked {
        id: usize,
        copies: Arc<AtomicUsize>,
        live: Arc<[AtomicUsize; 4]>,
    }

    impl Clone for Tracked {
        fn clone(&self) -> Self {
            self.copies.fetch_add(1, Ordering::Relaxed);
            self.live[self.id].fetch_add(1, Ordering::Relaxed);
            Self {
                id: self.id,
                copies: self.copies.clone(),
                live: self.live.clone(),
            }
        }
    }

    impl Drop for Tracked {
        fn drop(&mut self) {
            self.live[self.id].fetch_sub(1, Ordering::Relaxed);
        }
    }

    #[test]
    fn moves_first_uses_copies_repeats_and_releases_omitted_payloads_at_handoff() {
        let copies = Arc::new(AtomicUsize::new(0));
        let live = Arc::new(std::array::from_fn(|_| AtomicUsize::new(1)));
        let mut values = (0..4)
            .map(|id| Tracked {
                id,
                copies: copies.clone(),
                live: live.clone(),
            })
            .collect::<Vec<_>>();
        values.reserve(4);
        let buffer = values.as_ptr();
        let capacity = values.capacity();

        rearrange(&mut values, &[2, 2, 0]);

        assert_eq!(
            values.iter().map(|value| value.id).collect::<Vec<_>>(),
            [2, 0, 2]
        );
        assert_eq!(copies.load(Ordering::Relaxed), 1);
        assert_eq!(
            live.each_ref().map(|count| count.load(Ordering::Relaxed)),
            [1, 0, 2, 0]
        );
        assert_eq!(values.as_ptr(), buffer);
        assert_eq!(values.capacity(), capacity);
        drop(values);
        assert_eq!(
            live.each_ref().map(|count| count.load(Ordering::Relaxed)),
            [0; 4]
        );
    }

    #[test]
    fn fixed_width_transitions_reuse_the_same_buffer_without_copies() {
        let copies = Arc::new(AtomicUsize::new(0));
        let live = Arc::new(std::array::from_fn(|id| {
            AtomicUsize::new(usize::from(id < 3))
        }));
        let mut values = (0..3)
            .map(|id| Tracked {
                id,
                copies: copies.clone(),
                live: live.clone(),
            })
            .collect::<Vec<_>>();
        let buffer = values.as_ptr();
        let capacity = values.capacity();
        for _ in 0..1024 {
            rearrange(&mut values, &[2, 2, 2]);
            assert_eq!(values.as_ptr(), buffer);
            assert_eq!(values.capacity(), capacity);
        }
        assert_eq!(
            values.iter().map(|value| value.id).collect::<Vec<_>>(),
            [2, 0, 1]
        );
        assert_eq!(copies.load(Ordering::Relaxed), 0);
        drop(values);
        assert_eq!(
            live.each_ref().map(|count| count.load(Ordering::Relaxed)),
            [0; 4]
        );
    }

    #[test]
    fn shrinking_and_empty_outputs_retain_capacity_but_not_payloads() {
        let copies = Arc::new(AtomicUsize::new(0));
        let live = Arc::new(std::array::from_fn(|id| {
            AtomicUsize::new(usize::from(id == 0))
        }));
        let mut values = vec![Tracked {
            id: 0,
            copies: copies.clone(),
            live: live.clone(),
        }];
        rearrange(&mut values, &[0, 0, 0, 0, 0]);
        assert_eq!(copies.load(Ordering::Relaxed), 4);
        assert_eq!(live[0].load(Ordering::Relaxed), 5);
        let buffer = values.as_ptr();
        let capacity = values.capacity();

        rearrange(&mut values, &[4]);
        assert_eq!(live[0].load(Ordering::Relaxed), 1);
        assert_eq!(values[0].id, 0);
        rearrange(&mut values, &[]);
        assert!(values.is_empty());
        assert_eq!(live[0].load(Ordering::Relaxed), 0);
        assert_eq!(values.as_ptr(), buffer);
        assert_eq!(values.capacity(), capacity);
    }
}
