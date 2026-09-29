use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use super::function::{
    EvaluatedCustomFunction, EvaluatedFunction, EvaluatedFunctionIdentity, EvaluatedFunctionValue,
    EvaluatedFunctionValueKind,
};
use super::{EvaluatedFunctionRef, EvaluatedListRef, EvaluatedValue, EvaluatedValueRef};

pub(in crate::runtime) fn values_equal(
    storage: &crate::runtime::RuntimeListStorage,
    left: &EvaluatedValue,
    right: &EvaluatedValue,
) -> bool {
    value_refs_equal(storage, &left.into(), &right.into())
}

pub(in crate::runtime) fn value_refs_equal(
    storage: &crate::runtime::RuntimeListStorage,
    left: &EvaluatedValueRef<'_>,
    right: &EvaluatedValueRef<'_>,
) -> bool {
    match (left, right) {
        (EvaluatedValueRef::Int(left), EvaluatedValueRef::Int(right)) => left == right,
        (EvaluatedValueRef::Float(left), EvaluatedValueRef::Float(right)) => left == right,
        (EvaluatedValueRef::String(left), EvaluatedValueRef::String(right)) => left == right,
        (EvaluatedValueRef::BitArray(left), EvaluatedValueRef::BitArray(right)) => left == right,
        (EvaluatedValueRef::UtfCodepoint(left), EvaluatedValueRef::UtfCodepoint(right)) => {
            left == right
        }
        (EvaluatedValueRef::Custom(left), EvaluatedValueRef::Custom(right)) => {
            left.constructor == right.constructor
                && left.fields.len() == right.fields.len()
                && left
                    .fields
                    .iter()
                    .zip(right.fields.iter())
                    .all(|(left, right)| values_equal(storage, left, right))
        }
        (EvaluatedValueRef::External(left), EvaluatedValueRef::External(right)) => {
            super::external::source::values_equal(storage, left, right)
        }
        (EvaluatedValueRef::Bool(left), EvaluatedValueRef::Bool(right)) => left == right,
        (EvaluatedValueRef::Nil, EvaluatedValueRef::Nil) => true,
        (EvaluatedValueRef::Tuple(left), EvaluatedValueRef::Tuple(right)) => {
            left.len() == right.len()
                && left
                    .iter()
                    .zip(right.iter())
                    .all(|(left, right)| values_equal(storage, left, right))
        }
        (EvaluatedValueRef::ParameterList(left), EvaluatedValueRef::ParameterList(right)) => {
            left.type_id() == right.type_id()
        }
        (EvaluatedValueRef::List(left), EvaluatedValueRef::List(right)) => {
            lists_equal(storage, left, right)
        }
        (EvaluatedValueRef::Function(left), EvaluatedValueRef::Function(right)) => {
            functions_equal(left, right)
        }
        _ => false,
    }
}

pub(in crate::runtime) fn value_source_hash(
    storage: &crate::runtime::RuntimeListStorage,
    value: &EvaluatedValue,
) -> u64 {
    let mut hasher = DefaultHasher::new();
    hash_value(storage, value, &mut hasher);
    hasher.finish()
}

fn hash_value(
    storage: &crate::runtime::RuntimeListStorage,
    value: &EvaluatedValue,
    hasher: &mut DefaultHasher,
) {
    match value {
        EvaluatedValue::Int(value) => {
            0u8.hash(hasher);
            value.hash(hasher);
        }
        EvaluatedValue::Float(value) => {
            1u8.hash(hasher);
            if *value == 0.0 {
                0u64.hash(hasher);
            } else {
                value.to_bits().hash(hasher);
            }
        }
        EvaluatedValue::String(value) => {
            2u8.hash(hasher);
            value.hash(hasher);
        }
        EvaluatedValue::BitArray(value) => {
            3u8.hash(hasher);
            value.bits().len().hash(hasher);
            value.value.bytes().hash(hasher);
        }
        EvaluatedValue::UtfCodepoint(value) => {
            4u8.hash(hasher);
            value.hash(hasher);
        }
        EvaluatedValue::Custom(value) => {
            5u8.hash(hasher);
            value.constructor().hash(hasher);
            value.fields().len().hash(hasher);
            for field in value.fields() {
                hash_value(storage, field, hasher);
            }
        }
        EvaluatedValue::External(value) => {
            6u8.hash(hasher);
            value.type_id().hash(hasher);
            super::external::source::source_hash(storage, value).hash(hasher);
        }
        EvaluatedValue::Bool(value) => {
            7u8.hash(hasher);
            value.hash(hasher);
        }
        EvaluatedValue::Nil => {
            8u8.hash(hasher);
        }
        EvaluatedValue::Tuple(values) => {
            9u8.hash(hasher);
            values.len().hash(hasher);
            for value in values {
                hash_value(storage, value, hasher);
            }
        }
        EvaluatedValue::ParameterList(value) => {
            10u8.hash(hasher);
            value.type_id().hash(hasher);
        }
        EvaluatedValue::List(value) => {
            11u8.hash(hasher);
            value.list_type().hash(hasher);
            let values = storage.evaluated_values(value);
            values.len().hash(hasher);
            for value in &values {
                hash_value(storage, value, hasher);
            }
        }
        EvaluatedValue::Function(value) => {
            12u8.hash(hasher);
            hash_function(value, hasher);
        }
    }
}

fn hash_function(value: &EvaluatedFunctionValue, hasher: &mut DefaultHasher) {
    match value.kind() {
        EvaluatedFunctionValueKind::Generic(value) => {
            0u8.hash(hasher);
            hash_function_identity(&value.identity, hasher);
        }
        EvaluatedFunctionValueKind::Never(value) => {
            1u8.hash(hasher);
            hash_function_identity(&value.identity, hasher);
        }
        EvaluatedFunctionValueKind::Int(value) => {
            2u8.hash(hasher);
            hash_function_identity(&value.identity, hasher);
        }
        EvaluatedFunctionValueKind::Float(value) => {
            3u8.hash(hasher);
            hash_function_identity(&value.identity, hasher);
        }
        EvaluatedFunctionValueKind::String(value) => {
            4u8.hash(hasher);
            hash_function_identity(&value.identity, hasher);
        }
        EvaluatedFunctionValueKind::BitArray(value) => {
            5u8.hash(hasher);
            hash_function_identity(&value.identity, hasher);
        }
        EvaluatedFunctionValueKind::UtfCodepoint(value) => {
            6u8.hash(hasher);
            hash_function_identity(&value.identity, hasher);
        }
        EvaluatedFunctionValueKind::Custom(value) => {
            7u8.hash(hasher);
            match value {
                EvaluatedCustomFunction::Function(value) => {
                    0u8.hash(hasher);
                    hash_function_identity(&value.identity, hasher);
                }
                EvaluatedCustomFunction::Constructor(value) => {
                    1u8.hash(hasher);
                    hash_function_identity(&value.identity, hasher);
                }
            }
        }
        EvaluatedFunctionValueKind::External(value) => {
            8u8.hash(hasher);
            hash_function_identity(&value.identity, hasher);
        }
        EvaluatedFunctionValueKind::Bool(value) => {
            9u8.hash(hasher);
            hash_function_identity(&value.identity, hasher);
        }
        EvaluatedFunctionValueKind::Nil(value) => {
            10u8.hash(hasher);
            hash_function_identity(&value.identity, hasher);
        }
        EvaluatedFunctionValueKind::Tuple(value) => {
            11u8.hash(hasher);
            hash_function_identity(&value.identity, hasher);
        }
        EvaluatedFunctionValueKind::List(value) => {
            12u8.hash(hasher);
            hash_function_identity(&value.identity, hasher);
        }
        EvaluatedFunctionValueKind::Function(value) => {
            13u8.hash(hasher);
            hash_function_identity(value.identity(), hasher);
        }
    }
}

fn hash_function_identity(value: &EvaluatedFunctionIdentity, hasher: &mut DefaultHasher) {
    match value {
        EvaluatedFunctionIdentity::Reference(value) => {
            0u8.hash(hasher);
            value.hash(hasher);
        }
        EvaluatedFunctionIdentity::Instance(value) => {
            1u8.hash(hasher);
            value.0.hash(hasher);
        }
    }
}

fn lists_equal(
    storage: &crate::runtime::RuntimeListStorage,
    left: &EvaluatedListRef<'_>,
    right: &EvaluatedListRef<'_>,
) -> bool {
    if left.list_type() != right.list_type() {
        return false;
    }

    let length = left.len();
    length == right.len()
        && left
            .iter_prefix(length)
            .zip(right.iter_prefix(length))
            .all(|(left, right)| value_refs_equal(storage, &left, &right))
}

fn functions_equal(left: &EvaluatedFunctionRef<'_>, right: &EvaluatedFunctionRef<'_>) -> bool {
    match (left, right) {
        (EvaluatedFunctionRef::Generic(left), EvaluatedFunctionRef::Generic(right)) => {
            function_values_equal(left, right)
        }
        (EvaluatedFunctionRef::Never(left), EvaluatedFunctionRef::Never(right)) => {
            function_values_equal(left, right)
        }
        (EvaluatedFunctionRef::Int(left), EvaluatedFunctionRef::Int(right)) => {
            function_values_equal(left, right)
        }
        (EvaluatedFunctionRef::Float(left), EvaluatedFunctionRef::Float(right)) => {
            function_values_equal(left, right)
        }
        (EvaluatedFunctionRef::String(left), EvaluatedFunctionRef::String(right)) => {
            function_values_equal(left, right)
        }
        (EvaluatedFunctionRef::BitArray(left), EvaluatedFunctionRef::BitArray(right)) => {
            function_values_equal(left, right)
        }
        (EvaluatedFunctionRef::UtfCodepoint(left), EvaluatedFunctionRef::UtfCodepoint(right)) => {
            function_values_equal(left, right)
        }
        (EvaluatedFunctionRef::Custom(left), EvaluatedFunctionRef::Custom(right)) => {
            custom_function_values_equal(left, right)
        }
        (EvaluatedFunctionRef::External(left), EvaluatedFunctionRef::External(right)) => {
            function_values_equal(left, right)
        }
        (EvaluatedFunctionRef::Bool(left), EvaluatedFunctionRef::Bool(right)) => {
            function_values_equal(left, right)
        }
        (EvaluatedFunctionRef::Nil(left), EvaluatedFunctionRef::Nil(right)) => {
            function_values_equal(left, right)
        }
        (EvaluatedFunctionRef::Tuple(left), EvaluatedFunctionRef::Tuple(right)) => {
            function_values_equal(left, right)
        }
        (EvaluatedFunctionRef::List(left), EvaluatedFunctionRef::List(right)) => {
            function_values_equal(left, right)
        }
        (EvaluatedFunctionRef::ExternalList(left), EvaluatedFunctionRef::ExternalList(right)) => {
            function_values_equal(left, right)
        }
        (EvaluatedFunctionRef::List(left), EvaluatedFunctionRef::ExternalList(right)) => {
            function_values_equal(left, right)
        }
        (EvaluatedFunctionRef::ExternalList(left), EvaluatedFunctionRef::List(right)) => {
            function_values_equal(left, right)
        }
        (EvaluatedFunctionRef::CoreFunction(left), EvaluatedFunctionRef::CoreFunction(right)) => {
            function_values_equal(left, right)
        }
        (
            EvaluatedFunctionRef::ExternalFunction(left),
            EvaluatedFunctionRef::ExternalFunction(right),
        ) => function_values_equal(left, right),
        (
            EvaluatedFunctionRef::CoreFunction(left),
            EvaluatedFunctionRef::ExternalFunction(right),
        ) => function_values_equal(left, right),
        (
            EvaluatedFunctionRef::ExternalFunction(left),
            EvaluatedFunctionRef::CoreFunction(right),
        ) => function_values_equal(left, right),
        _ => false,
    }
}

fn function_values_equal<LeftId, RightId>(
    left: &EvaluatedFunction<LeftId>,
    right: &EvaluatedFunction<RightId>,
) -> bool {
    left.identity == right.identity
}

fn custom_function_values_equal(
    left: &EvaluatedCustomFunction,
    right: &EvaluatedCustomFunction,
) -> bool {
    match (left, right) {
        (EvaluatedCustomFunction::Function(left), EvaluatedCustomFunction::Function(right)) => {
            function_values_equal(left, right)
        }
        (
            EvaluatedCustomFunction::Constructor(left),
            EvaluatedCustomFunction::Constructor(right),
        ) => function_values_equal(left, right),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::super::function::{
        EvaluatedBitArrayFunction, EvaluatedBoolFunction, EvaluatedCustomFunction,
        EvaluatedFloatFunction, EvaluatedFunction, EvaluatedFunctionFunction,
        EvaluatedFunctionValue, EvaluatedIntFunction, EvaluatedListFunction,
        EvaluatedNeverFunction, EvaluatedNilFunction, EvaluatedStringFunction,
        EvaluatedTupleFunction, EvaluatedUtfCodepointFunction,
    };
    use super::super::{
        EvaluatedBitArray, EvaluatedCustomValue, EvaluatedExternalValue, EvaluatedValue,
    };
    use super::{lists_equal, value_source_hash, values_equal};
    use crate::plan::execution::function::{
        BitArrayFunctionId, BoolFunctionId, FloatFunctionId, IntFunctionFunctionId, IntFunctionId,
        ListFunctionId, NeverFunctionId, NilFunctionId, ProfiledFunctionFunctionId,
        RuntimeListFunctionId, StringFunctionId, TupleFunctionId, UtfCodepointFunctionId,
    };
    use crate::runtime::evaluated::EvaluatedListRef;
    use crate::runtime::state::RuntimeState;
    use crate::runtime::state::list::{ListValueId, ParameterListValueId, StoredListValueId};
    use bitvec::order::Msb0;

    #[test]
    fn function_returning_references_preserve_identity_across_return_families() {
        use crate::execution_fixture::TestHost;
        use crate::host::{HostComponentProfile, HostFutureStore, HostProfile, HostWorkProfile};
        use crate::runtime::execution::Domain;
        use crate::runtime::graph::RetainedValues;
        use crate::runtime::{HostCallOrigin, RuntimeListStorage};
        use crate::work_fixture::WorkComponent;
        use crate::{
            HostProviderSet, HostedExecution, ModuleSource, PackageSource,
            compile_typed_host_program, plan_host_program,
        };
        use std::{ptr, sync::Arc};

        struct Profile;
        impl HostProfile for Profile {
            type RunState = ();
            type ExternalStores = HostFutureStore;
            type ExecutionState = ();
        }
        impl HostWorkProfile for Profile {
            type Work = WorkComponent;
        }
        impl HostComponentProfile<WorkComponent> for Profile {
            fn component_stores(stores: &HostFutureStore) -> &HostFutureStore {
                stores
            }
            fn component_state(state: &mut ()) -> &mut () {
                state
            }
        }

        let source = r#"
import fixture/work
fn core() { fn() { 42 } }
fn external() { fn() { work.ready(42) } }
fn different() { fn() { work.ready(42) } }
pub fn main() { #(core, external, external, different) }
"#;
        let typed = compile_typed_host_program(
            "application",
            "main",
            [
                PackageSource::new(
                    "work_fixture",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "fixture/work",
                        "fixture/work.gleam",
                        WorkComponent::SOURCE,
                    )],
                ),
                PackageSource::new(
                    "application",
                    ["work_fixture"],
                    [ModuleSource::new("main", "main.gleam", source)],
                ),
            ],
            HostProviderSet::from_providers(WorkComponent::providers::<Profile>().unwrap())
                .unwrap(),
        )
        .unwrap();
        let mut execution =
            HostedExecution::try_from_module_plan(plan_host_program(typed).unwrap()).unwrap();
        let (plan, stores, captures) = execution.parts_mut();
        assert!(ptr::eq(Profile::component_stores(stores), stores));
        let host = TestHost::default();
        let mut state = ();
        assert!(ptr::eq(Profile::component_state(&mut state), &state));
        let mut echo = Vec::new();
        let domain = Domain::new(
            Arc::clone(plan),
            &host,
            &mut state,
            stores,
            &mut echo,
            captures.clone(),
            Domain::<Profile>::DEFAULT_BUDGET,
        );
        let context = domain.context();
        host.block_on(domain.drive(async {
            let values = context
                .call(
                    TupleFunctionId(0),
                    HostCallOrigin::Entry,
                    RetainedValues::empty(),
                )
                .await
                .unwrap()
                .unwrap();
            assert_eq!(values.len(), 4);
            let storage = RuntimeListStorage::default();
            let core = &values[0];
            let external = &values[1];
            let same = &values[2];
            let different = &values[3];
            assert!(!values_equal(&storage, core, external));
            assert!(!values_equal(&storage, external, core));
            assert!(values_equal(&storage, external, same));
            assert!(!values_equal(&storage, external, different));
            assert_eq!(
                value_source_hash(&storage, external),
                value_source_hash(&storage, same),
            );
        }))
        .unwrap();
        assert!(echo.is_empty());
    }

    const EVERY_LIST_FAMILY_SOURCE: &str = r#"
fn ints() -> List(Int) { [] }
fn strings() -> List(String) { [] }
fn bit_arrays() -> List(BitArray) { [] }
fn utf_codepoints() -> List(UtfCodepoint) { [] }
pub type Boxed { Boxed(Int) }
fn customs() -> List(Boxed) { [] }
fn custom() -> Boxed { Boxed(1) }
fn floats() -> List(Float) { [] }
fn bools() -> List(Bool) { [] }
fn nils() -> List(Nil) { [] }
fn tuples() -> List(#(Int)) { [] }
fn lists() -> List(List(Int)) { [] }
fn functions() -> List(fn() -> Int) { [] }
fn parameters(values: List(value)) { values }
fn parameter_lists(values: List(List(value))) { values }
fn take_function_function(value: fn() -> fn() -> Int) { 0 }
pub fn main() {
  let _ = #(
    ints,
    strings,
    bit_arrays,
    utf_codepoints,
    customs,
    custom,
    floats,
    bools,
    nils,
    tuples,
    lists,
    functions,
    take_function_function,
  )
  let _ = parameters([])
  let _ = parameter_lists([[]])
  0
}
"#;
    #[test]
    fn list_inspection_equality_stops_at_length_or_first_unequal_pair() {
        let plan = crate::runtime::plan_src(
            r#"
fn ints() -> List(Int) { [] }
pub fn main() { ints() }
"#,
        );
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        let type_id = plan.int_list_function_id(0).type_id();
        for (left, right, expected, reads) in [
            (vec![], vec![], true, 0),
            (vec![1, 2, 3], vec![], false, 0),
            (vec![], vec![1, 2, 3], false, 0),
            (vec![1, 2, 3], vec![1, 2], false, 0),
            (vec![1, 2, 3], vec![9, 2, 3], false, 1),
            (vec![1, 2, 3], vec![1, 9, 3], false, 2),
            (vec![1, 2, 3], vec![1, 2, 9], false, 3),
            (vec![1, 2, 3], vec![1, 2, 3], true, 3),
        ] {
            let left_handle: StoredListValueId = state
                .lists_mut()
                .int(type_id, left.into_iter().map(Into::into).collect())
                .into();
            let left = EvaluatedListRef::from(&left_handle);
            let right_handle: StoredListValueId = state
                .lists_mut()
                .int(type_id, right.into_iter().map(Into::into).collect())
                .into();
            let right = EvaluatedListRef::from(&right_handle);
            assert_eq!(lists_equal(state.lists(), &left, &right), expected);
            assert_eq!(left.item_reads(), reads);
            assert_eq!(right.item_reads(), reads);
        }
        assert!(echo.is_empty());
    }

    #[test]
    fn list_inspection_equality_rejects_different_exact_types_without_reading() {
        let plan = crate::runtime::plan_src(
            r#"
fn int_tuples() -> List(#(Int)) { [] }
fn string_tuples() -> List(#(String)) { [] }
pub fn main() { #(int_tuples(), string_tuples()) }
"#,
        );
        let storage = crate::runtime::RuntimeListStorage::default();
        let left_handle: StoredListValueId = storage
            .tuple(
                plan.tuple_list_function_id(0).type_id(),
                vec![vec![EvaluatedValue::Int(1.into())]],
            )
            .into();
        let left = EvaluatedListRef::from(&left_handle);
        let right_handle: StoredListValueId = storage
            .tuple(
                plan.tuple_list_function_id(1).type_id(),
                vec![vec![EvaluatedValue::String("one".into())]],
            )
            .into();
        let right = EvaluatedListRef::from(&right_handle);

        assert!(!lists_equal(&storage, &left, &right));
        assert_eq!(left.item_reads(), 0);
        assert_eq!(right.item_reads(), 0);
    }

    #[test]
    fn list_inspection_equality_uses_each_escaped_storage_owner() {
        let plan = crate::runtime::plan_src(
            r#"
fn ints() -> List(Int) { [] }
fn lists() -> List(List(Int)) { [] }
pub fn main() { #(ints(), lists()) }
"#,
        );
        let int_type = plan.int_list_function_id(0).type_id();
        let list_type = plan.list_list_function_id(0).type_id();
        let first_owner = crate::runtime::RuntimeListStorage::default();
        let second_owner = crate::runtime::RuntimeListStorage::default();
        let first = first_owner.int(int_type, vec![1.into(), 2.into()]);
        let second = second_owner.int(int_type, vec![1.into(), 2.into()]);
        let different = second_owner.int(int_type, vec![1.into(), 9.into()]);
        let left_handle: StoredListValueId = first_owner.list(list_type, vec![first.into()]).into();
        let left = EvaluatedListRef::from(&left_handle);
        let right_handle: StoredListValueId =
            second_owner.list(list_type, vec![second.into()]).into();
        let right = EvaluatedListRef::from(&right_handle);
        let unequal_handle: StoredListValueId =
            second_owner.list(list_type, vec![different.into()]).into();
        let unequal = EvaluatedListRef::from(&unequal_handle);
        drop(first_owner);
        drop(second_owner);

        let caller = crate::runtime::RuntimeListStorage::default();
        let unrelated = caller.int(int_type, vec![999.into()]);
        let unrelated_lists = caller.list(list_type, vec![unrelated.into()]);
        assert!(lists_equal(&caller, &left, &right));
        assert!(!lists_equal(&caller, &left, &unequal));
        assert_eq!(left.item_reads(), 2);
        assert_eq!(right.item_reads(), 1);
        assert_eq!(unequal.item_reads(), 1);
        drop(unrelated_lists);
    }

    #[test]
    fn list_inspection_equality_preserves_nan_and_signed_zero_semantics() {
        let plan = crate::runtime::plan_src(
            r#"
fn floats() -> List(Float) { [] }
pub fn main() { floats() }
"#,
        );
        let storage = crate::runtime::RuntimeListStorage::default();
        let type_id = plan.float_list_function_id(0).type_id();
        let nan_handle: StoredListValueId = storage.float(type_id, vec![f64::NAN]).into();
        let nan = EvaluatedListRef::from(&nan_handle);
        assert!(!lists_equal(&storage, &nan, &nan));
        assert_eq!(nan.item_reads(), 2);
        let value = EvaluatedValue::List(nan_handle.clone());
        assert!(!values_equal(&storage, &value, &value));

        let positive_handle: StoredListValueId = storage.float(type_id, vec![0.0]).into();
        let positive = EvaluatedListRef::from(&positive_handle);
        let negative_handle: StoredListValueId = storage.float(type_id, vec![-0.0]).into();
        let negative = EvaluatedListRef::from(&negative_handle);
        assert!(lists_equal(&storage, &positive, &negative));
        assert_eq!(positive.item_reads(), 1);
        assert_eq!(negative.item_reads(), 1);
        assert_eq!(
            value_source_hash(&storage, &EvaluatedValue::List(positive_handle.clone())),
            value_source_hash(&storage, &EvaluatedValue::List(negative_handle.clone())),
        );
    }

    #[test]
    fn bit_array_ranges_compare_and_hash_by_logical_content() {
        let storage = crate::runtime::RuntimeListStorage::default();
        let original = crate::BitArrayValue::from_bytes(vec![0xab, 0xcd, 0xab, 0xcd]);
        for (selected, expected) in [
            (
                original.bit_slice(0, 16).unwrap(),
                crate::BitArrayValue::from_bytes(vec![0xab, 0xcd]),
            ),
            (
                original.bit_slice(16, 16).unwrap(),
                crate::BitArrayValue::from_bytes(vec![0xab, 0xcd]),
            ),
            (
                original.bit_slice(4, 8).unwrap(),
                crate::BitArrayValue::from_bytes(vec![0xbc]),
            ),
            (
                original.bit_slice(0, 4).unwrap(),
                crate::BitArrayValue::try_from_parts(vec![0xa0], 4).unwrap(),
            ),
            (
                original.bit_slice(32, 0).unwrap(),
                crate::BitArrayValue::from_bytes(Vec::new()),
            ),
        ] {
            let selected = EvaluatedValue::BitArray(EvaluatedBitArray::from_value(selected));
            let expected = EvaluatedValue::BitArray(EvaluatedBitArray::from_value(expected));
            assert!(values_equal(&storage, &selected, &expected));
            assert_eq!(
                value_source_hash(&storage, &selected),
                value_source_hash(&storage, &expected)
            );
        }
        let first = EvaluatedValue::BitArray(EvaluatedBitArray::from_value(
            original.bit_slice(0, 8).unwrap(),
        ));
        let second = EvaluatedValue::BitArray(EvaluatedBitArray::from_value(
            original.bit_slice(8, 8).unwrap(),
        ));
        let short = EvaluatedValue::BitArray(EvaluatedBitArray::from_value(
            original.bit_slice(0, 4).unwrap(),
        ));
        assert!(!values_equal(&storage, &first, &second));
        assert!(!values_equal(&storage, &first, &short));
    }

    #[test]
    fn semantic_value_equality_covers_every_list_and_function_family() {
        fn external_equal(
            context: &crate::host::HostExternalEquality<'_>,
            left: &crate::host::HostStoredValue<num_bigint::BigInt>,
            right: &crate::host::HostStoredValue<num_bigint::BigInt>,
        ) -> bool {
            context.stored_values_equal(left, right)
        }

        fn inspect(
            context: &crate::host::HostExternalInspection<'_>,
            value: &crate::host::HostStoredValue<num_bigint::BigInt>,
        ) -> ecow::EcoString {
            context.inspect_stored_value(value)
        }

        let plan = crate::runtime::plan_src(EVERY_LIST_FAMILY_SOURCE);
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        let execution_int_type = crate::plan::execution::type_::FunctionType::new(
            Vec::new(),
            crate::plan::execution::type_::ValueType::Int,
        );
        let int_function = EvaluatedIntFunction::reference(
            IntFunctionId(0),
            Default::default(),
            execution_int_type.clone(),
        );
        let custom_type = plan.custom_list_function_id(0).type_id().item_type();
        let custom_function = EvaluatedCustomFunction::reference(
            plan.custom_function_id(0),
            Default::default(),
            crate::plan::execution::type_::FunctionType::new(
                Vec::new(),
                crate::plan::execution::type_::ValueType::Custom(custom_type),
            ),
        );
        let constructor_id = plan.custom_constructor_id(0, 0);
        let constructor = plan.custom_constructor(constructor_id);
        let constructor_function = EvaluatedCustomFunction::constructor(
            constructor_id,
            crate::plan::execution::type_::FunctionType::new(
                constructor
                    .fields()
                    .iter()
                    .map(|field| field.type_().clone())
                    .collect(),
                crate::plan::execution::type_::ValueType::Custom(constructor_id.type_id()),
            ),
        );
        let never_function = EvaluatedNeverFunction::reference(
            NeverFunctionId(0),
            Default::default(),
            crate::plan::execution::type_::FunctionType::new(
                Vec::new(),
                crate::plan::execution::type_::ValueType::Parameter(crate::plan::TypeParameterId(
                    0,
                )),
            ),
        );
        let function_pairs = [
            (
                EvaluatedFunctionValue::from(never_function.clone()),
                EvaluatedFunctionValue::from(never_function),
            ),
            (
                EvaluatedFunctionValue::from(int_function.clone()),
                EvaluatedFunctionValue::from(int_function.clone()),
            ),
            (
                EvaluatedFunctionValue::from(EvaluatedFloatFunction::reference(
                    FloatFunctionId(0),
                    Default::default(),
                    crate::plan::execution::type_::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::type_::ValueType::Float,
                    ),
                )),
                EvaluatedFunctionValue::from(EvaluatedFloatFunction::reference(
                    FloatFunctionId(0),
                    Default::default(),
                    crate::plan::execution::type_::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::type_::ValueType::Float,
                    ),
                )),
            ),
            (
                EvaluatedFunctionValue::from(EvaluatedStringFunction::reference(
                    StringFunctionId(0),
                    Default::default(),
                    crate::plan::execution::type_::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::type_::ValueType::String,
                    ),
                )),
                EvaluatedFunctionValue::from(EvaluatedStringFunction::reference(
                    StringFunctionId(0),
                    Default::default(),
                    crate::plan::execution::type_::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::type_::ValueType::String,
                    ),
                )),
            ),
            (
                EvaluatedFunctionValue::from(EvaluatedBitArrayFunction::reference(
                    BitArrayFunctionId(0),
                    Default::default(),
                    crate::plan::execution::type_::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::type_::ValueType::BitArray,
                    ),
                )),
                EvaluatedFunctionValue::from(EvaluatedBitArrayFunction::reference(
                    BitArrayFunctionId(0),
                    Default::default(),
                    crate::plan::execution::type_::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::type_::ValueType::BitArray,
                    ),
                )),
            ),
            (
                EvaluatedFunctionValue::from(EvaluatedUtfCodepointFunction::reference(
                    UtfCodepointFunctionId(0),
                    Default::default(),
                    crate::plan::execution::type_::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::type_::ValueType::UtfCodepoint,
                    ),
                )),
                EvaluatedFunctionValue::from(EvaluatedUtfCodepointFunction::reference(
                    UtfCodepointFunctionId(0),
                    Default::default(),
                    crate::plan::execution::type_::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::type_::ValueType::UtfCodepoint,
                    ),
                )),
            ),
            (
                EvaluatedFunctionValue::from(custom_function.clone()),
                EvaluatedFunctionValue::from(custom_function.clone()),
            ),
            (
                EvaluatedFunctionValue::from(constructor_function.clone()),
                EvaluatedFunctionValue::from(constructor_function.clone()),
            ),
            (
                EvaluatedFunctionValue::from(EvaluatedBoolFunction::reference(
                    BoolFunctionId(0),
                    Default::default(),
                    crate::plan::execution::type_::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::type_::ValueType::Bool,
                    ),
                )),
                EvaluatedFunctionValue::from(EvaluatedBoolFunction::reference(
                    BoolFunctionId(0),
                    Default::default(),
                    crate::plan::execution::type_::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::type_::ValueType::Bool,
                    ),
                )),
            ),
            (
                EvaluatedFunctionValue::from(EvaluatedNilFunction::reference(
                    NilFunctionId(0),
                    Default::default(),
                    crate::plan::execution::type_::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::type_::ValueType::Nil,
                    ),
                )),
                EvaluatedFunctionValue::from(EvaluatedNilFunction::reference(
                    NilFunctionId(0),
                    Default::default(),
                    crate::plan::execution::type_::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::type_::ValueType::Nil,
                    ),
                )),
            ),
            (
                EvaluatedFunctionValue::from(EvaluatedTupleFunction::reference(
                    TupleFunctionId(0),
                    Default::default(),
                    crate::plan::execution::type_::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::type_::ValueType::Tuple(
                            vec![crate::plan::execution::type_::ValueType::Int].into(),
                        ),
                    ),
                )),
                EvaluatedFunctionValue::from(EvaluatedTupleFunction::reference(
                    TupleFunctionId(0),
                    Default::default(),
                    crate::plan::execution::type_::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::type_::ValueType::Tuple(
                            vec![crate::plan::execution::type_::ValueType::Int].into(),
                        ),
                    ),
                )),
            ),
            (
                EvaluatedFunctionValue::from(EvaluatedListFunction::reference(
                    RuntimeListFunctionId::Core(ListFunctionId::Int(plan.int_list_function_id(0))),
                    Default::default(),
                    crate::plan::execution::type_::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::type_::ValueType::List(
                            plan.int_list_function_id(0).type_id().list_type(),
                        ),
                    ),
                )),
                EvaluatedFunctionValue::from(EvaluatedListFunction::reference(
                    RuntimeListFunctionId::Core(ListFunctionId::Int(plan.int_list_function_id(0))),
                    Default::default(),
                    crate::plan::execution::type_::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::type_::ValueType::List(
                            plan.int_list_function_id(0).type_id().list_type(),
                        ),
                    ),
                )),
            ),
            (
                EvaluatedFunctionValue::from(EvaluatedFunctionFunction::Core(
                    EvaluatedFunction::reference(
                        ProfiledFunctionFunctionId::<std::convert::Infallible>::Int(
                            IntFunctionFunctionId(0),
                        ),
                        Default::default(),
                        crate::plan::execution::type_::FunctionType::new(
                            Vec::new(),
                            crate::plan::execution::type_::ValueType::Function(
                                execution_int_type.clone(),
                            ),
                        ),
                    ),
                )),
                EvaluatedFunctionValue::from(EvaluatedFunctionFunction::Core(
                    EvaluatedFunction::reference(
                        ProfiledFunctionFunctionId::<std::convert::Infallible>::Int(
                            IntFunctionFunctionId(0),
                        ),
                        Default::default(),
                        crate::plan::execution::type_::FunctionType::new(
                            Vec::new(),
                            crate::plan::execution::type_::ValueType::Function(
                                execution_int_type.clone(),
                            ),
                        ),
                    ),
                )),
            ),
        ];

        for (left, right) in function_pairs {
            let family = left.kind().family();
            assert_eq!(family, right.kind().family());
            let left = EvaluatedValue::Function(left);
            let right = EvaluatedValue::Function(right);
            assert!(values_equal(state.lists(), &left, &right,));
            assert_eq!(
                value_source_hash(state.lists(), &left),
                value_source_hash(state.lists(), &right),
            );
        }
        assert!(!values_equal(
            state.lists(),
            &EvaluatedValue::Function(EvaluatedFunctionValue::from(custom_function)),
            &EvaluatedValue::Function(EvaluatedFunctionValue::from(constructor_function)),
        ));
        assert!(!values_equal(
            state.lists(),
            &EvaluatedValue::Function(EvaluatedFunctionValue::from(int_function.clone())),
            &EvaluatedValue::Function(EvaluatedFunctionValue::from(
                EvaluatedFloatFunction::reference(
                    FloatFunctionId(0),
                    Default::default(),
                    crate::plan::execution::type_::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::type_::ValueType::Float,
                    ),
                ),
            )),
        ));

        let int_lists = (
            state
                .lists_mut()
                .int(plan.int_list_function_id(0).type_id(), vec![1.into()]),
            state
                .lists_mut()
                .int(plan.int_list_function_id(0).type_id(), vec![1.into()]),
        );
        let string_lists = (
            state.lists_mut().string(
                plan.string_list_function_id(0).type_id(),
                vec!["one".into()],
            ),
            state.lists_mut().string(
                plan.string_list_function_id(0).type_id(),
                vec!["one".into()],
            ),
        );
        let float_lists = (
            state
                .lists_mut()
                .float(plan.float_list_function_id(0).type_id(), vec![1.5]),
            state
                .lists_mut()
                .float(plan.float_list_function_id(0).type_id(), vec![1.5]),
        );
        let utf_codepoint_lists = (
            state
                .lists_mut()
                .utf_codepoint(plan.utf_codepoint_list_function_id(0).type_id(), vec!['a']),
            state
                .lists_mut()
                .utf_codepoint(plan.utf_codepoint_list_function_id(0).type_id(), vec!['a']),
        );
        let bool_lists = (
            state
                .lists_mut()
                .bool(plan.bool_list_function_id(0).type_id(), vec![true]),
            state
                .lists_mut()
                .bool(plan.bool_list_function_id(0).type_id(), vec![true]),
        );
        let nil_lists = (
            state
                .lists_mut()
                .nil(plan.nil_list_function_id(0).type_id(), 1),
            state
                .lists_mut()
                .nil(plan.nil_list_function_id(0).type_id(), 1),
        );
        let tuple_lists = (
            state.lists_mut().tuple(
                plan.tuple_list_function_id(0).type_id(),
                vec![vec![EvaluatedValue::Int(1.into())]],
            ),
            state.lists_mut().tuple(
                plan.tuple_list_function_id(0).type_id(),
                vec![vec![EvaluatedValue::Int(1.into())]],
            ),
        );
        let left_child = state
            .lists_mut()
            .int(plan.int_list_function_id(0).type_id(), vec![1.into()]);
        let right_child = state
            .lists_mut()
            .int(plan.int_list_function_id(0).type_id(), vec![1.into()]);
        let nested_lists = (
            state.lists_mut().list(
                plan.list_list_function_id(0).type_id(),
                vec![left_child.into()],
            ),
            state.lists_mut().list(
                plan.list_list_function_id(0).type_id(),
                vec![right_child.into()],
            ),
        );
        let function_lists = (
            state.lists_mut().function(
                plan.function_list_function_id(0).type_id(),
                vec![EvaluatedFunctionValue::from(int_function.clone())],
            ),
            state.lists_mut().function(
                plan.function_list_function_id(0).type_id(),
                vec![EvaluatedFunctionValue::from(int_function.clone())],
            ),
        );
        let list_pairs = [
            (
                ListValueId::Int(int_lists.0.clone()),
                ListValueId::Int(int_lists.1.clone()),
            ),
            (
                ListValueId::String(string_lists.0.clone()),
                ListValueId::String(string_lists.1.clone()),
            ),
            (
                ListValueId::UtfCodepoint(utf_codepoint_lists.0.clone()),
                ListValueId::UtfCodepoint(utf_codepoint_lists.1.clone()),
            ),
            (
                ListValueId::Float(float_lists.0.clone()),
                ListValueId::Float(float_lists.1.clone()),
            ),
            (
                ListValueId::Bool(bool_lists.0.clone()),
                ListValueId::Bool(bool_lists.1.clone()),
            ),
            (
                ListValueId::Nil(nil_lists.0.clone()),
                ListValueId::Nil(nil_lists.1.clone()),
            ),
            (
                ListValueId::Tuple(tuple_lists.0.clone()),
                ListValueId::Tuple(tuple_lists.1.clone()),
            ),
            (
                ListValueId::List(nested_lists.0.clone()),
                ListValueId::List(nested_lists.1.clone()),
            ),
            (
                ListValueId::Function(function_lists.0.clone()),
                ListValueId::Function(function_lists.1.clone()),
            ),
        ];

        for (left, right) in list_pairs {
            let left = EvaluatedValue::from(left);
            let right = EvaluatedValue::from(right);
            assert!(values_equal(state.lists(), &left, &right,));
            assert_eq!(
                value_source_hash(state.lists(), &left),
                value_source_hash(state.lists(), &right),
            );
        }

        let bit_array = EvaluatedBitArray::new(bitvec::bitvec![u8, Msb0; 1, 0, 1]);
        let scalar_and_compound_pairs: Vec<(EvaluatedValue, EvaluatedValue)> = vec![
            (EvaluatedValue::Int(1.into()), EvaluatedValue::Int(1.into())),
            (EvaluatedValue::Float(0.0), EvaluatedValue::Float(-0.0)),
            (EvaluatedValue::Float(1.5), EvaluatedValue::Float(1.5)),
            (
                EvaluatedValue::String("one".into()),
                EvaluatedValue::String("one".into()),
            ),
            (
                EvaluatedValue::BitArray(bit_array.clone()),
                EvaluatedValue::BitArray(bit_array),
            ),
            (
                EvaluatedValue::UtfCodepoint('A'),
                EvaluatedValue::UtfCodepoint('A'),
            ),
            (EvaluatedValue::Bool(true), EvaluatedValue::Bool(true)),
            (EvaluatedValue::Nil, EvaluatedValue::Nil),
            (
                EvaluatedValue::Tuple(vec![EvaluatedValue::Int(1.into())]),
                EvaluatedValue::Tuple(vec![EvaluatedValue::Int(1.into())]),
            ),
            (
                EvaluatedValue::ParameterList(ParameterListValueId::new(
                    plan.parameter_list_function_id(0).type_id(),
                )),
                EvaluatedValue::ParameterList(ParameterListValueId::new(
                    plan.parameter_list_function_id(0).type_id(),
                )),
            ),
            (
                EvaluatedValue::Custom(EvaluatedCustomValue::from_fields(
                    constructor_id,
                    vec![EvaluatedValue::Int(1.into())].into_boxed_slice(),
                )),
                EvaluatedValue::Custom(EvaluatedCustomValue::from_fields(
                    constructor_id,
                    vec![EvaluatedValue::Int(1.into())].into_boxed_slice(),
                )),
            ),
        ];
        for (left, right) in scalar_and_compound_pairs {
            assert!(values_equal(state.lists(), &left, &right));
            assert_eq!(
                value_source_hash(state.lists(), &left),
                value_source_hash(state.lists(), &right),
            );
        }

        let external_store = crate::host::HostExternalStore::default();
        let first = external_store.insert(
            crate::host::HostStoredValue::<num_bigint::BigInt>::new(
                crate::runtime::StoredRuntimeValue::test_int(7.into()),
            ),
            external_equal,
            |_, _| 41,
            inspect,
            |_| None,
        );
        let equal = external_store.insert(
            crate::host::HostStoredValue::<num_bigint::BigInt>::new(
                crate::runtime::StoredRuntimeValue::test_int(7.into()),
            ),
            external_equal,
            |_, _| 41,
            inspect,
            |_| None,
        );
        let collision = external_store.insert(
            crate::host::HostStoredValue::<num_bigint::BigInt>::new(
                crate::runtime::StoredRuntimeValue::test_int(8.into()),
            ),
            external_equal,
            |_, _| 41,
            inspect,
            |_| None,
        );
        let stored_inspect = |_: &crate::runtime::RetainedValueRef| "stored".into();
        let inspection = crate::host::RetainedValueInspection::new(&stored_inspect);
        assert_eq!(first.inspection(&inspection), "stored");
        let external_type = crate::plan::execution::type_::ExternalTypeId::new(0);
        let first: EvaluatedValue =
            EvaluatedValue::External(EvaluatedExternalValue::new(external_type, first));
        let equal: EvaluatedValue =
            EvaluatedValue::External(EvaluatedExternalValue::new(external_type, equal));
        let collision: EvaluatedValue =
            EvaluatedValue::External(EvaluatedExternalValue::new(external_type, collision));
        assert!(values_equal(state.lists(), &first, &equal));
        assert_eq!(
            value_source_hash(state.lists(), &first),
            value_source_hash(state.lists(), &equal),
        );
        assert!(!values_equal(state.lists(), &first, &collision));
        assert_eq!(
            value_source_hash(state.lists(), &first),
            value_source_hash(state.lists(), &collision),
        );
        assert!(!values_equal(
            state.lists(),
            &EvaluatedValue::from(ListValueId::Int(int_lists.0)),
            &EvaluatedValue::from(ListValueId::String(string_lists.0,)),
        ));
        assert!(values_equal(
            state.lists(),
            &EvaluatedValue::Tuple(vec![EvaluatedValue::Int(1.into(),)]),
            &EvaluatedValue::Tuple(vec![EvaluatedValue::Int(1.into(),)]),
        ));
        assert!(!values_equal(
            state.lists(),
            &EvaluatedValue::Tuple(vec![EvaluatedValue::Int(1.into(),)]),
            &EvaluatedValue::Tuple(Vec::new()),
        ));
        assert!(!values_equal(
            state.lists(),
            &EvaluatedValue::Int(1.into()),
            &EvaluatedValue::String("one".into()),
        ));
    }
}
