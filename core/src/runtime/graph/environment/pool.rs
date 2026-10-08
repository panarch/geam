use super::{BlockEnvironment, BlockValues, RetainedValues};

#[derive(Default)]
pub(in crate::runtime::graph) struct StoragePool {
    idle: Vec<CachedValues>,
    bytes: usize,
}

struct CachedValues {
    values: Box<BlockValues>,
    bytes: usize,
}

const MAX_IDLE: usize = 8;
const MAX_BYTES: usize = 65_536;

impl StoragePool {
    pub(in crate::runtime::graph) fn acquire(&mut self) -> RetainedValues {
        match self.idle.pop() {
            Some(cached) => {
                self.bytes -= cached.bytes;
                RetainedValues {
                    values: cached.values,
                    callable_domain: None,
                }
            }
            None => RetainedValues::empty(),
        }
    }

    pub(in crate::runtime::graph) fn recycle(&mut self, environment: BlockEnvironment) {
        if self.idle.len() == MAX_IDLE {
            return;
        }
        let mut values = environment.values;
        let bytes = values.clear();
        if bytes > MAX_BYTES - self.bytes {
            return;
        }
        if self.idle.capacity() == 0 {
            self.idle.reserve_exact(MAX_IDLE);
        }
        self.bytes += bytes;
        self.idle.push(CachedValues { values, bytes });
    }
}

impl BlockValues {
    fn clear(&mut self) -> usize {
        // Exhaustive destructuring keeps reset/accounting complete when columns
        // change. Clear in declaration order, matching normal environment Drop.
        macro_rules! clear_columns {
            ($($field:ident),+ $(,)?) => {{
                let Self { $($field),+ } = self;
                let mut bytes = size_of::<Self>();
                $(bytes = bytes.saturating_add(clear_column($field));)+
                bytes
            }};
        }
        clear_columns!(
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
}

fn clear_column<Value>(column: &mut Vec<Value>) -> usize {
    let bytes = column.capacity().saturating_mul(size_of::<Value>());
    column.clear();
    bytes
}

#[cfg(test)]
mod tests {
    use super::{BlockEnvironment, BlockValues, RetainedValues, StoragePool, clear_column};
    use crate::ExecutionPlan;
    use crate::plan::execution::function::IntFunctionId;
    use crate::runtime::CaptureStorage;
    use crate::runtime::graph::tests::{CanonicalProgress, canonical_progress};
    use crate::runtime::graph::{CompletedGraph, GraphExecution, GraphStorage};
    use crate::runtime::state::RuntimeState;
    use std::mem::size_of_val;
    use std::ptr;

    #[test]
    fn checkout_reuses_empty_columns_in_lifo_order_with_fresh_provenance() {
        let mut pool = StoragePool::default();
        assert_eq!(pool.idle.capacity(), 0);
        let mut first = pool.acquire();
        let second = pool.acquire();
        let first_owner = &*first.values as *const BlockValues;
        let second_owner = &*second.values as *const BlockValues;
        assert_ne!(first_owner, second_owner);
        first.push_int(42.into());
        let capacity = first.values.ints.capacity();
        let buffer = first.values.ints.as_ptr();
        let domain = CaptureStorage::default();
        first.append_captures(&domain.capture(Vec::new()));
        assert!(!first.belongs_to(None));
        pool.recycle(BlockEnvironment::from_retained(first));
        assert_eq!(pool.idle.len(), 1);
        assert!(ptr::eq(&*pool.idle[0].values, first_owner));
        let first_bytes = pool.bytes;
        pool.recycle(BlockEnvironment::from_retained(second));
        assert_eq!(pool.idle.len(), 2);
        assert_eq!(pool.idle.capacity(), 8);
        assert_eq!(pool.bytes, first_bytes + size_of::<BlockValues>());
        let second = pool.acquire();
        assert!(ptr::eq(&*second.values, second_owner));
        assert_eq!(pool.bytes, first_bytes);
        let mut first = pool.acquire();
        assert!(ptr::eq(&*first.values, first_owner));
        assert_eq!(first.values.ints.len(), 0);
        assert_eq!(first.values.ints.capacity(), capacity);
        assert_eq!(first.values.ints.as_ptr(), buffer);
        assert!(first.belongs_to(None));
        assert_eq!(pool.bytes, 0);
        assert!(pool.idle.is_empty());
        first.push_string("new call".into());
        assert!(first.values.ints.is_empty());
        assert_eq!(first.values.strings, ["new call"]);
        assert!(second.values.strings.is_empty());
        pool.recycle(BlockEnvironment::from_retained(first));
        let recycled = pool.acquire();
        assert!(recycled.values.strings.is_empty());
        assert_eq!(recycled.values.ints.capacity(), capacity);
        assert_eq!(pool.bytes, 0);
    }

    #[test]
    fn idle_count_is_bounded_without_eager_storage_or_replacing_entries() {
        let mut pool = StoragePool::default();
        let live: Vec<_> = (0..9).map(|_| pool.acquire()).collect();
        assert!(pool.idle.is_empty());
        assert_eq!(pool.idle.capacity(), 0);
        assert_eq!(pool.bytes, 0);
        let owners: Vec<_> = live
            .iter()
            .map(|value| &*value.values as *const BlockValues)
            .collect();
        for value in live {
            pool.recycle(BlockEnvironment::from_retained(value));
        }
        assert_eq!(pool.idle.len(), 8);
        assert_eq!(pool.idle.capacity(), 8);
        assert_eq!(pool.bytes, 8 * size_of::<BlockValues>());
        for (entry, owner) in pool.idle.iter().zip(&owners[..8]) {
            assert!(ptr::eq(&*entry.values, *owner));
        }
        for owner in owners[..8].iter().rev() {
            let value = pool.acquire();
            assert!(ptr::eq(&*value.values, *owner));
        }
        assert!(pool.idle.is_empty());
        assert_eq!(pool.bytes, 0);
        assert_eq!(pool.idle.capacity(), 8);
    }

    #[test]
    fn byte_limit_counts_box_and_backing_capacity_across_all_idle_entries() {
        for bytes in [65_535, 65_536, 65_537] {
            let mut pool = StoragePool::default();
            let mut input = pool.acquire();
            input
                .values
                .bools
                .reserve_exact(bytes - size_of::<BlockValues>());
            assert_eq!(
                input.values.bools.capacity(),
                bytes - size_of::<BlockValues>()
            );
            pool.recycle(BlockEnvironment::from_retained(input));
            let accepted = usize::from(bytes <= 65_536);
            assert_eq!(pool.idle.len(), accepted);
            assert_eq!(pool.bytes, accepted * bytes);
        }
        let mut pool = StoragePool::default();
        let mut first = pool.acquire();
        let mut second = pool.acquire();
        let third = pool.acquire();
        first
            .values
            .bools
            .reserve_exact(32_768 - size_of::<BlockValues>());
        second
            .values
            .bools
            .reserve_exact(32_768 - size_of::<BlockValues>());
        let first_owner = &*first.values as *const BlockValues;
        pool.recycle(BlockEnvironment::from_retained(first));
        assert_eq!(pool.bytes, 32_768);
        pool.recycle(BlockEnvironment::from_retained(second));
        assert_eq!(pool.bytes, 65_536);
        pool.recycle(BlockEnvironment::from_retained(third));
        assert_eq!(pool.idle.len(), 2);
        assert_eq!(pool.bytes, 65_536);
        drop(pool.acquire());
        assert_eq!(pool.bytes, 32_768);
        assert!(ptr::eq(&*pool.idle[0].values, first_owner));
        drop(pool.acquire());
        assert_eq!(pool.bytes, 0);
    }

    #[test]
    fn every_column_contributes_its_capacity_and_keeps_its_allocation() {
        let mut input = RetainedValues::empty();
        let values = &mut input.values;
        macro_rules! reserve_columns {
            ($($field:ident),+ $(,)?) => {{
                let mut expected = size_of::<BlockValues>();
                $(
                    values.$field.reserve_exact(2);
                    expected += size_of_val(values.$field.spare_capacity_mut());
                )+
                expected
            }};
        }
        let expected = reserve_columns!(
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
        );
        assert!(expected > size_of::<BlockValues>());
        assert!(expected < 65_536);
        let owner = &**values as *const BlockValues;
        let mut pool = StoragePool::default();
        pool.recycle(BlockEnvironment::from_retained(input));
        assert_eq!(pool.bytes, expected);
        assert_eq!(pool.idle.len(), 1);
        let input = pool.acquire();
        assert!(ptr::eq(&*input.values, owner));
        assert_eq!(pool.bytes, 0);
        pool.recycle(BlockEnvironment::from_retained(input));
        assert_eq!(pool.bytes, expected);
    }

    #[test]
    fn zero_sized_columns_clear_without_charging_their_unbounded_capacity() {
        let mut values = vec![(); 3];
        assert_eq!(values.capacity(), usize::MAX);
        assert_eq!(clear_column(&mut values), 0);
        assert!(values.is_empty());
        assert_eq!(values.capacity(), usize::MAX);
    }

    #[test]
    fn clearing_releases_non_clone_payloads_immediately_in_column_order() {
        use std::sync::{Arc, Mutex};

        struct Payload(usize, Arc<Mutex<Vec<usize>>>);
        impl Drop for Payload {
            fn drop(&mut self) {
                self.1.lock().unwrap().push(self.0);
            }
        }
        let dropped = Arc::new(Mutex::new(Vec::new()));
        let mut first = vec![Payload(1, dropped.clone()), Payload(2, dropped.clone())];
        let mut second = vec![Payload(3, dropped.clone())];
        let capacity = first.capacity();
        let buffer = first.as_ptr();
        let returned = first.remove(0);
        assert_eq!(clear_column(&mut first), capacity * size_of::<Payload>());
        assert_eq!(*dropped.lock().unwrap(), [2]);
        assert!(first.is_empty());
        assert_eq!(first.capacity(), capacity);
        assert_eq!(first.as_ptr(), buffer);
        clear_column(&mut second);
        assert_eq!(*dropped.lock().unwrap(), [2, 3]);
        drop(first);
        drop(second);
        assert_eq!(*dropped.lock().unwrap(), [2, 3]);
        drop(returned);
        assert_eq!(*dropped.lock().unwrap(), [2, 3, 1]);
    }

    #[test]
    fn repeated_source_calls_and_constants_checkout_returned_storage() {
        for source in [
            "fn add(value) { value + 1 } pub fn main() { let a = add(20) let b = add(a) b + 20 }",
            "const number = 1 pub fn main() { number + number + 40 }",
        ] {
            let plan = crate::runtime::plan_src(source);
            let graph = plan
                .int_function(IntFunctionId(0))
                .body()
                .block_graph()
                .as_view();
            let mut execution = GraphExecution::new(graph, RetainedValues::empty(), None);
            let mut storage = GraphStorage::new();
            let mut echo = Vec::new();
            let mut state = RuntimeState::new(&mut echo);
            let mut pooled = None;
            let mut checked_out = false;
            let mut returned_again = false;
            let completed = loop {
                let step = execution
                    .advance(&plan, &mut state, &mut storage, &mut 0)
                    .unwrap();
                assert!(storage.pool.idle.len() <= 1);
                if let Some(entry) = storage.pool.idle.first() {
                    assert!(entry.values.ints.is_empty());
                    assert!(entry.values.ints.capacity() > 0);
                    let observed = (
                        &*entry.values as *const BlockValues,
                        entry.values.ints.capacity(),
                    );
                    if let Some(previous) = pooled {
                        assert_eq!(observed, previous);
                        returned_again |= checked_out;
                    } else {
                        pooled = Some(observed);
                    }
                } else if pooled.is_some() {
                    checked_out = true;
                    assert_eq!(storage.pool.bytes, 0);
                }
                match canonical_progress(step) {
                    CanonicalProgress::Continue(next) => execution = next,
                    CanonicalProgress::Complete(completed) => break completed,
                }
            };
            assert!(pooled.is_some());
            assert!(checked_out);
            assert!(returned_again);
            assert_eq!(storage.pool.idle.len(), 1);
            assert_eq!(completed.environment.values.ints.last(), Some(&42.into()));
            // Completing the root does not recycle its live result into the pool.
            assert!(!ptr::eq(
                &*completed.environment.values,
                &*storage.pool.idle[0].values
            ));
            drop(storage);
            assert_eq!(completed.environment.values.ints.last(), Some(&42.into()));
            assert!(echo.is_empty());
        }
    }

    #[test]
    fn source_external_values_lists_and_callables_clear_their_own_columns() {
        use crate::host::{ExternalTestProfile, ExternalTestRunState};
        use crate::runtime::profile::external_test::{
            RuntimeCounterProvider, RuntimeCounterSchema, RuntimeHostCounter,
        };
        use crate::{
            HostCall, HostCallCompletion, HostCallError, HostModule, HostProviderModule,
            HostProviderSet, HostTypeParameter, HostValue, HostedExecution, ModuleSource,
            PackageSource, Value,
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

        fn check<'call>(
            call: HostCall<'call, ExternalTestProfile, RuntimeCounterProvider, ()>,
            value: HostValue<'call, HostTypeParameter<0>>,
            column: BigInt,
        ) -> Result<HostCallCompletion<'call, ()>, HostCallError> {
            let value = call.retain_value::<HostTypeParameter<0>>(value);
            let mut input = RetainedValues::empty();
            input.push_evaluated(value.value().clone());
            let values = &input.values;
            assert_eq!(
                [
                    values.externals.len(),
                    values.external_lists.len(),
                    values.external_functions.len(),
                    values.external_list_functions.len(),
                    values.external_function_functions.len()
                ],
                std::array::from_fn(|index| usize::from(column == BigInt::from(index))),
            );
            let capacities = [
                values.externals.capacity(),
                values.external_lists.capacity(),
                values.external_functions.capacity(),
                values.external_list_functions.capacity(),
                values.external_function_functions.capacity(),
            ];
            let mut pool = StoragePool::default();
            pool.recycle(BlockEnvironment::from_retained(input));
            assert_eq!(pool.idle.len(), 1);
            let reused = pool.acquire();
            let values = &reused.values;
            assert_eq!(
                [
                    values.externals.len(),
                    values.external_lists.len(),
                    values.external_functions.len(),
                    values.external_list_functions.len(),
                    values.external_function_functions.len()
                ],
                [0; 5]
            );
            assert_eq!(
                [
                    values.externals.capacity(),
                    values.external_lists.capacity(),
                    values.external_functions.capacity(),
                    values.external_list_functions.capacity(),
                    values.external_function_functions.capacity()
                ],
                capacities
            );
            Ok(call.return_value(()))
        }

        let provider = HostProviderModule::<ExternalTestProfile>::new("application", "main")
            .unwrap()
            .with_external_type::<RuntimeCounterProvider, RuntimeCounterSchema>()
            .unwrap()
            .with_scoped_function::<RuntimeCounterProvider, (BigInt,), RuntimeHostCounter, _>(
                "counter", counter,
            )
            .unwrap()
            .with_scoped_function::<RuntimeCounterProvider, (HostTypeParameter<0>, BigInt), (), _>(
                "check", check,
            )
            .unwrap();
        let source = r#"
@external(erlang, "host", "Counter")
pub type Counter
@external(erlang, "host", "counter")
fn counter(value: Int) -> Counter
@external(erlang, "host", "check")
fn check(value: a, column: Int) -> Nil
fn counters() { [counter(42)] }
fn callback() { counter }
pub fn main() {
  check(counter(42), 0)
  check(counters(), 1)
  check(counter, 2)
  check(counters, 3)
  check(callback, 4)
  42
}
"#;
        let typed = crate::compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                Vec::<&str>::new(),
                [ModuleSource::new("main", "main.gleam", source)],
            )],
            HostProviderSet::with_providers(
                Vec::<HostModule<ExternalTestProfile>>::new(),
                [provider],
            )
            .unwrap(),
        )
        .unwrap();
        let mut execution =
            HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let mut echo = Vec::new();
        assert_eq!(
            crate::execution_fixture::run(
                &mut execution,
                &mut ExternalTestRunState::default(),
                &mut echo
            ),
            Ok(Value::Int(42.into()))
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn source_values_and_callables_are_cleared_from_every_plain_column() {
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
"#;
        type Column = fn(&BlockValues) -> (usize, usize);
        let cases: &[(&str, Column)] = &[
            ("7", |v| (v.ints.len(), v.ints.capacity())),
            ("1.5", |v| (v.floats.len(), v.floats.capacity())),
            ("\"text\"", |v| (v.strings.len(), v.strings.capacity())),
            ("<<42>>", |v| (v.bit_arrays.len(), v.bit_arrays.capacity())),
            ("codepoint()", |v| {
                (v.utf_codepoints.len(), v.utf_codepoints.capacity())
            }),
            ("Box(42)", |v| (v.customs.len(), v.customs.capacity())),
            ("True", |v| (v.bools.len(), v.bools.capacity())),
            ("#(42, True)", |v| (v.tuples.len(), v.tuples.capacity())),
            ("empty()", |v| {
                (v.parameter_lists.len(), v.parameter_lists.capacity())
            }),
            ("nested_empty()", |v| {
                (
                    v.parameter_list_lists.len(),
                    v.parameter_list_lists.capacity(),
                )
            }),
            ("[42]", |v| (v.int_lists.len(), v.int_lists.capacity())),
            ("[1.5]", |v| (v.float_lists.len(), v.float_lists.capacity())),
            ("[\"text\"]", |v| {
                (v.string_lists.len(), v.string_lists.capacity())
            }),
            ("[<<42>>]", |v| {
                (v.bit_array_lists.len(), v.bit_array_lists.capacity())
            }),
            ("[codepoint()]", |v| {
                (
                    v.utf_codepoint_lists.len(),
                    v.utf_codepoint_lists.capacity(),
                )
            }),
            ("[Box(42)]", |v| {
                (v.custom_lists.len(), v.custom_lists.capacity())
            }),
            ("[True]", |v| (v.bool_lists.len(), v.bool_lists.capacity())),
            ("[Nil]", |v| (v.nil_lists.len(), v.nil_lists.capacity())),
            ("[#(42, True)]", |v| {
                (v.tuple_lists.len(), v.tuple_lists.capacity())
            }),
            ("[[42]]", |v| (v.list_lists.len(), v.list_lists.capacity())),
            ("[integer]", |v| {
                (v.function_lists.len(), v.function_lists.capacity())
            }),
            ("integer", |v| {
                (v.int_functions.len(), v.int_functions.capacity())
            }),
            ("decimal", |v| {
                (v.float_functions.len(), v.float_functions.capacity())
            }),
            ("text", |v| {
                (v.string_functions.len(), v.string_functions.capacity())
            }),
            ("bits", |v| {
                (
                    v.bit_array_functions.len(),
                    v.bit_array_functions.capacity(),
                )
            }),
            ("codepoint", |v| {
                (
                    v.utf_codepoint_functions.len(),
                    v.utf_codepoint_functions.capacity(),
                )
            }),
            ("boxed", |v| {
                (v.custom_functions.len(), v.custom_functions.capacity())
            }),
            ("flag", |v| {
                (v.bool_functions.len(), v.bool_functions.capacity())
            }),
            ("nil", |v| {
                (v.nil_functions.len(), v.nil_functions.capacity())
            }),
            ("pair", |v| {
                (v.tuple_functions.len(), v.tuple_functions.capacity())
            }),
            ("empty", |v| {
                (
                    v.parameter_list_functions.len(),
                    v.parameter_list_functions.capacity(),
                )
            }),
            ("nested_empty", |v| {
                (
                    v.parameter_list_list_functions.len(),
                    v.parameter_list_list_functions.capacity(),
                )
            }),
            ("integers", |v| {
                (v.int_list_functions.len(), v.int_list_functions.capacity())
            }),
            ("decimals", |v| {
                (
                    v.float_list_functions.len(),
                    v.float_list_functions.capacity(),
                )
            }),
            ("texts", |v| {
                (
                    v.string_list_functions.len(),
                    v.string_list_functions.capacity(),
                )
            }),
            ("arrays", |v| {
                (
                    v.bit_array_list_functions.len(),
                    v.bit_array_list_functions.capacity(),
                )
            }),
            ("codepoints", |v| {
                (
                    v.utf_codepoint_list_functions.len(),
                    v.utf_codepoint_list_functions.capacity(),
                )
            }),
            ("boxes", |v| {
                (
                    v.custom_list_functions.len(),
                    v.custom_list_functions.capacity(),
                )
            }),
            ("flags", |v| {
                (
                    v.bool_list_functions.len(),
                    v.bool_list_functions.capacity(),
                )
            }),
            ("nils", |v| {
                (v.nil_list_functions.len(), v.nil_list_functions.capacity())
            }),
            ("pairs", |v| {
                (
                    v.tuple_list_functions.len(),
                    v.tuple_list_functions.capacity(),
                )
            }),
            ("nested", |v| {
                (
                    v.list_list_functions.len(),
                    v.list_list_functions.capacity(),
                )
            }),
            ("callbacks", |v| {
                (
                    v.function_list_functions.len(),
                    v.function_list_functions.capacity(),
                )
            }),
            ("callback", |v| {
                (
                    v.core_function_functions.len(),
                    v.core_function_functions.capacity(),
                )
            }),
            ("identity", |v| {
                (v.generic_functions.len(), v.generic_functions.capacity())
            }),
            ("stop", |v| {
                (v.never_functions.len(), v.never_functions.capacity())
            }),
        ];
        assert_eq!(cases.len(), 46);
        for (expression, column) in cases {
            let source = format!(
                "{declarations}\npub fn main() {{ let value = {expression} let pair = #(value, 42) pair.1 }}"
            );
            let plan = crate::runtime::plan_src(&source);
            let completed = complete_int_graph(&plan);
            assert_eq!(completed.environment.values.ints.last(), Some(&42.into()));
            let (len, capacity) = column(&completed.environment.values);
            assert!(len > 0, "{expression}");
            assert!(capacity >= len, "{expression}");
            let mut pool = StoragePool::default();
            pool.recycle(completed.environment);
            assert_eq!(pool.idle.len(), 1, "{expression}");
            assert_eq!(column(&pool.idle[0].values), (0, capacity), "{expression}");
            let reused = pool.acquire();
            assert_eq!(column(&reused.values), (0, capacity), "{expression}");
        }
    }

    fn complete_int_graph(plan: &ExecutionPlan) -> CompletedGraph {
        let graph = plan
            .int_function(IntFunctionId(0))
            .body()
            .block_graph()
            .as_view();
        let mut execution = GraphExecution::new(graph, RetainedValues::empty(), None);
        let mut storage = GraphStorage::new();
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        loop {
            match canonical_progress(
                execution
                    .advance(plan, &mut state, &mut storage, &mut 0)
                    .unwrap(),
            ) {
                CanonicalProgress::Continue(next) => execution = next,
                CanonicalProgress::Complete(completed) => return completed,
            }
        }
    }
}
