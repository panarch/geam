use super::{BlockValues, RetainedValues};
use crate::plan::execution::graph::StorageFamily;
use crate::runtime::evaluated::EvaluatedValue;

/// One lazily allocated, execution-owned result buffer. Moving it only moves its pointer.
#[derive(Default)]
pub(in crate::runtime::graph) struct MatchResults {
    storage: Option<Box<ResultStorage>>,
}

struct ResultStorage {
    values: BlockValues,
    order: Vec<Option<StorageFamily>>,
    used: Vec<StorageFamily>,
    resident: Vec<StorageFamily>,
    active: u64,
    allocated: u64,
    heap_bytes: usize,
    high_water: usize,
}

impl MatchResults {
    pub(in crate::runtime::graph) fn push(&mut self, value: EvaluatedValue) {
        let storage = self
            .storage
            .get_or_insert_with(|| Box::new(ResultStorage::new()));
        storage.push(value);
    }

    pub(in crate::runtime::graph) fn commit(
        &mut self,
        selected: &[usize],
        target: &mut RetainedValues,
    ) {
        let Some(storage) = self.storage.as_mut() else {
            return;
        };
        let bindings = storage.order.len();
        if selected.len() == bindings {
            // Admission/lowering guarantees sorted, unique, in-range binding indices.
            for &family in &storage.used {
                storage.values.append_column(family, &mut target.values);
            }
            storage.order.clear();
        } else {
            storage.reverse_columns();
            let mut selected = selected.iter().copied().peekable();
            for (index, family) in storage.order.drain(..).enumerate() {
                let keep = selected.peek() == Some(&index);
                if keep {
                    selected.next();
                }
                if let Some(family) = family {
                    if keep {
                        storage.values.move_last(family, &mut target.values);
                    } else {
                        storage.values.drop_last(family);
                    }
                }
            }
        }
        storage.finish(bindings);
    }

    pub(in crate::runtime::graph) fn clear(&mut self) {
        if let Some(storage) = self.storage.as_mut() {
            storage.clear();
        }
    }
}

impl Drop for MatchResults {
    fn drop(&mut self) {
        self.clear();
    }
}

impl ResultStorage {
    fn new() -> Self {
        Self {
            values: BlockValues::default(),
            order: Vec::new(),
            used: Vec::new(),
            resident: Vec::new(),
            active: 0,
            allocated: 0,
            heap_bytes: 0,
            high_water: 0,
        }
    }

    fn push(&mut self, value: EvaluatedValue) {
        let write = self.values.write_evaluated(value);
        let family = write.as_ref().map(|write| write.family);
        self.heap_bytes += push_record(&mut self.order, family);
        if let Some(write) = write {
            let mask = family_mask(write.family);
            if self.active & mask == 0 {
                self.heap_bytes += push_record(&mut self.used, write.family);
                self.active |= mask;
            }
            if write.growth != 0 {
                self.heap_bytes += write.growth;
                if self.allocated & mask == 0 {
                    self.heap_bytes += push_record(&mut self.resident, write.family);
                    self.allocated |= mask;
                }
            }
        }
    }

    fn clear(&mut self) {
        let bindings = self.order.len();
        self.reverse_columns();
        for family in self.order.drain(..).flatten() {
            self.values.drop_last(family);
        }
        self.finish(bindings);
    }

    fn reverse_columns(&mut self) {
        for &family in &self.used {
            self.values.reverse_column(family);
        }
    }

    fn finish(&mut self, bindings: usize) {
        self.high_water = self.high_water.max(bindings);
        let budget = 65_536usize.saturating_add(
            self.high_water
                .saturating_mul(2 * size_of::<EvaluatedValue>()),
        );
        if self.heap_bytes > budget {
            // First release inactive high-water columns. Only an over-budget growth
            // visits resident columns, rather than scanning every family per Match.
            self.trim(budget, false);
            self.trim(budget, true);
        }
        self.used.clear();
        self.active = 0;
    }

    fn trim(&mut self, budget: usize, include_active: bool) {
        let mut index = 0;
        while self.heap_bytes > budget && index < self.resident.len() {
            let family = self.resident[index];
            let mask = family_mask(family);
            if include_active || self.active & mask == 0 {
                self.heap_bytes -= self.values.release_column(family);
                self.allocated &= !mask;
                self.resident.swap_remove(index);
            } else {
                index += 1;
            }
        }
    }
}

fn family_mask(family: StorageFamily) -> u64 {
    1 << family as u8
}

fn push_record<Value>(records: &mut Vec<Value>, value: Value) -> usize {
    let capacity = records.capacity();
    records.push(value);
    (records.capacity() - capacity) * size_of::<Value>()
}

macro_rules! with_column {
    ($family:expr, $values:expr, $column:ident $(=> $target:expr, $destination:ident)?; $body:block) => {
        match $family {
            StorageFamily::Int => {
                let $column = &mut ($values).ints;
                $(let $destination = &mut ($target).ints;)?
                $body
            }
            StorageFamily::Float => {
                let $column = &mut ($values).floats;
                $(let $destination = &mut ($target).floats;)?
                $body
            }
            StorageFamily::String => {
                let $column = &mut ($values).strings;
                $(let $destination = &mut ($target).strings;)?
                $body
            }
            StorageFamily::BitArray => {
                let $column = &mut ($values).bit_arrays;
                $(let $destination = &mut ($target).bit_arrays;)?
                $body
            }
            StorageFamily::UtfCodepoint => {
                let $column = &mut ($values).utf_codepoints;
                $(let $destination = &mut ($target).utf_codepoints;)?
                $body
            }
            StorageFamily::Custom => {
                let $column = &mut ($values).customs;
                $(let $destination = &mut ($target).customs;)?
                $body
            }
            StorageFamily::External => {
                let $column = &mut ($values).externals;
                $(let $destination = &mut ($target).externals;)?
                $body
            }
            StorageFamily::Bool => {
                let $column = &mut ($values).bools;
                $(let $destination = &mut ($target).bools;)?
                $body
            }
            StorageFamily::Tuple => {
                let $column = &mut ($values).tuples;
                $(let $destination = &mut ($target).tuples;)?
                $body
            }
            StorageFamily::ParameterList => {
                let $column = &mut ($values).parameter_lists;
                $(let $destination = &mut ($target).parameter_lists;)?
                $body
            }
            StorageFamily::IntList => {
                let $column = &mut ($values).int_lists;
                $(let $destination = &mut ($target).int_lists;)?
                $body
            }
            StorageFamily::StringList => {
                let $column = &mut ($values).string_lists;
                $(let $destination = &mut ($target).string_lists;)?
                $body
            }
            StorageFamily::BitArrayList => {
                let $column = &mut ($values).bit_array_lists;
                $(let $destination = &mut ($target).bit_array_lists;)?
                $body
            }
            StorageFamily::UtfCodepointList => {
                let $column = &mut ($values).utf_codepoint_lists;
                $(let $destination = &mut ($target).utf_codepoint_lists;)?
                $body
            }
            StorageFamily::CustomList => {
                let $column = &mut ($values).custom_lists;
                $(let $destination = &mut ($target).custom_lists;)?
                $body
            }
            StorageFamily::ExternalList => {
                let $column = &mut ($values).external_lists;
                $(let $destination = &mut ($target).external_lists;)?
                $body
            }
            StorageFamily::FloatList => {
                let $column = &mut ($values).float_lists;
                $(let $destination = &mut ($target).float_lists;)?
                $body
            }
            StorageFamily::BoolList => {
                let $column = &mut ($values).bool_lists;
                $(let $destination = &mut ($target).bool_lists;)?
                $body
            }
            StorageFamily::NilList => {
                let $column = &mut ($values).nil_lists;
                $(let $destination = &mut ($target).nil_lists;)?
                $body
            }
            StorageFamily::TupleList => {
                let $column = &mut ($values).tuple_lists;
                $(let $destination = &mut ($target).tuple_lists;)?
                $body
            }
            StorageFamily::ParameterListList => {
                let $column = &mut ($values).parameter_list_lists;
                $(let $destination = &mut ($target).parameter_list_lists;)?
                $body
            }
            StorageFamily::ListList => {
                let $column = &mut ($values).list_lists;
                $(let $destination = &mut ($target).list_lists;)?
                $body
            }
            StorageFamily::FunctionList => {
                let $column = &mut ($values).function_lists;
                $(let $destination = &mut ($target).function_lists;)?
                $body
            }
            StorageFamily::IntFunction => {
                let $column = &mut ($values).int_functions;
                $(let $destination = &mut ($target).int_functions;)?
                $body
            }
            StorageFamily::FloatFunction => {
                let $column = &mut ($values).float_functions;
                $(let $destination = &mut ($target).float_functions;)?
                $body
            }
            StorageFamily::StringFunction => {
                let $column = &mut ($values).string_functions;
                $(let $destination = &mut ($target).string_functions;)?
                $body
            }
            StorageFamily::BitArrayFunction => {
                let $column = &mut ($values).bit_array_functions;
                $(let $destination = &mut ($target).bit_array_functions;)?
                $body
            }
            StorageFamily::UtfCodepointFunction => {
                let $column = &mut ($values).utf_codepoint_functions;
                $(let $destination = &mut ($target).utf_codepoint_functions;)?
                $body
            }
            StorageFamily::CustomFunction => {
                let $column = &mut ($values).custom_functions;
                $(let $destination = &mut ($target).custom_functions;)?
                $body
            }
            StorageFamily::ExternalFunction => {
                let $column = &mut ($values).external_functions;
                $(let $destination = &mut ($target).external_functions;)?
                $body
            }
            StorageFamily::BoolFunction => {
                let $column = &mut ($values).bool_functions;
                $(let $destination = &mut ($target).bool_functions;)?
                $body
            }
            StorageFamily::NilFunction => {
                let $column = &mut ($values).nil_functions;
                $(let $destination = &mut ($target).nil_functions;)?
                $body
            }
            StorageFamily::TupleFunction => {
                let $column = &mut ($values).tuple_functions;
                $(let $destination = &mut ($target).tuple_functions;)?
                $body
            }
            StorageFamily::ParameterListFunction => {
                let $column = &mut ($values).parameter_list_functions;
                $(let $destination = &mut ($target).parameter_list_functions;)?
                $body
            }
            StorageFamily::ParameterListListFunction => {
                let $column = &mut ($values).parameter_list_list_functions;
                $(let $destination = &mut ($target).parameter_list_list_functions;)?
                $body
            }
            StorageFamily::IntListFunction => {
                let $column = &mut ($values).int_list_functions;
                $(let $destination = &mut ($target).int_list_functions;)?
                $body
            }
            StorageFamily::StringListFunction => {
                let $column = &mut ($values).string_list_functions;
                $(let $destination = &mut ($target).string_list_functions;)?
                $body
            }
            StorageFamily::BitArrayListFunction => {
                let $column = &mut ($values).bit_array_list_functions;
                $(let $destination = &mut ($target).bit_array_list_functions;)?
                $body
            }
            StorageFamily::UtfCodepointListFunction => {
                let $column = &mut ($values).utf_codepoint_list_functions;
                $(let $destination = &mut ($target).utf_codepoint_list_functions;)?
                $body
            }
            StorageFamily::CustomListFunction => {
                let $column = &mut ($values).custom_list_functions;
                $(let $destination = &mut ($target).custom_list_functions;)?
                $body
            }
            StorageFamily::ExternalListFunction => {
                let $column = &mut ($values).external_list_functions;
                $(let $destination = &mut ($target).external_list_functions;)?
                $body
            }
            StorageFamily::FloatListFunction => {
                let $column = &mut ($values).float_list_functions;
                $(let $destination = &mut ($target).float_list_functions;)?
                $body
            }
            StorageFamily::BoolListFunction => {
                let $column = &mut ($values).bool_list_functions;
                $(let $destination = &mut ($target).bool_list_functions;)?
                $body
            }
            StorageFamily::NilListFunction => {
                let $column = &mut ($values).nil_list_functions;
                $(let $destination = &mut ($target).nil_list_functions;)?
                $body
            }
            StorageFamily::TupleListFunction => {
                let $column = &mut ($values).tuple_list_functions;
                $(let $destination = &mut ($target).tuple_list_functions;)?
                $body
            }
            StorageFamily::ListListFunction => {
                let $column = &mut ($values).list_list_functions;
                $(let $destination = &mut ($target).list_list_functions;)?
                $body
            }
            StorageFamily::FunctionListFunction => {
                let $column = &mut ($values).function_list_functions;
                $(let $destination = &mut ($target).function_list_functions;)?
                $body
            }
            StorageFamily::CoreFunctionFunction => {
                let $column = &mut ($values).core_function_functions;
                $(let $destination = &mut ($target).core_function_functions;)?
                $body
            }
            StorageFamily::ExternalFunctionFunction => {
                let $column = &mut ($values).external_function_functions;
                $(let $destination = &mut ($target).external_function_functions;)?
                $body
            }
            StorageFamily::GenericFunction => {
                let $column = &mut ($values).generic_functions;
                $(let $destination = &mut ($target).generic_functions;)?
                $body
            }
            StorageFamily::NeverFunction => {
                let $column = &mut ($values).never_functions;
                $(let $destination = &mut ($target).never_functions;)?
                $body
            }
        }
    };
}

impl BlockValues {
    fn append_column(&mut self, family: StorageFamily, target: &mut Self) {
        with_column!(family, self, column => target, destination; {
            destination.append(column);
        });
    }

    fn move_last(&mut self, family: StorageFamily, target: &mut Self) {
        with_column!(family, self, column => target, destination; {
            // The visitation record owns exactly one element in this column.
            destination.push(column.remove(column.len() - 1));
        });
    }

    fn drop_last(&mut self, family: StorageFamily) {
        with_column!(family, self, column; {
            column.truncate(column.len() - 1);
        });
    }

    fn reverse_column(&mut self, family: StorageFamily) {
        with_column!(family, self, column; {
            column.reverse();
        });
    }

    fn release_column(&mut self, family: StorageFamily) -> usize {
        with_column!(family, self, column; { release_empty_column(column) })
    }
}

fn release_empty_column<Value>(column: &mut Vec<Value>) -> usize {
    let bytes = column.capacity() * size_of::<Value>();
    *column = Vec::new();
    bytes
}

#[cfg(test)]
mod tests {
    use super::{BlockValues, MatchResults, ResultStorage, family_mask};
    use crate::host::HostExternalStore;
    use crate::plan::execution::function::TupleFunctionId;
    use crate::plan::execution::graph::StorageFamily;
    use crate::plan::execution::runtime::RuntimeExecutionPlan;
    use crate::plan::execution::type_::ExternalTypeId;
    use crate::runtime::evaluated::{
        EvaluatedExternalValue, EvaluatedValue, value_source_hash, values_equal,
    };
    use crate::runtime::function::run_tuple;
    use crate::runtime::graph::RetainedValues;
    use crate::runtime::retained::{RetainedValueInspection, RetainedValueRef};
    use crate::runtime::{HostCallOrigin, RuntimeListStorage, RuntimeState, Value};
    use num_bigint::BigInt;
    use std::sync::{Arc, Mutex};

    #[test]
    fn no_results_are_lazy_and_nil_still_owns_a_binding_index() {
        let mut results = MatchResults::default();
        let mut target = RetainedValues::empty();
        target.push_int(41.into());
        results.commit(&[], &mut target);
        results.clear();
        assert!(results.storage.is_none());
        assert_eq!(size_of::<MatchResults>(), size_of::<usize>());
        println!(
            "layout bytes: handle={}, result_headers={}, typed_headers={}, evaluated={}, order_entry={}",
            size_of::<MatchResults>(),
            size_of::<ResultStorage>(),
            size_of::<BlockValues>(),
            size_of::<EvaluatedValue>(),
            size_of::<Option<StorageFamily>>(),
        );
        results.push(EvaluatedValue::Nil);
        let storage = results.storage.as_ref().unwrap();
        assert_eq!(storage.order, [None]);
        assert_eq!(storage.used, []);
        assert_eq!(storage.resident, []);
        results.commit(&[0], &mut target);
        let storage = results.storage.as_ref().unwrap();
        assert_eq!(storage.order, []);
        assert_eq!(storage.active, 0);
        assert_eq!(target.values.ints, [BigInt::from(41)]);
    }

    #[test]
    fn bulk_commit_appends_each_used_column_and_keeps_payload_buffers() {
        let mut results = MatchResults::default();
        let mut target = RetainedValues::empty();
        target.push_int(41.into());
        let tuple = vec![EvaluatedValue::Int(7.into())];
        let pointer = tuple.as_ptr();
        for value in [
            EvaluatedValue::Int(7.into()),
            EvaluatedValue::Nil,
            EvaluatedValue::Tuple(tuple),
            EvaluatedValue::Int(9.into()),
            EvaluatedValue::String("label".into()),
        ] {
            results.push(value);
        }
        assert_eq!(
            results.storage.as_ref().unwrap().used,
            [
                StorageFamily::Int,
                StorageFamily::Tuple,
                StorageFamily::String
            ]
        );
        results.commit(&[0, 1, 2, 3, 4], &mut target);
        assert_eq!(
            target.values.ints,
            [BigInt::from(41), BigInt::from(7), BigInt::from(9)]
        );
        assert_eq!(target.values.strings, ["label"]);
        assert_eq!(target.values.tuples, [vec![EvaluatedValue::Int(7.into())]]);
        assert_eq!(target.values.tuples[0].as_ptr(), pointer);
        let storage = results.storage.as_ref().unwrap();
        assert_eq!(storage.order, []);
        assert_eq!(storage.used, []);
        assert!(storage.values.ints.is_empty());
        assert!(storage.values.tuples.is_empty());
    }

    #[test]
    fn smaller_and_repeated_matches_reuse_column_and_order_capacity() {
        let mut results = MatchResults::default();
        for index in 0..9 {
            results.push(EvaluatedValue::Int(index.into()));
        }
        let storage = results.storage.as_ref().unwrap();
        let allocation = &**storage as *const ResultStorage;
        let column = storage.values.ints.as_ptr();
        let order = storage.order.as_ptr();
        let capacity = storage.values.ints.capacity();
        let bytes = storage.heap_bytes;
        results.clear();
        for count in [1, 0, 9, 2, 9] {
            for index in 0..count {
                results.push(EvaluatedValue::Int(index.into()));
            }
            results.commit(&[], &mut RetainedValues::empty());
            let storage = results.storage.as_ref().unwrap();
            assert_eq!(&**storage as *const ResultStorage, allocation);
            assert_eq!(storage.values.ints.as_ptr(), column);
            assert_eq!(storage.order.as_ptr(), order);
            assert_eq!(storage.values.ints.capacity(), capacity);
            assert_eq!(storage.heap_bytes, bytes);
            assert_eq!(storage.heap_bytes, retained_heap_bytes(storage));
            assert_eq!(storage.high_water, 9);
            assert!(storage.values.ints.is_empty());
            assert_eq!(storage.order, []);
        }
    }

    #[test]
    fn sparse_commit_and_clear_release_cross_column_payloads_in_visit_order() {
        for selected in [Some(&[0, 3, 4][..]), None] {
            let dropped = Arc::new(Mutex::new(Vec::new()));
            let store = HostExternalStore::default();
            let mut results = MatchResults::default();
            let mut target = RetainedValues::empty();
            results.push(EvaluatedValue::Tuple(vec![tracked(&store, &dropped, 0)]));
            results.push(tracked(&store, &dropped, 1));
            results.push(EvaluatedValue::Tuple(vec![tracked(&store, &dropped, 2)]));
            results.push(EvaluatedValue::Nil);
            results.push(tracked(&store, &dropped, 3));
            if let Some(selected) = selected {
                results.commit(selected, &mut target);
                assert_eq!(*dropped.lock().unwrap(), [1, 2]);
                assert_eq!(target.values.tuples.len(), 1);
                assert_eq!(target.values.externals.len(), 1);
                drop(results);
                assert_eq!(*dropped.lock().unwrap(), [1, 2]);
                drop(target);
                assert_eq!(*dropped.lock().unwrap(), [1, 2, 3, 0]);
            } else {
                results.clear();
                assert_eq!(*dropped.lock().unwrap(), [0, 1, 2, 3]);
                assert_eq!(results.storage.as_ref().unwrap().order, []);
                assert_eq!(results.storage.as_ref().unwrap().used, []);
                drop(results);
                assert_eq!(*dropped.lock().unwrap(), [0, 1, 2, 3]);
            }
        }
    }

    #[test]
    fn dropping_partial_results_releases_all_payloads_in_visit_order() {
        let dropped = Arc::new(Mutex::new(Vec::new()));
        let store = HostExternalStore::default();
        let mut results = MatchResults::default();
        results.push(EvaluatedValue::Tuple(vec![tracked(&store, &dropped, 0)]));
        results.push(tracked(&store, &dropped, 1));
        results.push(EvaluatedValue::Tuple(vec![tracked(&store, &dropped, 2)]));
        drop(results);
        assert_eq!(*dropped.lock().unwrap(), [0, 1, 2]);
    }

    #[test]
    fn alternating_wide_families_are_bounded_without_reallocating_the_repeated_family() {
        let mut results = MatchResults::default();
        let plan = crate::runtime::plan_src(
            r#"pub fn main() { #(fn() { 42 }, fn() { 1.5 }, fn() { "text" }, fn() { True }) }"#,
        );
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        let families = run_tuple(
            &plan,
            &mut state,
            TupleFunctionId(0),
            HostCallOrigin::Entry,
            RetainedValues::empty(),
        )
        .unwrap();
        assert_eq!(families.len(), 4);
        results.push(EvaluatedValue::Int(0.into()));
        results.clear();
        let int_column = results.storage.as_ref().unwrap().values.ints.as_ptr();
        let mut saw_eviction = false;
        let mut seen = family_mask(StorageFamily::Int);
        for value in &families {
            results.push(EvaluatedValue::Int(0.into()));
            for _ in 0..4_097 {
                results.push(value.clone());
            }
            let column_family = results.storage.as_ref().unwrap().used[1];
            results.clear();
            let storage = results.storage.as_ref().unwrap();
            assert_eq!(storage.heap_bytes, retained_heap_bytes(storage));
            assert_eq!(storage.values.ints.as_ptr(), int_column);
            assert_ne!(storage.allocated & family_mask(StorageFamily::Int), 0);
            seen |= family_mask(column_family);
            saw_eviction |= storage.allocated != seen;
            assert!(
                storage.heap_bytes <= 65_536 + 2 * storage.high_water * size_of::<EvaluatedValue>()
            );
            assert_eq!(storage.order, []);
            assert_eq!(
                storage.allocated & family_mask(column_family),
                family_mask(column_family)
            );
        }
        assert!(saw_eviction);
        let storage = results.storage.as_ref().unwrap();
        let pointer = storage.values.bool_functions.as_ptr();
        let bytes = storage.heap_bytes;
        for _ in 0..3 {
            results.push(EvaluatedValue::Int(0.into()));
            for _ in 0..4_097 {
                results.push(families[3].clone());
            }
            results.clear();
            let storage = results.storage.as_ref().unwrap();
            assert_eq!(storage.values.bool_functions.as_ptr(), pointer);
            assert_eq!(storage.heap_bytes, bytes);
            assert_eq!(storage.heap_bytes, retained_heap_bytes(storage));
        }
    }

    #[test]
    fn sparse_never_functions_keep_identity_without_invocation_and_release_empty_capacity() {
        let plan = crate::runtime::plan_src(
            "fn stop(_value: Int) -> a { panic } pub fn main() { #(stop) }",
        );
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        let values = run_tuple(
            &plan,
            &mut state,
            TupleFunctionId(0),
            HostCallOrigin::Entry,
            RetainedValues::empty(),
        )
        .unwrap();
        assert_eq!(values.len(), 1);
        let mut results = MatchResults::default();
        results.push(values[0].clone());
        assert_eq!(
            results.storage.as_ref().unwrap().used,
            [StorageFamily::NeverFunction]
        );
        results.push(values[0].clone());
        let mut target = RetainedValues::empty();
        results.commit(&[1], &mut target);
        assert_eq!(target.values.never_functions.len(), 1);
        assert_eq!(
            EvaluatedValue::Function(target.values.never_functions[0].clone().into()),
            values[0],
        );
        results.push(values[0].clone());
        results.clear();
        let storage = results.storage.as_ref().unwrap();
        assert!(storage.values.never_functions.is_empty());
        assert_eq!(storage.order, []);
        assert_eq!(storage.heap_bytes, retained_heap_bytes(storage));

        let mut column = BlockValues::default();
        let write = column.write_evaluated(values[0].clone()).unwrap();
        assert_eq!(write.family, StorageFamily::NeverFunction);
        assert!(write.growth > 0);
        column.drop_last(write.family);
        assert_eq!(column.release_column(write.family), write.growth);
        assert_eq!(column.never_functions.capacity(), 0);
        assert!(echo.is_empty());
    }

    #[test]
    fn sparse_erased_list_callables_keep_loaded_types_identity_and_empty_capacity() {
        use crate::host::ExternalTestProfile;
        use crate::plan::execution::function::{ListFunctionId, RuntimeListFunctionId};
        use crate::plan::execution::graph::{
            ExternalListFunctionLocalId, ListFunctionLocal, ParameterListFunctionLocalId,
            ParameterListListFunctionLocalId,
        };
        use crate::plan::execution::type_::{FunctionType, ValueType};
        use crate::runtime::evaluated::EvaluatedListFunction;
        use crate::runtime::graph::BlockEnvironment;
        use crate::runtime::profile::external_test::{
            RuntimeCounterProvider, RuntimeCounterSchema,
        };
        use crate::runtime::state::list::ExternalListAllocation;
        use crate::{
            HostProviderModule, HostProviderSet, HostedExecution, ModuleSource, PackageSource,
        };

        let empty = crate::runtime::plan_src("pub fn main() { [] }");
        let nested = crate::runtime::plan_src("pub fn main() { [[]] }");
        let parameter = empty.parameter_list_function_id(0);
        let parameter_list = nested.parameter_list_list_function_id(0);
        let provider = HostProviderModule::<ExternalTestProfile>::new("application", "main")
            .unwrap()
            .with_external_type::<RuntimeCounterProvider, RuntimeCounterSchema>()
            .unwrap();
        let typed = crate::compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                Vec::<&str>::new(),
                [ModuleSource::new(
                    "main",
                    "main.gleam",
                    r#"
@external(erlang, "host", "Counter")
pub type Counter
pub fn main() -> List(Counter) { [] }
"#,
                )],
            )],
            HostProviderSet::from_providers([provider]).unwrap(),
        )
        .unwrap();
        let external =
            HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let external_id = external.external_list_function_id(0);
        let parameter_type =
            FunctionType::new(Vec::new(), ValueType::List(parameter.type_id().list_type()));
        let nested_type = FunctionType::new(
            Vec::new(),
            ValueType::List(parameter_list.type_id().list_type()),
        );
        let external_type = FunctionType::new(
            Vec::new(),
            ValueType::List(external_id.type_id().list_type()),
        );
        for (family, runtime_id, local, type_) in [
            (
                StorageFamily::ParameterListFunction,
                RuntimeListFunctionId::Core(ListFunctionId::Parameter(parameter)),
                ListFunctionLocal::Parameter {
                    local: ParameterListFunctionLocalId(0),
                    type_: parameter_type.clone(),
                    list_type: parameter.type_id(),
                },
                parameter_type,
            ),
            (
                StorageFamily::ParameterListListFunction,
                RuntimeListFunctionId::Core(ListFunctionId::ParameterList(parameter_list)),
                ListFunctionLocal::ParameterList {
                    local: ParameterListListFunctionLocalId(0),
                    type_: nested_type.clone(),
                    list_type: parameter_list.type_id(),
                },
                nested_type,
            ),
            (
                StorageFamily::ExternalListFunction,
                RuntimeListFunctionId::External(external_id),
                ListFunctionLocal::External {
                    local: ExternalListFunctionLocalId(0),
                    type_: external_type.clone(),
                    list_type: external_id.type_id(),
                },
                external_type,
            ),
        ] {
            // These loaded entry functions have no captures. Their ids and types
            // come from the compiled program, including its erased list metadata.
            let function = EvaluatedListFunction::reference(runtime_id, Default::default(), type_);
            let value = EvaluatedValue::Function(function.clone().into());
            let mut results = MatchResults::default();
            results.push(value.clone());
            results.push(EvaluatedValue::Nil);
            assert_eq!(results.storage.as_ref().unwrap().used, [family]);
            let mut target = RetainedValues::empty();
            results.commit(&[0], &mut target);
            let environment = BlockEnvironment::from_retained(target);
            assert_eq!(environment.list_function(&local), function);
            results.push(value.clone());
            results.clear();
            let storage = results.storage.as_ref().unwrap();
            assert_eq!(storage.order, []);
            assert_eq!(storage.used, []);
            assert_eq!(storage.heap_bytes, retained_heap_bytes(storage));
            let mut column = BlockValues::default();
            let write = column.write_evaluated(value).unwrap();
            assert_eq!(write.family, family);
            column.drop_last(family);
            assert_eq!(column.release_column(family), write.growth);
            assert_eq!(column.release_column(family), 0);
        }
        let lists = RuntimeListStorage::default();
        let original = lists.external(ExternalListAllocation::new(
            external_id.type_id(),
            Vec::new(),
        ));
        let value = EvaluatedValue::List(original.clone().into());
        let mut results = MatchResults::default();
        results.push(value.clone());
        results.push(EvaluatedValue::Nil);
        let mut target = RetainedValues::empty();
        results.commit(&[0], &mut target);
        assert_eq!(target.values.external_lists, [original]);
        let mut column = BlockValues::default();
        let write = column.write_evaluated(value).unwrap();
        assert_eq!(write.family, StorageFamily::ExternalList);
        column.drop_last(write.family);
        assert_eq!(column.release_column(write.family), write.growth);
        assert_eq!(column.release_column(write.family), 0);
    }

    #[test]
    fn sparse_external_entry_callables_preserve_loaded_types_and_identity() {
        use crate::host::{ExternalTestProfile, ExternalTestRunState};
        use crate::plan::execution::function::{CoreRuntimeFunctionId, RuntimeFunctionId};
        use crate::plan::execution::type_::{FunctionType, ValueType};
        use crate::runtime::evaluated::{EvaluatedFunctionFunction, EvaluatedFunctionValue};
        use crate::runtime::profile::external_test::{
            RuntimeCounterProvider, RuntimeCounterSchema, RuntimeHostCounter,
        };
        use crate::{
            HostCall, HostCallCompletion, HostCallError, HostProviderModule, HostProviderSet,
            HostedExecution, ModuleSource, PackageSource,
        };
        use num_bigint::BigInt;

        fn counter<'call>(
            mut call: HostCall<
                'call,
                ExternalTestProfile,
                RuntimeCounterProvider,
                RuntimeHostCounter,
            >,
            value: BigInt,
        ) -> Result<HostCallCompletion<'call, RuntimeHostCounter>, HostCallError> {
            let value = call.create_external(value);
            Ok(call.return_value(value))
        }

        for (family, body) in [
            (Some(StorageFamily::ExternalFunction), "counter(42)"),
            (Some(StorageFamily::ExternalFunctionFunction), "counter"),
            (None, "42"),
        ] {
            let source = format!(
                r#"
@external(erlang, "host", "Counter")
pub type Counter
@external(erlang, "host", "counter")
fn counter(value: Int) -> Counter
pub fn main() {{ {body} }}
"#,
            );
            let provider = HostProviderModule::<ExternalTestProfile>::new("application", "main")
                .unwrap()
                .with_external_type::<RuntimeCounterProvider, RuntimeCounterSchema>()
                .unwrap()
                .with_scoped_function::<RuntimeCounterProvider, (BigInt,), RuntimeHostCounter, _>(
                    "counter", counter,
                )
                .unwrap();
            let typed = crate::compile_typed_host_program(
                "application",
                "main",
                [PackageSource::new(
                    "application",
                    Vec::<&str>::new(),
                    [ModuleSource::new("main", "main.gleam", &source)],
                )],
                HostProviderSet::from_providers([provider]).unwrap(),
            )
            .unwrap();
            let mut execution =
                HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                    .unwrap();
            if body == "counter(42)" {
                let mut echo = Vec::new();
                let result = crate::execution_fixture::run(
                    &mut execution,
                    &mut ExternalTestRunState::default(),
                    &mut echo,
                )
                .unwrap();
                assert_eq!(result.inspect().to_string(), "Counter(42)");
                assert!(echo.is_empty());
            }
            let value = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let target = execution.execution().main_runtime();
                let result = match &target {
                    RuntimeFunctionId::External(id) => ValueType::External(id.return_type),
                    RuntimeFunctionId::Core(CoreRuntimeFunctionId::Function {
                        return_type,
                        ..
                    }) => ValueType::Function(return_type.clone()),
                    _ => panic!("fixture entry must return an external value or callable"),
                };
                EvaluatedValue::Function(EvaluatedFunctionValue::closure(
                    target,
                    Default::default(),
                    FunctionType::new(Vec::new(), result),
                ))
            }));
            if let Some(family) = family {
                let value = value.unwrap();
                let mut results = MatchResults::default();
                results.push(value.clone());
                results.push(EvaluatedValue::Nil);
                assert_eq!(results.storage.as_ref().unwrap().used, [family]);
                let mut target = RetainedValues::empty();
                results.commit(&[0], &mut target);
                let actual = target
                    .values
                    .external_functions
                    .iter()
                    .cloned()
                    .map(EvaluatedFunctionValue::from)
                    .chain(
                        target
                            .values
                            .external_function_functions
                            .iter()
                            .cloned()
                            .map(|value| EvaluatedFunctionFunction::External(value).into()),
                    )
                    .map(EvaluatedValue::Function)
                    .collect::<Vec<_>>();
                assert_eq!(actual.as_slice(), std::slice::from_ref(&value));
                results.push(value.clone());
                results.clear();
                let mut column = BlockValues::default();
                let write = column.write_evaluated(value).unwrap();
                assert_eq!(write.family, family);
                column.drop_last(family);
                assert_eq!(column.release_column(family), write.growth);
                assert_eq!(column.release_column(family), 0);
            } else {
                assert!(value.is_err());
            }
        }
    }

    #[test]
    fn compiled_matches_preserve_plain_values_lists_and_callable_families() {
        let declarations = r#"
pub type Box { Box(Int) }
fn identity(value) { value }
fn stop(_value: Int) -> a { panic }
fn empty() -> List(a) { [] }
fn nested_empty() -> List(List(a)) { [[]] }
fn codepoint() {
  let assert <<value:utf8_codepoint>> = <<65>>
  value
}
fn integer() { 42 }
fn decimal() { 1.5 }
fn text() { "text" }
fn bits() { <<42>> }
fn boxed() { Box(42) }
fn flag() { True }
fn nil() { Nil }
fn pair() { #(42, True) }
fn integers() { [42] }
fn decimals() { [1.5] }
fn texts() { ["text"] }
fn arrays() { [<<42>>] }
fn boxes() { [Box(42)] }
fn flags() { [True] }
fn nils() { [Nil] }
fn pairs() { [#(42, True)] }
fn nested() { [[42]] }
fn codepoints() { [codepoint()] }
fn callbacks() { [integer] }
fn callback() { integer }
fn selected(value) {
  let assert [head, ..] = [value]
  head
}
fn sparse(value) {
  let assert [head, unused, ..tail] = [value, value]
  head
}
"#;
        for expression in [
            "42",
            "1.5",
            "\"text\"",
            "<<42>>",
            "codepoint()",
            "Box(42)",
            "True",
            "Nil",
            "#(42, True)",
            "empty()",
            "nested_empty()",
            "[42]",
            "[1.5]",
            "[\"text\"]",
            "[<<42>>]",
            "[codepoint()]",
            "[Box(42)]",
            "[True]",
            "[Nil]",
            "[#(42, True)]",
            "[[42]]",
            "[integer]",
            "integer",
            "decimal",
            "text",
            "bits",
            "codepoint",
            "boxed",
            "flag",
            "nil",
            "pair",
            "empty",
            "nested_empty",
            "integers",
            "decimals",
            "texts",
            "arrays",
            "codepoints",
            "boxes",
            "flags",
            "nils",
            "pairs",
            "nested",
            "callbacks",
            "callback",
            "identity",
            "stop",
        ] {
            let source = format!(
                r#"
{declarations}
pub fn main() {{
  let original = {expression}
  let first = selected(original)
  let second = sparse(first)
  #(first == original, second == original)
}}
"#
            );
            assert_eq!(
                crate::runtime::run_src(&source),
                Value::Tuple(vec![Value::Bool(true), Value::Bool(true),]),
                "{expression}",
            );

            let source = format!("{declarations}\npub fn main() {{ #({expression}) }}");
            let plan = crate::runtime::plan_src(&source);
            let mut echo = Vec::new();
            let mut state = RuntimeState::new(&mut echo);
            let values = run_tuple(
                &plan,
                &mut state,
                TupleFunctionId(0),
                HostCallOrigin::Entry,
                RetainedValues::empty(),
            )
            .unwrap();
            assert_eq!(values.len(), 1);
            let mut column = BlockValues::default();
            if let Some(write) = column.write_evaluated(values[0].clone()) {
                assert!(write.growth > 0, "{expression}");
                column.drop_last(write.family);
                assert_eq!(column.release_column(write.family), write.growth);
                assert_eq!(column.release_column(write.family), 0);
            } else {
                assert_eq!(values, [EvaluatedValue::Nil]);
            }
            assert!(echo.is_empty());
        }
    }

    #[test]
    fn compiled_matches_preserve_external_values_lists_and_callable_identity() {
        use crate::host::{ExternalTestProfile, ExternalTestRunState};
        use crate::runtime::profile::external_test::{
            RuntimeCounterProvider, RuntimeCounterSchema, RuntimeHostCounter,
        };
        use crate::{HostCall, HostCallCompletion, HostCallError, HostModule, HostProviderModule};
        use num_bigint::BigInt;

        fn counter<'call>(
            mut call: HostCall<
                'call,
                ExternalTestProfile,
                RuntimeCounterProvider,
                RuntimeHostCounter,
            >,
            value: BigInt,
        ) -> Result<HostCallCompletion<'call, RuntimeHostCounter>, HostCallError> {
            let value = call.create_external(value);
            Ok(call.return_value(value))
        }

        let provider = HostProviderModule::<ExternalTestProfile>::new("application", "main")
            .unwrap()
            .with_external_type::<RuntimeCounterProvider, RuntimeCounterSchema>()
            .unwrap()
            .with_scoped_function::<RuntimeCounterProvider, (BigInt,), RuntimeHostCounter, _>(
                "counter", counter,
            )
            .unwrap();
        let source = r#"
@external(erlang, "host", "Counter")
pub type Counter
@external(erlang, "host", "counter")
fn counter(value: Int) -> Counter
fn selected(value) {
  let assert [head, unused, ..tail] = [value, value]
  head
}
fn counters() { [counter(10), counter(20)] }
fn callback() { counter }
pub fn main() {
  let value = counter(42)
  let values = counters()
  let make = selected(counter)
  let make_list = selected(counters)
  let make_function = selected(callback)
  #(
    selected(value) == counter(42),
    selected(values) == [counter(10), counter(20)],
    make(42) == value,
    make_list() == values,
    make_function()(42) == value,
    make == counter,
    make_list == counters,
    make_function == callback,
  )
}
"#;
        let typed = crate::compile_typed_host_program(
            "application",
            "main",
            [crate::PackageSource::new(
                "application",
                Vec::<&str>::new(),
                [crate::ModuleSource::new("main", "main.gleam", source)],
            )],
            crate::HostProviderSet::with_providers(
                Vec::<HostModule<ExternalTestProfile>>::new(),
                [provider],
            )
            .unwrap(),
        )
        .unwrap();
        let mut execution =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let mut echo = Vec::new();
        assert_eq!(
            crate::execution_fixture::run(
                &mut execution,
                &mut ExternalTestRunState::default(),
                &mut echo,
            ),
            Ok(Value::Tuple(vec![Value::Bool(true); 8])),
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn transferred_payloads_keep_runtime_identity_and_source_observations() {
        let plan = crate::runtime::plan_src("pub fn main() { 0 }");
        let lists = RuntimeListStorage::default();
        let dropped = Arc::new(Mutex::new(Vec::new()));
        let store = HostExternalStore::default();
        let first = tracked(&store, &dropped, 7);
        let equal = tracked(&store, &dropped, 7);
        let different = tracked(&store, &dropped, 8);
        let mut column = BlockValues::default();
        let write = column.write_evaluated(first.clone()).unwrap();
        assert_eq!(write.family, StorageFamily::External);
        column.drop_last(write.family);
        assert_eq!(column.release_column(write.family), write.growth);
        assert_eq!(column.release_column(write.family), 0);
        let mut results = MatchResults::default();
        results.push(first.clone());
        results.push(equal.clone());
        results.push(different.clone());
        let mut target = RetainedValues::empty();
        results.commit(&[0, 1], &mut target);
        assert_eq!(target.values.externals.len(), 2);
        assert_eq!(
            EvaluatedValue::External(target.values.externals[0].clone()),
            first
        );
        assert_eq!(
            EvaluatedValue::External(target.values.externals[1].clone()),
            equal
        );
        assert_ne!(first, equal);
        assert!(values_equal(&lists, &first, &equal));
        assert!(!values_equal(&lists, &first, &different));
        assert_eq!(
            value_source_hash(&lists, &first),
            value_source_hash(&lists, &equal)
        );
        let inspect = |value: &RetainedValueRef| {
            crate::runtime::materialize::value(plan.value_metadata(), &lists, value.value().clone())
                .inspect()
                .to_string()
                .into()
        };
        let context = RetainedValueInspection::new(&inspect);
        for value in &target.values.externals {
            assert_eq!(value.lease().inspection(&context), "7");
        }
        drop([first, equal, different]);
        assert_eq!(*dropped.lock().unwrap(), [8]);
        drop(results);
        assert_eq!(*dropped.lock().unwrap(), [8]);
        drop(target);
        assert_eq!(*dropped.lock().unwrap(), [8, 7, 7]);
    }

    #[test]
    fn rust_unwind_releases_partial_results_in_visit_order() {
        let dropped = Arc::new(Mutex::new(Vec::new()));
        let store = HostExternalStore::default();
        let unwind = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut results = MatchResults::default();
            results.push(EvaluatedValue::Tuple(vec![tracked(&store, &dropped, 0)]));
            results.push(tracked(&store, &dropped, 1));
            results.push(EvaluatedValue::Tuple(vec![tracked(&store, &dropped, 2)]));
            panic!("abort partial match");
        }));
        assert!(unwind.is_err());
        assert_eq!(*dropped.lock().unwrap(), [0, 1, 2]);
    }

    fn retained_heap_bytes(storage: &ResultStorage) -> usize {
        fn capacity_bytes<Value>(values: &Vec<Value>) -> usize {
            values.capacity() * size_of::<Value>()
        }
        macro_rules! columns {
            ($($column:ident),+ $(,)?) => {{
                let BlockValues { $($column),+ } = &storage.values;
                0 $(+ capacity_bytes($column))+
            }};
        }
        capacity_bytes(&storage.order)
            + capacity_bytes(&storage.used)
            + capacity_bytes(&storage.resident)
            + columns!(
                ints,
                floats,
                strings,
                bit_arrays,
                utf_codepoints,
                customs,
                externals,
                bools,
                tuples,
                parameter_lists,
                int_lists,
                string_lists,
                bit_array_lists,
                utf_codepoint_lists,
                custom_lists,
                external_lists,
                float_lists,
                bool_lists,
                nil_lists,
                tuple_lists,
                parameter_list_lists,
                list_lists,
                function_lists,
                int_functions,
                float_functions,
                string_functions,
                bit_array_functions,
                utf_codepoint_functions,
                custom_functions,
                external_functions,
                bool_functions,
                nil_functions,
                tuple_functions,
                parameter_list_functions,
                parameter_list_list_functions,
                int_list_functions,
                string_list_functions,
                bit_array_list_functions,
                utf_codepoint_list_functions,
                custom_list_functions,
                external_list_functions,
                float_list_functions,
                bool_list_functions,
                nil_list_functions,
                tuple_list_functions,
                list_list_functions,
                function_list_functions,
                core_function_functions,
                external_function_functions,
                generic_functions,
                never_functions,
            )
    }

    struct Payload {
        id: usize,
        dropped: Arc<Mutex<Vec<usize>>>,
    }
    impl Drop for Payload {
        fn drop(&mut self) {
            self.dropped.lock().unwrap().push(self.id);
        }
    }
    fn tracked(
        store: &HostExternalStore<Payload>,
        dropped: &Arc<Mutex<Vec<usize>>>,
        id: usize,
    ) -> EvaluatedValue {
        let lease = store.insert(
            Payload {
                id,
                dropped: dropped.clone(),
            },
            |context, left, right| {
                context.0.stored_values_equal(
                    &RetainedValueRef::new(&EvaluatedValue::Int(left.id.into())),
                    &RetainedValueRef::new(&EvaluatedValue::Int(right.id.into())),
                )
            },
            |context, value| {
                context
                    .0
                    .stored_value_hash(&RetainedValueRef::new(&EvaluatedValue::Int(
                        value.id.into(),
                    )))
            },
            |context, value| {
                context
                    .0
                    .inspect_stored_value(&RetainedValueRef::new(&EvaluatedValue::Int(
                        value.id.into(),
                    )))
            },
            |_| None,
        );
        EvaluatedValue::External(EvaluatedExternalValue::new(ExternalTypeId::new(0), lease))
    }
}
