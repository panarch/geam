use super::{BlockEnvironment, RetainedValues};
use crate::plan::execution::graph::{FamilyTransfer, StorageFamily, Transfer, TransferStep};
use crate::runtime::graph::pattern::MatchBindings;

impl BlockEnvironment {
    pub(in crate::runtime::graph) fn into_retained(self, transfer: &Transfer) -> RetainedValues {
        let mut retained = RetainedValues {
            values: self.values,
            callable_domain: None,
        };
        retained.transfer(transfer);
        retained
    }

    pub(in crate::runtime::graph) fn into_match_retained(
        self,
        transfer: &Transfer,
        selected: &[usize],
        bindings: MatchBindings,
    ) -> RetainedValues {
        let mut retained = RetainedValues {
            values: self.values,
            callable_domain: None,
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
            match route.family {
                StorageFamily::Int => rearrange(&mut self.values.ints, route),
                StorageFamily::Float => rearrange(&mut self.values.floats, route),
                StorageFamily::String => rearrange(&mut self.values.strings, route),
                StorageFamily::BitArray => rearrange(&mut self.values.bit_arrays, route),
                StorageFamily::UtfCodepoint => rearrange(&mut self.values.utf_codepoints, route),
                StorageFamily::Custom => rearrange(&mut self.values.customs, route),
                StorageFamily::External => rearrange(&mut self.values.externals, route),
                StorageFamily::Bool => rearrange(&mut self.values.bools, route),
                StorageFamily::Tuple => rearrange(&mut self.values.tuples, route),
                StorageFamily::ParameterList => rearrange(&mut self.values.parameter_lists, route),
                StorageFamily::ParameterListList => {
                    rearrange(&mut self.values.parameter_list_lists, route)
                }
                StorageFamily::IntList => rearrange(&mut self.values.int_lists, route),
                StorageFamily::StringList => rearrange(&mut self.values.string_lists, route),
                StorageFamily::BitArrayList => rearrange(&mut self.values.bit_array_lists, route),
                StorageFamily::UtfCodepointList => {
                    rearrange(&mut self.values.utf_codepoint_lists, route)
                }
                StorageFamily::CustomList => rearrange(&mut self.values.custom_lists, route),
                StorageFamily::ExternalList => rearrange(&mut self.values.external_lists, route),
                StorageFamily::FloatList => rearrange(&mut self.values.float_lists, route),
                StorageFamily::BoolList => rearrange(&mut self.values.bool_lists, route),
                StorageFamily::NilList => rearrange(&mut self.values.nil_lists, route),
                StorageFamily::TupleList => rearrange(&mut self.values.tuple_lists, route),
                StorageFamily::ListList => rearrange(&mut self.values.list_lists, route),
                StorageFamily::FunctionList => rearrange(&mut self.values.function_lists, route),
                StorageFamily::IntFunction => rearrange(&mut self.values.int_functions, route),
                StorageFamily::FloatFunction => rearrange(&mut self.values.float_functions, route),
                StorageFamily::StringFunction => {
                    rearrange(&mut self.values.string_functions, route)
                }
                StorageFamily::BitArrayFunction => {
                    rearrange(&mut self.values.bit_array_functions, route)
                }
                StorageFamily::UtfCodepointFunction => {
                    rearrange(&mut self.values.utf_codepoint_functions, route)
                }
                StorageFamily::CustomFunction => {
                    rearrange(&mut self.values.custom_functions, route)
                }
                StorageFamily::ExternalFunction => {
                    rearrange(&mut self.values.external_functions, route)
                }
                StorageFamily::BoolFunction => rearrange(&mut self.values.bool_functions, route),
                StorageFamily::NilFunction => rearrange(&mut self.values.nil_functions, route),
                StorageFamily::TupleFunction => rearrange(&mut self.values.tuple_functions, route),
                StorageFamily::GenericFunction => {
                    rearrange(&mut self.values.generic_functions, route)
                }
                StorageFamily::NeverFunction => rearrange(&mut self.values.never_functions, route),
                StorageFamily::ParameterListFunction => {
                    rearrange(&mut self.values.parameter_list_functions, route)
                }
                StorageFamily::ParameterListListFunction => {
                    rearrange(&mut self.values.parameter_list_list_functions, route)
                }
                StorageFamily::IntListFunction => {
                    rearrange(&mut self.values.int_list_functions, route)
                }
                StorageFamily::StringListFunction => {
                    rearrange(&mut self.values.string_list_functions, route)
                }
                StorageFamily::BitArrayListFunction => {
                    rearrange(&mut self.values.bit_array_list_functions, route)
                }
                StorageFamily::UtfCodepointListFunction => {
                    rearrange(&mut self.values.utf_codepoint_list_functions, route)
                }
                StorageFamily::CustomListFunction => {
                    rearrange(&mut self.values.custom_list_functions, route)
                }
                StorageFamily::ExternalListFunction => {
                    rearrange(&mut self.values.external_list_functions, route)
                }
                StorageFamily::FloatListFunction => {
                    rearrange(&mut self.values.float_list_functions, route)
                }
                StorageFamily::BoolListFunction => {
                    rearrange(&mut self.values.bool_list_functions, route)
                }
                StorageFamily::NilListFunction => {
                    rearrange(&mut self.values.nil_list_functions, route)
                }
                StorageFamily::TupleListFunction => {
                    rearrange(&mut self.values.tuple_list_functions, route)
                }
                StorageFamily::ListListFunction => {
                    rearrange(&mut self.values.list_list_functions, route)
                }
                StorageFamily::FunctionListFunction => {
                    rearrange(&mut self.values.function_list_functions, route)
                }
                StorageFamily::CoreFunctionFunction => {
                    rearrange(&mut self.values.core_function_functions, route)
                }
                StorageFamily::ExternalFunctionFunction => {
                    rearrange(&mut self.values.external_function_functions, route)
                }
            }
        }
    }
}

// The placed prefix holds the first retained use; only further uses need a copy.
fn rearrange<Value: Clone>(values: &mut Vec<Value>, route: &FamilyTransfer) {
    for &TransferStep {
        source,
        destination,
    } in route.steps.iter()
    {
        if source < destination {
            values.push(values[source].clone());
            let last = values.len() - 1;
            values.swap(last, destination);
        } else {
            values.swap(source, destination);
        }
    }
    values.truncate(route.length);
}

#[cfg(test)]
mod tests {
    use super::rearrange;
    use crate::plan::execution::graph::{FamilyTransfer, StorageFamily, TransferStep};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};

    struct Tracked {
        id: usize,
        copies: Arc<AtomicUsize>,
        live: Arc<[AtomicUsize; 4]>,
        dropped: Arc<Mutex<Vec<usize>>>,
    }

    impl Clone for Tracked {
        fn clone(&self) -> Self {
            self.copies.fetch_add(1, Ordering::Relaxed);
            self.live[self.id].fetch_add(1, Ordering::Relaxed);
            Self {
                id: self.id,
                copies: self.copies.clone(),
                live: self.live.clone(),
                dropped: self.dropped.clone(),
            }
        }
    }

    impl Drop for Tracked {
        fn drop(&mut self) {
            self.live[self.id].fetch_sub(1, Ordering::Relaxed);
            self.dropped.lock().unwrap().push(self.id);
        }
    }

    #[test]
    fn moves_first_uses_copies_repeats_and_releases_omitted_payloads_at_handoff() {
        let copies = Arc::new(AtomicUsize::new(0));
        let dropped = Arc::new(Mutex::new(Vec::new()));
        let live = Arc::new(std::array::from_fn(|_| AtomicUsize::new(1)));
        let mut values = (0..4)
            .map(|id| Tracked {
                id,
                copies: copies.clone(),
                live: live.clone(),
                dropped: dropped.clone(),
            })
            .collect::<Vec<_>>();
        values.reserve(4);
        let buffer = values.as_ptr();
        let capacity = values.capacity();

        rearrange(
            &mut values,
            &FamilyTransfer {
                family: StorageFamily::Int,
                length: 3,
                steps: vec![
                    TransferStep {
                        source: 2,
                        destination: 0,
                    },
                    TransferStep {
                        source: 2,
                        destination: 1,
                    },
                    TransferStep {
                        source: 0,
                        destination: 2,
                    },
                ]
                .into(),
            },
        );

        assert_eq!(
            values.iter().map(|value| value.id).collect::<Vec<_>>(),
            [2, 0, 2]
        );
        assert_eq!(copies.load(Ordering::Relaxed), 1);
        assert_eq!(*dropped.lock().unwrap(), [3, 1]);
        assert_eq!(
            live.each_ref().map(|count| count.load(Ordering::Relaxed)),
            [1, 0, 2, 0]
        );
        assert_eq!(values.as_ptr(), buffer);
        assert_eq!(values.capacity(), capacity);
        drop(values);
        assert_eq!(*dropped.lock().unwrap(), [3, 1, 2, 0, 2]);
        assert_eq!(
            live.each_ref().map(|count| count.load(Ordering::Relaxed)),
            [0; 4]
        );
    }

    #[test]
    fn fixed_width_transitions_reuse_the_same_buffer_without_copies() {
        let copies = Arc::new(AtomicUsize::new(0));
        let dropped = Arc::new(Mutex::new(Vec::new()));
        let live = Arc::new(std::array::from_fn(|id| {
            AtomicUsize::new(usize::from(id < 3))
        }));
        let mut values = (0..3)
            .map(|id| Tracked {
                id,
                copies: copies.clone(),
                live: live.clone(),
                dropped: dropped.clone(),
            })
            .collect::<Vec<_>>();
        let buffer = values.as_ptr();
        let capacity = values.capacity();
        for _ in 0..1024 {
            rearrange(
                &mut values,
                &FamilyTransfer {
                    family: StorageFamily::Int,
                    length: 3,
                    steps: vec![
                        TransferStep {
                            source: 2,
                            destination: 0,
                        },
                        TransferStep {
                            source: 2,
                            destination: 1,
                        },
                    ]
                    .into(),
                },
            );
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
        let dropped = Arc::new(Mutex::new(Vec::new()));
        let live = Arc::new(std::array::from_fn(|id| {
            AtomicUsize::new(usize::from(id == 0))
        }));
        let mut values = vec![Tracked {
            id: 0,
            copies: copies.clone(),
            live: live.clone(),
            dropped: dropped.clone(),
        }];
        rearrange(
            &mut values,
            &FamilyTransfer {
                family: StorageFamily::Int,
                length: 5,
                steps: vec![
                    TransferStep {
                        source: 0,
                        destination: 1,
                    },
                    TransferStep {
                        source: 0,
                        destination: 2,
                    },
                    TransferStep {
                        source: 0,
                        destination: 3,
                    },
                    TransferStep {
                        source: 0,
                        destination: 4,
                    },
                ]
                .into(),
            },
        );
        assert_eq!(copies.load(Ordering::Relaxed), 4);
        assert_eq!(live[0].load(Ordering::Relaxed), 5);
        let buffer = values.as_ptr();
        let capacity = values.capacity();

        rearrange(
            &mut values,
            &FamilyTransfer {
                family: StorageFamily::Int,
                length: 1,
                steps: vec![TransferStep {
                    source: 4,
                    destination: 0,
                }]
                .into(),
            },
        );
        assert_eq!(live[0].load(Ordering::Relaxed), 1);
        assert_eq!(values[0].id, 0);
        rearrange(
            &mut values,
            &FamilyTransfer {
                family: StorageFamily::Int,
                length: 0,
                steps: vec![].into(),
            },
        );
        assert!(values.is_empty());
        assert_eq!(live[0].load(Ordering::Relaxed), 0);
        assert_eq!(values.as_ptr(), buffer);
        assert_eq!(values.capacity(), capacity);
    }
}
