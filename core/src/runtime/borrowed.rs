use crate::runtime::evaluated::{EvaluatedBitArray, EvaluatedCustomValue, EvaluatedValue};
use crate::runtime::state::list::{ListValueId, ParameterListValueId, StoredListValueId};
use crate::runtime::{EvaluatedExternalValue, StoredRuntimeValue};
use ecow::EcoString;
use num_bigint::BigInt;
use std::slice;

/// A borrowed row in the existing value-family layout. Construction routes the
/// one root value; tuple/custom fields and list items are not traversed here.
/// Typed provider and embedding readers select their proven column. Opaque
/// functions remain in the stored owner; this view grants no call permission.
pub(crate) struct BorrowedValue<'value> {
    ints: &'value [BigInt],
    floats: &'value [f64],
    strings: &'value [EcoString],
    bit_arrays: &'value [EvaluatedBitArray],
    utf_codepoints: &'value [char],
    bools: &'value [bool],
    customs: &'value [EvaluatedCustomValue],
    tuples: &'value [Vec<EvaluatedValue>],
    lists: &'value [StoredListValueId],
    parameter_lists: &'value [ParameterListValueId],
    externals: &'value [EvaluatedExternalValue],
}

impl<'value> BorrowedValue<'value> {
    pub(crate) fn from_stored(value: &'value StoredRuntimeValue) -> Self {
        Self::from_value(value.value())
    }

    pub(in crate::runtime) fn from_value(value: &'value EvaluatedValue) -> Self {
        let mut row = Self::empty();
        match value {
            EvaluatedValue::Int(value) => row.ints = slice::from_ref(value),
            EvaluatedValue::Float(value) => row.floats = slice::from_ref(value),
            EvaluatedValue::String(value) => row.strings = slice::from_ref(value),
            EvaluatedValue::BitArray(value) => row.bit_arrays = slice::from_ref(value),
            EvaluatedValue::UtfCodepoint(value) => row.utf_codepoints = slice::from_ref(value),
            EvaluatedValue::Bool(value) => row.bools = slice::from_ref(value),
            EvaluatedValue::Custom(value) => row.customs = slice::from_ref(value),
            EvaluatedValue::Tuple(value) => row.tuples = slice::from_ref(value),
            EvaluatedValue::List(value) => row.lists = slice::from_ref(value),
            EvaluatedValue::External(value) => row.externals = slice::from_ref(value),
            EvaluatedValue::ParameterList(value) => row.parameter_lists = slice::from_ref(value),
            EvaluatedValue::Function(_) | EvaluatedValue::Nil => {}
        }
        row
    }

    pub(crate) fn int(&self) -> &'value BigInt {
        &self.ints[0]
    }
    pub(crate) fn float(&self) -> f64 {
        self.floats[0]
    }
    pub(crate) fn string(&self) -> &'value EcoString {
        &self.strings[0]
    }
    pub(crate) fn bit_array(&self) -> &'value crate::BitArrayValue {
        self.bit_arrays[0].as_value()
    }
    pub(crate) fn utf_codepoint(&self) -> char {
        self.utf_codepoints[0]
    }
    pub(crate) fn bool(&self) -> bool {
        self.bools[0]
    }
    pub(crate) fn variant(&self) -> usize {
        self.customs[0].constructor().index()
    }

    pub(crate) fn tuple_item(&self, index: usize) -> Self {
        Self::from_value(&self.tuples[0][index])
    }

    pub(crate) fn custom_field(&self, index: usize) -> Self {
        Self::from_value(&self.customs[0].fields()[index])
    }

    pub(crate) fn retained_custom(&self) -> crate::runtime::EmbeddingCustomInput {
        crate::runtime::EmbeddingCustomInput::retained(self.customs[0].clone())
    }

    pub(in crate::runtime) fn stored_list(&self) -> &'value StoredListValueId {
        &self.lists[0]
    }

    pub(in crate::runtime) fn list(&self) -> ListValueId {
        if let Some(value) = self.parameter_lists.first() {
            ListValueId::Parameter(*value)
        } else {
            self.stored_list().clone().into()
        }
    }

    pub(crate) fn external(&self) -> &'value EvaluatedExternalValue {
        &self.externals[0]
    }

    fn empty() -> Self {
        Self {
            ints: &[],
            floats: &[],
            strings: &[],
            bit_arrays: &[],
            utf_codepoints: &[],
            bools: &[],
            customs: &[],
            tuples: &[],
            lists: &[],
            parameter_lists: &[],
            externals: &[],
        }
    }
}

impl BorrowedValue<'_> {
    pub(super) fn read_list_item<Output>(
        value: &StoredListValueId,
        index: usize,
        read: impl FnOnce(BorrowedValue<'_>) -> Output,
    ) -> Option<Output> {
        use crate::runtime::RuntimeListStorage;
        let handle = value.clone().into_core();
        let storage = RuntimeListStorage::from_handle(&handle);
        macro_rules! item {
            ($value:expr, $read:ident, $field:ident) => {{
                let values = storage.$read($value);
                values.get(index).map(|value| {
                    let mut row = BorrowedValue::empty();
                    row.$field = slice::from_ref(value);
                    read(row)
                })
            }};
        }
        match value {
            StoredListValueId::Int(value) => item!(value, int_values, ints),
            StoredListValueId::Float(value) => item!(value, float_values, floats),
            StoredListValueId::String(value) => item!(value, string_values, strings),
            StoredListValueId::BitArray(value) => item!(value, bit_array_values, bit_arrays),
            StoredListValueId::UtfCodepoint(value) => {
                item!(value, utf_codepoint_values, utf_codepoints)
            }
            StoredListValueId::Bool(value) => item!(value, bool_values, bools),
            StoredListValueId::Tuple(value) => item!(value, tuple_values, tuples),
            StoredListValueId::Custom(value) => item!(value, custom_values, customs),
            StoredListValueId::External(value) => item!(value, external_values, externals),
            StoredListValueId::List(value) => item!(value, list_values, lists),
            StoredListValueId::Nil(_)
            | StoredListValueId::Function(_)
            | StoredListValueId::ParameterList(_) => (index
                < storage.list_len(&value.clone().into()))
            .then(|| read(BorrowedValue::empty())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BorrowedValue;
    use crate::runtime::evaluated::EvaluatedValue;
    use crate::runtime::state::list::StoredListValueId;
    use crate::runtime::{RuntimeListStorage, StoredRuntimeValue};
    use num_bigint::BigInt;

    struct Profile;

    impl crate::HostProfile for Profile {
        type RunState = ();
        type ExternalStores = crate::host::HostFutureStore;
    }

    impl crate::host::HostWorkProfile for Profile {
        type Work = crate::work_fixture::WorkComponent;
    }
    impl crate::host::HostComponentProfile<crate::work_fixture::WorkComponent> for Profile {
        fn component_stores(stores: &Self::ExternalStores) -> &Self::ExternalStores {
            stores
        }

        fn component_state(state: &mut ()) -> &mut () {
            state
        }
    }

    #[test]
    fn source_list_families_select_only_their_borrowed_column() {
        use crate::embedding::{FunctionDeclaration, HostedModuleBuilder};
        use crate::host::{HostListType, HostProviderModule, HostProviderSet, HostTypeParameter};
        use crate::work_fixture::WorkComponent;
        use crate::{ModuleSource, PackageSource};

        let mut providers = WorkComponent::providers::<Profile>().expect("Future module");
        providers.push(HostProviderModule::new("application", "library")
            .expect("native module")
            .with_scoped_function::<WorkComponent, (HostListType<HostTypeParameter<0>>, ecow::EcoString), (), _>("check", check_list)
            .expect("generic list observer"));
        let program = crate::frontend::compile_typed_host_program(
            "application",
            "library",
            [
                PackageSource::new(
                    "work_fixture",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "fixture/work",
                        "src/fixture/work.gleam",
                        WorkComponent::SOURCE,
                    )],
                ),
                PackageSource::new(
                    "application",
                    ["work_fixture"],
                    [ModuleSource::new(
                        "library",
                        "src/library.gleam",
                        r#"
import fixture/work as future
@external(erlang, "native", "check")
fn check(values: List(a), column: String) -> Nil
pub fn run() {
  check([1], "int")
  check([1.5], "float")
  check(["hello"], "string")
  check([<<1:3>>], "bit_array")
  let assert <<codepoint:utf8_codepoint>> = <<"x":utf8>>
  check([codepoint], "utf_codepoint")
  check([True], "bool")
  check([#(1, "hello")], "tuple")
  check([Ok(1)], "custom")
  check([future.ready(1)], "external")
  check([[1]], "list")
  check([Nil], "")
  check([fn(value: Int) { value + 1 }], "")
  check([[]], "")
  check([], "parameter_list")
}
"#,
                    )],
                ),
            ],
            HostProviderSet::from_providers(providers).expect("providers"),
        )
        .expect("ordinary typed source");
        let (bindings, run) = HostedModuleBuilder::new(program)
            .expect("plan")
            .function(FunctionDeclaration::<(), ()>::new("run"))
            .expect("entry");
        let mut module = bindings.seal().expect("sealed entry");
        let execution_host = crate::execution_fixture::TestHost::default();
        let mut state = ();
        let mut echo = drop;
        execution_host
            .block_on(module.with_execution(
                &execution_host,
                &mut state,
                &mut echo,
                async |scope| {
                    scope.call(&run, ()).await.expect("column assertions");
                },
            ))
            .expect("hosted source execution");
    }

    fn check_list<'call>(
        mut call: crate::host::HostCall<'call, Profile, crate::work_fixture::WorkComponent, ()>,
        values: crate::host::HostList<'call, crate::host::HostTypeParameter<0>>,
        expected: ecow::EcoString,
    ) -> Result<crate::host::HostCallCompletion<'call, ()>, crate::HostCallError> {
        assert_eq!(call.state(), &());
        let stored = call
            .retain_value::<crate::host::HostListType<crate::host::HostTypeParameter<0>>>(values);
        let row = BorrowedValue::from_stored(&stored);
        assert_eq!(row.list(), call.retain_list_value(values).handle());
        let expected = if expected.is_empty() {
            Vec::new()
        } else {
            vec![expected.as_str()]
        };
        if call.list_len(values) == 0 {
            assert_eq!(populated_columns(row), expected);
        } else {
            assert_eq!(
                BorrowedValue::read_list_item(row.stored_list(), 0, populated_columns),
                Some(expected)
            );
            assert_eq!(
                BorrowedValue::read_list_item(row.stored_list(), 1, populated_columns),
                None
            );
        }
        Ok(call.return_value(()))
    }

    fn populated_columns(row: BorrowedValue<'_>) -> Vec<&'static str> {
        [
            ("int", row.ints.len()),
            ("float", row.floats.len()),
            ("string", row.strings.len()),
            ("bit_array", row.bit_arrays.len()),
            ("utf_codepoint", row.utf_codepoints.len()),
            ("bool", row.bools.len()),
            ("custom", row.customs.len()),
            ("tuple", row.tuples.len()),
            ("list", row.lists.len()),
            ("parameter_list", row.parameter_lists.len()),
            ("external", row.externals.len()),
        ]
        .into_iter()
        .filter_map(|(name, length)| (length == 1).then_some(name))
        .collect()
    }

    #[test]
    fn recursive_reads_borrow_the_original_scalar_storage() {
        use crate::plan::execution::runtime::RuntimeExecutionPlan;
        let plan = crate::runtime::plan_src("pub fn main() { Nil }");
        let number = BigInt::from(1u64) << 256;
        let stored = StoredRuntimeValue::new(
            EvaluatedValue::Tuple(vec![
                EvaluatedValue::Int(number),
                EvaluatedValue::String("a string longer than the inline storage".into()),
                EvaluatedValue::Float(3.5),
                EvaluatedValue::Bool(true),
                EvaluatedValue::UtfCodepoint('x'),
                EvaluatedValue::Nil,
            ]),
            plan.value_metadata(),
        );
        let first = BorrowedValue::from_stored(&stored);
        let second = BorrowedValue::from_stored(&stored);
        assert!(std::ptr::eq(
            first.tuple_item(0).int(),
            second.tuple_item(0).int()
        ));
        assert_eq!(first.tuple_item(0).int(), &(BigInt::from(1u64) << 256));
        assert!(std::ptr::eq(
            first.tuple_item(1).string(),
            second.tuple_item(1).string()
        ));
        assert_eq!(first.tuple_item(2).float(), 3.5);
        assert!(first.tuple_item(3).bool());
        assert_eq!(first.tuple_item(4).utf_codepoint(), 'x');
        assert!(first.tuple_item(5).ints.is_empty());
    }

    #[test]
    fn list_item_reads_borrow_the_persistent_allocation_without_copying_items() {
        let plan = crate::runtime::plan_src("pub fn main() -> List(Int) { [1] }");
        let storage = RuntimeListStorage::default();
        let handle = storage.int(
            plan.int_list_function_id(0).type_id(),
            vec![BigInt::from(1u64) << 256],
        );
        let values = storage.int_values(&handle);
        let retained: StoredListValueId = handle.into();
        for _ in 0..2 {
            assert_eq!(
                BorrowedValue::read_list_item(&retained, 0, |value| {
                    assert!(std::ptr::eq(value.int(), &values[0]));
                    value.int().bits()
                }),
                Some(257)
            );
        }
        assert_eq!(BorrowedValue::read_list_item(&retained, 1, |_| true), None);
        drop(storage);
        assert_eq!(
            BorrowedValue::read_list_item(&retained, 0, |value| value.int().bits()),
            Some(257)
        );
    }
}
